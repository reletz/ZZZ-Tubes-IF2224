use super::parser::PascalParser;
use super::parse_tree::*;
use crate::lexer::token_types::{Token, TokenType};
use super::error::SyntaxError;

impl PascalParser {
    /// 1. parse_statement (Router)
    ///    Membaca token berikutnya dan memanggil parser statement yang sesuai.
    pub(super) fn parse_statement(&mut self) -> Result<Statement, SyntaxError> {
        // TODO (CST): Logika router ini sebagian besar tetap sama.
        // Panggil fungsi parser yang sesuai (misal `parse_compound_statement`),
        // lalu bungkus hasilnya di enum `Statement`.
        // Contoh: `Ok(Statement::Compound(self.parse_compound_statement()?))`
        //
        // Perubahan terbesar ada di `parse_assignment_or_procedure_call`.
        
        if self.check_keyword("mulai") {
            unimplemented!() // Ok(Statement::Compound(self.parse_compound_statement()?))
        } else if self.check_keyword("jika") {
            unimplemented!() // Ok(Statement::If(self.parse_if_statement()?))
        } else if self.check_keyword("selama") {
            unimplemented!() // Ok(Statement::While(self.parse_while_statement()?))
        } else if self.check_keyword("untuk") {
            unimplemented!() // Ok(Statement::For(self.parse_for_statement()?))
        } else if self.check_keyword("ulangi") {
            unimplemented!() // Ok(Statement::Repeat(self.parse_repeat_statement()?))
        } else if self.check_keyword("kasus") {
            unimplemented!() // Ok(Statement::Case(self.parse_case_statement()?))
        } else if self.check(TokenType::Identifier) {
            // Ini bisa jadi assignment ATAU procedure call.
            // Keduanya dimulai dengan <expression> (berdasarkan grammar kita saat ini)
            self.parse_assignment_or_procedure_call()
        } else {
            Err(self.error("Mengharapkan sebuah statement."))
        }
    }

    /// 2. parse_statement_list (Helper)
    ///    Mem-parse: statement (';' statement)*
    ///    Berhenti ketika `is_terminator` mengembalikan true.
    fn parse_statement_list<F>(&mut self, mut is_terminator: F) -> Result<StatementList, SyntaxError>
    where F: FnMut(&mut Self) -> bool,
    {
        // TODO (CST):
        // 1. `let mut initial_stmt = None;`
        // 2. `let mut rest = Vec::new();`
        // 3. `if is_terminator(self) { return Ok(StatementList { initial_stmt, rest }); }`
        // 4. `initial_stmt = Some(Box::new(self.parse_statement()?));` (Gunakan Box::new!)
        // 5. Loop `while self.check(TokenType::Semicolon)`:
        //    a. `let semi = self.advance().clone();`
        //    b. `if is_terminator(self) { break; }` (Handle trailing semicolon)
        //    c. `let stmt = Box::new(self.parse_statement()?);`
        //    d. `rest.push((semi, stmt));`
        // 6. `Ok(StatementList { initial_stmt, rest })`

        unimplemented!()
    }

    /// 3. parse_compound_statement
    ///    'mulai' <statement-list> 'selesai'
    pub(super) fn parse_compound_statement(&mut self) -> Result<CompoundStatement, SyntaxError> {
        // TODO (CST):
        // 1. `let begin_kw = self.consume_keyword("mulai", ...).clone()`
        // 2. `let statement_list = self.parse_statement_list(|p| p.check_keyword("selesai"))?`
        // 3. `let end_kw = self.consume_keyword("selesai", ...).clone()`
        // 4. `Ok(CompoundStatement { begin_kw, statement_list, end_kw })`

        unimplemented!()
    }

    /// 4. parse_if_statement
    ///    'jika' <expression> 'maka' <statement> ['selain-itu' <statement>]
    fn parse_if_statement(&mut self) -> Result<IfStatement, SyntaxError> {
        // TODO (CST):
        // 1. `let if_kw = self.consume_keyword("jika", ...).clone()`
        // 2. `let condition = self.parse_expression()?`
        // 3. `let then_kw = self.consume_keyword("maka", ...).clone()`
        // 4. `let then_branch = Box::new(self.parse_statement()?)`
        // 5. `let else_clause = self.parse_else_clause()?`
        // 6. `Ok(IfStatement { if_kw, condition, then_kw, then_branch, else_clause })`

        unimplemented!()
    }

