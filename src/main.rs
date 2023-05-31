mod ast;
mod evaluate;
mod parser;

use std::fs::read_to_string;

use ast::Instr;
use parser::Span;

use crate::{
    evaluate::{stack::StackStorage, Eval, evaluator::Evaluator},
    parser::parse,
};

fn main() {
    let file_path = "examples/hello_world.acryl";

    let file_content = read_to_string(file_path).expect("Can't read file");

    println!("reading file {}:", file_path);
    println!(
        "---\n{}\n--- length: {}\n",
        file_content,
        file_content.len()
    );

    let result = parse(&file_content);

    let mut evaluator = Evaluator::new(&result);

    evaluator.eval();


    // println!("{:?}", result);
    // let mut storage = StackStorage::new();

    // for (instr, _) in &result {
    //     println!("{}; \x1b[1;30m// {:?}\x1b[0m", instr, instr);

    //     let value = instr.eval(&mut storage);

    //     println!("-> {:?}", value);
    // }

    // for err in result.errors() {
    //     let span = err.span();

    //     let (lines, position) = get_line_position(&file_content, &span);

    //     println!("Error at col {}: {}\n{}\n{}↑ Here", position, err.reason(), lines, " ".repeat(position));
    // }
}
