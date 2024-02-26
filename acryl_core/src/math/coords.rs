use core::fmt::Debug;
use core::hash::Hash;

pub type DefaultCoords = ();

pub trait Coords:
    Clone + Copy + Debug + Default + PartialEq + Eq + PartialOrd + Ord + Hash
{
    fn name() -> &'static str;
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PdfCoords;

impl Coords for PdfCoords {
    fn name() -> &'static str {
        "Pdf"
    }
}

pub type AcrylCoords = ();

impl Coords for AcrylCoords {
    fn name() -> &'static str {
        "Acryl"
    }
}