    /// 4a. parse_else_clause (Helper)
    ///    'selain-itu' <statement>
    fn parse_else_clause(&mut self) -> Result<Option<ElseClause>, SyntaxError> {
        // TODO (CST):
        // 1. `if self.check_keyword("selain-itu")`:
        //    a. `let else_kw = self.advance().clone()`
        //    b. `let statement = Box::new(self.parse_statement()?)`
        //    c. `Ok(Some(ElseClause { else_kw, statement }))`
        // 2. `else { Ok(None) }`

        unimplemented!()
    }

    /// 5. parse_while_statement
    ///    'selama' <expression> 'lakukan' <statement>
    fn parse_while_statement(&mut self) -> Result<WhileStatement, SyntaxError> {
        // TODO (CST):
        // 1. `let while_kw = self.consume_keyword("selama", ...).clone()`
        // 2. `let condition = self.parse_expression()?`
        // 3. `let do_kw = self.consume_keyword("lakukan", ...).clone()`
        // 4. `let body = Box::new(self.parse_statement()?)`
        // 5. `Ok(WhileStatement { while_kw, condition, do_kw, body })`

        unimplemented!()
    }

    /// 6. parse_for_statement
    ///    'untuk' ID ':=' <expression> ('ke' | 'turun-ke') <expression> 'lakukan' <statement>
    fn parse_for_statement(&mut self) -> Result<ForStatement, SyntaxError> {
        // TODO (CST):
        // 1. `let for_kw = self.consume_keyword("untuk", ...).clone()`
        // 2. `let counter_variable = self.consume_token(TokenType::Identifier, ...).clone()`
        // 3. `let assign_op = self.consume_token(TokenType::AssignOperator, ...).clone()`
        // 4. `let start_value = self.parse_expression()?`
        // 5. `let direction_kw = ...` (Cek 'ke' atau 'turun-ke', consume, dan clone)
        // 6. `let end_value = self.parse_expression()?`
        // 7. `let do_kw = self.consume_keyword("lakukan", ...).clone()`
        // 8. `let body = Box::new(self.parse_statement()?)`
        // 9. `Ok(ForStatement { ... })`

        unimplemented!()
    }

    /// 7. parse_repeat_statement
    ///    'ulangi' <statement-list> 'sampai' <expression>
    fn parse_repeat_statement(&mut self) -> Result<RepeatStatement, SyntaxError> {
        // TODO (CST):
        // 1. `let repeat_kw = self.consume_keyword("ulangi", ...).clone()`
        // 2. `let statement_list = self.parse_statement_list(|p| p.check_keyword("sampai"))?`
        // 3. `let until_kw = self.consume_keyword("sampai", ...).clone()`
        // 4. `let condition = self.parse_expression()?`
        // 5. `Ok(RepeatStatement { repeat_kw, statement_list, until_kw, condition })`

        unimplemented!()
    }

    /// 8. parse_case_statement
    ///    'kasus' <expression> 'dari' <case-branch-list> ['selain-itu' <statement-list>] 'selesai'
    fn parse_case_statement(&mut self) -> Result<CaseStatement, SyntaxError> {
        // TODO (CST):
        // 1. `let case_kw = self.consume_keyword("kasus", ...).clone()`
        // 2. `let expression = self.parse_expression()?`
        // 3. `let of_kw = self.consume_keyword("dari", ...).clone()`
        // 4. `let mut branches = Vec::new()`
        // 5. Loop `while !self.check_keyword("selain-itu") && !self.check_keyword("selesai")`:
        //    a. `branches.push(self.parse_case_branch()?)`
        // 6. `let else_clause = self.parse_case_else_clause()?`
        // 7. `let end_kw = self.consume_keyword("selesai", ...).clone()`
        // 8. `Ok(CaseStatement { ... })`

        unimplemented!()
    }

