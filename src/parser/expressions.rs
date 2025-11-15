use super::parser::PascalParser;
use super::parse_tree::*;
use crate::lexer::token_types::TokenType;
use super::error::SyntaxError;

impl PascalParser {
    /// 1. parse_expression (Entry Point)
    ///    <expression> -> <simple-expression> ( <relational-operator> <simple-expression> )*
    pub(super) fn parse_expression(&mut self) -> Result<Expression, SyntaxError> {
        
        let initial_simple_expr = self.parse_simple_expression()?;
        let mut rest = Vec::new();

        while self.check(TokenType::RelationalOperator) {
            let op = self.advance().value.clone();
            let right_hand_side = self.parse_simple_expression()?;
            rest.push((op, Box::new(right_hand_side)));
        }

        Ok(Expression {
            initial_simple_expr: Box::new(initial_simple_expr),
            rest,
        })
    }

    /// 2. parse_simple_expression
    ///    <simple-expression> -> [sign] <term> ( <additive-operator> <term> )*
    fn parse_simple_expression(&mut self) -> Result<SimpleExpression, SyntaxError> {
        
        let mut unary_op = None;
        if self.check(TokenType::ArithmeticOperator) {
            let op_val = self.peek().value.clone();
            if op_val == "+" || op_val == "-" {
                self.advance(); // consume '+' or '-'
                unary_op = Some(op_val);
            }
        }

        let initial_term = self.parse_term()?;
        let mut rest = Vec::new();

        loop {
            let op_val = self.peek().value.clone();
            if self.check(TokenType::ArithmeticOperator) && (op_val == "+" || op_val == "-") {
                self.advance();
                let term = self.parse_term()?;
                rest.push((op_val, Box::new(term)));
            } else if self.check_keyword("atau") {
                self.advance(); // consume 'atau'
                let term = self.parse_term()?;
                rest.push(("atau".to_string(), Box::new(term)));
            } else {
                break;
            }
        }
        
        Ok(SimpleExpression {
            unary_op,
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
               matches!(op_val.as_str(), "*" | "/" | "div" | "mod") {
                
                self.advance();
                let factor = self.parse_factor()?;
                rest.push((op_val, Box::new(factor)));
            } else if self.check_keyword("dan") {
                self.advance(); // consume 'dan'
                let factor = self.parse_factor()?;
                rest.push(("dan".to_string(), Box::new(factor)));
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
            if self.match_token(&[TokenType::LBracket]) {
                // --- Kasus Array Access: ... [ index ] ---
                let index = self.parse_expression()?;
                self.consume_token(TokenType::RBracket, "Mengharapkan ']' setelah indeks array.")?;
                
                // "Bungkus" 'factor' yang ada sekarang
                let base_expr = self.factor_to_expression(factor);
                factor = Factor::ArrayAccess(ArrayAccess { 
                    array: Box::new(base_expr), 
                    index: Box::new(index) 
                });
                
            } else if self.check(TokenType::LParenthesis) {
                // --- Kasus Function Call: ... ( args ) ---
                // 'factor' yang ada HARUS berupa Identifier atau ArrayAccess
                let name = match factor {
                    Factor::Identifier(name) => name,
                    // TODO: Tambahkan support untuk `arr[i](args)` jika spek memperbolehkan
                    _ => return Err(self.error("Mengharapkan nama fungsi sebelum '('.")),
                };

                let arguments = self.parse_argument_list()?;
                factor = Factor::FunctionCall(FunctionCallNode { 
                    function_name: name, 
                    arguments 
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
            TokenType::Keyword if self.check_keyword("tidak") => {
                self.advance(); // consume 'tidak'
                let factor = self.parse_factor()?; // Panggil rekursif
                Ok(Factor::Not(Box::new(factor)))
            }

            // Kasus ( expression )
            TokenType::LParenthesis => {
                self.advance(); // consume '('
                let expr = self.parse_expression()?; // Panggil rekursif ke level atas
                self.consume_token(TokenType::RParenthesis, "Mengharapkan ')' setelah ekspresi.")?;
                Ok(Factor::Parenthesized(Box::new(expr)))
            }

            // Kasus IDENTIFIER
            TokenType::Identifier => {
                let name = self.advance().value.clone();
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
                unary_op: None,
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
    fn parse_literal_value(&mut self) -> Result<LiteralValue, SyntaxError> {
        
        let token = self.advance(); // Consume token literal
        
        let literal = match token.token_type {
            TokenType::IntegerLiteral => {
                let val = token.value.parse::<i64>().map_err(|_| self.error("Integer literal tidak valid."))?;
                Literal::Integer(val)
            }
            TokenType::RealLiteral => {
                let val = token.value.parse::<f64>().map_err(|_| self.error("Real literal tidak valid."))?;
                Literal::Real(val)
            }
            TokenType::StringLiteral => {
                // Hilangkan tanda kutip ' di awal dan akhir
                let val = token.value[1..token.value.len() - 1].to_string();
                Literal::String(val)
            }
            TokenType::CharLiteral => {
                // Hilangkan tanda kutip ' di awal dan akhir
                let val = token.value[1..token.value.len() - 1].chars().next()
                    .ok_or_else(|| self.error("Char literal kosong."))?;
                Literal::Char(val)
            }
            TokenType::Keyword if token.value.to_lowercase() == "benar" => Literal::Boolean(true),
            TokenType::Keyword if token.value.to_lowercase() == "salah" => Literal::Boolean(false),
            _ => return Err(self.error("Mengharapkan literal (angka, string, char, atau boolean)."))
        };

        Ok(LiteralValue { value: Box::new(literal) })
    }
    
    /// Meng-handle parsing `( [expression (, expression)*] )`
    pub(super) fn parse_argument_list(&mut self) -> Result<ParameterList, SyntaxError> {
        
        self.consume_token(TokenType::LParenthesis, "Mengharapkan '(' untuk pemanggilan fungsi/prosedur.")?;
        let mut expressions = Vec::new();

        // Cek jika list argumen tidak kosong (misal: `()`)
        if !self.check(TokenType::RParenthesis) {
            loop {
                expressions.push(self.parse_expression()?);
                if !self.match_token(&[TokenType::Comma]) {
                    // Jika bukan koma, harus ')'
                    break;
                }
                // Jika setelah koma adalah ')', itu error
                if self.check(TokenType::RParenthesis) {
                    return Err(self.error("Mengharapkan ekspresi setelah ','."));
                }
            }
        }
        
        self.consume_token(TokenType::RParenthesis, "Mengharapkan ')' setelah daftar argumen.")?;
        
        Ok(ParameterList { expressions })
    }
}