use super::parse_tree::*;
use crate::lexer::token_types::Token;

pub struct ParseTreePrinter {
    indent_level: usize,
    // Stack untuk melacak apakah level indentasi induk adalah anak terakhir.
    // Ini penting untuk menentukan apakah akan mencetak '│' (false) or ' ' (true).
    prefix_stack: Vec<bool>,
}

// Helper makro untuk PUSH/POP indentasi
macro_rules! with_indent {
    ($self:expr, $is_last:expr, $body:block) => {
        {
            if $self.indent_level > 0 {
                $self.prefix_stack.push($is_last);
            }
            $self.indent_level += 1;

            let result = $body;

            $self.indent_level -= 1;
            if $self.indent_level > 0 {
                $self.prefix_stack.pop();
            }
            result
        }
    };
}

impl ParseTreePrinter {
    pub fn new() -> Self {
        ParseTreePrinter {
            indent_level: 0,
            prefix_stack: Vec::new(),
        }
    }

    // ===================================================================
    // INTI MESIN PRINTER
    // ===================================================================

    /// Mencetak node non-terminal (pembungkus)
    fn print_node(&mut self, label: &str, is_last: bool) -> String {
        format!("{}{}\n", self.get_prefix(is_last), label)
    }

    /// Mencetak node terminal (daun)
    fn print_terminal(&mut self, label: &str, is_last: bool) -> String {
        format!("{}{}\n", self.get_prefix(is_last), label)
    }

    /// Helper baru untuk mencetak token CST
    fn print_terminal_token(&mut self, token: &Token, is_last: bool) -> String {
        let label = format!("{}({})", token.token_type, token.value);
        self.print_terminal(&label, is_last)
    }

    /// Menghasilkan prefix indentasi (misal: "│   └── ")
    fn get_prefix(&self, is_last: bool) -> String {
        if self.indent_level == 0 {
            return String::new();
        }

        let mut prefix = String::new();

        if let Some(grandparents) = self.prefix_stack.get(0..self.prefix_stack.len().saturating_sub(1)) {
            for &was_last in grandparents {
                if was_last {
                    prefix.push_str("    ");
                } else {
                    prefix.push_str("│   ");
                }
            }
        }

        if let Some(&parent_was_last) = self.prefix_stack.last() {
            if parent_was_last {
                prefix.push_str("    ");
            } else {
                prefix.push_str("│   ");
            }
        }

        if is_last {
            prefix.push_str("└── ");
        } else {
            prefix.push_str("├── ");
        }

        prefix
    }

    // --- Program ---

    pub fn print_program(&mut self, program: &Program) -> String {
        let mut out = self.print_node("<program>", true); // Root node
        out += &with_indent!(self, true, { 
        let mut body = String::new();
        
        body += &self.print_program_header(&program.header, false);
        body += &self.print_declaration_part(&program.declarations, false);
        body += &self.print_compound_statement(&program.body, false);
        body += &self.print_terminal_token(&program.dot, true);
        
        body
    });
    out
    }

    fn print_program_header(&mut self, header: &ProgramHeader, is_last: bool) -> String {
        let mut out = self.print_node("<program-header>", is_last);
        out += &with_indent!(self, is_last, {
            let mut body = String::new();
            body += &self.print_terminal_token(&header.program_kw, false);
            body += &self.print_terminal_token(&header.name, false);
            body += &self.print_terminal_token(&header.semicolon, true);
            body
        });
        out
    }

    // --- Declarations ---

