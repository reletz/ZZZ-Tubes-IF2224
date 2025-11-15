use super::parse_tree::*;

pub struct ParseTreePrinter {
    indent_level: usize,
    /// Stack untuk melacak apakah level indentasi induk adalah anak terakhir.
    /// Ini penting untuk menentukan apakah akan mencetak '│' (false) or ' ' (true).
    prefix_stack: Vec<bool>,
}

impl ParseTreePrinter {
    pub fn new() -> Self {
        ParseTreePrinter {
            indent_level: 0,
            prefix_stack: Vec::new(),
        }
    }

    // ===================================================================
    // INTI MESIN PRINTER (JANGAN DIUBAH)
    // ===================================================================

    /// Mencetak node non-terminal (pembungkus)
    fn print_node(&mut self, label: &str, is_last: bool) -> String {
        format!("{}{}\n", self.get_prefix(is_last), label)
    }

    /// Mencetak node terminal (daun)
    fn print_terminal(&mut self, label: &str, is_last: bool) -> String {
        format!("{}{}\n", self.get_prefix(is_last), label)
    }

    /// Menghasilkan prefix indentasi (misal: "│   └── ")
    fn get_prefix(&self, is_last: bool) -> String {
        if self.indent_level == 0 {
            return String::new();
        }
        
        let mut prefix = String::new();
        
        // Ulangi prefix orang tua dari stack
        for &was_last in &self.prefix_stack {
            if was_last {
                prefix.push_str("    "); // 4 spasi biasa
            } else {
                prefix.push_str("│   "); // 1 garis + 3 spasi biasa
            }
        }

        // Tambahkan prefix level saat ini
        if is_last {
            prefix.push_str("└── "); // 1 garis + 2 spasi biasa
        } else {
            prefix.push_str("├── "); // 1 garis + 2 spasi biasa
        }
        prefix
    }

    // ===================================================================
    // TODO: IMPLEMENTASI FUNGSI-FUNGSI PRINTER DI BAWAH INI
    // ===================================================================

    // --- Program ---

    pub fn print_program(&mut self, program: &Program) -> String {
        // TODO: Implement print_program
        // 1. Cetak "<program>" (is_last: true, karena ini root)
        // 2. PUSH/POP
        // 3. Panggil `print_program_header(&program.header, false)`
        // 4. Panggil `print_declaration_part(&program.declarations, false)`
        // 5. Panggil `print_compound_statement(&program.body, false)`
        // 6. Panggil `print_terminal("DOT(.)", true)`
        unimplemented!()
    }

    fn print_program_header(&mut self, header: &ProgramHeader, is_last: bool) -> String {
        // TODO: Implement print_program_header
        // 1. Cetak "<program-header>"
        // 2. PUSH/POP
        // 3. Panggil `print_terminal("KEYWORD(program)", false)`
        // 4. Panggil `print_terminal(&format!("IDENTIFIER({})", header.name), false)`
        // 5. Panggil `print_terminal("SEMICOLON(;)", true)`
        unimplemented!()
    }

    // --- Declarations ---

    fn print_declaration_part(&mut self, decls: &DeclarationPart, is_last: bool) -> String {
        // TODO: Implement print_declaration_part
        // 1. Cetak "<declaration-part>"
        // 2. PUSH/POP
        // 3. Loop `decls.const_declarations`, panggil `print_constant_declaration`
        // 4. Loop `decls.type_declarations`, panggil `print_type_declaration`
        // 5. Loop `decls.var_declarations`, panggil `print_variable_declaration`
        // 6. Loop `decls.subprogram_declarations`, panggil `print_subprogram_declaration`
        //    (Ingat atur `is_last` dengan benar untuk anak terakhir)
        unimplemented!()
    }

    fn print_constant_declaration(&mut self, decl: &ConstantDeclaration, is_last: bool) -> String {
        // TODO: Implement print_constant_declaration
        // 1. Cetak "<const-declaration>"
        // 2. PUSH/POP
        // 3. Panggil `print_terminal("KEYWORD(konstanta)", ...)`
        // 4. Loop `decl.constants`, panggil `print_constant_definition`
        unimplemented!()
    }

    fn print_constant_definition(&mut self, def: &ConstantDefinition, is_last: bool) -> String {
        // TODO: Implement print_constant_definition
        // 1. Cetak "<const-definition>"
        // 2. PUSH/POP
        // 3. Cetak `IDENTIFIER(...)`, `OPERATOR(=)`, panggil `print_expression(...)`, cetak `SEMICOLON(;)`.
        unimplemented!()
    }

