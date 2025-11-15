use super::parser::PascalParser;
use super::parse_tree::*;
use crate::lexer::token_types::TokenType;
use super::error::SyntaxError;

impl PascalParser {
    ///    Membaca token berikutnya dan memanggil parser statement yang sesuai.
    pub(super) fn parse_statement(&mut self) -> Result<Statement, SyntaxError> {
        
        // TODO: Implement parse_statement (Router)
        // 1. Gunakan `if-else if` untuk `self.check_keyword(...)`
        // 2. `if self.check_keyword("mulai")`:
        //    - Panggil `self.parse_compound_statement()`
        // 3. `else if self.check_keyword("jika")`:
        //    - Panggil `self.parse_if_statement()`
        // 4. `else if self.check_keyword("selama")`:
        //    - Panggil `self.parse_while_statement()`
        // 5. `else if self.check_keyword("untuk")`:
        //    - Panggil `self.parse_for_statement()`
        // 6. `else if self.check_keyword("ulangi")`:
        //    - Panggil `self.parse_repeat_statement()`
        // 7. `else if self.check_keyword("kasus")`:
        //    - Panggil `self.parse_case_statement()`
        // 8. `else if self.check(TokenType::Identifier)` (ATAU `LParenthesis`, `IntegerLiteral`, dll.):
        //    - (Ini adalah kasus `Assignment` atau `ProcedureCall`)
        //    - Panggil `self.parse_assignment_or_procedure_call()`
        // 9. `else`:
        //    - `Err(self.error("Mengharapkan sebuah statement."))`
        
        unimplemented!("parse_statement (router) belum diimplementasikan")
    }

    /// 2. parse_statement_list (Helper)
    ///    Mem-parse: statement (';' statement)*
    ///    Berhenti ketika `is_terminator` mengembalikan true.
    fn parse_statement_list<F>(&mut self, mut is_terminator: F) -> Result<StatementList, SyntaxError>
    where F: FnMut(&mut Self) -> bool,
    {
        // TODO: Implement parse_statement_list
        // 1. Buat `let mut statements = Vec::new()`.
        // 2. Cek `if is_terminator(self)` -> `return Ok(StatementList { statements })` (kosong).
        // 3. `loop`:
        //    a. `statements.push(self.parse_statement()?)`.
        //    b. `if !self.match_token(&[TokenType::Semicolon])`:
        //       - `break;` (Semicolon opsional di akhir list sebelum 'end'/'until')
        //    c. `if is_terminator(self)`:
        //       - `break;` (Menangani: `... ; end`)
        //    d. (Opsional: Cek error "trailing semicolon": `if self.check(...) { return Err(...) }`)
        // 4. Kembalikan `Ok(StatementList { statements })`.
        
        unimplemented!("parse_statement_list belum diimplementasikan")
    }

    /// 3. parse_compound_statement
    ///    'mulai' <statement-list> 'selesai'
    pub(super) fn parse_compound_statement(&mut self) -> Result<CompoundStatement, SyntaxError> {
        
        // TODO: Implement parse_compound_statement
        // 1. `self.consume_keyword("mulai", ...)`
        // 2. `let list = self.parse_statement_list(|p| p.check_keyword("selesai"))?`
        // 3. `self.consume_keyword("selesai", ...)`
        // 4. Kembalikan `Ok(CompoundStatement { statement_list: list })`
        //    (Perhatikan: `parse_tree.rs` kita membungkus list-nya)

        unimplemented!("parse_compound_statement belum diimplementasikan")
    }

    /// 4. parse_if_statement
    ///    'jika' <expression> 'maka' <statement> ['selain-itu' <statement>]
    fn parse_if_statement(&mut self) -> Result<Statement, SyntaxError> {
        
        // TODO: Implement parse_if_statement
        // 1. `self.consume_keyword("jika", ...)`
        // 2. `let condition = self.parse_expression()?`
        // 3. `self.consume_keyword("maka", ...)`
        // 4. `let then_branch = Box::new(self.parse_statement()?)`
        // 5. `let else_branch = if self.match_keyword(&["selain-itu"]) { ... }`
        //    - `Some(Box::new(self.parse_statement()?))`
        //    - `None`
        // 6. Kembalikan `Ok(Statement::If(IfStatement { ... }))`
        
        unimplemented!("parse_if_statement belum diimplementasikan")
    }

    /// 5. parse_while_statement
    ///    'selama' <expression> 'lakukan' <statement>
    fn parse_while_statement(&mut self) -> Result<Statement, SyntaxError> {
        
        // TODO: Implement parse_while_statement
        // 1. `self.consume_keyword("selama", ...)`
        // 2. `let condition = self.parse_expression()?`
        // 3. `self.consume_keyword("lakukan", ...)`
        // 4. `let body = Box::new(self.parse_statement()?)`
        // 5. Kembalikan `Ok(Statement::While(WhileStatement { ... }))`

        unimplemented!("parse_while_statement belum diimplementasikan")
    }