    fn print_declaration_part(&mut self, decls: &DeclarationPart, is_last: bool) -> String {
        let mut out = self.print_node("<declaration-part>", is_last);
        
        // Logic for determining the last child in each category
        let is_subprogram_last = !decls.subprogram_declarations.is_empty();
        let is_var_last = !decls.var_declarations.is_empty() && !is_subprogram_last;
        let is_type_last = !decls.type_declarations.is_empty() && !is_var_last && !is_subprogram_last;
        let is_const_last = !decls.const_declarations.is_empty() && !is_type_last && !is_var_last && !is_subprogram_last;

        out += &with_indent!(self, is_last, {
            let mut body = String::new();
            
            for (i, c) in decls.const_declarations.iter().enumerate() {
                let last = is_const_last && i == decls.const_declarations.len() - 1;
                body += &self.print_constant_declaration(c, last);
            }
            for (i, t) in decls.type_declarations.iter().enumerate() {
                let last = is_type_last && i == decls.type_declarations.len() - 1;
                body += &self.print_type_declaration(t, last);
            }
            for (i, v) in decls.var_declarations.iter().enumerate() {
                let last = is_var_last && i == decls.var_declarations.len() - 1;
                body += &self.print_variable_declaration(v, last);
            }
            for (i, s) in decls.subprogram_declarations.iter().enumerate() {
                let last = is_subprogram_last && i == decls.subprogram_declarations.len() - 1;
                body += &self.print_subprogram_declaration(s, last);
            }
            
            body
        });
        out
    }

    fn print_constant_declaration(&mut self, decl: &ConstantDeclaration, is_last: bool) -> String {
        let mut out = self.print_node("<const-declaration>", is_last);
        out += &with_indent!(self, is_last, {
            let mut body = String::new();
            let num_defs = decl.constants.len();
            body += &self.print_terminal_token(&decl.const_kw, num_defs == 0);
            
            for (i, def) in decl.constants.iter().enumerate() {
                let is_def_last = i == num_defs - 1;
                body += &self.print_constant_definition(def, is_def_last);
            }
            body
        });
        out
    }

    fn print_constant_definition(&mut self, def: &ConstantDefinition, is_last: bool) -> String {
        let mut out = String::new();
        out += &self.print_terminal_token(&def.name, false);
        out += &self.print_terminal_token(&def.equals_op, false);
        out += &self.print_expression(&def.value, false);
        out += &self.print_terminal_token(&def.semicolon, is_last);
        out
    }

    fn print_type_declaration(&mut self, decl: &TypeDeclaration, is_last: bool) -> String {
        let mut out = self.print_node("<type-declaration>", is_last);
        out += &with_indent!(self, is_last, {
            let mut body = String::new();
            let num_defs = decl.definitions.len();
            body += &self.print_terminal_token(&decl.type_kw, num_defs == 0);
            
            for (i, def) in decl.definitions.iter().enumerate() {
                let is_def_last = i == num_defs - 1;
                body += &self.print_type_definition(def, is_def_last);
            }
            body
        });
        out
    }

    fn print_type_definition(&mut self, def: &TypeDefinition, is_last: bool) -> String {
        let mut out = String::new();
        out += &self.print_terminal_token(&def.name, false);
        out += &self.print_terminal_token(&def.equals_op, false);
        out += &self.print_type(&def.type_def, false);
        out += &self.print_terminal_token(&def.semicolon, is_last);
        out
    }

    fn print_variable_declaration(&mut self, decl: &VariableDeclaration, is_last: bool) -> String {
        let mut out = self.print_node("<var-declaration>", is_last);
        out += &with_indent!(self, is_last, {
            let mut body = String::new();
            let num_groups = decl.groups.len();
            body += &self.print_terminal_token(&decl.var_kw, num_groups == 0);
            
            for (i, group) in decl.groups.iter().enumerate() {
                let is_group_last = i == num_groups - 1;
                body += &self.print_variable_group(group, is_group_last);
            }
            body
        });
        out
    }

    fn print_variable_group(&mut self, group: &VariableGroup, is_last: bool) -> String {
        let mut out = String::new();
        out += &self.print_identifier_list(&group.identifiers, false);
        out += &self.print_terminal_token(&group.colon, false);
        out += &self.print_type(&group.var_type, false);
        out += &self.print_terminal_token(&group.semicolon, is_last);
        out
    }

