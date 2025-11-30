use crate::semantic_analyzer::ast::ast::{
    ProgramAST, Decl, Stmt, Expr, ExprKind, BlockStmt, SemanticData
};

pub struct ASTPrinter {
    indent_level: usize,
    prefix_stack: Vec<bool>,
}

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

impl ASTPrinter {
    pub fn new() -> Self {
        Self {
            indent_level: 0,
            prefix_stack: Vec::new(),
        }
    }

    pub fn print(program: &ProgramAST) {
        let mut printer = Self::new();
        printer.print_program_internal(program);
    }

    // ===================================================================
    // CORE PRINTING LOGIC (Stack Based)
    // ===================================================================

    fn print_node(&mut self, label: &str, is_last: bool) {
        println!("{}{}", self.get_prefix(is_last), label);
    }

    fn get_prefix(&self, is_last: bool) -> String {
        if self.indent_level == 0 {
            return String::new();
        }

        let mut prefix = String::new();

        // Cetak indentasi untuk level-level di atasnya (grandparents)
        if let Some(grandparents) = self.prefix_stack.get(0..self.prefix_stack.len().saturating_sub(1)) {
            for &was_last in grandparents {
                if was_last {
                    prefix.push_str("    ");
                } else {
                    prefix.push_str("│   ");
                }
            }
        }

        // Cetak indentasi untuk parent langsung
        if let Some(&parent_was_last) = self.prefix_stack.last() {
            if parent_was_last {
                prefix.push_str("    ");
            } else {
                prefix.push_str("│   ");
            }
        }

        // Cetak konektor node saat ini
        if is_last {
            prefix.push_str("└── ");
        } else {
            prefix.push_str("├── ");
        }

        prefix
    }

    // ===================================================================
    // AST TRAVERSAL
    // ===================================================================

    fn print_program_internal(&mut self, program: &ProgramAST) {
        // Root node selalu dianggap last (atau tanpa prefix) di level 0
        let label = format!("ProgramNode(name: '{}')", program.name);
        println!("{}", label); 

        // Kita anggap Program punya 2 anak utama: Declarations dan Block
        // Declarations bukan anak terakhir, Block adalah anak terakhir.
        
        with_indent!(self, true, {
            self.print_declarations_section(&program.declarations, false);
            self.print_main_block(&program.main_body, true);
        });
    }

    fn print_declarations_section(&mut self, decls: &[Decl], is_last: bool) {
        self.print_node("Declarations", is_last);
        
        with_indent!(self, is_last, {
            for (i, decl) in decls.iter().enumerate() {
                let is_decl_last = i == decls.len() - 1;
                self.print_decl(decl, is_decl_last);
            }
        });
    }

    fn print_decl(&mut self, decl: &Decl, is_last_decl: bool) {
        match decl {
            Decl::Constant { name, value, line: _, column: _ } => {
                let val_str = Self::expr_to_string(value);
                let note = Self::fmt_annotation(&value.annotation);
                let label = format!("ConstDecl('{}' = {}){}", name, val_str, note);
                self.print_node(&label, is_last_decl);
            },
            Decl::Type { name, wrapped_type, line: _, column: _ } => {
                let label = format!("TypeDecl('{}' = {})", name, wrapped_type);
                self.print_node(&label, is_last_decl);
            },
            Decl::Variable { name, type_kind, line: _, column: _ } => {        
                for (j, var_name) in name.iter().enumerate() {
                    let is_name_last = j == name.len() - 1;
                    let is_node_last = is_last_decl && is_name_last;
                    
                    let label = format!("VarDecl('{}') -> type:{}", var_name, type_kind);
                    self.print_node(&label, is_node_last);
                }
            },
            Decl::Procedure { name, params, local_decls, body, line: _, column: _ } => {
                let label = format!("Procedure('{}')", name);
                self.print_node(&label, is_last_decl);
                
                with_indent!(self, is_last_decl, {
                    self.print_params(params, false); // Params bukan last
                    self.print_declarations_section(local_decls, false); // Local decls bukan last
                    self.print_node("Body", true); // Wrapper body
                    with_indent!(self, true, {
                        self.print_block_statements(body);
                    });
                });
            },
            Decl::Function { name, params, return_type, local_decls, body, line: _, column: _ } => {
                let label = format!("Function('{}') -> type:{}", name, return_type);
                self.print_node(&label, is_last_decl);
                
                with_indent!(self, is_last_decl, {
                    self.print_params(params, false);
                    self.print_declarations_section(local_decls, false);
                    self.print_node("Body", true);
                    with_indent!(self, true, {
                        self.print_block_statements(body);
                    });
                });
            }
        }
    }

