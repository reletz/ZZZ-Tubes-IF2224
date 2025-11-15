use super::parser::PascalParser;
use super::parse_tree::*;
use crate::lexer::token_types::TokenType;
use super::error::SyntaxError;

impl PascalParser {
    /// 1. parse_expression (Entry Point)
    ///    Sesuai Spek: <expression> -> <simple-expression> ( <relational-operator> <simple-expression> )*
    pub(super) fn parse_expression(&mut self) -> Result<Expression, SyntaxError> {
        
        // TODO: Implement parse_expression
        // 1. Panggil `self.parse_simple_expression()` untuk mendapatkan `initial_simple_expr`.
        // 2. Buat `rest = Vec::new()`.
        // 3. Loop selama `self.check(TokenType::RelationalOperator)`.
        // 4. Di dalam loop:
        //    a. Ambil `op = self.advance().value.clone()`.
        //    b. Panggil `self.parse_simple_expression()` lagi untuk `right_hand_side`.
        //    c. `rest.push((op, Box::new(right_hand_side)))`.
        // 5. Kembalikan `Ok(Expression { initial_simple_expr: Box::new(...), rest })`.
        
        unimplemented!("parse_expression belum diimplementasikan")
    }

    /// 2. parse_simple_expression
    ///    Sesuai Spek: <simple-expression> -> [sign] <term> ( <additive-operator> <term> )*
    fn parse_simple_expression(&mut self) -> Result<SimpleExpression, SyntaxError> {
        
        // TODO: Implement parse_simple_expression
        // 1. Cek `unary_op = Some(...)` jika ada `+` atau `-` di awal. Ingat `self.advance()`.
        // 2. Panggil `self.parse_term()` untuk `initial_term`.
        // 3. Buat `rest = Vec::new()`.
        // 4. Loop:
        //    a. Cek jika token berikutnya adalah `+`, `-` (ArithmeticOperator) atau `atau` (LogicalOperator).
        //    b. Jika ya, `self.advance()`, ambil `op`, panggil `self.parse_term()`, dan `rest.push(...)`.
        //    c. Jika tidak, `break`.
        // 5. Kembalikan `Ok(SimpleExpression { unary_op, initial_term: Box::new(...), rest })`.

        unimplemented!("parse_simple_expression belum diimplementasikan")
    }

    /// 3. parse_term
    ///    Sesuai Spek: <term> -> <factor> ( <multiplicative-operator> <factor> )*
    fn parse_term(&mut self) -> Result<Term, SyntaxError> {
        
        // TODO: Implement parse_term
        // 1. Panggil `self.parse_factor()` untuk `initial_factor`.
        // 2. Buat `rest = Vec::new()`.
        // 3. Loop:
        //    a. Cek jika token berikutnya adalah `*`, `/`, `div`, `mod` (ArithmeticOperator) atau `dan` (LogicalOperator).
        //    b. Jika ya, `self.advance()`, ambil `op`, panggil `self.parse_factor()`, dan `rest.push(...)`.
        //    c. Jika tidak, `break`.
        // 4. Kembalikan `Ok(Term { initial_factor: Box::new(...), rest })`.

        unimplemented!("parse_term belum diimplementasikan")
    }

    /// 4. parse_factor (DAN LOGIKA CHAINING)
    ///    Sesuai Spek: <factor> -> literal | ID | '(' <expression> ')' | 'tidak' <factor> | <function-call> | <array-access>
    fn parse_factor(&mut self) -> Result<Factor, SyntaxError> {
        
        // TODO: Implement parse_factor (LOGIKA PALING RUMIT ADA DI SINI)
        //       Logika ini harus menangani chaining (misal: `get_array()[i]`)
        //
        // 1. Buat helper `parse_primary()` untuk menangani "atom" (unit terkecil):
        //    - `Literal` (panggil `parse_literal_value()`)
        //    - `Identifier` (HANYA identifier, kembalikan `Factor::Identifier`)
        //    - `( expression )` (panggil `parse_expression()` rekursif)
        //    - `tidak factor` (panggil `parse_factor()` rekursif)
        //
        // 2. Di `parse_factor()`:
        //    a. Panggil `let mut factor = self.parse_primary()?`.
        //    b. Masuk ke `loop`.
        //    c. `if self.match_token(&[TokenType::LBracket])`:
        //       - Parse `index = self.parse_expression()?`.
        //       - `consume_token(RBracket)`.
        //       - Bungkus `factor` yang ada: `factor = Factor::ArrayAccess(ArrayAccess { array: Box::new(factor_to_expr(factor)), index: Box::new(index) })`.
        //         (Kita perlu helper `factor_to_expr` untuk konversi)
        //    d. `else if self.check(TokenType::LParenthesis)`:
        //       - Ubah `factor` dari `Factor::Identifier(name)` menjadi `Factor::FunctionCall`.
        //       - Panggil `arguments = self.parse_argument_list()?`.
        //       - `factor = Factor::FunctionCall(FunctionCallNode { function_name: name, arguments })`.
        //    e. `else`: `break;`
        //    f. Kembalikan `factor`.

        unimplemented!("parse_factor dan parse_primary belum diimplementasikan")
    }

    // --- HELPER UNTUK PARSING FACTOR ---

    /// Meng-handle parsing literal '5', 'true', 'a', dll.
    fn parse_literal_value(&mut self) -> Result<LiteralValue, SyntaxError> {
        
        // TODO: Implement parse_literal_value
        // 1. `let token = self.advance()`.
        // 2. `match token.token_type` untuk:
        //    - `IntegerLiteral` -> `Literal::Integer(val.parse()?)`
        //    - `RealLiteral` -> `Literal::Real(val.parse()?)`
        //    - `StringLiteral` -> `Literal::String(val.trim_matches('\'')...)`
        //    - `CharLiteral` -> `Literal::Char(val.trim_matches('\'').chars().next()?)`
        //    - `Keyword` "benar" -> `Literal::Boolean(true)`
        //    - `Keyword` "salah" -> `Literal::Boolean(false)`
        //    - `_` -> `Err(...)`
        // 3. Kembalikan `Ok(LiteralValue { value: Box::new(...) })`.
        
        unimplemented!("parse_literal_value belum diimplementasikan")
    }
    
    /// Meng-handle parsing `( [expression (, expression)*] )`
    pub(super) fn parse_argument_list(&mut self) -> Result<ParameterList, SyntaxError> {
        
        // TODO: Implement parse_argument_list (SESUAI REVISI 3)
        // 1. `consume_token(LParenthesis)`.
        // 2. Buat `expressions = Vec::new()`.
        // 3. Cek jika `!self.check(RParenthesis)`.
        // 4. Jika tidak kosong, masuk ke `loop`:
        //    a. `expressions.push(self.parse_expression()?)`.
        //    b. `if !self.match_token(Comma)` maka `break`.
        //    c. (Opsional: cek `self.check(RParenthesis)` di sini untuk error "trailing comma")
        // 5. `consume_token(RParenthesis)`.
        // 6. Kembalikan `Ok(ParameterList { expressions })`.
        
        unimplemented!("parse_argument_list belum diimplementasikan")
    }
}