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