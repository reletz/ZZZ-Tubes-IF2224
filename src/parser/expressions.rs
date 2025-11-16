use super::parser::PascalParser;
use super::parse_tree::*;
use crate::lexer::token_types::{Token, TokenType};
use super::error::SyntaxError;

impl PascalParser {
    /// 1. parse_expression (Entry Point)
    ///    <expression> -> <simple-expression> ( <relational-operator> <simple-expression> )*
    pub(super) fn parse_expression(&mut self) -> Result<Expression, SyntaxError> {
        
        let initial_simple_expr = self.parse_simple_expression()?;
        let mut rest = Vec::new();

        while self.check(TokenType::RelationalOperator) {
            let op_token = self.advance().clone(); 
            let right_hand_side = self.parse_simple_expression()?;
            rest.push((op_token, Box::new(right_hand_side)));
        }

        Ok(Expression {
            initial_simple_expr: Box::new(initial_simple_expr),
            rest,
        })
    }

    /// 2. parse_simple_expression
    ///    <simple-expression> -> [sign] <term> ( <additive-operator> <term> )*
    fn parse_simple_expression(&mut self) -> Result<SimpleExpression, SyntaxError> {
        
        // let mut unary_op = None;
        // if self.check(TokenType::ArithmeticOperator) {
        //     let op_val = self.peek().value.clone();
        //     if op_val == "+" || op_val == "-" {
        //         unary_op = Some(self.advance().clone());
        //     }
        // }

        let initial_term = self.parse_term()?;
        let mut rest = Vec::new();

        loop {
            let op_val = self.peek().value.clone();
            if self.check(TokenType::ArithmeticOperator) && (op_val == "+" || op_val == "-") {
                let op_token = self.advance().clone();
                let term = self.parse_term()?;
                rest.push((op_token, Box::new(term)));
            } else if self.check(TokenType::LogicalOperator) && self.peek().value.to_lowercase() == "atau" {
                let op_token = self.advance().clone();
                let term = self.parse_term()?;
                rest.push((op_token, Box::new(term)));
            } else {
                break;
            }
        }
        
        Ok(SimpleExpression {
            initial_term: Box::new(initial_term),
            rest,
        })
    }

    /// 3. parse_term
    ///    <term> -> <factor> ( <multiplicative-operator> <factor> )*
    fn parse_term(&mut self) -> Result<Term, SyntaxError> {
        
        let initial_factor = self.parse_factor()?;
        let mut rest = Vec::new();

        loop {
            let op_val = self.peek().value.clone();
            if self.check(TokenType::ArithmeticOperator) && 
               matches!(op_val.as_str(), "*" | "/" | "bagi" | "mod") { // 'div' di spek adalah 'bagi'
                
                let op_token = self.advance().clone();
                let factor = self.parse_factor()?;
                rest.push((op_token, Box::new(factor)));
            } else if self.check(TokenType::LogicalOperator) && self.peek().value.to_lowercase() == "dan" {
                let op_token = self.advance().clone();
                let factor = self.parse_factor()?;
                rest.push((op_token, Box::new(factor)));
            } else {
                break;
            }
        }

        Ok(Term {
            initial_factor: Box::new(initial_factor),
            rest,
        })
    }

    /// 4. parse_factor (DAN LOGIKA CHAINING)
    ///    <factor> -> <primary> ( '[' <expression> ']' | '(' <arg-list> ')' )*
    ///    Handle chaining kayak `get_array()[i]`
    fn parse_factor(&mut self) -> Result<Factor, SyntaxError> {
        
        // 1. Parse "atom" dasarnya (misal: "my_array", "5", "(a+b)", "tidak flag")
        let mut factor = self.parse_primary()?;

        // 2. Loop untuk menangani "postfix" operators (chaining)
        loop {
            if self.check(TokenType::LBracket) {
                // --- Kasus Array Access: ... [ index ] ---
                let l_bracket = self.advance().clone();
                let index = self.parse_expression()?;
                let r_bracket = self.consume_token(TokenType::RBracket, "Mengharapkan ']' setelah indeks array.")?.clone();
                
                // "Bungkus" 'factor' yang ada sekarang
                let base_expr = self.factor_to_expression(factor);
                factor = Factor::ArrayAccess(ArrayAccess { 
                    array: Box::new(base_expr), 
                    l_bracket,
                    index: Box::new(index),
                    r_bracket,
                });
                
            } else if self.check(TokenType::LParenthesis) {
                // --- Kasus Function Call: ... ( args ) ---
                // 'factor' yang ada HARUS berupa Identifier
                let name_token = match factor {
                    Factor::Identifier(name) => name,
                    _ => return Err(self.error("Mengharapkan nama fungsi sebelum '('.")),
                };

                // Helper baru ini akan mengurus '(', 'arg-list', dan ')'
                let (l_paren, arguments, r_paren) = self.parse_argument_list_cst()?;
                
                factor = Factor::FunctionCall(FunctionCallNode { 
                    function_name: name_token, 
                    l_paren,
                    arguments,
                    r_paren
                });

            } else {
                // Tidak ada lagi `[` atau `(`, loop selesai.
                break;
            }
        }

        Ok(factor)
    }

