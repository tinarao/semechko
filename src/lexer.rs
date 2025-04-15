use crate::tokens::BLOCK_END_SYMBOL;
use crate::tokens::KeywordKind;
use crate::tokens::Token;
use crate::tokens::get_operator_literals_map;
use crate::tokens::get_symbol_tokens_value;
use std::{fs, iter::Peekable, process::exit, str::Chars};

type Source<'s> = Peekable<Chars<'s>>;

pub struct Lexer {
    filepath: String,
}

impl Lexer {
    pub fn new(filepath: &String) -> Self {
        let fp = String::from(filepath);
        Lexer { filepath: fp }
    }

    pub fn parse(&mut self) -> Vec<Token> {
        let contents = match fs::read_to_string(&self.filepath) {
            Ok(s) => s,
            Err(e) => {
                println!("failed to read {}: {e}", &self.filepath);
                exit(1);
            }
        };

        let tokens = self.lex(contents.chars().peekable());
        return tokens;
    }

    fn tokenize(&mut self, src: &mut Source) -> Option<Token> {
        let ch = src.peek()?;
        if ch.is_ascii_digit() {
            return Some(self.parse_number(src));
        } else if ch.is_alphabetic() {
            return Some(self.parse_alphabetic(src));
        } else if self.is_operator(ch) {
            return Some(self.parse_operator(src));
        } else if ch == &BLOCK_END_SYMBOL {
            src.next();
            return Some(Token::BlockEnd);
        } else {
            let symbol = self.parse_symbol(ch);
            src.next();
            return symbol;
        }
    }

    fn parse_symbol(&self, ch: &char) -> Option<Token> {
        let symbols = get_symbol_tokens_value();
        let token = symbols.get(&ch.to_string());
        match token {
            Some(t) => return Some(t.clone()),
            None => return None,
        }
    }

    fn is_operator(&self, ch: &char) -> bool {
        let operators = get_operator_literals_map();
        let ke = ch.to_string();

        return operators.contains_key(&ke);
    }

    fn parse_operator(&self, src: &mut Source) -> Token {
        let operators = get_operator_literals_map();
        let buf = self.take_till(src, |c| !c.is_whitespace());
        let kind = operators.get(&buf).unwrap();

        return Token::Operator((*kind).clone());
    }

    fn parse_alphabetic(&self, src: &mut Source) -> Token {
        let buf = self.take_till(src, |c| c.is_alphabetic());

        if buf == "рыгнуть".to_string() {
            return Token::Keyword(KeywordKind::Print);
        }

        return Token::Str(buf);
    }

    fn parse_number(&self, src: &mut Source) -> Token {
        let buf = self.take_till(src, |c| c.is_ascii_digit());
        let token = buf.parse().unwrap();

        return Token::Number(token);
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

        while let Some(_) = src.peek() {
            // Skip whitespace
            while let Some(c) = src.peek() {
                if !c.is_whitespace() {
                    break;
                }
                src.next();
            }

            if let Some(t) = self.tokenize(&mut src) {
                tokens.push(t);
            } else {
                // If we couldn't tokenize and it's not whitespace, skip one character
                if let Some(_) = src.peek() {
                    src.next();
                }
            }
        }

        return tokens;
    }
}
