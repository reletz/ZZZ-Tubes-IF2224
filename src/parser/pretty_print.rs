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
}

impl AstPrinter {
    pub fn new() -> Self {
        AstPrinter { indent_level: 0 }
    }

    pub fn print_program(&mut self, program: &Program) -> String {
        let mut output = String::new();
        
        output.push_str("<program>\n");
        self.indent_level += 1;
        
        // Program header
        output.push_str(&self.print_node("<program-header>"));
        self.indent_level += 1;
        output.push_str(&self.print_terminal("KEYWORD(program)"));
        output.push_str(&self.print_terminal(&format!("IDENTIFIER({})", program.name)));
        output.push_str(&self.print_terminal("SEMICOLON(;)"));
        self.indent_level -= 1;
        
        // Declarations
        if !program.declarations.is_empty() {
            output.push_str(&self.print_node("<declaration-part>"));
            self.indent_level += 1;
            for decl in &program.declarations {
                output.push_str(&self.print_declaration(decl));
            }
            self.indent_level -= 1;
        }
        
        // Compound statement
        output.push_str(&self.print_compound_statement(&program.body));
        
        output.push_str(&self.print_terminal("DOT(.)"));
        
        self.indent_level -= 1;
        output
    }

    fn print_declaration(&mut self, decl: &Declaration) -> String {
        match decl {
            Declaration::Variable(var_decl) => self.print_variable_declaration(var_decl),
            Declaration::Constant(const_decl) => self.print_constant_declaration(const_decl),
            Declaration::Type(type_decl) => self.print_type_declaration(type_decl),
            Declaration::Procedure(proc_decl) => self.print_procedure_declaration(proc_decl),
            Declaration::Function(func_decl) => self.print_function_declaration(func_decl),
        }
    }

    fn print_variable_declaration(&mut self, var_decl: &VariableDeclaration) -> String {
        let mut output = String::new();
        
        output.push_str(&self.print_node("<var-declaration>"));
        self.indent_level += 1;
        
        output.push_str(&self.print_terminal("KEYWORD(variabel)"));
        
        for group in &var_decl.groups {
            // Identifier list
            output.push_str(&self.print_node("<identifier-list>"));
            self.indent_level += 1;
            
            for (i, id) in group.identifiers.iter().enumerate() {
                output.push_str(&self.print_terminal(&format!("IDENTIFIER({})", id)));
                if i < group.identifiers.len() - 1 {
                    output.push_str(&self.print_terminal("COMMA(,)"));
                }
            }
            
            self.indent_level -= 1;
            
            output.push_str(&self.print_terminal("COLON(:)"));
            
            // Type
            output.push_str(&self.print_node("<type>"));
            self.indent_level += 1;
            output.push_str(&self.print_type(&group.var_type));
            self.indent_level -= 1;
            
            output.push_str(&self.print_terminal("SEMICOLON(;)"));
        }
        
        self.indent_level -= 1;
        output
    }

    fn print_constant_declaration(&mut self, const_decl: &ConstantDeclaration) -> String {
        let mut output = String::new();

        output.push_str(&self.print_node("<const-declaration>"));
        self.indent_level += 1;

        output.push_str(&self.print_terminal("KEYWORD(konstanta)"));

        for def in &const_decl.constants {
            // Constant definition
            output.push_str(&self.print_node("<const-definition>"));
            self.indent_level += 1;

            output.push_str(&self.print_terminal(&format!("IDENTIFIER({})", def.name)));
            output.push_str(&self.print_terminal("OPERATOR(=)"));
            output.push_str(&self.print_expression(&def.value)); 
            output.push_str(&self.print_terminal("SEMICOLON(;)"));
            self.indent_level -= 1;
        }
        
        self.indent_level -= 1;
        output
    }

    fn print_type_declaration(&mut self, type_decl: &TypeDeclaration) -> String {
        let mut output = String::new();

        output.push_str(&self.print_node("<type-declaration>"));
        self.indent_level += 1;

        output.push_str(&self.print_terminal("KEYWORD(tipe)"));

        for def in &type_decl.definitions {
            // Type definition
            output.push_str(&self.print_node("<type-definition>"));
            self.indent_level += 1;

            output.push_str(&self.print_terminal(&format!("IDENTIFIER({})", def.name)));
            output.push_str(&self.print_terminal("OPERATOR(=)"));
            output.push_str(&self.print_node("<type>"));
            self.indent_level += 1;

            output.push_str(&self.print_type(&def.type_def));
            self.indent_level -= 1;

            output.push_str(&self.print_terminal("SEMICOLON(;)"));
            self.indent_level -= 1;
        }
        
        self.indent_level -= 1;
        output
    }