    /// Helper untuk `parse_factor`
    /// Mem-parse "atom" (unit terkecil) dari sebuah ekspresi.
    fn parse_primary(&mut self) -> Result<Factor, SyntaxError> {
        let token = self.peek();

        match token.token_type {
            TokenType::ArithmeticOperator if (token.value == "+" || token.value == "-") => {
            let op = self.advance().clone();
            let factor = self.parse_factor()?; // Panggil rekursif
            Ok(Factor::ArithmeticUnary(ArithmeticUnaryFactor {
                op,
                factor: Box::new(factor)
            }))
        }
            // Kasus Literal: 5, 3.14, 'hello', 'a', benar, salah
            TokenType::IntegerLiteral | TokenType::RealLiteral |
            TokenType::StringLiteral | TokenType::CharLiteral => {
                let literal_val = self.parse_literal_value()?;
                Ok(Factor::Literal(literal_val))
            }
            TokenType::Keyword if self.check_keyword("benar") || self.check_keyword("salah") => {
                let literal_val = self.parse_literal_value()?;
                Ok(Factor::Literal(literal_val))
            }

            // Kasus 'tidak' (not)
            TokenType::LogicalOperator if self.peek().value.to_lowercase() == "tidak"  => {
                let not_token = self.advance().clone(); // consume 'tidak'
                let factor = self.parse_factor()?; // Panggil rekursif
                Ok(Factor::Not(NotFactor {
                    not_token,
                    factor: Box::new(factor)
                }))
            }

            // Kasus ( expression )
            TokenType::LParenthesis => {
                let l_paren = self.advance().clone(); // consume '('
                let expr = self.parse_expression()?; // Panggil rekursif ke level atas
                let r_paren = self.consume_token(TokenType::RParenthesis, "Mengharapkan ')' setelah ekspresi.")?.clone();
                Ok(Factor::Parenthesized(ParenthesizedExpression {
                    l_paren,
                    expr: Box::new(expr),
                    r_paren
                }))
            }

            // Kasus IDENTIFIER
            TokenType::Identifier => {
                let name = self.advance().clone();
                Ok(Factor::Identifier(name))
            }

            _ => Err(self.error("Mengharapkan ekspresi (factor)."))
        }
    }

    /// Helper untuk `parse_factor`: Mengubah Factor -> Expression
    /// Ini dibutuhkan untuk membungkus `array` dalam `ArrayAccess`
    fn factor_to_expression(&self, factor: Factor) -> Expression {
        Expression {
            initial_simple_expr: Box::new(SimpleExpression {
                initial_term: Box::new(Term {
                    initial_factor: Box::new(factor),
                    rest: vec![]
                }),
                rest: vec![]
            }),
            rest: vec![]
        }
    }


    /// Meng-handle parsing literal '5', 'true', 'a', dll.
    /// Versi CST: Hanya menyimpan token-nya.
    fn parse_literal_value(&mut self) -> Result<LiteralValue, SyntaxError> {
        
        let token = self.peek();
        
        match token.token_type {
            TokenType::IntegerLiteral | 
            TokenType::RealLiteral |
            TokenType::StringLiteral |
            TokenType::CharLiteral => {
                let token = self.advance().clone();
                Ok(LiteralValue { token })
            }
            TokenType::Keyword if token.value.to_lowercase() == "benar" || token.value.to_lowercase() == "salah" => {
                let token = self.advance().clone();
                Ok(LiteralValue { token })
            }
            _ => Err(self.error("Mengharapkan literal (angka, string, char, atau boolean)."))
        }
    }
    
    /// Helper baru untuk `FunctionCallNode`
    /// Mem-parse: '(' [arg-list] ')'
    /// Mengembalikan token `(`, `Option<ActualParameterList>`, dan token `)`
    fn parse_argument_list_cst(&mut self) -> Result<(Token, Option<ActualParameterList>, Token), SyntaxError> {
        
        let l_paren = self.consume_token(TokenType::LParenthesis, "Mengharapkan '(' untuk pemanggilan fungsi/prosedur.")?.clone();
        
        let arguments = if !self.check(TokenType::RParenthesis) {
            // Jika tidak kosong, parse list-nya
            Some(self.parse_actual_parameter_list()?)
        } else {
            // Jika kosong `()`
            None
        };
        
        let r_paren = self.consume_token(TokenType::RParenthesis, "Mengharapkan ')' setelah daftar argumen.")?.clone();
        
        Ok((l_paren, arguments, r_paren))
    }

    /// Helper baru: Mem-parse isi dari daftar argumen
    /// Mem-parse: <expression> (',' <expression>)*
    fn parse_actual_parameter_list(&mut self) -> Result<ActualParameterList, SyntaxError> {
        
        let initial_arg = self.parse_expression()?;
        let mut rest = Vec::new();

        while self.check(TokenType::Comma) {
            let comma_token = self.advance().clone();
            
            // Jika setelah koma adalah ')', itu error
            if self.check(TokenType::RParenthesis) {
                return Err(self.error("Mengharapkan ekspresi setelah ','."));
            }

            let next_arg = self.parse_expression()?;
            rest.push((comma_token, Box::new(next_arg)));
        }
        
        Ok(ActualParameterList { 
            initial_arg: Box::new(initial_arg), 
            rest 
        })
    }
}