use std::{fs, iter::Peekable, process::exit, str::Chars};

#[derive(Debug)]
enum TokenType {
    Number(i64),
}

type Source<'s> = Peekable<Chars<'s>>;

#[derive(Debug)]
pub struct Token {
    variant: TokenType,
}

pub struct Lexer {
    filepath: String,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(filepath: &String) -> Lexer {
        let fp = String::from(filepath);
        Lexer {
            filepath: fp,
            current_char: None,
        }
    }

    pub fn parse(&mut self) {
        let contents = match fs::read_to_string(&self.filepath) {
            Ok(s) => s,
            Err(e) => {
                println!("failed to read {}: {e}", &self.filepath);
                exit(1);
            }
        };

        let tokens = self.lex(contents.chars().peekable());
        println!("tokens: {:?}", tokens);
    }

    fn tokenize(&mut self, src: &mut Source) -> Option<Token> {
        let ch = src.peek()?;
        if ch.is_ascii_digit() {
            return Some(self.parse_number(src));
        }

        return None;
    }

    fn parse_number(&self, src: &mut Source) -> Token {
        let buf = self.take_till(src, |c| c.is_ascii_digit());
        let token = buf.parse().unwrap();

        return Token {
            variant: TokenType::Number(token),
        };
    }

    fn take_till(&self, src: &mut Source, till: impl Fn(char) -> bool) -> String {
        let mut buf = String::new();
        while let Some(c) = src.peek() {
            if !till(*c) {
                break;
            }

            buf.push(*c);
            src.next();
        }

        return buf;
    }

    fn lex(&mut self, mut src: Source) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        while let Some(token) = self.tokenize(&mut src) {
            tokens.push(token)
        }

        return tokens;
    }
}