    fn print_type_declaration(&mut self, decl: &TypeDeclaration, is_last: bool) -> String {
        unimplemented!() // TODO: Mirip `print_constant_declaration`
    }

    fn print_type_definition(&mut self, def: &TypeDefinition, is_last: bool) -> String {
        unimplemented!() // TODO: Mirip `print_constant_definition`
    }

    fn print_variable_declaration(&mut self, decl: &VariableDeclaration, is_last: bool) -> String {
        // TODO: Implement print_variable_declaration
        // 1. Cetak "<var-declaration>"
        // 2. PUSH/POP
        // 3. Cetak `KEYWORD(variabel)`
        // 4. Loop `decl.groups`, panggil `print_variable_group`
        unimplemented!()
    }

    fn print_variable_group(&mut self, group: &VariableGroup, is_last: bool) -> String {
        // TODO: Implement print_variable_group
        // 1. (Mungkin tidak perlu node <var-group> terpisah? Cek spek. Jika perlu, PUSH/POP)
        // 2. Panggil `print_identifier_list(&group.identifiers, false)`
        // 3. Cetak `COLON(:)`
        // 4. Panggil `print_type(&group.var_type, false)`
        // 5. Cetak `SEMICOLON(;) `
        unimplemented!()
    }

    fn print_identifier_list(&mut self, list: &IdentifierList, is_last: bool) -> String {
        // TODO: Implement print_identifier_list
        // 1. Cetak "<identifier-list>"
        // 2. PUSH/POP
        // 3. Loop `list.identifiers`, cetak `IDENTIFIER(...)` dan `COMMA(,)`
        unimplemented!()
    }

    fn print_subprogram_declaration(&mut self, decl: &SubprogramDeclaration, is_last: bool) -> String {
        // TODO: Implement print_subprogram_declaration (Router)
        // 1. `match decl`:
        //    - `SubprogramDeclaration::Procedure(p)` -> `print_procedure_declaration(p, is_last)`
        //    - `SubprogramDeclaration::Function(f)` -> `print_function_declaration(f, is_last)`
        unimplemented!()
    }

    fn print_procedure_declaration(&mut self, decl: &ProcedureDeclaration, is_last: bool) -> String {
        // TODO: Implement print_procedure_declaration
        // 1. Cetak "<procedure-declaration>"
        // 2. PUSH/POP
        // 3. Cetak `KEYWORD(prosedur)`, `IDENTIFIER(...)`
        // 4. Panggil `print_formal_parameter_list(&decl.parameters, ...)`
        // 5. Cetak `SEMICOLON(;) `
        // 6. Panggil `print_declaration_part(&decl.declarations, ...)`
        // 7. Panggil `print_compound_statement(&decl.body, ...)`
        // 8. Cetak `SEMICOLON(;) `
        unimplemented!()
    }

    fn print_function_declaration(&mut self, decl: &FunctionDeclaration, is_last: bool) -> String {
        // TODO: Implement print_function_declaration
        // (Mirip prosedur, tapi ada `COLON(:)` dan `print_type(&decl.return_type, ...)`)
        unimplemented!()
    }

    fn print_formal_parameter_list(&mut self, list: &FormalParameterList, is_last: bool) -> String {
        // TODO: Implement print_formal_parameter_list
        // 1. Cetak "<formal-parameter-list>"
        // 2. PUSH/POP
        // 3. Cetak `LPARENTHESIS(()`
        // 4. Loop `list.parameters`, panggil `print_formal_parameter_group` dan `SEMICOLON(;) `
        // 5. Cetak `RPARENTHESIS())`
        unimplemented!()
    }

    fn print_formal_parameter_group(&mut self, group: &FormalParameterGroup, is_last: bool) -> String {
        // TODO: Implement print_formal_parameter_group
        // (Mungkin tidak perlu node <parameter-group> terpisah? Cek spek)
        // 1. Panggil `print_identifier_list(&group.identifiers, ...)`
        // 2. Cetak `COLON(:)`
        // 3. Panggil `print_type(&group.var_type, ...)`
        unimplemented!()
    }

    // --- Types ---

