mod parser;

use std::fs::read_to_string;

use chumsky::Parser;


fn main() {
  let file_path = "hello_world.acryl";

  // println!("{}", "-".repeat(10));
  println!("reading file {}:", file_path);
  println!();

  let file_content = read_to_string(file_path).expect("Can't read file");

  let test_content = "187 + 187 * (1 a+ -1000)";

  let result = parser::parser().parse(test_content);

  println!("{:?}", result);

  if let Some(expr) = result.output() {
    println!("= {}", expr.eval());
  }

  for err in result.errors() {
    println!("{}", err);
  }
}
