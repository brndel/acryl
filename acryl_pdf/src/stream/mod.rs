mod builder;
mod color;
mod graphics_state;
mod path_construction;
mod path_painting;
mod text;

use std::io;
use std::io::Write;

use crate::render::PdfObj;

pub use builder::Streambuilder;

pub use color::Color;

pub type StreamInstruction = (Vec<PdfObj>, &'static str);

pub struct Stream {
    instructions: Vec<StreamInstruction>,
}

impl Stream {
    pub fn new(instructions: Vec<StreamInstruction>) -> Self {
        Self { instructions }
    }
    pub fn render(self) -> io::Result<Vec<u8>> {
        let mut f = Vec::new();

        for (values, operator) in self.instructions {
            for value in values {
                value.render(&mut f)?;
                write!(f, " ")?;
            }
            writeln!(f, "{}", operator)?;
        }

        Ok(f)
    }
}

// trait Stream<E: StreamElement<Self>>: Sized + From<E> + Into<Vec<E>> {
//     fn get_start() -> &'static str;
//     fn get_end() -> &'static str;

//     fn push(&mut self, element: E);

//     fn then(mut self, element: E) -> Self {
//         self.push(element);
//         self
//     }

//     fn render(self) -> Result<String, fmt::Error> {
//         let mut s = String::new();

//         writeln!(s, "{}", Self::get_start())?;

//         for element in self.into() {
//             element.render(&mut s)?;
//         }

//         writeln!(s, "{}", Self::get_end())?;

//         Ok(s)
//     }
// }

// pub trait StreamElement<S: Stream<Self>>: Sized + Into<StreamInstruction> {
//     fn then(self, element: Self) -> S {
//         S::from(self).then(element)
//     }

//     fn render(self, f: &mut dyn fmt::Write) -> fmt::Result {
//         let (values, operator) = self.into();

//         for value in values {
//             value.render(f)?;
//             write!(f, " ")?;
//         }

//         writeln!(f, "{}", operator)
//     }
// }