    fn print_procedure_declaration(&mut self, proc_decl: &ProcedureDeclaration) -> String {
        let mut output = String::new();

        output.push_str(&self.print_node("<procedure-declaration>"));
        self.indent_level += 1;

        // header
        output.push_str(&self.print_node("<procedure-header>"));
        self.indent_level += 1;

        output.push_str(&self.print_terminal(&format!("KEYWORD(prosedur)")));
        output.push_str(&self.print_terminal(&format!("IDENTIFIER({})", proc_decl.name)));

        if !proc_decl.parameters.is_empty() {
            output.push_str(&self.print_formal_parameter_list(&proc_decl.parameters));
        }

        output.push_str(&self.print_terminal("SEMICOLON(;)"));
        self.indent_level -= 1;

        // deklarasi lokal
        if !proc_decl.declarations.is_empty() {
            output.push_str(&self.print_node("<declaration-part>"));
            self.indent_level += 1;
            for decl in &proc_decl.declarations {
                output.push_str(&self.print_declaration(decl));
            }
            self.indent_level -= 1;
        }

        // body
        output.push_str(&self.print_compound_statement(&proc_decl.body));
        output.push_str(&self.print_terminal("SEMICOLON(;)"));
        
        self.indent_level -= 1;
        output
    }

    fn print_function_declaration(&mut self, func_decl: &FunctionDeclaration) -> String {
        let mut output = String::new();

        output.push_str(&self.print_node("<function-declaration>"));
        self.indent_level += 1;

        // header
        output.push_str(&self.print_node("<function-header>"));
        self.indent_level += 1;

        output.push_str(&self.print_terminal(&format!("KEYWORD(fungsi)")));
        output.push_str(&self.print_terminal(&format!("IDENTIFIER({})", func_decl.name)));

        if !func_decl.parameters.is_empty() {
            output.push_str(&self.print_formal_parameter_list(&func_decl.parameters));
        }

        output.push_str(&self.print_terminal("COLON(:)"));
        output.push_str(&self.print_node("<type>"));
        self.indent_level += 1;

        output.push_str(&self.print_type(&func_decl.return_type));
        self.indent_level -= 1;
        
        output.push_str(&self.print_terminal("SEMICOLON(;)"));
        self.indent_level -= 1;

        // deklarasi lokal
        if !func_decl.declarations.is_empty() {
            output.push_str(&self.print_node("<declaration-part>"));
            self.indent_level += 1;
            for decl in &func_decl.declarations {
                output.push_str(&self.print_declaration(decl));
            }
            self.indent_level -= 1;
        }

        // body
        output.push_str(&self.print_compound_statement(&func_decl.body));
        output.push_str(&self.print_terminal("SEMICOLON(;)"));
        
        self.indent_level -= 1;
        output
    }

    fn print_formal_parameter_list(&mut self, params: &Vec<FormalParameterGroup>) -> String {
        let mut output = String::new();

        output.push_str(&self.print_node("<formal-parameter-list>"));
        self.indent_level += 1;

        output.push_str(&self.print_terminal("LPARENTHESIS(()"));

        for (i, group) in params.iter().enumerate() {
            // Parameter group
            output.push_str(&self.print_node("<parameter-group>"));
            self.indent_level += 1;
            
            output.push_str(&self.print_node("<identifier-list>"));
            self.indent_level += 1;

            for (j, id) in group.identifiers.iter().enumerate() {
                output.push_str(&self.print_terminal(&format!("IDENTIFIER({})", id)));
                if j < group.identifiers.len() - 1 {
                        output.push_str(&self.print_terminal("COMMA(,)"));
                }
            }
            self.indent_level -= 1;

            output.push_str(&self.print_terminal("COLON(:)"));
            output.push_str(&self.print_node("<type>"));
            self.indent_level += 1;

            output.push_str(&self.print_type(&group.var_type));
            self.indent_level -= 1;
            
            self.indent_level -= 1;

            if i < params.len() - 1 {
                output.push_str(&self.print_terminal("SEMICOLON(;)"));
            }
        }

        output.push_str(&self.print_terminal("RPARENTHESIS())"));
        self.indent_level -= 1;
        output
    }

