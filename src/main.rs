use std::{fs::File, io::Read};

use parser::lexer::Token;

use crate::parser::lexer::Lexer;

mod parser;
mod pdf;

fn main() {
  let file_path = "hello_world.acryl";

  println!("{}", "-".repeat(10));
  println!("reading file {}:", file_path);
  println!();

  let mut file = File::open(file_path).expect("Can't open file");
  let mut file_content = String::new();

  file
    .read_to_string(&mut file_content)
    .expect("Can't read file");

  let mut lexer = Lexer::new(file_content.as_str());

  loop {
    match lexer.next_token() {
      Ok(Token::EOF) => {
        println!("<EOF>");
        break;
      }
      Ok(token) => {
        println!("{:?}", token);
      }
      Err(error) => {
        println!("{:?}", error);
      }
    };
  }
}
