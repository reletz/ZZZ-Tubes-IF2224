use crate::semantic_analyzer::ast; 
use crate::parser::parse_tree as cst;
use crate::lexer::token_types::TokenType;

pub struct ASTBuilder;

impl ASTBuilder {
    pub fn build(program: &cst::Program) -> ast::ProgramAST {
        ast::ProgramAST {
            name: program.header.name.value.clone(),
            declarations: Self::build_declarations(&program.declarations),
            main_body: Self::build_block(&program.body),
        }
    }

    fn build_declarations(decls: &cst::DeclarationPart) -> Vec<ast::Decl> {
        let mut ast_decls = Vec::new();

        // Constants
        for const_decl in &decls.const_declarations {
            for constant in &const_decl.constants {
                ast_decls.push(ast::Decl::Constant {
                    name: constant.name.value.clone(),
                    value: Self::build_expr(&constant.value),
                });
            }
        }

        // Types
        for type_decl in &decls.type_declarations {
            for type_def in &type_decl.definitions {
                ast_decls.push(ast::Decl::Type {
                    name: type_def.name.value.clone(),
                    wrapped_type: Self::build_type(&type_def.type_def),
                });
            }
        }

        // Variables
        for var_decl in &decls.var_declarations {
            for group in &var_decl.groups {
                let type_kind = Self::build_type(&group.var_type);
                
                let mut names = Vec::new();
                names.push(group.identifiers.initial_id.value.clone());
                
                for (_, id_token) in &group.identifiers.rest {
                    names.push(id_token.value.clone());
                }

                ast_decls.push(ast::Decl::Variable {
                    name: names,
                    type_kind,
                });
            }
        }

        // Subprograms
        for sub in &decls.subprogram_declarations {
            ast_decls.push(Self::build_subprogram(sub));
        }

        ast_decls
    }

    fn build_type(cst_type: &cst::Type) -> ast::TypeKind {
        match cst_type {
            cst::Type::Integer(_) => ast::TypeKind::Integer,
            cst::Type::Real(_) => ast::TypeKind::Real,
            cst::Type::Boolean(_) => ast::TypeKind::Boolean,
            cst::Type::Char(_) => ast::TypeKind::Char,
            cst::Type::String(_) => ast::TypeKind::String,

            cst::Type::TypeIdentifier(tok) => ast::TypeKind::Custom(tok.value.clone()),

            cst::Type::Subrange(range) => {
                ast::TypeKind::Subrange(
                    Box::new(Self::build_expr(&range.start)),
                    Box::new(Self::build_expr(&range.end))
                )
            },
            
            cst::Type::Array(arr) => {
                ast::TypeKind::Array {
                    index_range: Box::new(Self::build_type(&arr.index_type)),
                    element_type: Box::new(Self::build_type(&arr.base_type)),
                }
            },
        }
    }

    fn build_subprogram(sub: &cst::SubprogramDeclaration) -> ast::Decl {
        match sub {
            cst::SubprogramDeclaration::Procedure(proc) => {
                ast::Decl::Procedure {
                    name: proc.name.value.clone(),
                    params: Self::build_params(&proc.parameters),
                    local_decls: Self::build_declarations(&proc.declarations),
                    body: Self::build_block(&proc.body),
                }
            },
            cst::SubprogramDeclaration::Function(func) => {
                ast::Decl::Function {
                    name: func.name.value.clone(),
                    params: Self::build_params(&func.parameters),
                    return_type: Self::build_type(&func.return_type),
                    local_decls: Self::build_declarations(&func.declarations),
                    body: Self::build_block(&func.body),
                }
            }
        }
    }

    fn build_params(params: &cst::FormalParameterList) -> Vec<ast::Param> {
        let mut ast_params = Vec::new();
        
        let mut process_group = |group: &cst::FormalParameterGroup| {
            // TODO: Logic cek 'var' (by reference)
            // Tunggu QnA dijawab
            let is_var = false; 

            let mut names = Vec::new();
            names.push(group.identifiers.initial_id.value.clone());
            for (_, id) in &group.identifiers.rest {
                names.push(id.value.clone());
            }

            ast_params.push(ast::Param {
                names,
                type_kind: Self::build_type(&group.var_type),
                is_var,
            });
        };

        if let Some(initial) = &params.initial_param {
            process_group(initial);
            for (_, group) in &params.rest {
                process_group(group);
            }
        }

        ast_params
    }

