use lexer::Lexer;
use parser::Parser;
use std::{env, path::Path, process::exit};

mod lexer;
mod parser;
mod tokens;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filepath = &args[1];

    let exists = Path::new(&filepath).exists();
    if !exists {
        println!("looks like {filepath} does not exist");
        exit(1);
    }

    if !filepath.ends_with(".sk") {
        println!("invalid file format. expected \".sk\"");
        exit(1);
    }

    let mut lexer = Lexer::new(filepath);
    let tokens = lexer.parse();

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    println!("ast: {:#?}", ast);
}