    fn print_params(&mut self, params: &[crate::semantic_analyzer::ast::ast::Param], is_last: bool) {
        if params.is_empty() { return; }
        self.print_node("Params", is_last);
        
        with_indent!(self, is_last, {
            for (i, p) in params.iter().enumerate() {
                let is_p_last = i == params.len() - 1;
                let prefix = if p.is_var { "var " } else { "" };
                let names = p.names.join(", ");
                let label = format!("{}{}: {}", prefix, names, p.type_kind);
                self.print_node(&label, is_p_last);
            }
        });
    }

    fn print_main_block(&mut self, block: &BlockStmt, is_last: bool) {
        self.print_node("Block", is_last);
        with_indent!(self, is_last, {
            self.print_block_statements(block);
        });
    }

    fn print_block_statements(&mut self, block: &BlockStmt) {
        for (i, stmt) in block.statements.iter().enumerate() {
            let is_stmt_last = i == block.statements.len() - 1;
            self.print_stmt(stmt, is_stmt_last);
        }
    }

    fn print_stmt(&mut self, stmt: &Stmt, is_last: bool) {
        match stmt {
            Stmt::Assignment { target, value, line: _, column: _ } => {
                let summary = format!("{} := {}", Self::expr_to_string(target), Self::expr_to_string(value));
                let note = Self::fmt_annotation(&target.annotation); // Show target type info on the Assign node usually
                let label = format!("Assign('{}'){}", summary, note);
                self.print_node(&label, is_last);

                with_indent!(self, is_last, {
                    // Target is not last, Value is last
                    self.print_expr_tree_node(target, "target", false);
                    self.print_expr_tree_node(value, "value", true);
                });
            },
            Stmt::ProcedureCall { name, args, line: _, column: _ } => {
                let label = format!("Call('{}')", name);
                self.print_node(&label, is_last);
                
                with_indent!(self, is_last, {
                    for (i, arg) in args.iter().enumerate() {
                        let is_arg_last = i == args.len() - 1;
                        let prefix = format!("arg[{}]", i);
                        self.print_expr_tree_node(arg, &prefix, is_arg_last);
                    }
                });
            },
            Stmt::If { condition, then_branch, else_branch, line: _, column: _ } => {
                self.print_node("If", is_last);
                
                with_indent!(self, is_last, {
                    self.print_expr_tree_node(condition, "cond", false);
                    
                    let has_else = else_branch.is_some();
                    
                    self.print_node("then", !has_else); // "then" is last if no "else"
                    with_indent!(self, !has_else, {
                        self.print_stmt(then_branch, true); // Inside "then" block, the stmt is the only child
                    });

                    if let Some(else_b) = else_branch {
                        self.print_node("else", true);
                        with_indent!(self, true, {
                            self.print_stmt(else_b, true);
                        });
                    }
                });
            },
            Stmt::While { condition, body, line: _, column: _ } => {
                self.print_node("While", is_last);
                with_indent!(self, is_last, {
                    self.print_expr_tree_node(condition, "cond", false);
                    self.print_node("do", true);
                    with_indent!(self, true, {
                        self.print_stmt(body, true);
                    });
                });
            },
            Stmt::For { iterator, start, end, direction, body, line: _, column: _ } => {
                let dir_str = format!("{:?}", direction);
                let label = format!("For('{}' := ... {} ...)", iterator, dir_str);
                self.print_node(&label, is_last);
                
                with_indent!(self, is_last, {
                    self.print_expr_tree_node(start, "start", false);
                    self.print_expr_tree_node(end, "end", false);
                    self.print_node("do", true);
                    with_indent!(self, true, {
                        self.print_stmt(body, true);
                    });
                });
            },
            Stmt::Repeat { body, condition, line: _, column: _ } => {
                self.print_node("Repeat", is_last);
                with_indent!(self, is_last, {
                    self.print_node("body", false);
                    with_indent!(self, false, {
                        for (i, s) in body.iter().enumerate() {
                            let is_s_last = i == body.len() - 1;
                            self.print_stmt(s, is_s_last);
                        }
                    });
                    self.print_expr_tree_node(condition, "until", true);
                });
            },
            Stmt::Case { operand, branches, else_branch, line: _, column: _ } => {
                self.print_node("Case", is_last);
                with_indent!(self, is_last, {
                    self.print_expr_tree_node(operand, "operand", branches.is_empty() && else_branch.is_none());
                    
                    let total_branches = branches.len();
                    let has_else = else_branch.is_some();

                    for (i, branch) in branches.iter().enumerate() {
                        let is_branch_last = (i == total_branches - 1) && !has_else;
                        let label = format!("Branch {}", i + 1);
                        self.print_node(&label, is_branch_last);
                        
                        with_indent!(self, is_branch_last, {
                            // Print labels
                            for (j, lbl) in branch.labels.iter().enumerate() {
                                self.print_expr_tree_node(lbl, "label", false); // Labels are never last children because Stmt follows
                            }
                            // Print stmt
                            self.print_stmt(&branch.stmt, true);
                        });
                    }

                    if let Some(else_stmts) = else_branch {
                        self.print_node("Else", true);
                        with_indent!(self, true, {
                            for (i, s) in else_stmts.iter().enumerate() {
                                let is_s_last = i == else_stmts.len() - 1;
                                self.print_stmt(s, is_s_last);
                            }
                        });
                    }
                });
            },
            Stmt::Compound(block) => {
                self.print_node("Block", is_last);
                with_indent!(self, is_last, {
                    self.print_block_statements(block);
                });
            }
        }
    }

