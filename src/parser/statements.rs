use super::parser::PascalParser;
use super::parse_tree::*;
use crate::lexer::token_types::TokenType;
use super::error::SyntaxError;

impl PascalParser {
    ///    Membaca token berikutnya dan memanggil parser statement yang sesuai.
    pub(super) fn parse_statement(&mut self) -> Result<Statement, SyntaxError> {
        if self.check_keyword("mulai") {
            let compound = self.parse_compound_statement()?;
            Ok(Statement::Compound(compound))
        } else if self.check_keyword("jika") {
            self.parse_if_statement()
        } else if self.check_keyword("selama") {
            self.parse_while_statement()
        } else if self.check_keyword("untuk") {
            self.parse_for_statement()
        } else if self.check_keyword("ulangi") {
            self.parse_repeat_statement()
        } else if self.check_keyword("kasus") {
            self.parse_case_statement()
        } else if self.check(TokenType::Identifier) {
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
        let mut statements = Vec::new();
        
        // Empty statement list is valid
        if is_terminator(self) {
            return Ok(StatementList { statements });
        }

        loop {
            statements.push(self.parse_statement()?);
            
            // Check for semicolon
            if !self.match_token(&[TokenType::Semicolon]) {
                break; // No more statements
            }
            
            // Check if we've reached the terminator (e.g., 'selesai', 'sampai')
            if is_terminator(self) {
                break; // Allow trailing semicolon before terminator
            }
        }

        Ok(StatementList { statements })
    }

    /// 3. parse_compound_statement
    ///    'mulai' <statement-list> 'selesai'
    pub(super) fn parse_compound_statement(&mut self) -> Result<CompoundStatement, SyntaxError> {
        self.consume_keyword("mulai", "Mengharapkan 'mulai'.")?;
        let list = self.parse_statement_list(|p| p.check_keyword("selesai"))?;
        self.consume_keyword("selesai", "Mengharapkan 'selesai'.")?;
        
        Ok(CompoundStatement { statement_list: list })
    }

    /// 4. parse_if_statement
    ///    'jika' <expression> 'maka' <statement> ['selain-itu' <statement>]
    fn parse_if_statement(&mut self) -> Result<Statement, SyntaxError> {
        self.consume_keyword("jika", "Mengharapkan 'jika'.")?;
        let condition = self.parse_expression()?;
        self.consume_keyword("maka", "Mengharapkan 'maka' setelah kondisi.")?;
        let then_branch = Box::new(self.parse_statement()?);
        
        let else_branch = if self.match_keyword(&["selain-itu"]) {
            Some(Box::new(self.parse_statement()?))
        } else {
            None
        };

        Ok(Statement::If(IfStatement {
            condition,
            then_branch,
            else_branch,
        }))
    }

    /// 5. parse_while_statement
    ///    'selama' <expression> 'lakukan' <statement>
    fn parse_while_statement(&mut self) -> Result<Statement, SyntaxError> {
        self.consume_keyword("selama", "Mengharapkan 'selama'.")?;
        let condition = self.parse_expression()?;
        self.consume_keyword("lakukan", "Mengharapkan 'lakukan' setelah kondisi.")?;
        let body = Box::new(self.parse_statement()?);

        Ok(Statement::While(WhileStatement { condition, body }))
    }

    /// 6. parse_for_statement
    ///    'untuk' ID ':=' <expression> ('ke' | 'turun-ke') <expression> 'lakukan' <statement>
    fn parse_for_statement(&mut self) -> Result<Statement, SyntaxError> {
        self.consume_keyword("untuk", "Mengharapkan 'untuk'.")?;
        let counter = self.consume_token(TokenType::Identifier, "Mengharapkan variabel counter.")?.value.clone();
        self.consume_token(TokenType::AssignOperator, "Mengharapkan ':=' setelah variabel counter.")?;
        let start = self.parse_expression()?;
        
        let direction = if self.match_keyword(&["ke"]) {
            ForDirection::To
        } else if self.match_keyword(&["turun-ke"]) {
            ForDirection::DownTo
        } else {
            return Err(self.error("Mengharapkan 'ke' atau 'turun-ke' dalam for loop."));
        };

        let end = self.parse_expression()?;
        self.consume_keyword("lakukan", "Mengharapkan 'lakukan' setelah range for loop.")?;
        let body = Box::new(self.parse_statement()?);

        Ok(Statement::For(ForStatement {
            counter_variable: counter,
            start_value: start,
            end_value: end,
            direction,
            body,
        }))
    }

    /// 7. parse_repeat_statement
    ///    'ulangi' <statement-list> 'sampai' <expression>
    fn parse_repeat_statement(&mut self) -> Result<Statement, SyntaxError> {
        self.consume_keyword("ulangi", "Mengharapkan 'ulangi'.")?;
        let list = self.parse_statement_list(|p| p.check_keyword("sampai"))?;
        self.consume_keyword("sampai", "Mengharapkan 'sampai' di akhir repeat-until.")?;
        let condition = self.parse_expression()?;

        Ok(Statement::Repeat(RepeatStatement {
            statement_list: list,
            condition,
        }))
    }

    /// 8. parse_case_statement
    ///    'kasus' <expression> 'dari' <case-branch-list> ['selain-itu' <statement-list>] 'selesai'
    fn parse_case_statement(&mut self) -> Result<Statement, SyntaxError> {
        self.consume_keyword("kasus", "Mengharapkan 'kasus'.")?;
        let expr = self.parse_expression()?;
        self.consume_keyword("dari", "Mengharapkan 'dari' setelah ekspresi case.")?;
        
        let mut branches = Vec::new();
        while !self.check_keyword("selain-itu") && !self.check_keyword("selesai") {
            branches.push(self.parse_case_branch()?);
        }

        let else_branch = if self.match_keyword(&["selain-itu"]) {
            Some(self.parse_statement_list(|p| p.check_keyword("selesai"))?)
        } else {
            None
        };

        self.consume_keyword("selesai", "Mengharapkan 'selesai' di akhir case statement.")?;

        Ok(Statement::Case(CaseStatement {
            expression: expr,
            branches,
            else_branch,
        }))
    }

    /// 8a. parse_case_branch (Helper)
    ///     <expression-list> ':' <statement> ';'
    fn parse_case_branch(&mut self) -> Result<CaseBranch, SyntaxError> {
        let mut labels = vec![self.parse_expression()?];
        
        while self.match_token(&[TokenType::Comma]) {
            labels.push(self.parse_expression()?);
        }

        self.consume_token(TokenType::Colon, "Mengharapkan ':' setelah case label.")?;
        let statement = Box::new(self.parse_statement()?);
        self.consume_token(TokenType::Semicolon, "Mengharapkan ';' setelah case statement.")?;

        Ok(CaseBranch { labels, statement })
    }

    /// 9. parse_assignment_or_procedure_call
    ///    Ini adalah kasus paling rumit.
    ///    Bisa jadi: <expression> ':=' <expression>
    ///    ATAU       <expression> (yang harus berupa ProcedureCall)
    fn parse_assignment_or_procedure_call(&mut self) -> Result<Statement, SyntaxError> {
        let left_expr = self.parse_expression()?;

        // Check for assignment operator
        if self.match_token(&[TokenType::AssignOperator]) {
            let right_expr = self.parse_expression()?;
            return Ok(Statement::Assignment(AssignmentStatement {
                variable: left_expr,
                expression: right_expr,
            }));
        }

        // If no ':=', it must be a procedure call
        // Try to extract procedure call from expression
        if let Some(proc_call) = Self::expr_to_proc_call(left_expr) {
            Ok(Statement::ProcedureCall(proc_call))
        } else {
            Err(self.error("Statement tidak valid. Mengharapkan ':=' untuk assignment atau pemanggilan prosedur."))
        }
    }

    /// Helper: Try to convert an Expression to a ProcedureCallStatement
    /// Only succeeds if the expression is a plain function call (no operators)
    fn expr_to_proc_call(expr: Expression) -> Option<ProcedureCallStatement> {
        // Check if expression has no relational operators
        if !expr.rest.is_empty() {
            return None;
        }

        let simple_expr = &expr.initial_simple_expr;

        // Check if simple expression has no unary op and no additive operators
        if simple_expr.unary_op.is_some() || !simple_expr.rest.is_empty() {
            return None;
        }

        let term = &simple_expr.initial_term;

        // Check if term has no multiplicative operators
        if !term.rest.is_empty() {
            return None;
        }

        let factor = &term.initial_factor;

        // Check if factor is a FunctionCall
        match factor {
            Factor::FunctionCall(func_call) => {
                Some(ProcedureCallStatement {
                    procedure_name: func_call.function_name.clone(),
                    arguments: func_call.arguments.clone(),
                })
            }
            _ => None,
        }
    }
}