    /// 6. parse_for_statement
    ///    'untuk' ID ':=' <expression> ('ke' | 'turun-ke') <expression> 'lakukan' <statement>
    fn parse_for_statement(&mut self) -> Result<Statement, SyntaxError> {
        
        // TODO: Implement parse_for_statement
        // 1. `self.consume_keyword("untuk", ...)`
        // 2. `let counter = self.consume_token(Identifier, ...).value.clone()`
        // 3. `self.consume_token(AssignOperator, ...)`
        // 4. `let start = self.parse_expression()?`
        // 5. Cek `direction`: `if self.match_keyword(&["ke"]) { ForDirection::To } ...`
        // 6. `let end = self.parse_expression()?`
        // 7. `self.consume_keyword("lakukan", ...)`
        // 8. `let body = Box::new(self.parse_statement()?)`
        // 9. Kembalikan `Ok(Statement::For(ForStatement { ... }))`
        
        unimplemented!("parse_for_statement belum diimplementasikan")
    }

    /// 7. parse_repeat_statement
    ///    'ulangi' <statement-list> 'sampai' <expression>
    fn parse_repeat_statement(&mut self) -> Result<Statement, SyntaxError> {
        
        // TODO: Implement parse_repeat_statement
        // 1. `self.consume_keyword("ulangi", ...)`
        // 2. `let list = self.parse_statement_list(|p| p.check_keyword("sampai"))?`
        // 3. `self.consume_keyword("sampai", ...)`
        // 4. `let condition = self.parse_expression()?`
        // 5. Kembalikan `Ok(Statement::Repeat(RepeatStatement { statement_list: list, condition }))`

        unimplemented!("parse_repeat_statement belum diimplementasikan")
    }

    /// 8. parse_case_statement
    ///    'kasus' <expression> 'dari' <case-branch-list> ['selain-itu' <statement-list>] 'selesai'
    fn parse_case_statement(&mut self) -> Result<Statement, SyntaxError> {
        
        // TODO: Implement parse_case_statement
        // 1. `self.consume_keyword("kasus", ...)`
        // 2. `let expr = self.parse_expression()?`
        // 3. `self.consume_keyword("dari", ...)`
        // 4. `let mut branches = Vec::new()`.
        // 5. Loop `while !self.check_keyword("selain-itu") && !self.check_keyword("selesai")`:
        //    - `branches.push(self.parse_case_branch()?)`
        // 6. Cek `else_branch`: `if self.match_keyword(&["selain-itu"]) { ... }`
        //    - `Some(self.parse_statement_list(|p| p.check_keyword("selesai"))?)`
        // 7. `self.consume_keyword("selesai", ...)`
        // 8. Kembalikan `Ok(Statement::Case(CaseStatement { ... }))`

        unimplemented!("parse_case_statement belum diimplementasikan")
    }

    /// 8a. parse_case_branch (Helper)
    ///     <expression-list> ':' <statement> ';'
    fn parse_case_branch(&mut self) -> Result<CaseBranch, SyntaxError> {
        
        // TODO: Implement parse_case_branch
        // 1. Buat `let mut labels = vec![self.parse_expression()?]`.
        // 2. Loop `while self.match_token(&[TokenType::Comma])`:
        //    - `labels.push(self.parse_expression()?)`
        // 3. `self.consume_token(Colon, ...)`
        // 4. `let statement = Box::new(self.parse_statement()?)`
        // 5. `self.consume_token(Semicolon, ...)` (Semicolon wajib antar branch)
        // 6. Kembalikan `Ok(CaseBranch { labels, statement })`
        
        unimplemented!("parse_case_branch belum diimplementasikan")
    }

    /// 9. parse_assignment_or_procedure_call
    ///    Ini adalah kasus paling rumit.
    ///    Bisa jadi: <expression> ':=' <expression>
    ///    ATAU       <expression> (yang harus berupa ProcedureCall)
    fn parse_assignment_or_procedure_call(&mut self) -> Result<Statement, SyntaxError> {
        
        // TODO: Implement parse_assignment_or_procedure_call
        // 1. `let left_expr = self.parse_expression()?`
        // 2. `if self.match_token(&[TokenType::AssignOperator])`:
        //    - (Ini adalah Assignment)
        //    - `let right_expr = self.parse_expression()?`
        //    - `return Ok(Statement::Assignment(AssignmentStatement { variable: left_expr, expression: right_expr }))`
        // 3. `// Jika bukan assignment, 'left_expr' PASTI sebuah procedure call.`
        // 4. `// Kita harus "mengkonversi" Expression -> ProcedureCallStatement`
        // 5. Buat helper `fn expr_to_proc_call(expr: Expression) -> Option<ProcedureCallStatement>`
        // 6. Helper ini akan "membongkar" `expr` -> `sexpr` -> `term` -> `factor`
        // 7. Jika `factor` adalah `Factor::FunctionCall(func_call)` DAN SEMUA `rest` di atasnya kosong:
        //    - `return Some(ProcedureCallStatement { procedure_name: func_call.function_name, arguments: func_call.arguments })`
        // 8. `if let Some(proc_call) = expr_to_proc_call(left_expr)`:
        //    - `return Ok(Statement::ProcedureCall(proc_call))`
        // 9. `else`:
        //    - `return Err(self.error("Statement tidak valid. Mengharapkan ':=' atau pemanggilan prosedur."))`
        
        unimplemented!("parse_assignment_or_procedure_call belum diimplementasikan")
    }
}