    fn build_block(compound: &cst::CompoundStatement) -> ast::BlockStmt {
        let mut stmts = Vec::new();
        
        if let Some(initial) = &compound.statement_list.initial_stmt {
            stmts.push(Self::build_stmt(initial));
        }
        for (_, stmt) in &compound.statement_list.rest {
            stmts.push(Self::build_stmt(stmt));
        }
        
        ast::BlockStmt { statements: stmts }
    }

    fn build_stmt(stmt: &cst::Statement) -> ast::Stmt {
        match stmt {
            cst::Statement::Assignment(assign) => {
                ast::Stmt::Assignment {
                    target: Self::build_expr(&assign.variable),
                    value: Self::build_expr(&assign.expression),
                }
            },
            cst::Statement::Compound(block) => {
                ast::Stmt::Compound(Self::build_block(block))
            },
            cst::Statement::If(if_stmt) => {
                ast::Stmt::If {
                    condition: Self::build_expr(&if_stmt.condition),
                    then_branch: Box::new(Self::build_stmt(&if_stmt.then_branch)),
                    else_branch: if_stmt.else_clause.as_ref().map(|e| {
                        Box::new(Self::build_stmt(&e.statement))
                    }),
                }
            },
            cst::Statement::While(while_stmt) => {
                ast::Stmt::While {
                    condition: Self::build_expr(&while_stmt.condition),
                    body: Box::new(Self::build_stmt(&while_stmt.body)),
                }
            },
            cst::Statement::For(for_stmt) => {
                let direction = if for_stmt.direction_kw.value.to_lowercase() == "ke" {
                    ast::ForDirection::To
                } else {
                    ast::ForDirection::Downto
                };

                ast::Stmt::For {
                    iterator: for_stmt.counter_variable.value.clone(),
                    start: Self::build_expr(&for_stmt.start_value),
                    end: Self::build_expr(&for_stmt.end_value),
                    direction,
                    body: Box::new(Self::build_stmt(&for_stmt.body)),
                }
            },
            cst::Statement::ProcedureCall(call_stmt) => {
                let args = if let Some(arg_list) = &call_stmt.call.arguments {
                    Self::build_arg_list(arg_list)
                } else {
                    Vec::new()
                };

                ast::Stmt::ProcedureCall {
                    name: call_stmt.call.function_name.value.clone(),
                    args,
                }
            },
            cst::Statement::Repeat(repeat_stmt) => {
                let mut stmts = Vec::new();
                if let Some(initial) = &repeat_stmt.statement_list.initial_stmt {
                    stmts.push(Self::build_stmt(initial));
                }
                for (_, stmt) in &repeat_stmt.statement_list.rest {
                    stmts.push(Self::build_stmt(stmt));
                }

                ast::Stmt::Repeat {
                    body: stmts,
                    condition: Self::build_expr(&repeat_stmt.condition),
                }
            },
            cst::Statement::Case(case_stmt) => {
                let mut branches = Vec::new();
                for branch in &case_stmt.branches {
                    let mut labels = Vec::new();
                    labels.push(Self::build_expr(&branch.labels.initial_label));
                    for (_, expr) in &branch.labels.rest {
                        labels.push(Self::build_expr(expr));
                    }
                    branches.push(ast::CaseBranch {
                        labels,
                        stmt: Self::build_stmt(&branch.statement),
                    });
                }

                let else_branch = case_stmt.else_clause.as_ref().map(|ec| {
                    let mut stmts = Vec::new();
                    if let Some(initial) = &ec.statement_list.initial_stmt {
                        stmts.push(Self::build_stmt(initial));
                    }
                    for (_, stmt) in &ec.statement_list.rest {
                        stmts.push(Self::build_stmt(stmt));
                    }
                    stmts
                });

                ast::Stmt::Case {
                    operand: Self::build_expr(&case_stmt.expression),
                    branches,
                    else_branch,
                }
            },
        }
    }

    fn build_expr(expr: &cst::Expression) -> ast::Expr {
        let mut left = Self::build_simple_expr(&expr.initial_simple_expr);
        
        for (op, right_simple) in &expr.rest {
            let right = Self::build_simple_expr(right_simple);
            let kind = ast::ExprKind::Binary {
                left: Box::new(left),
                op: Self::map_bin_op(&op.value),
                right: Box::new(right),
            };
            left = ast::Expr::new(kind);
        }
        left
    }

