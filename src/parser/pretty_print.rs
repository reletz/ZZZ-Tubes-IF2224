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
            _ => self.print_node("<declaration>"),
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
            Statement::Assignment(assign) => {
                let mut output = String::new();
                output.push_str(&self.print_node("<assignment-statement>"));
                self.indent_level += 1;
                
                // Print variable (left side) - handle different cases
                match &assign.variable {
                    Expression::Identifier(name) => {
                        output.push_str(&self.print_terminal(&format!("IDENTIFIER({})", name)));
                    }
                    Expression::ArrayAccess { array, index } => {
                        // Print array access directly without wrapping in <expression>
                        if let Expression::Identifier(name) = &**array {
                            output.push_str(&self.print_terminal(&format!("IDENTIFIER({})", name)));
                        } else {
                            output.push_str(&self.print_expression(array));
                        }
                        output.push_str(&self.print_terminal("LBRACKET([)"));
                        output.push_str(&self.print_expression(index));
                        output.push_str(&self.print_terminal("RBRACKET(])"));
                    }
                    _ => {
                        // Fallback for complex l-values
                        output.push_str(&self.print_expression(&assign.variable));
                    }
                }
                
                // Print := operator
                output.push_str(&self.print_terminal("ASSIGN_OPERATOR(:=)"));
                
                // Print expression (right side)
                output.push_str(&self.print_expression(&assign.expression));
                
                self.indent_level -= 1;
                output
            }
            Statement::ExpressionStatement(expr) => {
                let mut output = String::new();
                
                // For procedure calls like writeln, print them nicely
                match expr {
                    Expression::FunctionCall { function_name, arguments } => {
                        output.push_str(&self.print_node("<procedure-call>"));
                        self.indent_level += 1;
                        output.push_str(&self.print_terminal(&format!("IDENTIFIER({})", function_name)));
                        output.push_str(&self.print_terminal("LPARENTHESIS(()"));
                        
                        if !arguments.is_empty() {
                            output.push_str(&self.print_node("<parameter-list>"));
                            self.indent_level += 1;
                            for (i, arg) in arguments.iter().enumerate() {
                                output.push_str(&self.print_expression(arg));
                                if i < arguments.len() - 1 {
                                    output.push_str(&self.print_terminal("COMMA(,)"));
                                }
                            }
                            self.indent_level -= 1;
                        }
                        
                        output.push_str(&self.print_terminal("RPARENTHESIS())"));
                        self.indent_level -= 1;
                    }
                    _ => {
                        output.push_str(&self.print_node("<expression-statement>"));
                        self.indent_level += 1;
                        output.push_str(&self.print_expression(expr));
                        self.indent_level -= 1;
                    }
                }
                
                output
            }
            Statement::Placeholder => self.print_node("<placeholder-statement>"),
            _ => self.print_node("<statement>"),
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
}