    fn print_type(&mut self, type_def: &Type, is_last: bool) -> String {
        // TODO: Implement print_type (Router)
        // 1. Cetak "<type>"
        // 2. PUSH/POP
        // 3. `match type_def`:
        //    - `Type::Integer` -> `print_terminal("KEYWORD(integer)", true)`
        //    - `Type::Real` -> `print_terminal("KEYWORD(real)", true)`
        //    - (Sama untuk Boolean, String, Char)
        //    - `Type::TypeIdentifier(name)` -> `print_terminal(&format!("IDENTIFIER({})", name), true)`
        //    - `Type::Array(arr_def)` -> `print_array_type(arr_def, true)`
        //    - `Type::Subrange(range)` -> `print_range(range, true)`
        unimplemented!()
    }

    fn print_array_type(&mut self, decl: &ArrayType, is_last: bool) -> String {
        // TODO: Implement print_array_type
        // 1. Cetak "<array-type>"
        // 2. PUSH/POP
        // 3. Cetak `KEYWORD(larik)`, `LBRACKET([)`
        // 4. Panggil `print_range(&decl.range, ...)`
        // 5. Cetak `RBRACKET(])`, `KEYWORD(dari)`
        // 6. Panggil `print_type(&decl.base_type, ...)` (REKURSIF)
        unimplemented!()
    }

    fn print_range(&mut self, range: &Range, is_last: bool) -> String {
        // TODO: Implement print_range
        // 1. Cetak "<range>"
        // 2. PUSH/POP
        // 3. Panggil `print_expression(&range.start, ...)`
        // 4. Cetak `RANGE_OPERATOR(..)`
        // 5. Panggil `print_expression(&range.end, ...)`
        unimplemented!()
    }

    // --- Statements ---

    fn print_compound_statement(&mut self, stmt: &CompoundStatement, is_last: bool) -> String {
        // TODO: Implement print_compound_statement
        // 1. Cetak "<compound-statement>"
        // 2. PUSH/POP
        // 3. Cetak `KEYWORD(mulai)`
        // 4. Panggil `print_statement_list(&stmt.statement_list, ...)`
        // 5. Cetak `KEYWORD(selesai)`
        unimplemented!()
    }

    fn print_statement_list(&mut self, list: &StatementList, is_last: bool) -> String {
        // TODO: Implement print_statement_list
        // 1. Cetak "<statement-list>"
        // 2. PUSH/POP
        // 3. Loop `list.statements`, panggil `print_statement` dan `SEMICOLON(;) `
        unimplemented!()
    }

    fn print_statement(&mut self, stmt: &Statement, is_last: bool) -> String {
        // TODO: Implement print_statement (Router)
        // 1. `match stmt`:
        //    - `Statement::Compound(s)` -> `print_compound_statement(s, is_last)`
        //    - `Statement::If(s)` -> `print_if_statement(s, is_last)`
        //    - (dan seterusnya untuk semua varian Statement)
        unimplemented!()
    }

    fn print_assignment_statement(&mut self, stmt: &AssignmentStatement, is_last: bool) -> String {
        // TODO: Implement print_assignment_statement
        // 1. Cetak "<assignment-statement>"
        // 2. PUSH/POP
        // 3. Panggil `print_expression(&stmt.variable, ...)`
        // 4. Cetak `ASSIGN_OPERATOR(:=)`
        // 5. Panggil `print_expression(&stmt.expression, ...)`
        unimplemented!()
    }

    fn print_if_statement(&mut self, stmt: &IfStatement, is_last: bool) -> String {
        unimplemented!() // TODO: Cetak <if-statement>, KEYWORD(jika), panggil print_expression, cetak KEYWORD(maka), panggil print_statement, dst.
    }
    
    fn print_while_statement(&mut self, stmt: &WhileStatement, is_last: bool) -> String {
        unimplemented!() // TODO: Cetak <while-statement>, KEYWORD(selama), panggil print_expression, cetak KEYWORD(lakukan), panggil print_statement
    }
    
    fn print_for_statement(&mut self, stmt: &ForStatement, is_last: bool) -> String {
        unimplemented!() // TODO: ...
    }
    
    fn print_repeat_statement(&mut self, stmt: &RepeatStatement, is_last: bool) -> String {
        unimplemented!() // TODO: ...
    }
    
    fn print_case_statement(&mut self, stmt: &CaseStatement, is_last: bool) -> String {
        unimplemented!() // TODO: ...
    }
    
    fn print_case_branch(&mut self, branch: &CaseBranch, is_last: bool) -> String {
        unimplemented!() // TODO: ...
    }
    
