use super::parser::PascalParser;
use super::ast::*;
use crate::lexer::token_types::TokenType;
use super::error::SyntaxError;


impl PascalParser {
    /// Parse expression (top-level)
    /// expression -> simple-expression (relational-op simple-expression)?
    pub(super) fn parse_expression(&mut self) -> Result<Expression, SyntaxError> {
        let left = self.parse_simple_expression()?;

        // Check for relational operators
        if self.check(TokenType::RelationalOperator) {
            let op = self.advance().value.clone();
            let right = self.parse_simple_expression()?;
            return Ok(Expression::BinaryOp {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
            });
        }

        Ok(left)
    }

    /// Parse simple expression (addition/subtraction/or level)
    /// simple-expression -> [sign] term (('+' | '-' | 'atau') term)*
    fn parse_simple_expression(&mut self) -> Result<Expression, SyntaxError> {
        // Handle unary sign (+ or -)
        let mut expr = if self.check(TokenType::ArithmeticOperator) {
            let op = self.peek().value.clone();
            if op == "+" || op == "-" {
                self.advance();
                let operand = self.parse_term()?;
                if op == "-" {
                    Expression::UnaryOp {
                        operator: op,
                        operand: Box::new(operand),
                    }
                } else {
                    operand // Unary + is a no-op
                }
            } else {
                self.parse_term()?
            }
        } else {
            self.parse_term()?
        };

        // Handle binary operators at this level
        loop {
            if self.check(TokenType::ArithmeticOperator) {
                let peek_val = self.peek().value.clone();
                if peek_val == "+" || peek_val == "-" {
                    self.advance();
                    let right = self.parse_term()?;
                    expr = Expression::BinaryOp {
                        left: Box::new(expr),
                        operator: peek_val,
                        right: Box::new(right),
                    };
                } else {
                    break;
                }
            } else if self.check(TokenType::LogicalOperator) {
                let peek_val = self.peek().value.clone();
                if peek_val == "atau" {
                    self.advance();
                    let right = self.parse_term()?;
                    expr = Expression::BinaryOp {
                        left: Box::new(expr),
                        operator: peek_val,
                        right: Box::new(right),
                    };
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        Ok(expr)
    }

    /// Parse term (multiplication/division/mod/and level)
    /// term -> factor (('*' | '/' | 'div' | 'mod' | 'dan') factor)*
    fn parse_term(&mut self) -> Result<Expression, SyntaxError> {
        let mut left = self.parse_factor()?;

        loop {
            if self.check(TokenType::ArithmeticOperator) {
                let peek_val = self.peek().value.clone();
                if peek_val == "*" || peek_val == "/" || peek_val == "div" || peek_val == "mod" {
                    self.advance();
                    let right = self.parse_factor()?;
                    left = Expression::BinaryOp {
                        left: Box::new(left),
                        operator: peek_val,
                        right: Box::new(right),
                    };
                } else {
                    break;
                }
            } else if self.check(TokenType::LogicalOperator) {
                let peek_val = self.peek().value.clone();
                if peek_val == "dan" {
                    self.advance();
                    let right = self.parse_factor()?;
                    left = Expression::BinaryOp {
                        left: Box::new(left),
                        operator: peek_val,
                        right: Box::new(right),
                    };
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        Ok(left)
    }

    /// Parse factor (primary expressions)
    /// factor -> 'tidak' factor | NUMBER | BOOLEAN | STRING | CHAR | IDENTIFIER | '(' expression ')'
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

            TokenType::StringLiteral => {
                let value = self.advance().value.clone();
                let trimmed = value.trim_matches('\'');
                Ok(Expression::Literal(LiteralValue::String(trimmed.to_string())))
            }

            TokenType::CharLiteral => {
                let value = self.advance().value.clone();
                let trimmed = value.trim_matches('\'');
                let ch = trimmed.chars().next()
                    .ok_or_else(|| self.error("Empty character literal"))?;
                Ok(Expression::Literal(LiteralValue::Char(ch)))
            }

            TokenType::Keyword => {
                let keyword = self.peek().value.to_lowercase();
                if keyword == "benar" || keyword == "true" {
                    self.advance();
                    Ok(Expression::Literal(LiteralValue::Boolean(true)))
                } else if keyword == "salah" || keyword == "false" {
                    self.advance();
                    Ok(Expression::Literal(LiteralValue::Boolean(false)))
                } else {
                    Err(self.error("Unexpected keyword in expression"))
                }
            }

            TokenType::LogicalOperator => {
                let op = self.peek().value.clone();
                if op == "tidak" {
                    self.advance();
                    let operand = self.parse_factor()?;
                    Ok(Expression::UnaryOp {
                        operator: op,
                        operand: Box::new(operand),
                    })
                } else {
                    Err(self.error("Expected 'tidak' (not) operator"))
                }
            }

            TokenType::Identifier => {
                let name = self.advance().value.clone();
                
                // Check for array indexing
                if self.check(TokenType::LBracket) {
                    self.advance(); // consume '['
                    let index = self.parse_expression()?;
                    self.consume_token(TokenType::RBracket, "Expected ']' after array index")?;
                    
                    Ok(Expression::ArrayAccess {
                        array: Box::new(Expression::Identifier(name)),
                        index: Box::new(index),
                    })
                }
                // Check for function call
                else if self.check(TokenType::LParenthesis) {
                    self.advance(); // consume '('
                    let arguments = self.parse_argument_list()?;
                    self.consume_token(TokenType::RParenthesis, "Expected ')' after arguments")?;
                    
                    Ok(Expression::FunctionCall {
                        function_name: name,
                        arguments,
                    })
                }
                // Just an identifier
                else {
                    Ok(Expression::Identifier(name))
                }
            }

            TokenType::LParenthesis => {
                self.advance();
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