    fn print_type(&self, var_type: &Type) -> String {
        match var_type {
            Type::Integer => self.print_terminal("KEYWORD(integer)"),
            Type::Real => self.print_terminal("KEYWORD(real)"),
            Type::Boolean => self.print_terminal("KEYWORD(boolean)"),
            Type::String => self.print_terminal("KEYWORD(string)"),
            Type::Char => self.print_terminal("KEYWORD(char)"),
            Type::Array(array_def) => {
                // Show as: larik[range] dari base_type
                self.print_terminal(&format!(
                    "ARRAY_TYPE(larik[...] dari {})",
                    match *array_def.base_type {
                        Type::Integer => "integer",
                        Type::Real => "real",
                        Type::Boolean => "boolean",
                        Type::String => "string",
                        Type::Char => "char",
                        _ => "complex"
                    }
                ))
            }
            Type::Subrange(subrange) => {
                self.print_terminal("TYPE(Subrange)")
            }
            Type::TypeIdentifier(name) => {
                self.print_terminal(&format!("TYPE(Identifier: {})", name))
            }
        }
    }

    fn print_compound_statement(&mut self, stmt: &CompoundStatement) -> String {
        let mut output = String::new();
        
        output.push_str(&self.print_node("<compound-statement>"));
        self.indent_level += 1;
        
        output.push_str(&self.print_terminal("KEYWORD(mulai)"));
        
        if !stmt.statements.is_empty() {
            output.push_str(&self.print_node("<statement-list>"));
            self.indent_level += 1;
            
            for (i, statement) in stmt.statements.iter().enumerate() {
                output.push_str(&self.print_statement(statement));
                if i < stmt.statements.len() - 1 {
                    output.push_str(&self.print_terminal("SEMICOLON(;)"));
                }
            }
            
            self.indent_level -= 1;
        }
        
        output.push_str(&self.print_terminal("KEYWORD(selesai)"));
        
        self.indent_level -= 1;
        output
    }

    fn print_statement(&mut self, stmt: &Statement) -> String {
        match stmt {
            Statement::Compound(stmt) => self.print_compound_statement(stmt),
            Statement::Assignment(stmt) => self.print_assignment_statement(stmt),
            Statement::If(stmt) => self.print_if_statement(stmt),
            Statement::While(stmt) => self.print_while_statement(stmt),
            Statement::For(stmt) => self.print_for_statement(stmt),
            Statement::Repeat(stmt) => self.print_repeat_statement(stmt),
            Statement::Case(stmt) => self.print_case_statement(stmt),
            Statement::Read(stmt) => self.print_read_statement(stmt),
            Statement::Write(stmt) => self.print_write_statement(stmt),
            Statement::ProcedureCall(stmt) => self.print_procedure_call_statement(stmt),
            
            Statement::ExpressionStatement(expr) => {
                match expr {
                    Expression::FunctionCall { function_name, arguments } => {
                        if function_name.to_lowercase() == "writeln" {
                            self.print_write_statement(&WriteStatement { expressions: arguments.clone() })
                        } else if function_name.to_lowercase() == "readln" {
                             self.print_read_statement(&ReadStatement { variables: arguments.clone() })
                        } else {
                            self.print_procedure_call_statement(&ProcedureCallStatement {
                                procedure_name: function_name.clone(),
                                arguments: arguments.clone(),
                            })
                        }
                    }
                    _ => {
                        let mut output = String::new();
                        output.push_str(&self.print_node("<expression-statement>"));
                        self.indent_level += 1;
                        output.push_str(&self.print_expression(expr));
                        self.indent_level -= 1;
                        output
                    }
                }
            },
            Statement::Empty => self.print_terminal("<empty-statement>"),
            Statement::Placeholder => self.print_terminal("<placeholder-statement>"),
        }
    }