    fn build_simple_expr(simple: &cst::SimpleExpression) -> ast::Expr {
        let mut left = Self::build_term(&simple.initial_term);
        
        for (op, right_term) in &simple.rest {
            let right = Self::build_term(right_term);
            let kind = ast::ExprKind::Binary {
                left: Box::new(left),
                op: Self::map_bin_op(&op.value),
                right: Box::new(right),
            };
            left = ast::Expr::new(kind);
        }
        left
    }

    fn build_term(term: &cst::Term) -> ast::Expr {
        let mut left = Self::build_factor(&term.initial_factor);
        
        for (op, right_factor) in &term.rest {
            let right = Self::build_factor(right_factor);
            let kind = ast::ExprKind::Binary {
                left: Box::new(left),
                op: Self::map_bin_op(&op.value),
                right: Box::new(right),
            };
            left = ast::Expr::new(kind);
        }
        left
    }

    fn build_factor(factor: &cst::Factor) -> ast::Expr {
        let kind = match factor {
            cst::Factor::Literal(lit) => {
                match lit.token.token_type {
                    TokenType::IntegerLiteral => {
                        ast::ExprKind::LiteralInt(lit.token.value.parse().unwrap_or(0))
                    },
                    TokenType::RealLiteral => {
                        ast::ExprKind::LiteralReal(lit.token.value.parse().unwrap_or(0.0))
                    },
                    TokenType::StringLiteral => {
                        let s = lit.token.value.trim_matches('\'').to_string();
                        ast::ExprKind::LiteralString(s)
                    },
                    TokenType::CharLiteral => {
                        let c = lit.token.value.trim_matches('\'').chars().next().unwrap_or('\0');
                        ast::ExprKind::LiteralChar(c)
                    },
                    TokenType::Keyword if lit.token.value == "benar" => ast::ExprKind::LiteralBool(true),
                    TokenType::Keyword if lit.token.value == "salah" => ast::ExprKind::LiteralBool(false),
                    _ => panic!("Unknown literal type"),
                }
            },
            cst::Factor::Identifier(tok) => {
                ast::ExprKind::Variable(tok.value.clone())
            },
            cst::Factor::Parenthesized(paren) => {
                return Self::build_expr(&paren.expr);
            },
            cst::Factor::Not(not_factor) => {
                ast::ExprKind::Unary {
                    op: ast::UnOp::Not,
                    operand: Box::new(Self::build_factor(&not_factor.factor))
                }
            },
            cst::Factor::ArithmeticUnary(unary) => {
                let op = if unary.op.value == "+" { ast::UnOp::Plus } else { ast::UnOp::Neg };
                ast::ExprKind::Unary {
                    op,
                    operand: Box::new(Self::build_factor(&unary.factor))
                }
            },
            cst::Factor::FunctionCall(call_node) => {
                 let args = if let Some(arg_list) = &call_node.arguments {
                    Self::build_arg_list(arg_list)
                } else {
                    Vec::new()
                };
                ast::ExprKind::FunctionCall {
                    name: call_node.function_name.value.clone(),
                    args
                }
            },
            cst::Factor::ArrayAccess(access) => {
                ast::ExprKind::ArrayAccess {
                    array: Box::new(Self::build_expr(&access.array)),
                    index: Box::new(Self::build_expr(&access.index)),
                }
            }
        };
        
        ast::Expr::new(kind)
    }

    fn build_arg_list(list: &cst::ActualParameterList) -> Vec<ast::Expr> {
        let mut args = Vec::new();
        args.push(Self::build_expr(&list.initial_arg));
        for (_, expr) in &list.rest {
             args.push(Self::build_expr(expr));
        }
        args
    }

    fn map_bin_op(op: &str) -> ast::BinOp {
        match op.to_lowercase().as_str() {
            "+" => ast::BinOp::Add,
            "-" => ast::BinOp::Sub,
            "*" => ast::BinOp::Mul,
            "/" => ast::BinOp::DivReal,
            "div" | "bagi" => ast::BinOp::DivInt,
            "mod" => ast::BinOp::Mod,
            "and" | "dan" => ast::BinOp::And,
            "or" | "atau" => ast::BinOp::Or,
            "=" => ast::BinOp::Eq,
            "<>" => ast::BinOp::Neq,
            "<" => ast::BinOp::Lt,
            "<=" => ast::BinOp::Lte,
            ">" => ast::BinOp::Gt,
            ">=" => ast::BinOp::Gte,
            _ => panic!("Unknown binary operator: {}", op),
        }
    }
}