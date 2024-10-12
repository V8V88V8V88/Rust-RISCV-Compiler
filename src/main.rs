use std::env;
use std::fs;

mod ast;
mod code_gen;
mod lexer;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }

    let input = fs::read_to_string(&args[1]).expect("Failed to read input file");

    let tokens = lexer::tokenize(&input);
    let ast = parser::parse(tokens);
    let assembly = code_gen::generate(ast);

    println!("{}", assembly);
}
