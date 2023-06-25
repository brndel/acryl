
mod text;
mod graphics_state;
mod path_construction;
mod path_painting;
mod color;

use std::fmt;
use std::fmt::Write;

pub use text::TextStream;
pub use text::TextStreamElement;

use crate::render::PdfObj;

pub type StreamInstruction = (Vec<PdfObj>, &'static str);

pub trait Stream<E: StreamElement<Self>>: Sized + From<E> + Into<Vec<E>> {
    fn get_start() -> &'static str;
    fn get_end() -> &'static str;

    fn push(&mut self, element: E);

    fn then(mut self, element: E) -> Self {
        self.push(element);
        self
    }

    fn render(self) -> Result<String, fmt::Error> {
        let mut s = String::new();

        writeln!(s, "{}", Self::get_start())?;

        for element in self.into() {
            element.render(&mut s)?;
        }

        writeln!(s, "{}", Self::get_end())?;

        Ok(s)
    }
}

pub trait StreamElement<S: Stream<Self>>: Sized + Into<StreamInstruction> {
    fn then(self, element: Self) -> S {
        S::from(self).then(element)
    }

    fn render(self, f: &mut dyn fmt::Write) -> fmt::Result {
        let (values, operator) = self.into();

        for value in values {
            value.render(f)?;
            write!(f, " ")?;
        }

        writeln!(f, "{}", operator)
    }
}
