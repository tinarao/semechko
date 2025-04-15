use codegen::CodeGenerator;
use lexer::Lexer;
use parser::Parser;
use std::{env, fs, path::Path, process::exit};

mod codegen;
mod lexer;
mod parser;
mod tokens;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filepath = &args[1];
    let mut output_filename = String::from(&args[2]);

    if !output_filename.ends_with(".rs") {
        output_filename.push_str(".rs");
    }

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

    let mut codegen = CodeGenerator::new();
    let code = codegen.generate(ast);

    match fs::write(&output_filename, code) {
        Ok(()) => println!("Transpiled .sk code to Rust. Saved at {}", output_filename),
        Err(e) => panic!("Failed to save result code: {}", e),
    };
}
