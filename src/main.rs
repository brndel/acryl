mod parser;

use std::fs::read_to_string;

use chumsky::{span::SimpleSpan};

use crate::parser::{parse};

fn main() {
    let file_path = "examples/hello_world.acryl";

    let file_content = read_to_string(file_path).expect("Can't read file");

    println!("reading file {}:", file_path);
    println!("---\n{}\n--- length: {}\n", file_content, file_content.len());

    let result = parse(&file_content);

    // println!("{:?}", result);

    for (expr, _) in result {
        println!("{:?}\n{}\n", expr, expr)
    }


    // for err in result.errors() {
    //     let span = err.span();

    //     let (lines, position) = get_line_position(&file_content, &span);

    //     println!("Error at col {}: {}\n{}\n{}↑ Here", position, err.reason(), lines, " ".repeat(position));
    // }
}

