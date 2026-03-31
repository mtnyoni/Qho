mod lexer;
mod parser;
mod interpreter;
mod stdlib;

use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: qho <file.ndebele>");
        std::process::exit(1);
    }

    let path = &args[1];

    if !path.ends_with(".ndebele") {
        eprintln!("Error: file must have a .ndebele extension");
        std::process::exit(1);
    }

    let source = fs::read_to_string(path)
        .unwrap_or_else(|e| {
            eprintln!("Error reading file '{}': {}", path, e);
            std::process::exit(1);
        });

    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize();

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    let mut interpreter = Interpreter::new();
    interpreter.run(&ast);
}
