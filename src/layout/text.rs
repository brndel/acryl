use acryl_core::{unit::Pt, Vector2, Area};
use acryl_pdf::{
    font::{Font, FontRef},
    stream::Streambuilder,
};

use super::LayoutElement;

pub struct TextElement {
    words: Vec<Word>,
    font: FontRef,
    font_size: f64,
}

struct Word {
    text: String,
    width: Pt,
}

impl Word {
    fn new(text: &str, font: &Font, font_size: f64) -> Self {
        let width = font.measure_text(text, font_size);
        Self {
            text: text.to_owned(),
            width,
        }
    }
}

struct Line<'a> {
    words: Vec<&'a Word>,
    width: Pt,
    space_width: Pt,
}

impl<'a> Line<'a> {
    fn new(space_width: Pt) -> Self {
        Self {
            words: Default::default(),
            width: Default::default(),
            space_width,
        }
    }

    fn add_word(&mut self, word: &'a Word) {
        self.width = self.width_with_word(word);

        self.words.push(word);
    }

    fn width_with_word(&self, word: &Word) -> Pt {
        let mut width = self.width;
        if !self.words.is_empty() {
            width += self.space_width;
        }

        width += word.width;

        width
    }
}

impl LayoutElement for TextElement {
    fn get_min_size(&self, max_size: Vector2<Pt>) -> Vector2<Pt> {
        let lines = self.layout_words(max_size.x);

        Vector2 {
            x: max_size.x,
            y: self.font.font().metrics().height(self.font_size) * (lines.len() as f64).into(),
        }
    }

    fn render(&self, area: Area<Pt>, builder: &mut Streambuilder) {
        let mut text_builder = builder.text(&self.font, self.font_size);
        let font = self.font.font();

        text_builder.set_position(area.position.clone());
        // Todo: figure out how to get the real leading of a font
        let leading = font.metrics().height(self.font_size);
        text_builder.set_leading(leading);

        let lines = self.layout_words(area.size.x);

        for line in lines {
            let mut first = true;
            for word in line.words {
                if first {
                    first = false;
                } else {
                    text_builder.draw_text(" ");
                }
                text_builder.draw_text(&word.text);
            }
            text_builder.next_line();
        }
    }
}

impl TextElement {
    pub fn new(font: &FontRef, font_size: f64) -> Self {
        Self {
            words: Vec::new(),
            font: font.to_owned(),
            font_size,
        }
    }

    pub fn add_word(&mut self, word: &str) {
        self.words.push(Word::new(&word, self.font.font(), self.font_size));
    }

    fn layout_words(&self, max_width: Pt) -> Vec<Line> {
        let space_width = self.font.font().measure_text(" ", self.font_size);
        let mut lines = Vec::new();

        lines.push(Line::new(space_width));

        for word in &self.words {
            if let Some(line) = lines.last_mut() {
                if line.width_with_word(word) < max_width {
                    line.add_word(word);
                } else {
                    let mut line = Line::new(space_width);
                    line.add_word(word);
                    lines.push(line);
                }
            }
        }

        lines
    }
}
