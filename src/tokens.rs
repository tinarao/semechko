use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Token {
    Number(i64),
    Str(String),
    Operator(OperatorKind),
    Keyword(KeywordKind),
    BlockEnd,
    LParen,
    RParen,
}

#[derive(Debug, Clone)]
pub enum KeywordKind {
    Print,
}

#[derive(Debug, Clone)]
pub enum OperatorKind {
    Plus,
    Minus,
}

pub const BLOCK_END_SYMBOL: char = ';';
pub const LPAREN_SYMBOL: char = '(';
pub const RPAREN_SYMBOL: char = ')';

pub fn get_symbol_tokens_value() -> HashMap<String, Token> {
    let mut symbols: HashMap<String, Token> = HashMap::new();

    symbols.insert(BLOCK_END_SYMBOL.to_string(), Token::BlockEnd);
    symbols.insert(LPAREN_SYMBOL.to_string(), Token::LParen);
    symbols.insert(RPAREN_SYMBOL.to_string(), Token::RParen);

    return symbols;
}

pub fn get_operator_literals_map() -> HashMap<String, OperatorKind> {
    let mut literals: HashMap<String, OperatorKind> = HashMap::new();

    literals.insert('+'.to_string(), OperatorKind::Plus);
    literals.insert('-'.to_string(), OperatorKind::Minus);

    return literals;
}
