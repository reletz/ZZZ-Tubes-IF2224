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
        let type_str = match var_type {
            Type::Integer => "KEYWORD(integer)",
            Type::Real => "KEYWORD(real)",
            Type::Boolean => "KEYWORD(boolean)",
            Type::String => "KEYWORD(string)",
            Type::Char => "KEYWORD(char)",
            Type::Array { .. } => "ARRAY_TYPE",
        };
        self.print_terminal(type_str)
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
            Statement::ExpressionStatement(expr) => {
                let mut output = String::new();
                output.push_str(&self.print_node("<expression-statement>"));
                self.indent_level += 1;
                output.push_str(&self.print_expression(expr));
                self.indent_level -= 1;
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
            Expression::BinaryOp { left, operator, right } if operator == "+" || operator == "-" => {
                // simple-expression level (additive)
                output.push_str(&self.print_node("<simple-expression>"));
                self.indent_level += 1;
                output.push_str(&self.print_term_or_factor(left));
                output.push_str(&self.print_terminal(&format!("ARITHMETIC_OPERATOR({})", operator)));
                output.push_str(&self.print_term_or_factor(right));
                self.indent_level -= 1;
            }
            _ => {
                // Just a simple-expression with one term
                output.push_str(&self.print_node("<simple-expression>"));
                self.indent_level += 1;
                output.push_str(&self.print_term_or_factor(expr));
                self.indent_level -= 1;
            }
        }
        
        self.indent_level -= 1;
        output
    }

    fn print_term_or_factor(&mut self, expr: &Expression) -> String {
        let mut output = String::new();
        
        match expr {
            Expression::BinaryOp { left, operator, right } 
                if operator == "*" || operator == "/" || operator == "div" || operator == "mod" => {
                // term level (multiplicative)
                output.push_str(&self.print_node("<term>"));
                self.indent_level += 1;
                output.push_str(&self.print_factor_inner(left));
                output.push_str(&self.print_terminal(&format!("ARITHMETIC_OPERATOR({})", operator)));
                output.push_str(&self.print_factor_inner(right));
                self.indent_level -= 1;
            }
            _ => {
                // Just a factor
                output.push_str(&self.print_node("<term>"));
                self.indent_level += 1;
                output.push_str(&self.print_factor_inner(expr));
                self.indent_level -= 1;
            }
        }
        
        output
    }

    fn print_factor_inner(&mut self, expr: &Expression) -> String {
        let mut output = String::new();
        output.push_str(&self.print_node("<factor>"));
        self.indent_level += 1;
        
        match expr {
            Expression::Literal(lit) => {
                output.push_str(&self.print_literal(lit));
            }
            Expression::Identifier(name) => {
                output.push_str(&self.print_terminal(&format!("IDENTIFIER({})", name)));
            }
            Expression::BinaryOp { .. } => {
                // Nested expression - go back to expression level
                self.indent_level -= 1;
                return self.print_expression(expr);
            }
            _ => {
                output.push_str(&self.print_node("<complex-expression>"));
            }
        }
        
        self.indent_level -= 1;
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