    fn print_procedure_call_statement(&mut self, stmt: &ProcedureCallStatement, is_last: bool) -> String {
        // TODO: Implement print_procedure_call_statement
        // 1. Cetak "<procedure-call>" (atau <procedure/function-call> sesuai spek)
        // 2. PUSH/POP
        // 3. Cetak `IDENTIFIER(...)`
        // 4. Panggil `print_parameter_list(&stmt.arguments, ...)`
        unimplemented!()
    }

    // --- Expressions (Hierarki 4 Level) ---

    fn print_expression(&mut self, expr: &Expression, is_last: bool) -> String {
        // TODO: Implement print_expression
        // 1. Cetak "<expression>"
        // 2. PUSH/POP
        // 3. Panggil `print_simple_expression(&expr.initial_simple_expr, ...)`
        // 4. Loop `expr.rest`:
        //    a. Cetak `RELATIONAL_OPERATOR(...)`
        //    b. Panggil `print_simple_expression(...)`
        unimplemented!()
    }

    fn print_simple_expression(&mut self, expr: &SimpleExpression, is_last: bool) -> String {
        // TODO: Implement print_simple_expression
        // 1. Cetak "<simple-expression>"
        // 2. PUSH/POP
        // 3. (Opsional) Cetak `UNARY_OP` jika `expr.unary_op.is_some()`
        // 4. Panggil `print_term(&expr.initial_term, ...)`
        // 5. Loop `expr.rest`:
        //    a. Cetak `ADDITIVE_OPERATOR(...)`
        //    b. Panggil `print_term(...)`
        unimplemented!()
    }

    fn print_term(&mut self, term: &Term, is_last: bool) -> String {
        // TODO: Implement print_term
        // 1. Cetak "<term>"
        // 2. PUSH/POP
        // 3. Panggil `print_factor(&term.initial_factor, ...)`
        // 4. Loop `term.rest`:
        //    a. Cetak `MULTIPLICATIVE_OPERATOR(...)`
        //    b. Panggil `print_factor(...)`
        unimplemented!()
    }

    fn print_factor(&mut self, factor: &Factor, is_last: bool) -> String {
        // TODO: Implement print_factor (Router)
        // 1. Cetak "<factor>"
        // 2. PUSH/POP
        // 3. `match factor`:
        //    - `Factor::Literal(lit)` -> `print_literal_value(lit, true)`
        //    - `Factor::Identifier(name)` -> `print_terminal(&format!("IDENTIFIER({})", name), true)`
        //    - `Factor::FunctionCall(f)` -> `print_function_call_node(f, true)`
        //    - `Factor::ArrayAccess(a)` -> `print_array_access(a, true)`
        //    - `Factor::Parenthesized(e)` -> `print_expression(e, true)`
        //    - `Factor::Not(f)` -> Cetak `LOGICAL_OPERATOR(tidak)`, panggil `print_factor(f, true)`
        unimplemented!()
    }

    // --- Expression Helpers ---

    fn print_literal_value(&mut self, lit: &LiteralValue, is_last: bool) -> String {
        // TODO: Implement print_literal_value
        // 1. `match &*lit.value`:
        //    - `Literal::Integer(i)` -> `print_terminal(&format!("INT_LITERAL({})", i), is_last)`
        //    - (dan seterusnya untuk Real, String, Char, Boolean)
        unimplemented!()
    }

    fn print_function_call_node(&mut self, call: &FunctionCallNode, is_last: bool) -> String {
        // TODO: Implement print_function_call_node
        // 1. Cetak "<function-call>"
        // 2. PUSH/POP
        // 3. Cetak `IDENTIFIER(...)`
        // 4. Panggil `print_parameter_list(&call.arguments, ...)`
        unimplemented!()
    }

    fn print_array_access(&mut self, access: &ArrayAccess, is_last: bool) -> String {
        // TODO: Implement print_array_access
        // 1. (Tidak ada node <array-access> di spek? Cek lagi. Jika tidak ada, jangan cetak node ini)
        // 2. Panggil `print_expression(&access.array, ...)`
        // 3. Cetak `LBRACKET([)`
        // 4. Panggil `print_expression(&access.index, ...)`
        // 5. Cetak `RBRACKET(])`
        unimplemented!()
    }

    fn print_parameter_list(&mut self, list: &ParameterList, is_last: bool) -> String {
        // TODO: Implement print_parameter_list
        // 1. Cetak "<parameter-list>"
        // 2. PUSH/POP
        // 3. Loop `list.expressions`, panggil `print_expression` dan `COMMA(,)`
        unimplemented!()
    }
}