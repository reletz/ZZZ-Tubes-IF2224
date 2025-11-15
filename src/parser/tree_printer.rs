use super::parse_tree::*;
use crate::lexer::token_types::Token;

pub struct ParseTreePrinter {
    indent_level: usize,
    /// Stack untuk melacak apakah level indentasi induk adalah anak terakhir.
    /// Ini penting untuk menentukan apakah akan mencetak '│' (false) or ' ' (true).
    prefix_stack: Vec<bool>,
}

// Helper makro untuk PUSH/POP indentasi
macro_rules! with_indent {
    ($self:expr, $is_last:expr, $body:block) => {
        {
            $self.indent_level += 1;
            $self.prefix_stack.push($is_last);
            let result = $body;
            $self.indent_level -= 1;
            $self.prefix_stack.pop();
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
        
        // Ulangi prefix orang tua dari stack (semua kecuali yang terakhir)
        if let Some(parents) = self.prefix_stack.get(0..self.prefix_stack.len().saturating_sub(1)) {
            for &was_last in parents {
                if was_last {
                    prefix.push_str("    "); // 4 spasi biasa
                } else {
                    prefix.push_str("│   "); // 1 garis + 3 spasi biasa
                }
            }
        }

        // Tambahkan prefix level saat ini
        if is_last {
            prefix.push_str("└── ");
        } else {
            prefix.push_str("├── ");
        }
        prefix
    }

    // ===================================================================
    // TODO (CST): IMPLEMENTASI FUNGSI-FUNGSI PRINTER DI BAWAH INI
    // ===================================================================

    // --- Program ---

    pub fn print_program(&mut self, program: &Program) -> String {
        // TODO (CST):
        // 1. `let mut out = self.print_node("<program>", true);`
        // 2. `out += &with_indent!(self, true, { ... });`
        // 3. Di dalam block:
        //    a. `let mut body = String::new();`
        //    b. `body += &self.print_program_header(&program.header, false);`
        //    c. `body += &self.print_declaration_part(&program.declarations, false);`
        //    d. `body += &self.print_compound_statement(&program.body, false);`
        //    e. `body += &self.print_terminal_token(&program.dot, true);`
        //    f. `body`
        // 4. `out`
        unimplemented!()
    }

    fn print_program_header(&mut self, header: &ProgramHeader, is_last: bool) -> String {
        // TODO (CST):
        // 1. Cetak node "<program-header>"
        // 2. with_indent!
        // 3. print_terminal_token(&header.program_kw, false)
        // 4. print_terminal_token(&header.name, false)
        // 5. print_terminal_token(&header.semicolon, true)
        unimplemented!()
    }

    // --- Declarations ---

    fn print_declaration_part(&mut self, decls: &DeclarationPart, is_last: bool) -> String {
        // TODO (CST): Cetak node "<declaration-part>".
        // Loop `decls.const_declarations`, panggil `print_constant_declaration`
        // Loop `decls.type_declarations`, panggil `print_type_declaration`
        // Loop `decls.var_declarations`, panggil `print_variable_declaration`
        // Loop `decls.subprogram_declarations`, panggil `print_subprogram_declaration`
        // (Ingat atur `is_last` dengan benar untuk anak terakhir)
        unimplemented!()
    }

    fn print_constant_declaration(&mut self, decl: &ConstantDeclaration, is_last: bool) -> String {
        // TODO (CST): Cetak "<const-declaration>".
        // print_terminal_token(&decl.const_kw)
        // Loop `decl.constants`, panggil `print_constant_definition`
        unimplemented!()
    }

    fn print_constant_definition(&mut self, def: &ConstantDefinition, is_last: bool) -> String {
        // TODO (CST): (Mungkin tidak perlu node <const-definition>? Cek spek)
        // print_terminal_token(&def.name)
        // print_terminal_token(&def.equals_op)
        // print_expression(&def.value)
        // print_terminal_token(&def.semicolon)
        unimplemented!()
    }

    fn print_type_declaration(&mut self, decl: &TypeDeclaration, is_last: bool) -> String {
        // TODO (CST): Mirip `print_constant_declaration`
        unimplemented!()
    }

    fn print_type_definition(&mut self, def: &TypeDefinition, is_last: bool) -> String {
        // TODO (CST): Mirip `print_constant_definition`
        unimplemented!()
    }

    fn print_variable_declaration(&mut self, decl: &VariableDeclaration, is_last: bool) -> String {
        // TODO (CST): Cetak "<var-declaration>".
        // print_terminal_token(&decl.var_kw)
        // Loop `decl.groups`, panggil `print_variable_group`
        unimplemented!()
    }

    fn print_variable_group(&mut self, group: &VariableGroup, is_last: bool) -> String {
        // TODO (CST): (Mungkin tidak perlu node <var-group>?)
        // print_identifier_list(&group.identifiers)
        // print_terminal_token(&group.colon)
        // print_type(&group.var_type)
        // print_terminal_token(&group.semicolon)
        unimplemented!()
    }

    fn print_identifier_list(&mut self, list: &IdentifierList, is_last: bool) -> String {
        // TODO (CST): Cetak "<identifier-list>".
        // print_terminal_token(&list.initial_id)
        // Loop `list.rest`, print `comma` dan `id`
        unimplemented!()
    }

    fn print_subprogram_declaration(&mut self, decl: &SubprogramDeclaration, is_last: bool) -> String {
        // TODO (CST): (Router)
        // match decl { Procedure(p) => ..., Function(f) => ... }
        unimplemented!()
    }

    fn print_procedure_declaration(&mut self, decl: &ProcedureDeclaration, is_last: bool) -> String {
        // TODO (CST): Cetak "<procedure-declaration>".
        // print `proc_kw`, `name`, panggil `print_formal_parameter_list`,
        // print `header_semicolon`, panggil `print_declaration_part`,
        // panggil `print_compound_statement`, print `block_semicolon`.
        unimplemented!()
    }

    fn print_function_declaration(&mut self, decl: &FunctionDeclaration, is_last: bool) -> String {
        // TODO (CST): Mirip `print_procedure_declaration`, tapi ada `colon` dan `return_type`.
        unimplemented!()
    }

    fn print_formal_parameter_list(&mut self, list: &FormalParameterList, is_last: bool) -> String {
        // TODO (CST): Cetak "<formal-parameter-list>".
        // print `l_paren`.
        // `if let Some(initial) = &list.initial_param`, panggil `print_formal_parameter_group(initial)`.
        // Loop `list.rest`, print `semicolon` dan `group`.
        // print `r_paren`.
        unimplemented!()
    }

    fn print_formal_parameter_group(&mut self, group: &FormalParameterGroup, is_last: bool) -> String {
        // TODO (CST): (Mungkin tidak perlu node <parameter-group>?)
        // panggil `print_identifier_list`, print `colon`, panggil `print_type`.
        unimplemented!()
    }

    // --- Types ---

    fn print_type(&mut self, type_def: &Type, is_last: bool) -> String {
        // TODO (CST): (Router)
        // Cetak "<type>".
        // match type_def:
        //   Type::Integer(tok) => print_terminal_token(tok)
        //   ... (dan lainnya) ...
        //   Type::Array(arr) => print_array_type(arr)
        //   Type::Subrange(range) => print_range(range)
        //   Type::TypeIdentifier(tok) => print_terminal_token(tok)
        unimplemented!()
    }

    fn print_array_type(&mut self, decl: &ArrayType, is_last: bool) -> String {
        // TODO (CST): Cetak "<array-type>".
        // print `larik_kw`, `l_bracket`, panggil `print_range`,
        // print `r_bracket`, `dari_kw`, panggil `print_type(&decl.base_type)`.
        unimplemented!()
    }

    fn print_range(&mut self, range: &Range, is_last: bool) -> String {
        // TODO (CST): Cetak "<range>".
        // panggil `print_expression(&range.start)`, print `range_op`, panggil `print_expression(&range.end)`.
        unimplemented!()
    }

    // --- Statements ---

    fn print_statement(&mut self, stmt: &Statement, is_last: bool) -> String {
        // TODO (CST): (Router)
        // match stmt { Compound(s) => ..., Assignment(s) => ..., If(s) => ... }
        unimplemented!()
    }
    
    fn print_statement_list(&mut self, list: &StatementList, is_last: bool) -> String {
        // TODO (CST): Cetak "<statement-list>".
        // `if let Some(initial) = &list.initial_stmt`, panggil `print_statement(initial)`.
        // Loop `list.rest`, print `semicolon` dan `stmt`.
        unimplemented!()
    }

    fn print_compound_statement(&mut self, stmt: &CompoundStatement, is_last: bool) -> String {
        // TODO (CST): Cetak "<compound-statement>".
        // print `begin_kw`, panggil `print_statement_list`, print `end_kw`.
        unimplemented!()
    }

    fn print_assignment_statement(&mut self, stmt: &AssignmentStatement, is_last: bool) -> String {
        // TODO (CST): Cetak "<assignment-statement>".
        // panggil `print_expression(&stmt.variable)`, print `assign_op`, panggil `print_expression(&stmt.expression)`.
        unimplemented!()
    }

    fn print_if_statement(&mut self, stmt: &IfStatement, is_last: bool) -> String {
        // TODO (CST): Cetak "<if-statement>".
        // print `if_kw`, panggil `print_expression(&stmt.condition)`, print `then_kw`,
        // panggil `print_statement(&stmt.then_branch)`.
        // `if let Some(else_c) = &stmt.else_clause`, panggil `print_else_clause(else_c)`.
        unimplemented!()
    }

    fn print_else_clause(&mut self, clause: &ElseClause, is_last: bool) -> String {
        // TODO (CST): (Tidak perlu node <else-clause>?)
        // print `else_kw`, panggil `print_statement(&clause.statement)`.
        unimplemented!()
    }
    
    fn print_while_statement(&mut self, stmt: &WhileStatement, is_last: bool) -> String {
        // TODO (CST): Cetak "<while-statement>".
        // print `while_kw`, `condition`, `do_kw`, `body`.
        unimplemented!()
    }
    
    fn print_for_statement(&mut self, stmt: &ForStatement, is_last: bool) -> String {
        // TODO (CST): Cetak "<for-statement>".
        // print `for_kw`, `counter_variable`, `assign_op`, `start_value`,
        // `direction_kw`, `end_value`, `do_kw`, `body`.
        unimplemented!()
    }
    
    fn print_procedure_call_statement(&mut self, stmt: &ProcedureCallStatement, is_last: bool) -> String {
        // TODO (CST): Cetak "<procedure-call>".
        // Cukup panggil `print_function_call_node(&stmt.call, is_last)`.
        unimplemented!()
    }

    fn print_repeat_statement(&mut self, stmt: &RepeatStatement, is_last: bool) -> String {
        // TODO (CST): Cetak "<repeat-statement>".
        // print `repeat_kw`, `statement_list`, `until_kw`, `condition`.
        unimplemented!()
    }
    
    fn print_case_statement(&mut self, stmt: &CaseStatement, is_last: bool) -> String {
        // TODO (CST): Cetak "<case-statement>".
        // print `case_kw`, `expression`, `of_kw`.
        // Loop `stmt.branches`, panggil `print_case_branch`.
        // `if let Some(else_c) = &stmt.else_clause`, panggil `print_case_else_clause(else_c)`.
        // print `end_kw`.
        unimplemented!()
    }
    
    fn print_case_branch(&mut self, branch: &CaseBranch, is_last: bool) -> String {
        // TODO (CST): (Mungkin tidak perlu node <case-branch>?)
        // panggil `print_case_label_list`, print `colon`, `statement`, `semicolon`.
        unimplemented!()
    }

    fn print_case_label_list(&mut self, list: &CaseLabelList, is_last: bool) -> String {
        // TODO (CST): Cetak "<case-label-list>".
        // panggil `print_expression(&list.initial_label)`.
        // Loop `list.rest`, print `comma` dan `expr`.
        unimplemented!()
    }

    fn print_case_else_clause(&mut self, clause: &CaseElseClause, is_last: bool) -> String {
        // TODO (CST): (Mungkin tidak perlu node <case-else-clause>?)
        // print `else_kw`, panggil `print_statement_list(&clause.statement_list)`.
        unimplemented!()
    }

    // --- Expressions (Hierarki 4 Level) ---

    fn print_expression(&mut self, expr: &Expression, is_last: bool) -> String {
        // TODO (CST): Cetak "<expression>".
        // panggil `print_simple_expression(&expr.initial_simple_expr)`.
        // Loop `expr.rest`, print `op_token` dan `simple_expr`.
        unimplemented!()
    }

    fn print_simple_expression(&mut self, expr: &SimpleExpression, is_last: bool) -> String {
        // TODO (CST): Cetak "<simple-expression>".
        // `if let Some(op) = &expr.unary_op`, print `op`.
        // panggil `print_term(&expr.initial_term)`.
        // Loop `expr.rest`, print `op_token` dan `term`.
        unimplemented!()
    }

    fn print_term(&mut self, term: &Term, is_last: bool) -> String {
        // TODO (CST): Cetak "<term>".
        // panggil `print_factor(&term.initial_factor)`.
        // Loop `term.rest`, print `op_token` dan `factor`.
        unimplemented!()
    }

    fn print_factor(&mut self, factor: &Factor, is_last: bool) -> String {
        // TODO (CST): (Router) Cetak "<factor>".
        // match factor:
        //   Literal(lit) => print_literal_value(lit)
        //   Identifier(tok) => print_terminal_token(tok)
        //   FunctionCall(call) => print_function_call_node(call)
        //   ArrayAccess(acc) => print_array_access(acc)
        //   Parenthesized(p) => print_parenthesized_expression(p)
        //   Not(n) => print_not_factor(n)
        unimplemented!()
    }

    // --- Expression Helpers ---

    fn print_literal_value(&mut self, lit: &LiteralValue, is_last: bool) -> String {
        // TODO (CST): Cukup `print_terminal_token(&lit.token, is_last)`
        unimplemented!()
    }

    fn print_function_call_node(&mut self, call: &FunctionCallNode, is_last: bool) -> String {
        // TODO (CST): Cetak "<function-call>".
        // print `function_name`, `l_paren`.
        // `if let Some(args) = &call.arguments`, panggil `print_actual_parameter_list(args)`.
        // print `r_paren`.
        unimplemented!()
    }

    fn print_actual_parameter_list(&mut self, list: &ActualParameterList, is_last: bool) -> String {
        // TODO (CST): Cetak "<parameter-list>".
        // panggil `print_expression(&list.initial_arg)`.
        // Loop `list.rest`, print `comma` dan `expr`.
        unimplemented!()
    }

    fn print_array_access(&mut self, access: &ArrayAccess, is_last: bool) -> String {
        // TODO (CST): (Tidak perlu node <array-access>?)
        // panggil `print_expression(&access.array)`, print `l_bracket`,
        // panggil `print_expression(&access.index)`, print `r_bracket`.
        unimplemented!()
    }
    
    fn print_parenthesized_expression(&mut self, p: &ParenthesizedExpression, is_last: bool) -> String {
        // TODO (CST): (Tidak perlu node <parenthesized-expression>?)
        // print `l_paren`, panggil `print_expression(&p.expr)`, print `r_paren`.
        unimplemented!()
    }

    fn print_not_factor(&mut self, n: &NotFactor, is_last: bool) -> String {
        // TODO (CST): (Tidak perlu node <not-factor>?)
        // print `not_token`, panggil `print_factor(&n.factor)`.
        unimplemented!()
    }
}