    /// 8a. parse_case_branch (Helper)
    ///     <case-label-list> ':' <statement> ';'
    fn parse_case_branch(&mut self) -> Result<CaseBranch, SyntaxError> {
        // TODO (CST):
        // 1. `let labels = self.parse_case_label_list()?`
        // 2. `let colon = self.consume_token(TokenType::Colon, ...).clone()`
        // 3. `let statement = Box::new(self.parse_statement()?)`
        // 4. `let semicolon = self.consume_token(TokenType::Semicolon, ...).clone()`
        // 5. `Ok(CaseBranch { labels, colon, statement, semicolon })`

        unimplemented!()
    }

    /// 8b. parse_case_label_list (Helper)
    ///     <expression> (',' <expression>)*
    fn parse_case_label_list(&mut self) -> Result<CaseLabelList, SyntaxError> {
        // TODO (CST):
        // 1. `let initial_label = self.parse_expression()?`
        // 2. `let mut rest = Vec::new()`
        // 3. Loop `while self.check(TokenType::Comma)`:
        //    a. `let comma = self.advance().clone()`
        //    b. `let next_label = self.parse_expression()?`
        //    c. `rest.push((comma, next_label))`
        // 4. `Ok(CaseLabelList { initial_label, rest })`
        
        unimplemented!()
    }

    /// 8c. parse_case_else_clause (Helper)
    ///     'selain-itu' <statement-list>
    fn parse_case_else_clause(&mut self) -> Result<Option<CaseElseClause>, SyntaxError> {
        // TODO (CST):
        // 1. `if self.check_keyword("selain-itu")`:
        //    a. `let else_kw = self.advance().clone()`
        //    b. `let statement_list = self.parse_statement_list(|p| p.check_keyword("selesai"))?`
        //    c. `Ok(Some(CaseElseClause { else_kw, statement_list }))`
        // 2. `else { Ok(None) }`

        unimplemented!()
    }


    /// 9. parse_assignment_or_procedure_call
    ///    Bisa jadi: <expression> ':=' <expression>
    ///    ATAU       <expression> (yang harus berupa ProcedureCall)
    fn parse_assignment_or_procedure_call(&mut self) -> Result<Statement, SyntaxError> {
        // TODO (CST):
        // 1. `let left_expr = self.parse_expression()?`
        // 2. `if self.check(TokenType::AssignOperator)`:
        //    a. `let assign_op = self.advance().clone()`
        //    b. `let expression = self.parse_expression()?`
        //    c. `let assign_stmt = AssignmentStatement { variable: left_expr, assign_op, expression }`
        //    d. `Ok(Statement::Assignment(assign_stmt))`
        // 3. `else`: (Harus Procedure Call)
        //    a. `match Self::expr_to_proc_call(left_expr)`:
        //       i. `Ok(call) => Ok(Statement::ProcedureCall(call))`
        //       ii. `Err(e) => Err(e)` (atau error baru yang lebih spesifik)

        unimplemented!()
    }

    /// Helper: Try to convert an Expression to a ProcedureCallStatement
    /// Only succeeds if the expression is a plain function call (no operators)
    fn expr_to_proc_call(expr: Expression) -> Result<ProcedureCallStatement, SyntaxError> {
        // TODO (CST):
        // 1. Cek `expr.rest.is_empty()`
        // 2. Cek `expr.initial_simple_expr.unary_op.is_none()` dan `...rest.is_empty()`
        // 3. Cek `expr.initial_simple_expr.initial_term.rest.is_empty()`
        // 4. `match *expr.initial_simple_expr.initial_term.initial_factor`:
        //    a. `Factor::FunctionCall(call) => Ok(ProcedureCallStatement { call })`
        //    b. `_ => Err(SyntaxError::new("Statement tidak valid. Mengharapkan ':=' atau pemanggilan prosedur.", 0, 0))`
        //       (Catatan: line/column 0,0 tidak ideal, mungkin perlu meneruskan token)
        
        unimplemented!()
    }
}