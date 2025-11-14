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
        // Tidak ada push/pop untuk root, karena level 0

        let has_decls = !program.declarations.is_empty();

        // --- Child 1: Program Header ---
        let header_is_last = false; // Selalu diikuti oleh body dan dot
        output.push_str(&self.print_program_header_with_last(&program.name, header_is_last));
        
        // --- Child 2: Declarations (Opsional) ---
        if has_decls {
            let decls_is_last = false; // Selalu diikuti oleh body dan dot
            self.prefix_stack.push(decls_is_last); // PUSH (untuk node <declaration-part>)
            output.push_str(&self.print_node_with_last("<declaration-part>", decls_is_last));
            self.indent_level += 1;
            
            let num_decls = program.declarations.len();
            for (i, decl) in program.declarations.iter().enumerate() {
                let is_last_decl = i == num_decls - 1;
                output.push_str(&self.print_declaration_with_last(decl, is_last_decl));
            }
            
            self.indent_level -= 1;
            self.prefix_stack.pop(); // POP (untuk node <declaration-part>)
        }
        
        // --- Child 3: Compound Statement (Body) ---
        let body_is_last = false; // Selalu diikuti oleh dot
        output.push_str(&self.print_compound_statement_with_last(&program.body, body_is_last));
        
        // --- Child 4: Dot ---
        let dot_is_last = true; // Selalu terakhir
        output.push_str(&self.print_terminal_with_last("DOT(.)", dot_is_last));
        
        self.indent_level -= 1;
        output
    }

    fn print_program_header_with_last(&mut self, name: &str, is_last: bool) -> String {
        let mut output = String::new();
        
        output.push_str(&self.print_node_with_last("<program-header>", is_last));
        
        self.indent_level += 1;
        self.prefix_stack.push(is_last); // PUSH status <program-header>

        output.push_str(&self.print_terminal_with_last("KEYWORD(program)", false));
        output.push_str(&self.print_terminal_with_last(&format!("IDENTIFIER({})", name), false));
        output.push_str(&self.print_terminal_with_last("SEMICOLON(;)", true));

        self.indent_level -= 1;
        self.prefix_stack.pop(); // POP status <program-header>
        
        output
    }

    fn print_declaration_with_last(&mut self, decl: &Declaration, is_last: bool) -> String {
        match decl {
            Declaration::Variable(var_decl) => self.print_variable_declaration_with_last(var_decl, is_last),
            Declaration::Constant(const_decl) => self.print_constant_declaration_with_last(const_decl, is_last),
            Declaration::Type(type_decl) => self.print_type_declaration_with_last(type_decl, is_last),
            Declaration::Procedure(proc_decl) => self.print_procedure_declaration_with_last(proc_decl, is_last),
            Declaration::Function(func_decl) => self.print_function_declaration_with_last(func_decl, is_last),
        }
    }

    fn print_type_node_with_last(&mut self, var_type: &Type, is_last: bool) -> String {
         let mut output = String::new();
         output.push_str(&self.print_node_with_last("<type>", is_last));
         self.indent_level += 1;
         self.prefix_stack.push(is_last);

         // Tipe itu sendiri adalah satu-satunya anak, jadi 'is_last: true'
         output.push_str(&self.print_type(var_type, true)); // Ini memanggil print_terminal

         self.indent_level -= 1;
         self.prefix_stack.pop();
         output
    }

    /// Helper untuk grup `identifiers: type`
    fn print_identifier_type_group(&mut self, identifiers: &Vec<String>, var_type: &Type) -> String {
        // Fungsi ini tidak menerima `is_last` karena ia mencetak *beberapa* node
        // (bukan satu node pembungkus). Logika `is_last` ditangani di dalamnya.
        let mut output = String::new();

        // --- Child 1: Identifier List ---
        let list_is_last = false; // Diikuti oleh COLON dan TYPE
        self.prefix_stack.push(list_is_last); // PUSH <identifier-list>
        output.push_str(&self.print_node_with_last("<identifier-list>", list_is_last));
        self.indent_level += 1;

        // Ini akan diisi di loop, tapi kita perlu tahu `is_last`
        let num_ids = identifiers.len();
        if num_ids > 0 {
            for (j, id) in identifiers.iter().enumerate() {
                let is_last_id = j == num_ids - 1;
                
                output.push_str(&self.print_terminal_with_last(&format!("IDENTIFIER({})", id), is_last_id));

                if !is_last_id {
                    // Cetak COMMA jika bukan ID terakhir
                    output.push_str(&self.print_terminal_with_last("COMMA(,)", false));
                }
            }
        }
        self.indent_level -= 1;
        self.prefix_stack.pop(); // POP <identifier-list>

        // --- Child 2: Colon ---
        let colon_is_last = false; // Diikuti oleh TYPE
        output.push_str(&self.print_terminal_with_last("COLON(:)", colon_is_last));

        // --- Child 3: Type ---
        let type_is_last = true;
        output.push_str(&self.print_type_node_with_last(var_type, type_is_last));

        output
    }

    fn print_variable_declaration_with_last(&mut self, var_decl: &VariableDeclaration, is_last: bool) -> String {
        let mut output = String::new();
        
        output.push_str(&self.print_node_with_last("<var-declaration>", is_last));
        self.indent_level += 1;
        self.prefix_stack.push(is_last);
        
        let num_groups = var_decl.groups.len();
        let has_groups = num_groups > 0;

        // --- Child 1: KEYWORD ---
        let keyword_is_last = !has_groups;
        output.push_str(&self.print_terminal_with_last("KEYWORD(variabel)", keyword_is_last));
        
        // --- Children 2..N: Groups ---
        if has_groups {
            for (i, group) in var_decl.groups.iter().enumerate() {
                // `print_identifier_type_group` mencetak 3 node: <id-list>, COLON, <type>
                // Ini BUKAN node, jadi tidak perlu push/pop di sekitarnya
                output.push_str(&self.print_identifier_type_group(&group.identifiers, &group.var_type));
                
                let is_last_group = i == num_groups - 1;
                // --- Child Terakhir Grup: SEMICOLON ---
                output.push_str(&self.print_terminal_with_last("SEMICOLON(;)", is_last_group));
            }
        }
        
        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }

    fn print_parameter_group_with_last(&mut self, group: &FormalParameterGroup, is_last: bool) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node_with_last("<parameter-group>", is_last));
        self.indent_level += 1;
        self.prefix_stack.push(is_last);

        // `print_identifier_type_group` mencetak anak-anak (id-list, colon, type)
        // Kita tidak perlu push/pop lagi di sini karena `print_identifier_type_group`
        // sudah mengurus anak-anaknya sendiri.
        output.push_str(&self.print_identifier_type_group(&group.identifiers, &group.var_type));
        
        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }

    fn print_formal_parameter_list_with_last(&mut self, params: &Vec<FormalParameterGroup>, is_last: bool) -> String {
        let mut output = String::new();

        output.push_str(&self.print_node_with_last("<formal-parameter-list>", is_last));
        self.indent_level += 1;
        self.prefix_stack.push(is_last);

        let num_params = params.len();
        let has_params = num_params > 0;

        // --- Child 1: LPAREN ---
        let lparen_is_last = !has_params;
        output.push_str(&self.print_terminal_with_last("LPARENTHESIS(()", lparen_is_last));

        // --- Children 2..N-1: Groups ---
        for (i, group) in params.iter().enumerate() {
            // Grup parameter *bukan* anak terakhir, karena RPAREN selalu ada
            let is_last_group_node = false; 
            output.push_str(&self.print_parameter_group_with_last(group, is_last_group_node));

            // Jika bukan grup terakhir *di antara grup*, cetak semicolon
            if i < num_params - 1 {
                output.push_str(&self.print_terminal_with_last("SEMICOLON(;", false));
            }
        }

        // --- Last Child: RPAREN ---
        if has_params { // Hanya cetak RPAREN jika ada params
            output.push_str(&self.print_terminal_with_last("RPARENTHESIS())", true));
        }

        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }

    fn print_constant_declaration_with_last(&mut self, const_decl: &ConstantDeclaration, is_last: bool) -> String {
        let mut output = String::new();

        output.push_str(&self.print_node_with_last("<const-declaration>", is_last));
        self.indent_level += 1;
        self.prefix_stack.push(is_last);

        let num_defs = const_decl.constants.len();
        let has_defs = num_defs > 0;

        output.push_str(&self.print_terminal_with_last("KEYWORD(konstanta)", !has_defs));

        if has_defs {
            for (i, def) in const_decl.constants.iter().enumerate() {
                let is_last_def = i == num_defs - 1;
                // Ini akan jadi `print_const_definition_with_last`
                output.push_str(&self.print_const_definition_with_last(def, is_last_def));
            }
        }
        
        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }

    fn print_const_definition_with_last(&mut self, def: &ConstantDefinition, is_last: bool) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node_with_last("<const-definition>", is_last));
        self.indent_level += 1;
        self.prefix_stack.push(is_last);

        output.push_str(&self.print_terminal_with_last(&format!("IDENTIFIER({})", def.name), false));
        output.push_str(&self.print_terminal_with_last("OPERATOR(=)", false));
        output.push_str(&self.print_expression_with_last(&def.value, false)); 
        output.push_str(&self.print_terminal_with_last("SEMICOLON(;)", true));

        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }

    fn print_type_declaration_with_last(&mut self, type_decl: &TypeDeclaration, is_last: bool) -> String {
        let mut output = String::new();

        output.push_str(&self.print_node_with_last("<type-declaration>", is_last));
        self.indent_level += 1;
        self.prefix_stack.push(is_last);

        let num_defs = type_decl.definitions.len();
        let has_defs = num_defs > 0;

        output.push_str(&self.print_terminal_with_last("KEYWORD(tipe)", !has_defs));

        if has_defs {
            for (i, def) in type_decl.definitions.iter().enumerate() {
                let is_last_def = i == num_defs - 1;
                output.push_str(&self.print_type_definition_with_last(def, is_last_def));
            }
        }
        
        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }

    fn print_type_definition_with_last(&mut self, def: &TypeDefinition, is_last: bool) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node_with_last("<type-definition>", is_last));
        self.indent_level += 1;
        self.prefix_stack.push(is_last);

        output.push_str(&self.print_terminal_with_last(&format!("IDENTIFIER({})", def.name), false));
        output.push_str(&self.print_terminal_with_last("OPERATOR(=)", false));
        output.push_str(&self.print_type_node_with_last(&def.type_def, false)); // <type> node, bukan terakhir
        output.push_str(&self.print_terminal_with_last("SEMICOLON(;)", true));

        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }


    fn print_procedure_declaration_with_last(&mut self, proc_decl: &ProcedureDeclaration, is_last: bool) -> String {
        let mut output = String::new();

        output.push_str(&self.print_node_with_last("<procedure-declaration>", is_last));
        self.indent_level += 1;
        self.prefix_stack.push(is_last);

        let has_decls = !proc_decl.declarations.is_empty();

        // --- Child 1: Header ---
        output.push_str(&self.print_procedure_header_with_last(proc_decl, false)); // Header bukan terakhir

        // --- Child 2: Deklarasi lokal (Opsional) ---
        if has_decls {
            let decls_is_last = false; // Body selalu ada
            self.prefix_stack.push(decls_is_last); // PUSH <declaration-part>
            output.push_str(&self.print_node_with_last("<declaration-part>", decls_is_last));
            self.indent_level += 1;
            
            let num_decls = proc_decl.declarations.len();
            for (i, decl) in proc_decl.declarations.iter().enumerate() {
                let is_last_decl = i == num_decls - 1;
                output.push_str(&self.print_declaration_with_last(decl, is_last_decl));
            }

            self.indent_level -= 1;
            self.prefix_stack.pop(); // POP <declaration-part>
        }

        // --- Child 3: Body ---
        output.push_str(&self.print_compound_statement_with_last(&proc_decl.body, false)); // Body bukan terakhir

        // --- Child 4: Semicolon ---
        output.push_str(&self.print_terminal_with_last("SEMICOLON(;", true));
        
        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }

    fn print_procedure_header_with_last(&mut self, proc_decl: &ProcedureDeclaration, is_last: bool) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node_with_last("<procedure-header>", is_last));
        self.indent_level += 1;
        self.prefix_stack.push(is_last);

        let has_params = !proc_decl.parameters.is_empty();

        output.push_str(&self.print_terminal_with_last(&format!("KEYWORD(prosedur)"), false));
        // 'Identifier' terakhir HANYA jika tidak ada params DAN semicolon
        output.push_str(&self.print_terminal_with_last(&format!("IDENTIFIER({})", proc_decl.name), !has_params));

        if has_params {
            // Parameter list bukan terakhir, semicolon ada
            output.push_str(&self.print_formal_parameter_list_with_last(&proc_decl.parameters, false));
        }

        output.push_str(&self.print_terminal_with_last("SEMICOLON(;", true));

        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }

    fn print_function_declaration_with_last(&mut self, func_decl: &FunctionDeclaration, is_last: bool) -> String {
        let mut output = String::new();

        output.push_str(&self.print_node_with_last("<function-declaration>", is_last));
        self.indent_level += 1;
        self.prefix_stack.push(is_last);

        let has_decls = !func_decl.declarations.is_empty();

        // --- Child 1: Header ---
        output.push_str(&self.print_function_header_with_last(func_decl, false)); // Header bukan terakhir

        // --- Child 2: Deklarasi lokal (Opsional) ---
        if has_decls {
            let decls_is_last = false; // Body selalu ada
            self.prefix_stack.push(decls_is_last); // PUSH <declaration-part>
            output.push_str(&self.print_node_with_last("<declaration-part>", decls_is_last));
            self.indent_level += 1;
            
            let num_decls = func_decl.declarations.len();
            for (i, decl) in func_decl.declarations.iter().enumerate() {
                let is_last_decl = i == num_decls - 1;
                output.push_str(&self.print_declaration_with_last(decl, is_last_decl));
            }

            self.indent_level -= 1;
            self.prefix_stack.pop(); // POP <declaration-part>
        }

        // --- Child 3: Body ---
        output.push_str(&self.print_compound_statement_with_last(&func_decl.body, false)); // Body bukan terakhir

        // --- Child 4: Semicolon ---
        output.push_str(&self.print_terminal_with_last("SEMICOLON(;", true));
        
        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }

    fn print_function_header_with_last(&mut self, func_decl: &FunctionDeclaration, is_last: bool) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node_with_last("<function-header>", is_last));
        self.indent_level += 1;
        self.prefix_stack.push(is_last);

        let has_params = !func_decl.parameters.is_empty();

        output.push_str(&self.print_terminal_with_last(&format!("KEYWORD(fungsi)"), false));
        output.push_str(&self.print_terminal_with_last(&format!("IDENTIFIER({})", func_decl.name), false)); // Tidak pernah terakhir

        if has_params {
            output.push_str(&self.print_formal_parameter_list_with_last(&func_decl.parameters, false));
        }

        output.push_str(&self.print_terminal_with_last("COLON(:)", false));
        output.push_str(&self.print_type_node_with_last(&func_decl.return_type, false)); // <type> node
        output.push_str(&self.print_terminal_with_last("SEMICOLON(;", true));

        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }

    /// Ini HANYA mencetak terminal, BUKAN node <type>
    fn print_type(&self, var_type: &Type, is_last: bool) -> String {
        match var_type {
            Type::Integer => self.print_terminal_with_last("KEYWORD(integer)", is_last),
            Type::Real => self.print_terminal_with_last("KEYWORD(real)", is_last),
            Type::Boolean => self.print_terminal_with_last("KEYWORD(boolean)", is_last),
            Type::String => self.print_terminal_with_last("KEYWORD(string)", is_last),
            Type::Char => self.print_terminal_with_last("KEYWORD(char)", is_last),
            Type::Array(array_def) => {
                // Show as: larik[range] dari base_type
                self.print_terminal_with_last(&format!(
                    "ARRAY_TYPE(larik[...] dari {})",
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
                self.print_terminal_with_last("TYPE(Subrange)", is_last)
            }
            Type::TypeIdentifier(name) => {
                self.print_terminal_with_last(&format!("TYPE(Identifier: {})", name), is_last)
            }
        }
    }

    fn print_compound_statement_with_last(&mut self, stmt: &CompoundStatement, is_last: bool) -> String {
        let mut output = String::new();
        
        output.push_str(&self.print_node_with_last("<compound-statement>", is_last));
        self.indent_level += 1;
        self.prefix_stack.push(is_last); // PUSH status <compound-statement>

        let has_statements = !stmt.statements.is_empty();
        
        // --- Child 1: 'mulai' ---
        // 'mulai' adalah terakhir HANYA jika tidak ada statement-list
        let mulai_is_last = !has_statements;
        output.push_str(&self.print_terminal_with_last("KEYWORD(mulai)", mulai_is_last));
        
        // --- Child 2: 'statement-list' (Opsional) ---
        if has_statements {
            let list_is_last = false; // 'selesai' selalu ada
            self.prefix_stack.push(list_is_last); // PUSH: 'statement-list'
            output.push_str(&self.print_node_with_last("<statement-list>", list_is_last));

            self.indent_level += 1;
            
            let num_statements = stmt.statements.len();
            for (i, statement) in stmt.statements.iter().enumerate() {
                let is_last_stmt = i == num_statements - 1;
                
                output.push_str(&self.print_statement_with_last(statement, is_last_stmt));
                
                if !is_last_stmt {
                    output.push_str(&self.print_terminal_with_last("SEMICOLON(;", false));
                }
            }
            
            self.indent_level -= 1;
            self.prefix_stack.pop(); // POP: Selesai dengan 'statement-list'
        }
        
        // --- Child 3: 'selesai' ---
        // 'selesai' selalu terakhir (relatif terhadap 'mulai'/'statement-list')
        output.push_str(&self.print_terminal_with_last("KEYWORD(selesai)", true));

        self.indent_level -= 1;
        self.prefix_stack.pop(); // POP status <compound-statement>
        output
    }

    fn print_statement_with_last(&mut self, stmt: &Statement, is_last: bool) -> String {
        match stmt {
            Statement::Compound(stmt) => self.print_compound_statement_with_last(stmt, is_last),
            Statement::Assignment(stmt) => self.print_assignment_statement_with_last(stmt, is_last),
            Statement::If(stmt) => self.print_if_statement_with_last(stmt, is_last),
            Statement::While(stmt) => self.print_while_statement_with_last(stmt, is_last),
            Statement::For(stmt) => self.print_for_statement_with_last(stmt, is_last),
            Statement::Repeat(stmt) => self.print_repeat_statement_with_last(stmt, is_last),
            Statement::Case(stmt) => self.print_case_statement_with_last(stmt, is_last),
            Statement::Read(stmt) => self.print_read_statement_with_last(stmt, is_last),
            Statement::Write(stmt) => self.print_write_statement_with_last(stmt, is_last),
            Statement::ProcedureCall(stmt) => self.print_procedure_call_statement_with_last(stmt, is_last),
            
            Statement::ExpressionStatement(expr) => {
                match expr {
                    Expression::FunctionCall { function_name, arguments } => {
                        if function_name.to_lowercase() == "writeln" {
                            self.print_write_statement_with_last(&WriteStatement { expressions: arguments.clone() }, is_last)
                        } else if function_name.to_lowercase() == "readln" {
                             self.print_read_statement_with_last(&ReadStatement { variables: arguments.clone() }, is_last)
                        } else {
                            self.print_procedure_call_statement_with_last(&ProcedureCallStatement {
                                procedure_name: function_name.clone(),
                                arguments: arguments.clone(),
                            }, is_last)
                        }
                    }
                    _ => {
                        let mut output = String::new();
                        output.push_str(&self.print_node_with_last("<expression-statement>", is_last));
                        self.indent_level += 1;
                        self.prefix_stack.push(is_last);
                        
                        output.push_str(&self.print_expression_with_last(expr, true)); // Expr adalah anak tunggal
                        
                        self.indent_level -= 1;
                        self.prefix_stack.pop();
                        output
                    }
                }
            },
        }
    }

    fn print_expression_with_last(&mut self, expr: &Expression, is_last: bool) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node_with_last("<expression>", is_last));
        self.indent_level += 1;
        self.prefix_stack.push(is_last);
        
        match expr {
            Expression::BinaryOp { left, operator, right } 
            if matches!(operator.as_str(), "=" | "<>" | "<" | "<=" | ">" | ">=") => {
                // <simple-expression> (kiri)
                output.push_str(&self.print_simple_expression_with_last(left, false)); 
                // RELATIONAL_OPERATOR
                output.push_str(&self.print_terminal_with_last(&format!("RELATIONAL_OPERATOR({})", operator), false));
                // <simple-expression> (kanan)
                output.push_str(&self.print_simple_expression_with_last(right, true));
            }
            _ => {
                output.push_str(&self.print_simple_expression_with_last(expr, true));
            }
        }
        
        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }
    
    fn print_simple_expression_with_last(&mut self, expr: &Expression, is_last: bool) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node_with_last("<simple-expression>", is_last));
        self.indent_level += 1;
        self.prefix_stack.push(is_last);

        match expr {
            Expression::BinaryOp { left, operator, right } => {
                if matches!(operator.as_str(), "+" | "-" | "atau") {
                    // <simple-expression> (kiri)
                    output.push_str(&self.print_simple_expression_with_last(left, false));
                    // OPERATOR
                    output.push_str(&self.print_terminal_with_last(&format!("OPERATOR({})", operator), false));
                    // <term> (kanan)
                    output.push_str(&self.print_term_with_last(right, true));
                }
            }
            Expression::UnaryOp { operator, operand } => {
                if matches!(operator.as_str(), "+" | "-") {
                    output.push_str(&self.print_terminal_with_last(&format!("UNARY_OP({})", operator), false));
                    // <simple-expression> (operand)
                    output.push_str(&self.print_simple_expression_with_last(operand, true));
                }
            }
            // base case: literal, identifier, dll, dibungkus <term>
            _ => {
                output.push_str(&self.print_term_with_last(expr, true));

            }
        }

        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }

    fn print_term_with_last(&mut self, expr: &Expression, is_last: bool) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node_with_last("<term>", is_last));
        self.indent_level += 1;
        self.prefix_stack.push(is_last);

        match expr {
            // Kasus Multiplikatif
            Expression::BinaryOp { left, operator, right }
                if matches!(operator.as_str(), "*" | "/" | "div" | "mod" | "dan") => {
                    // <term> (kiri)
                    output.push_str(&self.print_term_with_last(left, false));
                    // OPERATOR
                    output.push_str(&self.print_terminal_with_last(&format!("OPERATOR({})", operator), false));
                    // <factor> (kanan)
                    output.push_str(&self.print_factor_with_last(right, true));
                }
            // Jika bukan, delegasikan ke level di bawahnya
            _ => {
                output.push_str(&self.print_factor_with_last(expr, true));
            }
        }

        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }

    fn print_factor_with_last(&mut self, expr: &Expression, is_last: bool) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node_with_last("<factor>", is_last));
        self.indent_level += 1;
        self.prefix_stack.push(is_last);

        match expr {
            // Kasus Unary 'tidak' (not)
            Expression::UnaryOp { operator, operand } =>
            if operator == "tidak" {
                output.push_str(&self.print_terminal_with_last(&format!("LOGICAL_OPERATOR({})", operator), false));
                output.push_str(&self.print_factor_with_last(operand, true));
            }
            
            // Kasus "daun" (Literal, Identifier, Array, FuncCall)
            Expression::Literal(lit) => output.push_str(&self.print_literal_with_last(lit, true)),
            Expression::Identifier(name) => output.push_str(&self.print_terminal_with_last(&format!("IDENTIFIER({})", name), true)),
            Expression::ArrayAccess { array, index } => output.push_str(&self.print_array_access_with_last(array, index, true)),
            Expression::FunctionCall { function_name, arguments } => output.push_str(&self.print_function_call_with_last(function_name, arguments, true)),

            // Kasus dalam kurung '(...)', yang di-parse sebagai ekspresi biasa
            // (Ini menangkap BinaryOp atau UnaryOp sign yang 'lolos' dari level di atas)
            _ => {
                // Delegasikan kembali ke level <expression> (level tertinggi)
                // untuk memulai ulang pemeriksaan precedence dari dalam kurung.
                output.push_str(&self.print_expression_with_last(expr, true));
            }
        }

        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }

    fn print_array_access_with_last(&mut self, array: &Expression, index: &Expression, is_last: bool) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node_with_last("<array-access>", is_last));
        self.indent_level += 1;
        self.prefix_stack.push(is_last);
        
        if let Expression::Identifier(ref name) = *array {
            self.prefix_stack.push(false); // PUSH: ID
            output.push_str(&self.print_terminal_with_last(&format!("IDENTIFIER({})", name), false));
            self.prefix_stack.pop(); // POP
        } else {
            // Jika array adalah ekspresi lain yang kompleks
            output.push_str(&self.print_expression_with_last(array, false));
        }
        output.push_str(&self.print_terminal_with_last("LBRACKET([)", false));
        output.push_str(&self.print_expression_with_last(index, false));
        output.push_str(&self.print_terminal_with_last("RBRACKET(])", true));
        
        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }

    fn print_function_call_with_last(&mut self, function_name: &str, arguments: &Vec<Expression>, is_last: bool) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node_with_last("<function-call>", is_last));
        self.indent_level += 1;
        self.prefix_stack.push(is_last);
        
        let has_args = !arguments.is_empty();
        
        output.push_str(&self.print_terminal_with_last(&format!("IDENTIFIER({})", function_name), false));
        output.push_str(&self.print_terminal_with_last("LPARENTHESIS(()", !has_args)); // Terakhir jika tidak ada argumen
        
        if has_args {
            output.push_str(&self.print_parameter_list_with_last(arguments, false)); // <parameter-list>
            output.push_str(&self.print_terminal_with_last("RPARENTHESIS())", true));
        }
        
        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }

    fn print_literal_with_last(&mut self, lit: &LiteralValue, is_last: bool) -> String {
        let lit_str = match lit {
            LiteralValue::Integer(i) => format!("INT_LITERAL({})", i),
            LiteralValue::Real(r) => format!("REAL_LITERAL({})", r),
            LiteralValue::String(s) => format!("STRING_LITERAL('{}')", s),
            LiteralValue::Char(c) => format!("CHAR_LITERAL('{}')", c),
            LiteralValue::Boolean(b) => format!("BOOLEAN({})", if *b { "benar" } else { "salah" }),
        };
        self.print_terminal_with_last(&lit_str, is_last)
    }

    fn print_node_with_last(&self, label: &str, is_last: bool) -> String {
        format!("{}{}\n", self.get_prefix(is_last), label)
    }

    fn print_terminal_with_last(&self, label: &str, is_last: bool) -> String {
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

    fn print_assignment_statement_with_last(&mut self, assign: &AssignmentStatement, is_last: bool) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node_with_last("<assignment-statement>", is_last));
        self.indent_level += 1;
        self.prefix_stack.push(is_last);
        
        output.push_str(&self.print_expression_with_last(&assign.variable, false)); // Variabel
        output.push_str(&self.print_terminal_with_last("ASSIGN_OPERATOR(:=)", false));
        output.push_str(&self.print_expression_with_last(&assign.expression, true)); // Ekspresi
        
        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }

    fn print_if_statement_with_last(&mut self, stmt: &IfStatement, is_last: bool) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node_with_last("<if-statement>", is_last));
        self.indent_level += 1;
        self.prefix_stack.push(is_last);

        let has_else = stmt.else_branch.is_some();

        output.push_str(&self.print_terminal_with_last("KEYWORD(jika)", false));
        output.push_str(&self.print_expression_with_last(&stmt.condition, false));
        output.push_str(&self.print_terminal_with_last("KEYWORD(maka)", false));
        
        // then_branch terakhir HANYA jika tidak ada else
        output.push_str(&self.print_statement_with_last(&stmt.then_branch, !has_else)); 

        if let Some(else_branch) = &stmt.else_branch {
            output.push_str(&self.print_terminal_with_last("KEYWORD(selain-itu)", false));
            output.push_str(&self.print_statement_with_last(else_branch, true)); // else_branch selalu terakhir
        }

        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }

    fn print_while_statement_with_last(&mut self, stmt: &WhileStatement, is_last: bool) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node_with_last("<while-statement>", is_last));
        self.indent_level += 1;
        self.prefix_stack.push(is_last);

        output.push_str(&self.print_terminal_with_last("KEYWORD(selama)", false));
        output.push_str(&self.print_expression_with_last(&stmt.condition, false));
        output.push_str(&self.print_terminal_with_last("KEYWORD(lakukan)", false));
        output.push_str(&self.print_statement_with_last(&stmt.body, true)); // body selalu terakhir

        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }

    fn print_for_statement_with_last(&mut self, stmt: &ForStatement, is_last: bool) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node_with_last("<for-statement>", is_last));
        self.indent_level += 1;
        self.prefix_stack.push(is_last);

        output.push_str(&self.print_terminal_with_last("KEYWORD(untuk)", false));
        output.push_str(&self.print_terminal_with_last(&format!("IDENTIFIER({})", stmt.counter_variable), false));
        output.push_str(&self.print_terminal_with_last("ASSIGN_OPERATOR(:=)", false));
        output.push_str(&self.print_expression_with_last(&stmt.start_value, false));

        let direction = match stmt.direction {
            ForDirection::To => "KEYWORD(ke)",
            ForDirection::DownTo => "KEYWORD(turun-ke)",
        };
        output.push_str(&self.print_terminal_with_last(direction, false));
        output.push_str(&self.print_expression_with_last(&stmt.end_value, false));
        output.push_str(&self.print_terminal_with_last("KEYWORD(lakukan)", false));
        output.push_str(&self.print_statement_with_last(&stmt.body, true)); // body selalu terakhir

        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }

    fn print_repeat_statement_with_last(&mut self, stmt: &RepeatStatement, is_last: bool) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node_with_last("<repeat-statement>", is_last));
        self.indent_level += 1;
        self.prefix_stack.push(is_last);
        
        let has_statements = !stmt.statements.is_empty();

        output.push_str(&self.print_terminal_with_last("KEYWORD(ulangi)", !has_statements));
        
        if has_statements {
            // --- Child 2: <statement-list> ---
            let list_is_last = false; // 'sampai' selalu ada
            self.prefix_stack.push(list_is_last); // PUSH: 'statement-list'
            output.push_str(&self.print_node_with_last("<statement-list>", list_is_last));
            self.indent_level += 1;
            
            let num_statements = stmt.statements.len();
            for (i, statement) in stmt.statements.iter().enumerate() {
                let is_last_stmt = i == num_statements - 1;
                output.push_str(&self.print_statement_with_last(statement, is_last_stmt));
                if !is_last_stmt {
                    output.push_str(&self.print_terminal_with_last("SEMICOLON(;", false));
                }
            }
            self.indent_level -= 1;
            self.prefix_stack.pop(); // POP 'statement-list'
        }

        // --- Child 3: 'sampai' ---
        output.push_str(&self.print_terminal_with_last("KEYWORD(sampai)", false));
        
        // --- Child 4: condition ---
        output.push_str(&self.print_expression_with_last(&stmt.condition, true));

        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }

    fn print_case_statement_with_last(&mut self, stmt: &CaseStatement, is_last: bool) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node_with_last("<case-statement>", is_last));
        self.indent_level += 1;
        self.prefix_stack.push(is_last);

        let has_else = stmt.else_branch.is_some();
        let num_branches = stmt.branches.len();
        let has_branches = num_branches > 0;

        output.push_str(&self.print_terminal_with_last("KEYWORD(kasus)", false));
        output.push_str(&self.print_expression_with_last(&stmt.expression, false));
        
        let dari_is_last = !has_branches && !has_else;
        output.push_str(&self.print_terminal_with_last("KEYWORD(dari)", dari_is_last));

        for (i, branch) in stmt.branches.iter().enumerate() {
            let is_last_branch = i == num_branches - 1 && !has_else;
            output.push_str(&self.print_case_branch_with_last(branch, is_last_branch));
        }

        if let Some(else_branch) = &stmt.else_branch {
            // 'else' selalu terakhir di antara 'branches'
            output.push_str(&self.print_else_branch_with_last(else_branch, true)); 
        }

        output.push_str(&self.print_terminal_with_last("KEYWORD(selesai)", true));
        
        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }

    fn print_case_branch_with_last(&mut self, branch: &CaseBranch, is_last: bool) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node_with_last("<case-branch>", is_last));
        self.indent_level += 1;
        self.prefix_stack.push(is_last);

        // --- Child 1: <label-list> ---
        let list_is_last = false; // Diikuti oleh COLON, dll.
        self.prefix_stack.push(list_is_last); // PUSH: <label-list>
        output.push_str(&self.print_node_with_last("<label-list>", list_is_last));
        self.indent_level += 1;
        
        let num_labels = branch.labels.len();
        if num_labels > 0 {
            for (i, label) in branch.labels.iter().enumerate() {
                let is_last_label = i == num_labels - 1;
                output.push_str(&self.print_expression_with_last(label, is_last_label));
                if !is_last_label {
                    output.push_str(&self.print_terminal_with_last("COMMA(,", false));
                }
            }
        }
        self.indent_level -= 1;
        self.prefix_stack.pop(); // POP <label-list>

        // --- Child 2: COLON ---
        output.push_str(&self.print_terminal_with_last("COLON(:)", false));

        // --- Child 3: Statement ---
        output.push_str(&self.print_statement_with_last(&branch.statement, false)); // Semicolon selalu ada

        // --- Child 4: SEMICOLON ---
        output.push_str(&self.print_terminal_with_last("SEMICOLON(;", true));
        
        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }
    
    fn print_else_branch_with_last(&mut self, else_branch: &Vec<Statement>, is_last: bool) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node_with_last("<else-branch>", is_last));
        self.indent_level += 1;
        self.prefix_stack.push(is_last);

        let num_statements = else_branch.len();
        let has_statements = num_statements > 0;

        output.push_str(&self.print_terminal_with_last("KEYWORD(selain-itu)", !has_statements));

        if has_statements {
            for (i, statement) in else_branch.iter().enumerate() {
                let is_last_stmt = i == num_statements - 1;
                output.push_str(&self.print_statement_with_last(statement, is_last_stmt));
                if !is_last_stmt {
                    output.push_str(&self.print_terminal_with_last("SEMICOLON(;", false));
                }
            }
        }

        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }

    /// Helper untuk ( ... )
    fn print_parameter_list_with_last(&mut self, arguments: &Vec<Expression>, is_last: bool) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node_with_last("<parameter-list>", is_last));
        self.indent_level += 1;
        self.prefix_stack.push(is_last);

        let num_args = arguments.len();
        if num_args > 0 {
            for (i, arg) in arguments.iter().enumerate() {
                let is_last_arg = i == num_args - 1;
                output.push_str(&self.print_expression_with_last(arg, is_last_arg));
                if !is_last_arg {
                    output.push_str(&self.print_terminal_with_last("COMMA(,", false));
                }
            }
        }
        
        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }
    
    fn print_procedure_call_statement_with_last(&mut self, call: &ProcedureCallStatement, is_last: bool) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node_with_last("<procedure-call>", is_last));
        self.indent_level += 1;
        self.prefix_stack.push(is_last);
        
        // Reuse print_function_call karena identical
        output.push_str(&self.print_function_call_with_last(&call.procedure_name, &call.arguments, true));

        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }
    
    fn print_read_statement_with_last(&mut self, stmt: &ReadStatement, is_last: bool) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node_with_last("<procedure-call>", is_last)); // readln adalah procedure call
        self.indent_level += 1;
        self.prefix_stack.push(is_last);
       
        output.push_str(&self.print_function_call_with_last("readln", &stmt.variables, true));

        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }
    
    fn print_write_statement_with_last(&mut self, stmt: &WriteStatement, is_last: bool) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node_with_last("<procedure-call>", is_last)); // writeln adalah procedure call
        self.indent_level += 1;
        self.prefix_stack.push(is_last);
        
        output.push_str(&self.print_function_call_with_last("writeln", &stmt.expressions, true));

        self.indent_level -= 1;
        self.prefix_stack.pop();
        output
    }
}