    fn print_expr_tree_node(&mut self, expr: &Expr, prefix_label: &str, is_last: bool) {
        // Format annotation string: " -> tab_index:30, type:integer"
        let note = Self::fmt_annotation(&expr.annotation);
        
        let node_label = match &expr.kind {
            ExprKind::Binary { op, .. } => format!("BinOp '{:?}'", op),
            ExprKind::Unary { op, .. } => format!("UnOp '{:?}'", op),
            ExprKind::LiteralInt(v) => format!("Number({})", v),
            ExprKind::LiteralReal(v) => format!("Number({})", v),
            ExprKind::LiteralString(v) => format!("String('{}')", v),
            ExprKind::LiteralChar(v) => format!("Char('{}')", v),
            ExprKind::LiteralBool(v) => format!("Bool({})", v),
            ExprKind::Variable(name) => format!("Variable('{}')", name),
            ExprKind::ArrayAccess { .. } => "ArrayAccess".to_string(),
            ExprKind::FunctionCall { name, .. } => format!("FuncCall('{}')", name),
        };

        let full_label = if prefix_label.is_empty() {
            format!("{}{}", node_label, note)
        } else {
            format!("{} {}{}", prefix_label, node_label, note)
        };

        self.print_node(&full_label, is_last);

        // Recursion for children
        match &expr.kind {
            ExprKind::Binary { left, right, .. } => {
                with_indent!(self, is_last, {
                    self.print_expr_tree_node(left, "", false);
                    self.print_expr_tree_node(right, "", true);
                });
            },
            ExprKind::Unary { operand, .. } => {
                with_indent!(self, is_last, {
                    self.print_expr_tree_node(operand, "", true);
                });
            },
            ExprKind::ArrayAccess { array, index } => {
                with_indent!(self, is_last, {
                    self.print_expr_tree_node(array, "array", false);
                    self.print_expr_tree_node(index, "index", true);
                });
            },
            ExprKind::FunctionCall { args, .. } => {
                with_indent!(self, is_last, {
                    for (i, arg) in args.iter().enumerate() {
                        let is_arg_last = i == args.len() - 1;
                        self.print_expr_tree_node(arg, "", is_arg_last);
                    }
                });
            },
            _ => {} // Literals and Variables are leaves
        }
    }

    // ===================================================================
    // HELPERS
    // ===================================================================

    fn fmt_annotation(data: &SemanticData) -> String {
        let mut parts = Vec::new();
        
        if let Some(idx) = data.tab_index {
            parts.push(format!("tab_index:{}", idx));
        }
        
        if let Some(t) = &data.type_kind {
            parts.push(format!("type:{}", t));
        }
        
        if data.is_const {
            parts.push("const".to_string());
        }

        if parts.is_empty() {
            String::new()
        } else {
            format!(" -> {}", parts.join(", "))
        }
    }

    fn expr_to_string(expr: &Expr) -> String {
        match &expr.kind {
            ExprKind::Variable(n) => format!("'{}'", n),
            ExprKind::LiteralInt(v) => format!("{}", v),
            ExprKind::LiteralReal(v) => format!("{}", v),
            ExprKind::LiteralString(v) => format!("'{}'", v),
            ExprKind::LiteralBool(v) => format!("{}", v),
            ExprKind::Binary { left, op, right } => {
                format!("{} {:?} {}", Self::expr_to_string(left), op, Self::expr_to_string(right))
            },
            ExprKind::FunctionCall { name, .. } => format!("{}()", name),
            ExprKind::ArrayAccess { array, index } => {
                format!("{}[{}]", Self::expr_to_string(array), Self::expr_to_string(index))
            },
            _ => "...".to_string()
        }
    }
}