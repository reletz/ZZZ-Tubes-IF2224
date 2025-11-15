//! Converts the raw Abstract Syntax Tree into a human-readable visual tree format.
//! 
//! The parser generates nested Rust structs representing the AST. This module transforms
//! that structure into a tree visualization matching the grammar specification.
//! 
//! Example Output
//! ```text
//! <program>
//! ├── <program-header>
//! │   ├── KEYWORD(program)
//! │   └── IDENTIFIER(TestArrayFunc)
//! ├── <compound-statement>
//! ...
//! ```
//! 
//! Uses recursive traversal with `indent_level` tracking for proper tree formatting.

use super::ast::*;

pub struct AstPrinter {
    indent_level: usize,
    /// Stack untuk melacak apakah level indentasi induk adalah anak terakhir.
    /// Unuk menentukan apakah akan mencetak '│' (false) or ' ' (true).
    prefix_stack: Vec<bool>,
}

impl AstPrinter {
    pub fn new() -> Self {
        AstPrinter {
            indent_level: 0,
            prefix_stack: Vec::new(),
        }
    }

    pub fn print_program(&mut self, program: &Program) -> String {
        let mut output = String::new();
        
        output.push_str("<program>\n");
        self.indent_level += 1;
        // (Root level tidak PUSH)

        let has_decls = !program.declarations.is_empty();

        // --- Child 1: Program Header ---
        let header_is_last = false;
        output.push_str(&self.print_program_header(&program.name, header_is_last));
        
        // --- Child 2: Declarations ---
        if has_decls {
            let decls_is_last = false; // false, karena 2 anak lagi menyusul
            output.push_str(&self.print_declaration_part(&program.declarations, decls_is_last));
        }
        
        // --- Child 3: Compound Statement (Body) ---
        let body_is_last = false; // false, karena DOT menyusul
        output.push_str(&self.print_compound_statement(&program.body, body_is_last));
        
        // --- Child 4: Dot ---
        let dot_is_last = true; // Selalu terakhir
        output.push_str(&self.print_terminal("DOT(.)", dot_is_last));
        
        self.indent_level -= 1;
        output
    }

    fn print_program_header(&mut self, name: &str, is_last: bool) -> String {
        let mut output = String::new();
        
        output.push_str(&self.print_node("<program-header>", is_last));
        
        self.indent_level += 1;
        self.prefix_stack.push(is_last); // PUSH status <program-header>

        output.push_str(&self.print_terminal("KEYWORD(program)", false));
        output.push_str(&self.print_terminal(&format!("IDENTIFIER({})", name), false));
        output.push_str(&self.print_terminal("SEMICOLON(;)", true));

        self.indent_level -= 1;
        self.prefix_stack.pop(); // POP status <program-header>
        
        output
    }

    fn print_declaration(&mut self, decl: &Declaration, is_last: bool) -> String {
        match decl {
            Declaration::Variable(var_decl) => self.print_variable_declaration(var_decl, is_last),
            Declaration::Constant(const_decl) => self.print_constant_declaration(const_decl, is_last),
            Declaration::Type(type_decl) => self.print_type_declaration(type_decl, is_last),
            Declaration::Procedure(proc_decl) => self.print_procedure_declaration(proc_decl, is_last),
            Declaration::Function(func_decl) => self.print_function_declaration(func_decl, is_last),
        }
    }

    fn print_declaration_part(&mut self, declarations: &Vec<Declaration>, is_last: bool) -> String {
        let mut output = String::new();

        output.push_str(&self.print_node("<declaration-part>", is_last));
        
        self.prefix_stack.push(is_last);
        self.indent_level += 1;

        let num_decls = declarations.len();
        for (i, decl) in declarations.iter().enumerate() {
            let is_last_decl = i == num_decls - 1;
            output.push_str(&self.print_declaration(decl, is_last_decl));
        }
        self.indent_level -= 1;
        self.prefix_stack.pop();
        
        output
    }

    fn print_type_node(&mut self, var_type: &Type, is_last: bool) -> String {
         let mut output = String::new();
         output.push_str(&self.print_node("<type>", is_last));
         self.indent_level += 1;
         self.prefix_stack.push(is_last);

         // Tipe itu sendiri adalah satu-satunya anak, jadi 'is_last: true'
         output.push_str(&self.print_type(var_type, true)); // Ini memanggil print_terminal

         self.indent_level -= 1;
         self.prefix_stack.pop();
         output
    }

