use crate::tokens::{KeywordKind, OperatorKind, SemechkoNumber, Token};

#[derive(Debug)]
pub enum Expression {
    Number(SemechkoNumber),
    String(String),
    BinaryOperation {
        op: OperatorKind,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Print(Box<Expression>), // replace with something more generic
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Vec<Expression> {
        let mut expressions: Vec<Expression> = Vec::new();
        while !self.is_at_end() {
            let expr = self.parse_expression();
            expressions.push(expr);
            self.move_forward();
        }

        return expressions;
    }

    fn parse_expression(&mut self) -> Expression {
        return self.equality_expr();
    }

    fn term(&mut self) -> Expression {
        if self.match_token(&[Token::Keyword(KeywordKind::Print)]) {
            let val = self.parse_expression();
            return Expression::Print(Box::new(val));
        }

        return match self.move_forward() {
            Token::Number(n) => Expression::Number(n),
            Token::Str(s) => Expression::String(s),
            _ => panic!("unexpected token"),
        };
    }

    fn match_token(&mut self, tokens: &[Token]) -> bool {
        for token in tokens {
            if self.check(token) {
                self.move_forward();
                return true;
            }
        }

        return false;
    }

    fn check(&self, token: &Token) -> bool {
        if self.is_at_end() {
            return false;
        }

        return &self.tokens[self.current] == token;
    }

    fn equality_expr(&mut self) -> Expression {
        let mut expr = self.term();

        while self.match_token(&[
            Token::Operator(OperatorKind::Plus),
            Token::Operator(OperatorKind::Minus),
        ]) {
            let op = self.previous().clone();
            let right = self.term();
            expr = Expression::BinaryOperation {
                op: match op {
                    Token::Operator(op) => op,
                    _ => unreachable!(),
                },
                left: Box::new(expr),
                right: Box::new(right),
            };
        }

        return expr;
    }

    fn move_forward(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        return self.previous();
    }

    fn previous(&self) -> Token {
        return self.tokens[self.current - 1].clone();
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.tokens.len();
    }
}
