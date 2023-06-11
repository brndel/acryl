
mod text_stream;

use std::fmt;
use std::fmt::Write;

pub use text_stream::TextStream;
pub use text_stream::TextStreamElement;

use crate::render::PdfObj;

pub type StreamInstruction = (Vec<PdfObj>, &'static str);

pub trait Stream<E: StreamElement<Self>>: Sized + From<E> + Into<Vec<E>> {
    fn get_name() -> &'static str;

    fn get_start() -> &'static str {
        "B"
    }

    fn get_end() -> &'static str {
        "E"
    }

    fn push(&mut self, element: E);

    fn then(mut self, element: E) -> Self {
        self.push(element);
        self
    }

    fn render(self) -> Result<String, fmt::Error> {
        let mut s = String::new();

        writeln!(s, "{}{}", Self::get_start(), Self::get_name())?;

        for element in self.into() {
            element.render(&mut s)?;
        }

        writeln!(s, "{}{}", Self::get_end(), Self::get_name())?;

        Ok(s)
    }
}

pub trait StreamElement<S: Stream<Self>>: Sized + Into<StreamInstruction> {
    fn get_prefix() -> &'static str {
        S::get_name()
    }

    fn then(self, element: Self) -> S {
        S::from(self).then(element)
    }

    fn render(self, f: &mut dyn fmt::Write) -> fmt::Result {
        let (values, operator) = self.into();

        for value in values {
            value.render(f)?;
        }

        writeln!(f, "{}{}", Self::get_prefix(), operator)
    }
}
