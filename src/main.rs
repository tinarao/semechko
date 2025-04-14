use std::{env, path::Path, process::exit};

use lexer::Lexer;
mod lexer;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filepath = &args[1];

    let exists = Path::new(&filepath).exists();
    if !exists {
        println!("looks like {filepath} does not exist");
        exit(1);
    }

    let mut lexer = Lexer::new(filepath);
    lexer.parse();
    // open file
    // tokenise
}