    fn print_identifier_list(&mut self, identifiers: &Vec<String>, is_last: bool) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node("<identifier-list>", is_last));
        
        self.indent_level += 1;
        self.prefix_stack.push(is_last);

        let num_ids = identifiers.len();
        if num_ids > 0 {
            for (j, id) in identifiers.iter().enumerate() {
                let is_last_id = j == num_ids - 1;
                output.push_str(&self.print_terminal(&format!("IDENTIFIER({})", id), is_last_id));
                if !is_last_id {
                    output.push_str(&self.print_terminal("COMMA(,)", false));
                }
            }
        }
        
        self.indent_level -= 1;
        self.prefix_stack.pop(); 

        output
    }

    /// Helper untuk grup `identifiers: type`
    fn print_identifier_type_group(&mut self, identifiers: &Vec<String>, var_type: &Type) -> String {
        let mut output = String::new();

        let list_is_last = false;
        output.push_str(&self.print_identifier_list(identifiers, list_is_last));

        // --- Child 2: Colon ---
        let colon_is_last = false; // Diikuti oleh TYPE
        output.push_str(&self.print_terminal("COLON(:)", colon_is_last));

        // --- Child 3: Type ---
        let type_is_last = true;
        output.push_str(&self.print_type_node(var_type, type_is_last));

        output
    }

    /// Mencetak satu node <var-group> yang berisi id-list, type, dan semicolon
    fn print_variable_group(&mut self, group: &VariableGroup, is_last: bool) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node("<var-group>", is_last));
        
        self.indent_level += 1;
        self.prefix_stack.push(is_last);

        output.push_str(&self.print_identifier_type_group(&group.identifiers, &group.var_type));

        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }

    fn print_variable_declaration(&mut self, var_decl: &VariableDeclaration, is_last: bool) -> String {
        let mut output = String::new();
        
        output.push_str(&self.print_node("<var-declaration>", is_last));
        self.indent_level += 1;
        self.prefix_stack.push(is_last);
        
        let num_groups = var_decl.groups.len();
        let has_groups = num_groups > 0;

        // --- Child 1: KEYWORD ---
        let keyword_is_last = !has_groups;
        output.push_str(&self.print_terminal("KEYWORD(variabel)", keyword_is_last));
        
        // --- Children 2..N: Groups ---
        if has_groups {
            for (i, group) in var_decl.groups.iter().enumerate() {
                let is_last_group = i == num_groups - 1;
                // Panggil helper yang sudah direfactor
                output.push_str(&self.print_variable_group(group, is_last_group));

                // FIX: Semicolon ditangani di sini sebagai SEPARATOR
                if !is_last_group {
                    output.push_str(&self.print_terminal("SEMICOLON(;)", false));
                }
            }
        }
        
        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }

    fn print_parameter_group(&mut self, group: &FormalParameterGroup, is_last: bool) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node("<parameter-group>", is_last));
        self.indent_level += 1;
        self.prefix_stack.push(is_last);

        output.push_str(&self.print_identifier_type_group(&group.identifiers, &group.var_type));
        
        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }

    fn print_formal_parameter_list(&mut self, params: &Vec<FormalParameterGroup>, is_last: bool) -> String {
        let mut output = String::new();

        output.push_str(&self.print_node("<formal-parameter-list>", is_last));
        self.indent_level += 1;
        self.prefix_stack.push(is_last);

        let num_params = params.len();
        let has_params = num_params > 0;

        // --- Child 1: LPAREN ---
        let lparen_is_last = !has_params;
        output.push_str(&self.print_terminal("LPARENTHESIS(()", lparen_is_last));

        // --- Children 2..N-1: Groups ---
        for (i, group) in params.iter().enumerate() {
            // Grup parameter *bukan* anak terakhir, karena RPAREN selalu ada
            let is_last_group_node = false; 
            output.push_str(&self.print_parameter_group(group, is_last_group_node));

            // Jika bukan grup terakhir *di antara grup*, cetak semicolon
            if i < num_params - 1 {
                output.push_str(&self.print_terminal("SEMICOLON(;)", false));
            }
        }

        // --- Last Child: RPAREN ---
        if has_params { // Hanya cetak RPAREN jika ada params
            output.push_str(&self.print_terminal("RPARENTHESIS())", true));
        }

        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }

    fn print_constant_declaration(&mut self, const_decl: &ConstantDeclaration, is_last: bool) -> String {
        let mut output = String::new();

        output.push_str(&self.print_node("<const-declaration>", is_last));
        self.indent_level += 1;
        self.prefix_stack.push(is_last);

        let num_defs = const_decl.constants.len();
        let has_defs = num_defs > 0;

        output.push_str(&self.print_terminal("KEYWORD(konstanta)", !has_defs));

        if has_defs {
            for (i, def) in const_decl.constants.iter().enumerate() {
                let is_last_def = i == num_defs - 1;
                // Ini akan jadi `print_const_definition`
                output.push_str(&self.print_const_definition(def, is_last_def));
            }
        }
        
        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }

    fn print_const_definition(&mut self, def: &ConstantDefinition, is_last: bool) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node("<const-definition>", is_last));
        self.indent_level += 1;
        self.prefix_stack.push(is_last);

        output.push_str(&self.print_terminal(&format!("IDENTIFIER({})", def.name), false));
        output.push_str(&self.print_terminal("OPERATOR(=)", false));
        output.push_str(&self.print_expression(&def.value, false)); 
        output.push_str(&self.print_terminal("SEMICOLON(;)", true));

        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }

    fn print_type_declaration(&mut self, type_decl: &TypeDeclaration, is_last: bool) -> String {
        let mut output = String::new();

        output.push_str(&self.print_node("<type-declaration>", is_last));
        self.indent_level += 1;
        self.prefix_stack.push(is_last);

        let num_defs = type_decl.definitions.len();
        let has_defs = num_defs > 0;

        output.push_str(&self.print_terminal("KEYWORD(tipe)", !has_defs));

        if has_defs {
            for (i, def) in type_decl.definitions.iter().enumerate() {
                let is_last_def = i == num_defs - 1;
                output.push_str(&self.print_type_definition(def, is_last_def));
            }
        }
        
        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }

    fn print_type_definition(&mut self, def: &TypeDefinition, is_last: bool) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node("<type-definition>", is_last));
        self.indent_level += 1;
        self.prefix_stack.push(is_last);

        output.push_str(&self.print_terminal(&format!("IDENTIFIER({})", def.name), false));
        output.push_str(&self.print_terminal("OPERATOR(=)", false));
        output.push_str(&self.print_type_node(&def.type_def, false)); // <type> node, bukan terakhir
        output.push_str(&self.print_terminal("SEMICOLON(;)", true));

        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }


    fn print_procedure_declaration(&mut self, proc_decl: &ProcedureDeclaration, is_last: bool) -> String {
        let mut output = String::new();

        output.push_str(&self.print_node("<procedure-declaration>", is_last));
        self.indent_level += 1;
        self.prefix_stack.push(is_last);

        let has_params = !proc_decl.parameters.is_empty();
        let has_decls = !proc_decl.declarations.is_empty();

        output.push_str(&self.print_terminal(&format!("KEYWORD(prosedur)"), false));
        output.push_str(&self.print_terminal(&format!("IDENTIFIER({})", proc_decl.name), false));

        if has_params {
            output.push_str(&self.print_formal_parameter_list(&proc_decl.parameters, false));
        }

        let header_semicolon_is_last = false; 
        output.push_str(&self.print_terminal("SEMICOLON(;)", header_semicolon_is_last));

        // --- Child 2: Deklarasi lokal (Opsional) ---
        if has_decls {
            let decls_is_last = false;
            output.push_str(&self.print_declaration_part(&proc_decl.declarations, decls_is_last));
        }

        // --- Child 3: Body ---
        output.push_str(&self.print_compound_statement(&proc_decl.body, false)); // Body bukan terakhir

        // --- Child 4: Semicolon ---
        output.push_str(&self.print_terminal("SEMICOLON(;)", true));
        
        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }

    fn print_function_declaration(&mut self, func_decl: &FunctionDeclaration, is_last: bool) -> String {
        let mut output = String::new();

        output.push_str(&self.print_node("<function-declaration>", is_last));
        self.indent_level += 1;
        self.prefix_stack.push(is_last);

        let has_params = !func_decl.parameters.is_empty();
        let has_decls = !func_decl.declarations.is_empty();

        output.push_str(&self.print_terminal(&format!("KEYWORD(fungsi)"), false));
        output.push_str(&self.print_terminal(&format!("IDENTIFIER({})", func_decl.name), false));

        if has_params {
            output.push_str(&self.print_formal_parameter_list(&func_decl.parameters, false));
        }

        output.push_str(&self.print_terminal("COLON(:)", false));
        output.push_str(&self.print_type_node(&func_decl.return_type, false)); // <type> node
        
        let header_semicolon_is_last = false;
        output.push_str(&self.print_terminal("SEMICOLON(;)", header_semicolon_is_last));

        // --- Child 2: Deklarasi lokal (Opsional) ---
        if has_decls {
            let decls_is_last = false; // Body selalu ada
            output.push_str(&self.print_declaration_part(&func_decl.declarations, decls_is_last));
        }

        // --- Child 3: Body ---
        output.push_str(&self.print_compound_statement(&func_decl.body, false)); // Body bukan terakhir

        // --- Child 4: Semicolon ---
        output.push_str(&self.print_terminal("SEMICOLON(;)", true));
        
        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }

    /// Ini HANYA mencetak terminal, BUKAN node <type>
    fn print_type(&self, var_type: &Type, is_last: bool) -> String {
        match var_type {
            Type::Integer => self.print_terminal("KEYWORD(integer)", is_last),
            Type::Real => self.print_terminal("KEYWORD(real)", is_last),
            Type::Boolean => self.print_terminal("KEYWORD(boolean)", is_last),
            Type::String => self.print_terminal("KEYWORD(string)", is_last),
            Type::Char => self.print_terminal("KEYWORD(char)", is_last),
            Type::Array(array_def) => {
                // Show as: larik[range] dari base_type
                self.print_terminal(&format!(
                    "larik[...] dari {}",
                    match *array_def.base_type {
                        Type::Integer => "integer",
                        Type::Real => "real",
                        Type::Boolean => "boolean",
                        Type::String => "string",
                        Type::Char => "char",
                        _ => "complex"
                    }
                ), is_last)
            }
            Type::Subrange(_) => {
                self.print_terminal("TYPE(Subrange)", is_last)
            }
            Type::TypeIdentifier(name) => {
                self.print_terminal(&format!("IDENTIFIER({})", name), is_last)
            }
        }
    }

    fn print_statement_list(&mut self, statements: &Vec<Statement>, is_last_node: bool) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node("<statement-list>", is_last_node));

        self.indent_level += 1;
        self.prefix_stack.push(is_last_node);
        
        let num_statements = statements.len();
        if num_statements > 0 {
            for (i, statement) in statements.iter().enumerate() {
                let is_last_stmt = i == num_statements - 1;
                
                output.push_str(&self.print_statement(statement, is_last_stmt));
                
                if !is_last_stmt {
                    output.push_str(&self.print_terminal("SEMICOLON(;)", false));
                }
            }
        }
        
        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }

    fn print_compound_statement(&mut self, stmt: &CompoundStatement, is_last: bool) -> String {
        let mut output = String::new();
        
        output.push_str(&self.print_node("<compound-statement>", is_last));
        self.indent_level += 1;
        self.prefix_stack.push(is_last); // PUSH status <compound-statement>

        let has_statements = !stmt.statements.is_empty();
        
        // --- Child 1: 'mulai' ---
        // 'mulai' adalah terakhir HANYA jika tidak ada statement-list
        let mulai_is_last = !has_statements;
        output.push_str(&self.print_terminal("KEYWORD(mulai)", mulai_is_last));
        
        // --- Child 2: 'statement-list' (Opsional) ---
        if has_statements {
            output.push_str(&self.print_statement_list(&stmt.statements, false));
        }
        
        // --- Child 3: 'selesai' ---
        // 'selesai' selalu terakhir (relatif terhadap 'mulai'/'statement-list')
        output.push_str(&self.print_terminal("KEYWORD(selesai)", true));

        self.indent_level -= 1;
        self.prefix_stack.pop(); // POP status <compound-statement>
        output
    }

    fn print_statement(&mut self, stmt: &Statement, is_last: bool) -> String {
        match stmt {
            Statement::Compound(stmt) => self.print_compound_statement(stmt, is_last),
            Statement::Assignment(stmt) => self.print_assignment_statement(stmt, is_last),
            Statement::If(stmt) => self.print_if_statement(stmt, is_last),
            Statement::While(stmt) => self.print_while_statement(stmt, is_last),
            Statement::For(stmt) => self.print_for_statement(stmt, is_last),
            Statement::Repeat(stmt) => self.print_repeat_statement(stmt, is_last),
            Statement::Case(stmt) => self.print_case_statement(stmt, is_last),
            Statement::Read(stmt) => self.print_read_statement(stmt, is_last),
            Statement::Write(stmt) => self.print_write_statement(stmt, is_last),
            Statement::ProcedureCall(stmt) => self.print_procedure_call_statement(stmt, is_last),
            
            Statement::ExpressionStatement(expr) => {
                match expr {
                    Expression::FunctionCall { function_name, arguments } => {
                        if function_name.to_lowercase() == "writeln" {
                            self.print_write_statement(&WriteStatement { expressions: arguments.clone() }, is_last)
                        } else if function_name.to_lowercase() == "readln" {
                             self.print_read_statement(&ReadStatement { variables: arguments.clone() }, is_last)
                        } else {
                            self.print_procedure_call_statement(&ProcedureCallStatement {
                                procedure_name: function_name.clone(),
                                arguments: arguments.clone(),
                            }, is_last)
                        }
                    }
                    _ => {
                        let mut output = String::new();
                        output.push_str(&self.print_node("<expression-statement>", is_last));
                        self.indent_level += 1;
                        self.prefix_stack.push(is_last);
                        
                        output.push_str(&self.print_expression(expr, true)); // Expr adalah anak tunggal
                        
                        self.indent_level -= 1;
                        self.prefix_stack.pop();
                        output
                    }
                }
            },
        }
    }

    fn print_expression(&mut self, expr: &Expression, is_last: bool) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node("<expression>", is_last));
        self.indent_level += 1;
        self.prefix_stack.push(is_last);
        
        match expr {
            Expression::BinaryOp { left, operator, right } 
            if matches!(operator.as_str(), "=" | "<>" | "<" | "<=" | ">" | ">=") => {
                // <simple-expression> (kiri)
                output.push_str(&self.print_simple_expression(left, false)); 
                // RELATIONAL_OPERATOR
                output.push_str(&self.print_terminal(&format!("RELATIONAL_OPERATOR({})", operator), false));
                // <simple-expression> (kanan)
                output.push_str(&self.print_simple_expression(right, true));
            }
            _ => {
                output.push_str(&self.print_simple_expression(expr, true));
            }
        }
        
        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }
    
    fn print_simple_expression(&mut self, expr: &Expression, is_last: bool) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node("<simple-expression>", is_last));
        self.indent_level += 1;
        self.prefix_stack.push(is_last);

        match expr {
            Expression::BinaryOp { left, operator, right } 
                if matches!(operator.as_str(), "+" | "-" | "atau") => {
                    // <simple-expression> (kiri)
                    output.push_str(&self.print_simple_expression(left, false));
                    // OPERATOR
                    output.push_str(&self.print_terminal(&format!("OPERATOR({})", operator), false));
                    // <term> (kanan)
                    output.push_str(&self.print_term(right, true));
                }

            Expression::UnaryOp { operator, operand }
                if matches!(operator.as_str(), "+" | "-") => {
                    output.push_str(&self.print_terminal(&format!("UNARY_OP({})", operator), false));
                    // <simple-expression> (operand)
                    output.push_str(&self.print_simple_expression(operand, true));
                }
            // base case: literal, identifier, dll, dibungkus <term>
            _ => {
                output.push_str(&self.print_term(expr, true));

            }
        }

        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }

    fn print_term(&mut self, expr: &Expression, is_last: bool) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node("<term>", is_last));
        self.indent_level += 1;
        self.prefix_stack.push(is_last);

        match expr {
            // Kasus Multiplikatif
            Expression::BinaryOp { left, operator, right }
                if matches!(operator.as_str(), "*" | "/" | "div" | "mod" | "dan") => {
                    // <term> (kiri)
                    output.push_str(&self.print_term(left, false));
                    // OPERATOR
                    output.push_str(&self.print_terminal(&format!("OPERATOR({})", operator), false));
                    // <factor> (kanan)
                    output.push_str(&self.print_factor(right, true));
                }
            // Jika bukan, delegasikan ke level di bawahnya
            _ => {
                output.push_str(&self.print_factor(expr, true));
            }
        }

        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }

    fn print_factor(&mut self, expr: &Expression, is_last: bool) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node("<factor>", is_last));
        self.indent_level += 1;
        self.prefix_stack.push(is_last);

        match expr {
            // Kasus Unary 'tidak' (not)
            Expression::UnaryOp { operator, operand }
            if operator == "tidak" => {
                output.push_str(&self.print_terminal(&format!("LOGICAL_OPERATOR({})", operator), false));
                output.push_str(&self.print_factor(operand, true));
            }
            
            // Kasus "daun" (Literal, Identifier, Array, FuncCall)
            Expression::Literal(lit) => output.push_str(&self.print_literal(lit, true)),
            Expression::Identifier(name) => output.push_str(&self.print_terminal(&format!("IDENTIFIER({})", name), true)),
            Expression::ArrayAccess { array, index } => output.push_str(&self.print_array_access(array, index, true)),
            Expression::FunctionCall { function_name, arguments } => output.push_str(&self.print_function_call(function_name, arguments, true)),

            // Kasus dalam kurung '(...)', yang di-parse sebagai ekspresi biasa
            // (Ini menangkap BinaryOp atau UnaryOp sign yang 'lolos' dari level di atas)
            _ => {
                // Delegasikan kembali ke level <expression> (level tertinggi)
                // untuk memulai ulang pemeriksaan precedence dari dalam kurung.
                output.push_str(&self.print_expression(expr, true));
            }
        }

        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }

    fn print_array_access(&mut self, array: &Expression, index: &Expression, is_last: bool) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node("<array-access>", is_last));
        self.indent_level += 1;
        self.prefix_stack.push(is_last);
        
        if let Expression::Identifier(ref name) = *array {
            self.prefix_stack.push(false); // PUSH: ID
            output.push_str(&self.print_terminal(&format!("IDENTIFIER({})", name), false));
            self.prefix_stack.pop(); // POP
        } else {
            // Jika array adalah ekspresi lain yang kompleks
            output.push_str(&self.print_expression(array, false));
        }
        output.push_str(&self.print_terminal("LBRACKET([)", false));
        output.push_str(&self.print_expression(index, false));
        output.push_str(&self.print_terminal("RBRACKET(])", true));
        
        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }

    fn print_function_call(&mut self, function_name: &str, arguments: &Vec<Expression>, is_last: bool) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node("<function-call>", is_last));
        self.indent_level += 1;
        self.prefix_stack.push(is_last);
        
        output.push_str(&self.print_call_structure(function_name, arguments));
        
        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }

    fn print_literal(&mut self, lit: &LiteralValue, is_last: bool) -> String {
        let lit_str = match lit {
            LiteralValue::Integer(i) => format!("INT_LITERAL({})", i),
            LiteralValue::Real(r) => format!("REAL_LITERAL({})", r),
            LiteralValue::String(s) => format!("STRING_LITERAL('{}')", s),
            LiteralValue::Char(c) => format!("CHAR_LITERAL('{}')", c),
            LiteralValue::Boolean(b) => format!("BOOLEAN({})", if *b { "benar" } else { "salah" }),
        };
        self.print_terminal(&lit_str, is_last)
    }

    fn print_node(&self, label: &str, is_last: bool) -> String {
        format!("{}{}\n", self.get_prefix(is_last), label)
    }

    fn print_terminal(&self, label: &str, is_last: bool) -> String {
        format!("{}{}\n", self.get_prefix(is_last), label)
    }

    fn get_prefix(&self, is_last: bool) -> String {
        if self.indent_level == 0 {
            return String::new();
        }
        
        let mut prefix = String::new();
        
        for &was_last in &self.prefix_stack {
            if was_last {
                prefix.push_str("    ");
            } else {
                prefix.push_str("│   ");
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

    fn print_assignment_statement(&mut self, assign: &AssignmentStatement, is_last: bool) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node("<assignment-statement>", is_last));
        self.indent_level += 1;
        self.prefix_stack.push(is_last);
        
        output.push_str(&self.print_expression(&assign.variable, false)); // Variabel
        output.push_str(&self.print_terminal("ASSIGN_OPERATOR(:=)", false));
        output.push_str(&self.print_expression(&assign.expression, true)); // Ekspresi
        
        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }

    fn print_if_statement(&mut self, stmt: &IfStatement, is_last: bool) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node("<if-statement>", is_last));
        self.indent_level += 1;
        self.prefix_stack.push(is_last);

        let has_else = stmt.else_branch.is_some();

        output.push_str(&self.print_terminal("KEYWORD(jika)", false));
        output.push_str(&self.print_expression(&stmt.condition, false));
        output.push_str(&self.print_terminal("KEYWORD(maka)", false));
        
        // then_branch terakhir HANYA jika tidak ada else
        output.push_str(&self.print_statement(&stmt.then_branch, !has_else)); 

        if let Some(else_branch) = &stmt.else_branch {
            output.push_str(&self.print_terminal("KEYWORD(selain-itu)", false));
            output.push_str(&self.print_statement(else_branch, true)); // else_branch selalu terakhir
        }

        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }

    fn print_while_statement(&mut self, stmt: &WhileStatement, is_last: bool) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node("<while-statement>", is_last));
        self.indent_level += 1;
        self.prefix_stack.push(is_last);

        output.push_str(&self.print_terminal("KEYWORD(selama)", false));
        output.push_str(&self.print_expression(&stmt.condition, false));
        output.push_str(&self.print_terminal("KEYWORD(lakukan)", false));
        output.push_str(&self.print_statement(&stmt.body, true)); // body selalu terakhir

        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }

    fn print_for_statement(&mut self, stmt: &ForStatement, is_last: bool) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node("<for-statement>", is_last));
        self.indent_level += 1;
        self.prefix_stack.push(is_last);

        output.push_str(&self.print_terminal("KEYWORD(untuk)", false));
        output.push_str(&self.print_terminal(&format!("IDENTIFIER({})", stmt.counter_variable), false));
        output.push_str(&self.print_terminal("ASSIGN_OPERATOR(:=)", false));
        output.push_str(&self.print_expression(&stmt.start_value, false));

        let direction = match stmt.direction {
            ForDirection::To => "KEYWORD(ke)",
            ForDirection::DownTo => "KEYWORD(turun-ke)",
        };
        output.push_str(&self.print_terminal(direction, false));
        output.push_str(&self.print_expression(&stmt.end_value, false));
        output.push_str(&self.print_terminal("KEYWORD(lakukan)", false));
        output.push_str(&self.print_statement(&stmt.body, true)); // body selalu terakhir

        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }

    fn print_repeat_statement(&mut self, stmt: &RepeatStatement, is_last: bool) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node("<repeat-statement>", is_last));
        self.indent_level += 1;
        self.prefix_stack.push(is_last);
        
        let has_statements = !stmt.statements.is_empty();

        output.push_str(&self.print_terminal("KEYWORD(ulangi)", !has_statements));
        
        if has_statements {
            output.push_str(&self.print_statement_list(&stmt.statements, false));
        }

        // --- Child 3: 'sampai' ---
        output.push_str(&self.print_terminal("KEYWORD(sampai)", false));
        
        // --- Child 4: condition ---
        output.push_str(&self.print_expression(&stmt.condition, true));

        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }

    fn print_case_statement(&mut self, stmt: &CaseStatement, is_last: bool) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node("<case-statement>", is_last));
        self.indent_level += 1;
        self.prefix_stack.push(is_last);

        let has_else = stmt.else_branch.is_some();
        let num_branches = stmt.branches.len();
        let has_branches = num_branches > 0;

        output.push_str(&self.print_terminal("KEYWORD(kasus)", false));
        output.push_str(&self.print_expression(&stmt.expression, false));
        
        let dari_is_last = !has_branches && !has_else;
        output.push_str(&self.print_terminal("KEYWORD(dari)", dari_is_last));

        for (i, branch) in stmt.branches.iter().enumerate() {
            let is_last_branch = i == num_branches - 1 && !has_else;
            output.push_str(&self.print_case_branch(branch, is_last_branch));
        }

        if let Some(else_branch) = &stmt.else_branch {
            // 'else' selalu terakhir di antara 'branches'
            output.push_str(&self.print_else_branch(else_branch, true)); 
        }

        output.push_str(&self.print_terminal("KEYWORD(selesai)", true));
        
        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }

    fn print_case_branch(&mut self, branch: &CaseBranch, is_last: bool) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node("<case-branch>", is_last));
        self.indent_level += 1;
        self.prefix_stack.push(is_last);

        // --- Child 1: <label-list> ---
        let list_is_last = false; // Diikuti oleh COLON, dll.
        self.prefix_stack.push(list_is_last); // PUSH: <label-list>
        output.push_str(&self.print_node("<label-list>", list_is_last));
        self.indent_level += 1;
        
        output.push_str(&self.print_expression_list(&branch.labels));

        self.indent_level -= 1;
        self.prefix_stack.pop(); // POP <label-list>

        // --- Child 2: COLON ---
        output.push_str(&self.print_terminal("COLON(:)", false));

        // --- Child 3: Statement ---
        output.push_str(&self.print_statement(&branch.statement, false)); // Semicolon selalu ada

        // --- Child 4: SEMICOLON ---
        output.push_str(&self.print_terminal("SEMICOLON(;)", true));
        
        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }
    
    fn print_else_branch(&mut self, else_branch: &Vec<Statement>, is_last: bool) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node("<else-branch>", is_last));
        self.indent_level += 1;
        self.prefix_stack.push(is_last);

        let num_statements = else_branch.len();
        let has_statements = num_statements > 0;

        output.push_str(&self.print_terminal("KEYWORD(selain-itu)", !has_statements));

        if has_statements {
            for (i, statement) in else_branch.iter().enumerate() {
                let is_last_stmt = i == num_statements - 1;
                output.push_str(&self.print_statement(statement, is_last_stmt));
                if !is_last_stmt {
                    output.push_str(&self.print_terminal("SEMICOLON(;)", false));
                }
            }
        }

        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }

    fn print_expression_list(&mut self, expressions: &Vec<Expression>) -> String {
        let mut output = String::new();
        let num_exprs = expressions.len();
        
        if num_exprs > 0 {
            for (i, expr) in expressions.iter().enumerate() {
                let is_last_expr = i == num_exprs - 1;
                output.push_str(&self.print_expression(expr, is_last_expr));
                if !is_last_expr {
                    output.push_str(&self.print_terminal("COMMA(,)", false));
                }
            }
        }
        output
    }

    /// Helper untuk ( ... )
    fn print_parameter_list(&mut self, arguments: &Vec<Expression>, is_last: bool) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node("<parameter-list>", is_last));
        self.indent_level += 1;
        self.prefix_stack.push(is_last);

        output.push_str(&self.print_expression_list(arguments));
        
        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }

    fn print_call_structure(&mut self, name: &str, arguments: &Vec<Expression>) -> String {
        let mut output = String::new();
        let has_args = !arguments.is_empty();
        
        output.push_str(&self.print_terminal(&format!("IDENTIFIER({})", name), false));
        output.push_str(&self.print_terminal("LPARENTHESIS(()", !has_args));

        if has_args {
            output.push_str(&self.print_parameter_list(arguments, false)); // <parameter-list>
            output.push_str(&self.print_terminal("RPARENTHESIS())", true));
        }
        output
    }
    
    fn print_procedure_call_statement(&mut self, call: &ProcedureCallStatement, is_last: bool) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node("<procedure-call>", is_last));
        self.indent_level += 1;
        self.prefix_stack.push(is_last);
        
        output.push_str(&self.print_call_structure(&call.procedure_name, &call.arguments));

        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }
    
    fn print_read_statement(&mut self, stmt: &ReadStatement, is_last: bool) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node("<procedure-call>", is_last)); // readln adalah procedure call
        self.indent_level += 1;
        self.prefix_stack.push(is_last);
       
        output.push_str(&self.print_call_structure("readln", &stmt.variables));

        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }
    
    fn print_write_statement(&mut self, stmt: &WriteStatement, is_last: bool) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node("<procedure-call>", is_last)); // writeln adalah procedure call
        self.indent_level += 1;
        self.prefix_stack.push(is_last);
        
        output.push_str(&self.print_call_structure("writeln", &stmt.expressions));

        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }
}