    fn print_identifier_list(&mut self, list: &IdentifierList, is_last: bool) -> String {
        let mut out = self.print_node("<identifier-list>", is_last);
        out += &with_indent!(self, is_last, {
            let mut body = String::new();
            let num_rest = list.rest.len();
            body += &self.print_terminal_token(&list.initial_id, num_rest == 0);
            
            for (i, (comma, id)) in list.rest.iter().enumerate() {
                let is_item_last = i == num_rest - 1;
                body += &self.print_terminal_token(comma, false);
                body += &self.print_terminal_token(id, is_item_last);
            }
            body
        });
        out
    }

    fn print_subprogram_declaration(&mut self, decl: &SubprogramDeclaration, is_last: bool) -> String {
        match decl {
            SubprogramDeclaration::Procedure(p) => self.print_procedure_declaration(p, is_last),
            SubprogramDeclaration::Function(f) => self.print_function_declaration(f, is_last),
        }
    }

    fn print_procedure_declaration(&mut self, decl: &ProcedureDeclaration, is_last: bool) -> String {
        let mut out = self.print_node("<procedure-declaration>", is_last);
        out += &with_indent!(self, is_last, {
            let mut body = String::new();
            body += &self.print_terminal_token(&decl.proc_kw, false);
            body += &self.print_terminal_token(&decl.name, false);
            body += &self.print_formal_parameter_list(&decl.parameters, false);
            body += &self.print_terminal_token(&decl.header_semicolon, false);
            body += &self.print_declaration_part(&decl.declarations, false);
            body += &self.print_compound_statement(&decl.body, false);
            body += &self.print_terminal_token(&decl.block_semicolon, true);
            body
        });
        out
    }

    fn print_function_declaration(&mut self, decl: &FunctionDeclaration, is_last: bool) -> String {
        let mut out = self.print_node("<function-declaration>", is_last);
        out += &with_indent!(self, is_last, {
            let mut body = String::new();
            body += &self.print_terminal_token(&decl.func_kw, false);
            body += &self.print_terminal_token(&decl.name, false);
            body += &self.print_formal_parameter_list(&decl.parameters, false);
            body += &self.print_terminal_token(&decl.colon, false);
            body += &self.print_type(&decl.return_type, false);
            body += &self.print_terminal_token(&decl.header_semicolon, false);
            body += &self.print_declaration_part(&decl.declarations, false);
            body += &self.print_compound_statement(&decl.body, false);
            body += &self.print_terminal_token(&decl.block_semicolon, true);
            body
        });
        out
    }

    fn print_formal_parameter_list(&mut self, list: &FormalParameterList, is_last: bool) -> String {
        let mut out = self.print_node("<formal-parameter-list>", is_last);
        out += &with_indent!(self, is_last, {
            let mut body = String::new();
            
            let has_initial = list.initial_param.is_some();
            let num_rest = list.rest.len();
            let is_empty = !has_initial;

            body += &self.print_terminal_token(&list.l_paren, is_empty);
            
            if let Some(initial) = &list.initial_param {
                body += &self.print_formal_parameter_group(initial, num_rest == 0);
            }
            
            for (i, (semicolon, group)) in list.rest.iter().enumerate() {
                let is_item_last = i == num_rest - 1;
                body += &self.print_terminal_token(semicolon, false);
                body += &self.print_formal_parameter_group(group, is_item_last);
            }
            
            body += &self.print_terminal_token(&list.r_paren, true);
            body
        });
        out
    }

    fn print_formal_parameter_group(&mut self, group: &FormalParameterGroup, is_last: bool) -> String {
        let mut out = String::new();
        if let Some(token) = &group.var_kw {
            out += &self.print_terminal_token(token, false);
        }
        
        out += &self.print_identifier_list(&group.identifiers, false);
        out += &self.print_terminal_token(&group.colon, false);
        out += &self.print_type(&group.var_type, is_last);
        out
    }

