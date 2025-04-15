use crate::{parser::Expression, tokens::OperatorKind};

pub struct CodeGenerator {
    output: String,
}

impl CodeGenerator {
    pub fn new() -> Self {
        CodeGenerator {
            output: String::new(),
        }
    }

    pub fn generate(&mut self, expressions: Vec<Expression>) -> String {
        self.output.push_str("fn main() {\n");

        for expr in expressions {
            self.generate_expression(&expr);
            self.output.push_str(";\n");
        }

        self.output.push_str("}\n");
        return self.output.clone();
    }

    fn generate_expression(&mut self, expression: &Expression) {
        match expression {
            Expression::Number(n) => {
                self.output.push_str(&n.to_string());
            }
            Expression::String(s) => {
                self.output.push_str(&format!("\"{}\"", s));
            }
            Expression::BinaryOperation { op, left, right } => {
                self.output.push('(');
                self.generate_expression(left);
                self.output.push_str(match op {
                    OperatorKind::Plus => " + ",
                    OperatorKind::Minus => " - ",
                });
                self.generate_expression(right);
                self.output.push(')');
            }
            Expression::Print(expr) => {
                self.output.push_str("println!(\"{}\", ");
                self.generate_expression(expr);
                self.output.push(')');
            }
        }
    }
}
