use super::parser::PascalParser;
use super::ast::*;
use crate::lexer::token_types::TokenType;
use super::error::SyntaxError;

// TODO: Boolean/String/Char literals, Relational ops (=,<>,<,<=,>,>=), 
//       Boolean ops (dan,atau,tidak), Unary minus, Array indexing, Function calls

impl PascalParser {
    /// Parse expression (top-level)
    /// expression -> simple-expression (relational-op simple-expression)?
    pub(super) fn parse_expression(&mut self) -> Result<Expression, SyntaxError> {
        self.parse_simple_expression()
    }

    /// Parse simple expression (addition/subtraction level)
    /// simple-expression -> term (('+' | '-') term)*
    fn parse_simple_expression(&mut self) -> Result<Expression, SyntaxError> {
        let mut left = self.parse_term()?;

        while self.check(TokenType::ArithmeticOperator) {
            let peek_val = self.peek().value.clone();
            
            // Only handle + and - at this level
            if peek_val == "+" || peek_val == "-" {
                self.advance();
                let op = peek_val;
                let right = self.parse_term()?;
                left = Expression::BinaryOp {
                    left: Box::new(left),
                    operator: op,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }

        Ok(left)
    }

    /// Parse term (multiplication/division level)
    /// term -> factor (('*' | '/' | 'div' | 'mod') factor)*
    fn parse_term(&mut self) -> Result<Expression, SyntaxError> {
        let mut left = self.parse_factor()?;

        while self.check(TokenType::ArithmeticOperator) {
            let peek_val = self.peek().value.clone();
            
            // Handle *, /, div, mod
            if peek_val == "*" || peek_val == "/" || peek_val == "div" || peek_val == "mod" {
                self.advance();
                let op = peek_val;
                let right = self.parse_factor()?;
                left = Expression::BinaryOp {
                    left: Box::new(left),
                    operator: op,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }

        Ok(left)
    }

    /// Parse factor (primary expressions)
    /// factor -> NUMBER | IDENTIFIER | '(' expression ')'
    fn parse_factor(&mut self) -> Result<Expression, SyntaxError> {
        let token = self.peek();

        match token.token_type {
            TokenType::IntegerLiteral => {
                let value = self.advance().value.parse::<i64>()
                    .map_err(|_| self.error("Invalid integer literal"))?;
                Ok(Expression::Literal(LiteralValue::Integer(value)))
            }

            TokenType::RealLiteral => {
                let value = self.advance().value.parse::<f64>()
                    .map_err(|_| self.error("Invalid real literal"))?;
                Ok(Expression::Literal(LiteralValue::Real(value)))
            }

            TokenType::Identifier => {
                let name = self.advance().value.clone();
                Ok(Expression::Identifier(name))
            }

            TokenType::LParenthesis => {
                self.advance(); // consume '('
                let expr = self.parse_expression()?;
                self.consume_token(TokenType::RParenthesis, "Expected ')' after expression")?;
                Ok(expr)
            }

            _ => {
                Err(self.error("Expected expression"))
            }
        }
    }
}