    // --- Types ---

    fn print_type(&mut self, type_def: &Type, is_last: bool) -> String {
        let mut out = self.print_node("<type>", is_last);
        out += &with_indent!(self, is_last, {
            // Setiap anak adalah `is_last = true` karena hanya ada satu
            match type_def {
                Type::Integer(tok) => self.print_terminal_token(tok, true),
                Type::Real(tok) => self.print_terminal_token(tok, true),
                Type::Boolean(tok) => self.print_terminal_token(tok, true),
                Type::String(tok) => self.print_terminal_token(tok, true),
                Type::Char(tok) => self.print_terminal_token(tok, true),
                Type::Array(arr) => self.print_array_type(arr, true),
                Type::Subrange(range) => self.print_range(range, true),
                Type::TypeIdentifier(tok) => self.print_terminal_token(tok, true),
            }
        });
        out
    }

    fn print_array_type(&mut self, decl: &ArrayType, is_last: bool) -> String {
        let mut out = self.print_node("<array-type>", is_last);
        out += &with_indent!(self, is_last, {
            let mut body = String::new();
            body += &self.print_terminal_token(&decl.larik_kw, false);
            body += &self.print_terminal_token(&decl.l_bracket, false);
            body += &self.print_type(&decl.index_type, false);
            body += &self.print_terminal_token(&decl.r_bracket, false);
            body += &self.print_terminal_token(&decl.dari_kw, false);
            body += &self.print_type(&decl.base_type, true);
            body
        });
        out
    }

    fn print_range(&mut self, range: &Range, is_last: bool) -> String {
        let mut out = self.print_node("<range>", is_last);
        out += &with_indent!(self, is_last, {
            let mut body = String::new();
            body += &self.print_expression(&range.start, false);
            body += &self.print_terminal_token(&range.range_op, false);
            body += &self.print_expression(&range.end, true);
            body
        });
        out
    }

    // --- Statements ---

    fn print_statement(&mut self, stmt: &Statement, is_last: bool) -> String {
        match stmt {
            Statement::Compound(s) => self.print_compound_statement(s, is_last),
            Statement::Assignment(s) => self.print_assignment_statement(s, is_last),
            Statement::If(s) => self.print_if_statement(s, is_last),
            Statement::While(s) => self.print_while_statement(s, is_last),
            Statement::For(s) => self.print_for_statement(s, is_last),
            Statement::ProcedureCall(s) => self.print_procedure_call_statement(s, is_last),
            Statement::Repeat(s) => self.print_repeat_statement(s, is_last),
            Statement::Case(s) => self.print_case_statement(s, is_last),
        }
    }
    
    fn print_statement_list(&mut self, list: &StatementList, is_last: bool) -> String {
        let has_initial = list.initial_stmt.is_some();
        
        // Jika list kosong, jangan cetak node sama sekali
        if !has_initial {
            return String::new();
        }
        
        let mut out = self.print_node("<statement-list>", is_last);
        out += &with_indent!(self, is_last, {
            let mut body = String::new();
            
            let num_rest = list.rest.len();
            let has_trailing = list.trailing_semicolon.is_some();

            if let Some(initial) = &list.initial_stmt {
                body += &self.print_statement(initial, num_rest == 0 && !has_trailing);
            }
            
            for (i, (semicolon, stmt)) in list.rest.iter().enumerate() {
                let is_item_last = (i == num_rest - 1) && !has_trailing;

                body += &self.print_terminal_token(semicolon, false);

                body += &self.print_statement(stmt, is_item_last);
            }
            if let Some(semi) = &list.trailing_semicolon {
                body += &self.print_terminal_token(semi, true);
            }
            body
        });
        out
    }