    fn print_expression(&mut self, expr: &Expression) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node("<expression>"));
        self.indent_level += 1;
        
        match expr {
            Expression::Literal(lit) => {
                output.push_str(&self.print_node("<simple-expression>"));
                self.indent_level += 1;
                output.push_str(&self.print_node("<term>"));
                self.indent_level += 1;
                output.push_str(&self.print_node("<factor>"));
                self.indent_level += 1;
                output.push_str(&self.print_literal(lit));
                self.indent_level -= 3;
            }
            Expression::Identifier(name) => {
                output.push_str(&self.print_node("<simple-expression>"));
                self.indent_level += 1;
                output.push_str(&self.print_node("<term>"));
                self.indent_level += 1;
                output.push_str(&self.print_node("<factor>"));
                self.indent_level += 1;
                output.push_str(&self.print_terminal(&format!("IDENTIFIER({})", name)));
                self.indent_level -= 3;
            }
            Expression::BinaryOp { left, operator, right } => {
                output.push_str(&self.print_node("<simple-expression>"));
                self.indent_level += 1;
                output.push_str(&self.print_expression_flat(left));
                output.push_str(&self.print_terminal(&format!("OPERATOR({})", operator)));
                output.push_str(&self.print_expression_flat(right));
                self.indent_level -= 1;
            }
            Expression::UnaryOp { operator, operand } => {
                output.push_str(&self.print_node("<simple-expression>"));
                self.indent_level += 1;
                output.push_str(&self.print_terminal(&format!("UNARY_OP({})", operator)));
                output.push_str(&self.print_expression_flat(operand));
                self.indent_level -= 1;
            }
            _ => {
                output.push_str(&self.print_node("<complex-expression>"));
            }
        }
        
        self.indent_level -= 1;
        output
    }
    
    fn print_expression_flat(&mut self, expr: &Expression) -> String {
        let mut output = String::new();
        
        match expr {
            Expression::Literal(lit) => {
                output.push_str(&self.print_node("<term>"));
                self.indent_level += 1;
                output.push_str(&self.print_node("<factor>"));
                self.indent_level += 1;
                output.push_str(&self.print_literal(lit));
                self.indent_level -= 2;
            }
            Expression::Identifier(name) => {
                output.push_str(&self.print_node("<term>"));
                self.indent_level += 1;
                output.push_str(&self.print_node("<factor>"));
                self.indent_level += 1;
                output.push_str(&self.print_terminal(&format!("IDENTIFIER({})", name)));
                self.indent_level -= 2;
            }
            Expression::BinaryOp { left, operator, right } => {
                output.push_str(&self.print_expression_flat(left));
                output.push_str(&self.print_terminal(&format!("OPERATOR({})", operator)));
                output.push_str(&self.print_expression_flat(right));
            }
            Expression::UnaryOp { operator, operand } => {
                output.push_str(&self.print_terminal(&format!("UNARY_OP({})", operator)));
                output.push_str(&self.print_expression_flat(operand));
            }
            Expression::ArrayAccess { array, index } => {
                output.push_str(&self.print_node("<term>"));
                self.indent_level += 1;
                output.push_str(&self.print_node("<factor>"));
                self.indent_level += 1;
                output.push_str(&self.print_node("<array-access>"));
                self.indent_level += 1;
                
                if let Expression::Identifier(name) = &**array {
                    output.push_str(&self.print_terminal(&format!("IDENTIFIER({})", name)));
                } else {
                    output.push_str(&self.print_expression_flat(array));
                }
                output.push_str(&self.print_terminal("LBRACKET([)"));
                output.push_str(&self.print_expression_flat(index));
                output.push_str(&self.print_terminal("RBRACKET(])"));
                
                self.indent_level -= 3;
            }
            Expression::FunctionCall { function_name, arguments } => {
                output.push_str(&self.print_node("<term>"));
                self.indent_level += 1;
                output.push_str(&self.print_node("<factor>"));
                self.indent_level += 1;
                output.push_str(&self.print_node("<function-call>"));
                self.indent_level += 1;
                
                output.push_str(&self.print_terminal(&format!("IDENTIFIER({})", function_name)));
                output.push_str(&self.print_terminal("LPARENTHESIS(()"));
                
                if !arguments.is_empty() {
                    output.push_str(&self.print_node("<parameter-list>"));
                    self.indent_level += 1;
                    for (i, arg) in arguments.iter().enumerate() {
                        output.push_str(&self.print_expression_flat(arg));
                        if i < arguments.len() - 1 {
                            output.push_str(&self.print_terminal("COMMA(,)"));
                        }
                    }
                    self.indent_level -= 1;
                }
                
                output.push_str(&self.print_terminal("RPARENTHESIS())"));
                self.indent_level -= 3;
            }
            _ => {
                output.push_str(&self.print_node("<nested-expr>"));
            }
        }
        
        output
    }

    fn print_literal(&self, lit: &LiteralValue) -> String {
        let lit_str = match lit {
            LiteralValue::Integer(i) => format!("NUMBER({})", i),
            LiteralValue::Real(r) => format!("NUMBER({})", r),
            LiteralValue::String(s) => format!("STRING_LITERAL('{}')", s),
            LiteralValue::Char(c) => format!("CHAR_LITERAL('{}')", c),
            LiteralValue::Boolean(b) => format!("BOOLEAN({})", if *b { "benar" } else { "salah" }),
        };
        self.print_terminal(&lit_str)
    }

    fn print_node(&self, label: &str) -> String {
        format!("{}{}\n", self.get_prefix(), label)
    }

    fn print_terminal(&self, label: &str) -> String {
        format!("{}{}\n", self.get_prefix(), label)
    }

    fn get_prefix(&self) -> String {
        if self.indent_level == 0 {
            return String::new();
        }
        
        let mut prefix = String::new();
        for i in 0..self.indent_level {
            if i == self.indent_level - 1 {
                prefix.push_str("├── ");
            } else {
                prefix.push_str("│   ");
            }
        }
        prefix
    }

    fn print_assignment_statement(&mut self, assign: &AssignmentStatement) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node("<assignment-statement>"));
        self.indent_level += 1;
        
        output.push_str(&self.print_expression(&assign.variable));
        
        output.push_str(&self.print_terminal("ASSIGN_OPERATOR(:=)"));
        output.push_str(&self.print_expression(&assign.expression));
        
        self.indent_level -= 1;
        output
    }

    fn print_if_statement(&mut self, stmt: &IfStatement) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node("<if-statement>"));
        self.indent_level += 1;

        output.push_str(&self.print_terminal("KEYWORD(jika)"));
        output.push_str(&self.print_expression(&stmt.condition));
        output.push_str(&self.print_terminal("KEYWORD(maka)"));
        
        self.indent_level += 1;
        output.push_str(&self.print_statement(&stmt.then_branch));
        self.indent_level -= 1;

        if let Some(else_branch) = &stmt.else_branch {
            output.push_str(&self.print_terminal("KEYWORD(selain-itu)"));
            self.indent_level += 1;
            output.push_str(&self.print_statement(else_branch));
            self.indent_level -= 1;
        }

        self.indent_level -= 1;
        output
    }

    fn print_while_statement(&mut self, stmt: &WhileStatement) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node("<while-statement>"));
        self.indent_level += 1;

        output.push_str(&self.print_terminal("KEYWORD(selama)"));
        output.push_str(&self.print_expression(&stmt.condition));
        output.push_str(&self.print_terminal("KEYWORD(lakukan)"));
        
        self.indent_level += 1;
        output.push_str(&self.print_statement(&stmt.body));
        self.indent_level -= 1;

        self.indent_level -= 1;
        output
    }

    fn print_for_statement(&mut self, stmt: &ForStatement) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node("<for-statement>"));
        self.indent_level += 1;

        output.push_str(&self.print_terminal("KEYWORD(untuk)"));
        output.push_str(&self.print_terminal(&format!("IDENTIFIER({})", stmt.counter_variable)));
        output.push_str(&self.print_terminal("ASSIGN_OPERATOR(:=)"));
        output.push_str(&self.print_expression(&stmt.start_value));

        let direction = match stmt.direction {
            ForDirection::To => "KEYWORD(ke)",
            ForDirection::DownTo => "KEYWORD(turun-ke)",
        };
        output.push_str(&self.print_terminal(direction));
        output.push_str(&self.print_expression(&stmt.end_value));

        output.push_str(&self.print_terminal("KEYWORD(lakukan)"));
        
        self.indent_level += 1;
        output.push_str(&self.print_statement(&stmt.body));
        self.indent_level -= 1;

        self.indent_level -= 1;
        output
    }

    fn print_repeat_statement(&mut self, stmt: &RepeatStatement) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node("<repeat-statement>"));
        self.indent_level += 1;

        output.push_str(&self.print_terminal("KEYWORD(ulangi)"));
        
        output.push_str(&self.print_node("<statement-list>"));
        self.indent_level += 1;
        for (i, statement) in stmt.statements.iter().enumerate() {
            output.push_str(&self.print_statement(statement));
            if i < stmt.statements.len() - 1 {
                output.push_str(&self.print_terminal("SEMICOLON(;)"));
            }
        }
        self.indent_level -= 1;

        output.push_str(&self.print_terminal("KEYWORD(sampai)"));
        output.push_str(&self.print_expression(&stmt.condition));

        self.indent_level -= 1;
        output
    }

    fn print_case_statement(&mut self, stmt: &CaseStatement) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node("<case-statement>"));
        self.indent_level += 1;

        output.push_str(&self.print_terminal("KEYWORD(kasus)"));
        output.push_str(&self.print_expression(&stmt.expression));
        output.push_str(&self.print_terminal("KEYWORD(dari)"));

        for branch in &stmt.branches {
            output.push_str(&self.print_node("<case-branch>"));
            self.indent_level += 1;

            output.push_str(&self.print_node("<label-list>"));
            self.indent_level += 1;
            for (i, label) in branch.labels.iter().enumerate() {
                output.push_str(&self.print_expression(label));
                if i < branch.labels.len() - 1 {
                    output.push_str(&self.print_terminal("COMMA(,)"));
                }
            }
            self.indent_level -= 1;

            output.push_str(&self.print_terminal("COLON(:)"));
            output.push_str(&self.print_statement(&branch.statement));
            output.push_str(&self.print_terminal("SEMICOLON(;)"));
            
            self.indent_level -= 1;
        }

        if let Some(else_branch) = &stmt.else_branch {
            output.push_str(&self.print_node("<else-branch>"));
            self.indent_level += 1;
            output.push_str(&self.print_terminal("KEYWORD(selain-itu)"));
            for (i, statement) in else_branch.iter().enumerate() {
                output.push_str(&self.print_statement(statement));
                if i < else_branch.len() - 1 {
                    output.push_str(&self.print_terminal("SEMICOLON(;)"));
                }
            }
            self.indent_level -= 1;
        }

        output.push_str(&self.print_terminal("KEYWORD(selesai)"));
        self.indent_level -= 1;
        output
    }
    
    fn print_procedure_call_statement(&mut self, call: &ProcedureCallStatement) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node("<procedure-call>"));
        self.indent_level += 1;
        output.push_str(&self.print_terminal(&format!("IDENTIFIER({})", call.procedure_name)));
        
        output.push_str(&self.print_terminal("LPARENTHESIS(()"));
        if !call.arguments.is_empty() {
            output.push_str(&self.print_node("<parameter-list>"));
            self.indent_level += 1;
            for (i, arg) in call.arguments.iter().enumerate() {
                output.push_str(&self.print_expression(arg));
                if i < call.arguments.len() - 1 {
                    output.push_str(&self.print_terminal("COMMA(,)"));
                }
            }
            self.indent_level -= 1;
        }
        output.push_str(&self.print_terminal("RPARENTHESIS())"));

        self.indent_level -= 1;
        output
    }
    
    fn print_read_statement(&mut self, stmt: &ReadStatement) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node("<procedure-call>"));
        self.indent_level += 1;
        output.push_str(&self.print_terminal("IDENTIFIER(readln)"));
        
        output.push_str(&self.print_terminal("LPARENTHESIS(()"));
        if !stmt.variables.is_empty() {
            output.push_str(&self.print_node("<parameter-list>"));
            self.indent_level += 1;
            for (i, arg) in stmt.variables.iter().enumerate() {
                output.push_str(&self.print_expression(arg));
                if i < stmt.variables.len() - 1 {
                    output.push_str(&self.print_terminal("COMMA(,)"));
                }
            }
            self.indent_level -= 1;
        }
        output.push_str(&self.print_terminal("RPARENTHESIS())"));

        self.indent_level -= 1;
        output
    }
    
    fn print_write_statement(&mut self, stmt: &WriteStatement) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node("<procedure-call>"));
        self.indent_level += 1;
        output.push_str(&self.print_terminal("IDENTIFIER(writeln)"));
        
        output.push_str(&self.print_terminal("LPARENTHESIS(()"));
        if !stmt.expressions.is_empty() {
            output.push_str(&self.print_node("<parameter-list>"));
            self.indent_level += 1;
            for (i, arg) in stmt.expressions.iter().enumerate() {
                output.push_str(&self.print_expression(arg));
                if i < stmt.expressions.len() - 1 {
                    output.push_str(&self.print_terminal("COMMA(,)"));
                }
            }
            self.indent_level -= 1;
        }
        output.push_str(&self.print_terminal("RPARENTHESIS())"));

        self.indent_level -= 1;
        output
    }
}