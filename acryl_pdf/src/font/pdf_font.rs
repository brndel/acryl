use owned_ttf_parser::{name_id::POST_SCRIPT_NAME, Face};

use crate::{data::{PdfObj, PdfObjRef}, pdf_dict, write::WritePdf};

use super::{cmap::CMap, Font};


pub struct PdfFont<'a> {
    data: &'a [u8],
    name: String,
    ascender: i16,
    descender: i16,
    line_gap: i16,
    capital_height: i16,
    cmap: CMap,
}

impl<'a> PdfFont<'a> {
    pub fn new<'b>(face: &'a Face, used_chars: impl Iterator<Item = &'b char>) -> Self {
        let data = face.raw_face().data;
        let name = face_name(face);

        let ascender = face.ascender();
        let descender = face.descender();
        let line_gap = face.line_gap();
        let capital_height = face.capital_height().unwrap_or_default();

        let cmap = CMap::new(face, used_chars);

        Self {
            data,
            name,
            ascender,
            descender,
            line_gap,
            capital_height,
            cmap
        }
    }
}

fn face_name(face: &Face) -> String {
    face
        .names()
        .into_iter()
        .find(|name| name.name_id == POST_SCRIPT_NAME && name.is_unicode())
        .map(|name| name.to_string())
        .flatten()
        .unwrap_or_default()
}

impl<D> WritePdf<D> for &PdfFont<'_> {
    fn write(self, writer: &mut crate::write::PdfWriter<D>) -> PdfObjRef {
        let file_data = self.data.to_owned();

        let font_file = PdfObj::Stream(file_data.into()).add_to(writer);

        let cid_to_unicode_map =
            PdfObj::Stream(self.cmap.create_to_unicode_map(&self.name).into()).add_to(writer);

        let widths = self.cmap.create_width_vector();

        let bbox = self.cmap.create_bbox();

        let descriptor = pdf_dict!(
            "Type" => PdfObj::name("FontDescriptor"),
            "FontName" => PdfObj::name(self.name.to_owned()),
            "Ascent" => self.ascender,
            "Descent" => self.descender,
            "Leading" => self.line_gap,
            "CapHeight" => self.capital_height,
            "ItalicAngle" => 0,
            "FontFile2" => font_file,
            "FontBBox" => bbox,
        )
        .add_to(writer);

        let desc_font = pdf_dict!(
            "Type" => PdfObj::name("Font"),
            "Subtype" => PdfObj::name("CIDFontType2"),
            "BaseFont" => PdfObj::name(self.name.to_owned()),
            "CIDSystemInfo" => pdf_dict!(
                "Registry" => PdfObj::string_literal("Adobe"),
                "Ordering" => PdfObj::string_literal("Identity"),
                "Supplement" => PdfObj::Int(0),
            ),
            "W" => widths,
            "DW" => Font::DEFAULT_GLYPH_UNITS,
            "FontDescriptor" => descriptor,
            "CIDToGIDMap" => PdfObj::name("Identity")
        )
        .add_to(writer);

        let font_dict = pdf_dict!(
            "Type" => PdfObj::name("Font"),
            "Subtype" =>  PdfObj::name("Type0"),
            "BaseFont" => PdfObj::name(self.name.to_owned()),
            "Encoding" => PdfObj::name("Identity-H"),
            "ToUnicode" => cid_to_unicode_map,
            "DescendantFonts" => vec![desc_font],
        );

        writer.add(font_dict)
    }
}