    fn print_compound_statement(&mut self, stmt: &CompoundStatement, is_last: bool) -> String {
        let mut out = self.print_node("<compound-statement>", is_last);
        out += &with_indent!(self, is_last, {
            let mut body = String::new();
            body += &self.print_terminal_token(&stmt.begin_kw, false);
            body += &self.print_statement_list(&stmt.statement_list, false);
            body += &self.print_terminal_token(&stmt.end_kw, true);
            body
        });
        out
    }

    fn print_assignment_statement(&mut self, stmt: &AssignmentStatement, is_last: bool) -> String {
        let mut out = self.print_node("<assignment-statement>", is_last);
        out += &with_indent!(self, is_last, {
            let mut body = String::new();
            body += &self.print_expression(&stmt.variable, false);
            body += &self.print_terminal_token(&stmt.assign_op, false);
            body += &self.print_expression(&stmt.expression, true);
            body
        });
        out
    }

    fn print_if_statement(&mut self, stmt: &IfStatement, is_last: bool) -> String {
        let mut out = self.print_node("<if-statement>", is_last);
        out += &with_indent!(self, is_last, {
            let mut body = String::new();
            let has_else = stmt.else_clause.is_some();
            
            body += &self.print_terminal_token(&stmt.if_kw, false);
            body += &self.print_expression(&stmt.condition, false);
            body += &self.print_terminal_token(&stmt.then_kw, false);
            body += &self.print_statement(&stmt.then_branch, !has_else);
            
            if let Some(else_c) = &stmt.else_clause {
                body += &self.print_else_clause(else_c, true);
            }
            body
        });
        out
    }

    fn print_else_clause(&mut self, clause: &ElseClause, is_last: bool) -> String {
        let mut out = String::new();
        out += &self.print_terminal_token(&clause.else_kw, false);
        out += &self.print_statement(&clause.statement, is_last);
        out
    }
    
    fn print_while_statement(&mut self, stmt: &WhileStatement, is_last: bool) -> String {
        let mut out = self.print_node("<while-statement>", is_last);
        out += &with_indent!(self, is_last, {
            let mut body = String::new();
            body += &self.print_terminal_token(&stmt.while_kw, false);
            body += &self.print_expression(&stmt.condition, false);
            body += &self.print_terminal_token(&stmt.do_kw, false);
            body += &self.print_statement(&stmt.body, true);
            body
        });
        out
    }
    
    fn print_for_statement(&mut self, stmt: &ForStatement, is_last: bool) -> String {
        let mut out = self.print_node("<for-statement>", is_last);
        out += &with_indent!(self, is_last, {
            let mut body = String::new();
            body += &self.print_terminal_token(&stmt.for_kw, false);
            body += &self.print_terminal_token(&stmt.counter_variable, false);
            body += &self.print_terminal_token(&stmt.assign_op, false);
            body += &self.print_expression(&stmt.start_value, false);
            body += &self.print_terminal_token(&stmt.direction_kw, false);
            body += &self.print_expression(&stmt.end_value, false);
            body += &self.print_terminal_token(&stmt.do_kw, false);
            body += &self.print_statement(&stmt.body, true);
            body
        });
        out
    }
    
    fn print_procedure_call_statement(&mut self, stmt: &ProcedureCallStatement, is_last: bool) -> String {
        let mut out = self.print_node("<procedure-call>", is_last);
        out += &with_indent!(self, is_last, {
            let call = &stmt.call;
            let mut body = String::new();
            
            body += &self.print_terminal_token(&call.function_name, false);
            body += &self.print_terminal_token(&call.l_paren, false);
            
            if let Some(args) = &call.arguments {
                body += &self.print_actual_parameter_list(args, false);
            }
            
            body += &self.print_terminal_token(&call.r_paren, true);

            body
        });
        out
    }

