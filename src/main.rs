mod parser;
mod evaluate;
mod ast;

use std::fs::read_to_string;

use crate::{parser::{parse}, evaluate::stack::StackStorage};

fn main() {
    let file_path = "examples/hello_world.acryl";

    let file_content = read_to_string(file_path).expect("Can't read file");

    println!("reading file {}:", file_path);
    println!("---\n{}\n--- length: {}\n", file_content, file_content.len());

    let result = parse(&file_content);

    // println!("{:?}", result);

    let storage = StackStorage::new();

    for (expr, _) in result {
        println!("{:?}\n{}\n", expr, expr)
    }


    // for err in result.errors() {
    //     let span = err.span();

    //     let (lines, position) = get_line_position(&file_content, &span);

    //     println!("Error at col {}: {}\n{}\n{}↑ Here", position, err.reason(), lines, " ".repeat(position));
    // }
}