    fn print_repeat_statement(&mut self, stmt: &RepeatStatement, is_last: bool) -> String {
        let mut out = self.print_node("<repeat-statement>", is_last);
        out += &with_indent!(self, is_last, {
            let mut body = String::new();
            body += &self.print_terminal_token(&stmt.repeat_kw, false);
            body += &self.print_statement_list(&stmt.statement_list, false);
            body += &self.print_terminal_token(&stmt.until_kw, false);
            body += &self.print_expression(&stmt.condition, true);
            body
        });
        out
    }
    
    fn print_case_statement(&mut self, stmt: &CaseStatement, is_last: bool) -> String {
        let mut out = self.print_node("<case-statement>", is_last);
        out += &with_indent!(self, is_last, {
            let mut body = String::new();
            
            body += &self.print_terminal_token(&stmt.case_kw, false);
            body += &self.print_expression(&stmt.expression, false);
            body += &self.print_terminal_token(&stmt.of_kw, false);
            
            for branch in &stmt.branches {
                body += &self.print_case_branch(branch, false);
            }
            
            if let Some(else_c) = &stmt.else_clause {
                body += &self.print_case_else_clause(else_c, false);
            }
            
            body += &self.print_terminal_token(&stmt.end_kw, true);
            body
        });
        out
    }
    
    fn print_case_branch(&mut self, branch: &CaseBranch, is_last: bool) -> String {
        let mut out = String::new();
        out += &self.print_case_label_list(&branch.labels, false);
        out += &self.print_terminal_token(&branch.colon, false);
        out += &self.print_statement(&branch.statement, false);
        out += &self.print_terminal_token(&branch.semicolon, is_last);
        out
    }

    fn print_case_label_list(&mut self, list: &CaseLabelList, is_last: bool) -> String {
        let mut out = self.print_node("<case-label-list>", is_last);
        out += &with_indent!(self, is_last, {
            let mut body = String::new();
            let num_rest = list.rest.len();
            body += &self.print_expression(&list.initial_label, num_rest == 0);
            
            for (i, (comma, expr)) in list.rest.iter().enumerate() {
                let is_item_last = i == num_rest - 1;
                body += &self.print_terminal_token(comma, false);
                body += &self.print_expression(expr, is_item_last);
            }
            body
        });
        out
    }

    fn print_case_else_clause(&mut self, clause: &CaseElseClause, is_last: bool) -> String {
        let mut out = String::new();
        out += &self.print_terminal_token(&clause.else_kw, false);
        out += &self.print_statement_list(&clause.statement_list, is_last);
        out
    }

    // --- Expressions (Hierarki 4 Level) ---

    fn print_expression(&mut self, expr: &Expression, is_last: bool) -> String {
        let mut out = self.print_node("<expression>", is_last);
        out += &with_indent!(self, is_last, {
            let mut body = String::new();
            let num_rest = expr.rest.len();
            body += &self.print_simple_expression(&expr.initial_simple_expr, num_rest == 0);
            
            for (i, (op, simple_expr)) in expr.rest.iter().enumerate() {
                let is_item_last = i == num_rest - 1;
                body += &self.print_terminal_token(op, false);
                body += &self.print_simple_expression(simple_expr, is_item_last);
            }
            body
        });
        out
    }

    fn print_simple_expression(&mut self, expr: &SimpleExpression, is_last: bool) -> String {
        let mut out = self.print_node("<simple-expression>", is_last);
        out += &with_indent!(self, is_last, {
            let mut body = String::new();
            let num_rest = expr.rest.len();

            body += &self.print_term(&expr.initial_term, num_rest == 0);
            
            for (i, (op, term)) in expr.rest.iter().enumerate() {
                let is_item_last = i == num_rest - 1;
                body += &self.print_terminal_token(op, false);
                body += &self.print_term(term, is_item_last);
            }
            body
        });
        out
    }

    fn print_term(&mut self, term: &Term, is_last: bool) -> String {
        let mut out = self.print_node("<term>", is_last);
        out += &with_indent!(self, is_last, {
            let mut body = String::new();
            let num_rest = term.rest.len();
            body += &self.print_factor(&term.initial_factor, num_rest == 0);
            
            for (i, (op, factor)) in term.rest.iter().enumerate() {
                let is_item_last = i == num_rest - 1;
                body += &self.print_terminal_token(op, false);
                body += &self.print_factor(factor, is_item_last);
            }
            body
        });
        out
    }

    fn print_factor(&mut self, factor: &Factor, is_last: bool) -> String {
        let mut out = self.print_node("<factor>", is_last);
        out += &with_indent!(self, is_last, {
            // Setiap anak adalah `is_last = true` karena hanya ada satu
            match factor {
                Factor::Literal(lit) => self.print_literal_value(lit, true),
                Factor::Identifier(tok) => self.print_terminal_token(tok, true),
                Factor::FunctionCall(call) => self.print_function_call_node(call, true),
                Factor::ArrayAccess(acc) => self.print_array_access(acc, true),
                Factor::Parenthesized(p) => self.print_parenthesized_expression(p, true),
                Factor::Not(n) => self.print_not_factor(n, true),
                Factor::ArithmeticUnary(u) => self.print_arithmetic_unary_factor(u, true),
            }
        });
        out
    }

    // --- Expression Helpers ---

    fn print_arithmetic_unary_factor(&mut self, u: &ArithmeticUnaryFactor, is_last: bool) -> String {
        let mut out = String::new();
        out += &self.print_terminal_token(&u.op, false);
        out += &self.print_factor(&u.factor, is_last);
        out
    }

    fn print_literal_value(&mut self, lit: &LiteralValue, is_last: bool) -> String {
        self.print_terminal_token(&lit.token, is_last)
    }

    fn print_function_call_node(&mut self, call: &FunctionCallNode, is_last: bool) -> String {
        let mut out = self.print_node("<function-call>", is_last);
        out += &with_indent!(self, is_last, {
            let mut body = String::new();
            let has_args = call.arguments.is_some();
            
            body += &self.print_terminal_token(&call.function_name, false);
            body += &self.print_terminal_token(&call.l_paren, !has_args);
            
            if let Some(args) = &call.arguments {
                body += &self.print_actual_parameter_list(args, false);
            }
            
            body += &self.print_terminal_token(&call.r_paren, true);
            body
        });
        out
    }

    fn print_actual_parameter_list(&mut self, list: &ActualParameterList, is_last: bool) -> String {
        let mut out = self.print_node("<parameter-list>", is_last);
        out += &with_indent!(self, is_last, {
            let mut body = String::new();
            let num_rest = list.rest.len();
            body += &self.print_expression(&list.initial_arg, num_rest == 0);
            
            for (i, (comma, expr)) in list.rest.iter().enumerate() {
                let is_item_last = i == num_rest - 1;
                body += &self.print_terminal_token(comma, false);
                body += &self.print_expression(expr, is_item_last);
            }
            body
        });
        out
    }

    fn print_array_access(&mut self, access: &ArrayAccess, is_last: bool) -> String {
        let mut out = String::new();
        out += &self.print_expression(&access.array, false);
        out += &self.print_terminal_token(&access.l_bracket, false);
        out += &self.print_expression(&access.index, false);
        out += &self.print_terminal_token(&access.r_bracket, is_last);
        out
    }
    
    fn print_parenthesized_expression(&mut self, p: &ParenthesizedExpression, is_last: bool) -> String {
        let mut out = String::new();
        out += &self.print_terminal_token(&p.l_paren, false);
        out += &self.print_expression(&p.expr, false);
        out += &self.print_terminal_token(&p.r_paren, is_last);
        out
    }

    fn print_not_factor(&mut self, n: &NotFactor, is_last: bool) -> String {
        let mut out = String::new();
        out += &self.print_terminal_token(&n.not_token, false);
        out += &self.print_factor(&n.factor, is_last);
        out
    }
}