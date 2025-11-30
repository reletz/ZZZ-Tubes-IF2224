use crate::semantic_analyzer::ast::ast;
use crate::parser::parse_tree as cst;
use crate::lexer::token_types::{TokenType, Token};
use crate::semantic_analyzer::error::{SemanticError, SemanticErrorKind};

pub struct ASTBuilder;

impl ASTBuilder {
    pub fn build(program: &cst::Program) -> Result<ast::ProgramAST, SemanticError> {
        Ok(ast::ProgramAST {
            name: program.header.name.value.clone(),
            declarations: Self::build_declarations(&program.declarations)?,
            main_body: Self::build_block(&program.body)?,
        })
    }

    fn build_declarations(decls: &cst::DeclarationPart) -> Result<Vec<ast::Decl>, SemanticError> {
        let mut ast_decls = Vec::new();

        // Constants
        for const_decl in &decls.const_declarations {
            for constant in &const_decl.constants {
                ast_decls.push(ast::Decl::Constant {
                    name: constant.name.value.clone(),
                    value: Self::build_expr(&constant.value)?,
                    line: constant.name.line,
                    column: constant.name.column,
                });
            }
        }

        // Types
        for type_decl in &decls.type_declarations {
            for type_def in &type_decl.definitions {
                ast_decls.push(ast::Decl::Type {
                    name: type_def.name.value.clone(),
                    wrapped_type: Self::build_type(&type_def.type_def)?,
                    line: type_def.name.line,
                    column: type_def.name.column,
                });
            }
        }

        // Variables
        for var_decl in &decls.var_declarations {
            for group in &var_decl.groups {
                let type_kind = Self::build_type(&group.var_type)?;
                
                let mut names = Vec::new();
                names.push(group.identifiers.initial_id.value.clone());
                
                for (_, id_token) in &group.identifiers.rest {
                    names.push(id_token.value.clone());
                }

                ast_decls.push(ast::Decl::Variable {
                    name: names,
                    type_kind,
                    line: group.identifiers.initial_id.line,
                    column: group.identifiers.initial_id.column,
                });
            }
        }

        // Subprograms
        for sub in &decls.subprogram_declarations {
            ast_decls.push(Self::build_subprogram(sub)?);
        }

        Ok(ast_decls)
    }

    fn build_type(cst_type: &cst::Type) -> Result<ast::TypeKind, SemanticError> {
        match cst_type {
            cst::Type::Integer(_) => Ok(ast::TypeKind::Integer),
            cst::Type::Real(_) => Ok(ast::TypeKind::Real),
            cst::Type::Boolean(_) => Ok(ast::TypeKind::Boolean),
            cst::Type::Char(_) => Ok(ast::TypeKind::Char),
            cst::Type::String(_) => Ok(ast::TypeKind::String),

            cst::Type::TypeIdentifier(tok) => Ok(ast::TypeKind::Custom(tok.value.clone())),

            cst::Type::Subrange(range) => {
                Ok(ast::TypeKind::Subrange(
                    Box::new(Self::build_expr(&range.start)?),
                    Box::new(Self::build_expr(&range.end)?)
                ))
            },
            
            cst::Type::Array(arr) => {
                Ok(ast::TypeKind::Array {
                    index_range: Box::new(Self::build_type(&arr.index_type)?),
                    element_type: Box::new(Self::build_type(&arr.base_type)?),
                })
            },
        }
    }

    fn build_subprogram(sub: &cst::SubprogramDeclaration) -> Result<ast::Decl, SemanticError> {
        match sub {
            cst::SubprogramDeclaration::Procedure(proc) => {
                Ok(ast::Decl::Procedure {
                    name: proc.name.value.clone(),
                    params: Self::build_params(&proc.parameters)?,
                    local_decls: Self::build_declarations(&proc.declarations)?,
                    body: Self::build_block(&proc.body)?,
                    line: proc.proc_kw.line,
                    column: proc.proc_kw.column,
                })
            },
            cst::SubprogramDeclaration::Function(func) => {
                Ok(ast::Decl::Function {
                    name: func.name.value.clone(),
                    params: Self::build_params(&func.parameters)?,
                    return_type: Self::build_type(&func.return_type)?,
                    local_decls: Self::build_declarations(&func.declarations)?,
                    body: Self::build_block(&func.body)?,
                    line: func.func_kw.line,
                    column: func.func_kw.column,
                })
            }
        }
    }

    fn build_params(params: &cst::FormalParameterList) -> Result<Vec<ast::Param>, SemanticError> {
        let mut ast_params = Vec::new();
        
        if let Some(initial) = &params.initial_param {
            Self::process_param_group(initial, &mut ast_params)?;
        }
        
        for (_, group) in &params.rest {
            Self::process_param_group(group, &mut ast_params)?;
        }

        Ok(ast_params)
    }

    fn process_param_group(group: &cst::FormalParameterGroup, ast_params: &mut Vec<ast::Param>) -> Result<(), SemanticError> {
        // TODO: Logic cek 'var' (by reference)
        // Tunggu QnA dijawab / Parser update
        let is_var = group.var_kw.is_some();

        let mut names = Vec::new();
        names.push(group.identifiers.initial_id.value.clone());
        for (_, id) in &group.identifiers.rest {
            names.push(id.value.clone());
        }

        ast_params.push(ast::Param {
            names,
            type_kind: Self::build_type(&group.var_type)?,
            is_var,
        });
        
        Ok(())
    }

    fn build_block(compound: &cst::CompoundStatement) -> Result<ast::BlockStmt, SemanticError> {
        let mut stmts = Vec::new();
        
        if let Some(initial) = &compound.statement_list.initial_stmt {
            stmts.push(Self::build_stmt(initial)?);
        }
        for (_, stmt) in &compound.statement_list.rest {
            stmts.push(Self::build_stmt(stmt)?);
        }
        
        Ok(ast::BlockStmt { statements: stmts })
    }

    fn build_stmt(stmt: &cst::Statement) -> Result<ast::Stmt, SemanticError> {
        match stmt {
            cst::Statement::Assignment(assign) => {
                Ok(ast::Stmt::Assignment {
                    target: Self::build_expr(&assign.variable)?,
                    value: Self::build_expr(&assign.expression)?,
                    line: assign.assign_op.line,
                    column: assign.assign_op.column,
                })
            },
            cst::Statement::Compound(block) => {
                Ok(ast::Stmt::Compound(Self::build_block(block)?))
            },
            cst::Statement::If(if_stmt) => {
                Ok(ast::Stmt::If {
                    condition: Self::build_expr(&if_stmt.condition)?,
                    then_branch: Box::new(Self::build_stmt(&if_stmt.then_branch)?),
                    else_branch: if let Some(else_clause) = &if_stmt.else_clause {
                        Some(Box::new(Self::build_stmt(&else_clause.statement)?))
                    } else {
                        None
                    },
                    line: if_stmt.if_kw.line,
                    column: if_stmt.if_kw.column,
                })
            },
            cst::Statement::While(while_stmt) => {
                Ok(ast::Stmt::While {
                    condition: Self::build_expr(&while_stmt.condition)?,
                    body: Box::new(Self::build_stmt(&while_stmt.body)?),
                    line: while_stmt.while_kw.line,
                    column: while_stmt.while_kw.column,
                })
            },
            cst::Statement::For(for_stmt) => {
                let direction = if for_stmt.direction_kw.value.to_lowercase() == "ke" {
                    ast::ForDirection::To
                } else {
                    ast::ForDirection::Downto
                };

                Ok(ast::Stmt::For {
                    iterator: for_stmt.counter_variable.value.clone(),
                    start: Self::build_expr(&for_stmt.start_value)?,
                    end: Self::build_expr(&for_stmt.end_value)?,
                    direction,
                    body: Box::new(Self::build_stmt(&for_stmt.body)?),
                    line: for_stmt.for_kw.line,
                    column: for_stmt.for_kw.column,
                })
            },
            cst::Statement::ProcedureCall(call_stmt) => {
                let args = if let Some(arg_list) = &call_stmt.call.arguments {
                    Self::build_arg_list(arg_list)?
                } else {
                    Vec::new()
                };

                Ok(ast::Stmt::ProcedureCall {
                    name: call_stmt.call.function_name.value.clone(),
                    args,
                    line: call_stmt.call.function_name.line,
                    column: call_stmt.call.function_name.column,
                })
            },
            cst::Statement::Repeat(repeat_stmt) => {
                let mut stmts = Vec::new();
                if let Some(initial) = &repeat_stmt.statement_list.initial_stmt {
                    stmts.push(Self::build_stmt(initial)?);
                }
                for (_, stmt) in &repeat_stmt.statement_list.rest {
                    stmts.push(Self::build_stmt(stmt)?);
                }

                Ok(ast::Stmt::Repeat {
                    body: stmts,
                    condition: Self::build_expr(&repeat_stmt.condition)?,
                    line: repeat_stmt.repeat_kw.line,
                    column: repeat_stmt.repeat_kw.column,
                })
            },
            cst::Statement::Case(case_stmt) => {
                let mut branches = Vec::new();
                for branch in &case_stmt.branches {
                    let mut labels = Vec::new();
                    labels.push(Self::build_expr(&branch.labels.initial_label)?);
                    for (_, expr) in &branch.labels.rest {
                        labels.push(Self::build_expr(expr)?);
                    }
                    branches.push(ast::CaseBranch {
                        labels,
                        stmt: Self::build_stmt(&branch.statement)?,
                    });
                }

                let else_branch = if let Some(ec) = &case_stmt.else_clause {
                    let mut stmts = Vec::new();
                    if let Some(initial) = &ec.statement_list.initial_stmt {
                        stmts.push(Self::build_stmt(initial)?);
                    }
                    for (_, stmt) in &ec.statement_list.rest {
                        stmts.push(Self::build_stmt(stmt)?);
                    }
                    Some(stmts)
                } else {
                    None
                };

                Ok(ast::Stmt::Case {
                    operand: Self::build_expr(&case_stmt.expression)?,
                    branches,
                    else_branch,
                    line: case_stmt.case_kw.line,
                    column: case_stmt.case_kw.column,
                })
            },
        }
    }

    fn build_expr(expr: &cst::Expression) -> Result<ast::Expr, SemanticError> {
        let mut left = Self::build_simple_expr(&expr.initial_simple_expr)?;
        
        for (op, right_simple) in &expr.rest {
            let right = Self::build_simple_expr(right_simple)?;
            
            // Simpan posisi sebelum `left` dipindahkan (moved)
            let line = left.line;
            let col = left.column;

            let kind = ast::ExprKind::Binary {
                left: Box::new(left), 
                op: Self::map_bin_op(op)?,
                right: Box::new(right),
            };
            left = ast::Expr::new(kind, line, col);
        }
        Ok(left)
    }

    fn build_simple_expr(simple: &cst::SimpleExpression) -> Result<ast::Expr, SemanticError> {
        let mut left = Self::build_term(&simple.initial_term)?;
        
        for (op, right_term) in &simple.rest {
            let right = Self::build_term(right_term)?;

            let line = left.line;
            let col = left.column;

            let kind = ast::ExprKind::Binary {
                left: Box::new(left),
                op: Self::map_bin_op(op)?,
                right: Box::new(right),
            };
            left = ast::Expr::new(kind, line, col);
        }
        Ok(left)
    }

    fn build_term(term: &cst::Term) -> Result<ast::Expr, SemanticError> {
        let mut left = Self::build_factor(&term.initial_factor)?;
        
        for (op, right_factor) in &term.rest {
            let right = Self::build_factor(right_factor)?;

            let line = left.line;
            let col = left.column;

            let kind = ast::ExprKind::Binary {
                left: Box::new(left),
                op: Self::map_bin_op(op)?,
                right: Box::new(right),
            };
            left = ast::Expr::new(kind, line, col);
        }
        Ok(left)
    }

    fn build_factor(factor: &cst::Factor) -> Result<ast::Expr, SemanticError> {
        let (kind, line, col) = match factor {
            cst::Factor::Literal(lit) => {
                let k = match lit.token.token_type {
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
                    _ => return Err(SemanticError::new(
                        SemanticErrorKind::GenericError(format!("Unknown literal type: {}", lit.token.value)),
                        lit.token.line,
                        lit.token.column
                    )),
                };
                (k, lit.token.line, lit.token.column)
            },
            cst::Factor::Identifier(tok) => {
                (ast::ExprKind::Variable(tok.value.clone()), tok.line, tok.column)
            },
            cst::Factor::Parenthesized(paren) => {
                return Self::build_expr(&paren.expr);
            },
            cst::Factor::Not(not_factor) => {
                let inner = Self::build_factor(&not_factor.factor)?;
                (
                    ast::ExprKind::Unary {
                        op: ast::UnOp::Not,
                        operand: Box::new(inner)
                    },
                    not_factor.not_token.line,
                    not_factor.not_token.column
                )
            },
            cst::Factor::ArithmeticUnary(unary) => {
                let op = if unary.op.value == "+" { ast::UnOp::Plus } else { ast::UnOp::Neg };
                let inner = Self::build_factor(&unary.factor)?;
                (
                    ast::ExprKind::Unary {
                        op,
                        operand: Box::new(inner)
                    },
                    unary.op.line,
                    unary.op.column
                )
            },
            cst::Factor::FunctionCall(call_node) => {
                 let args = if let Some(arg_list) = &call_node.arguments {
                    Self::build_arg_list(arg_list)?
                } else {
                    Vec::new()
                };
                (
                    ast::ExprKind::FunctionCall {
                        name: call_node.function_name.value.clone(),
                        args
                    },
                    call_node.function_name.line,
                    call_node.function_name.column
                )
            },
            cst::Factor::ArrayAccess(access) => {
                let array_expr = Self::build_expr(&access.array)?;
                let l = array_expr.line;
                let c = array_expr.column;
                (
                    ast::ExprKind::ArrayAccess {
                        array: Box::new(array_expr),
                        index: Box::new(Self::build_expr(&access.index)?),
                    },
                    l, c
                )
            }
        };
        
        Ok(ast::Expr::new(kind, line, col))
    }

    fn build_arg_list(list: &cst::ActualParameterList) -> Result<Vec<ast::Expr>, SemanticError> {
        let mut args = Vec::new();
        args.push(Self::build_expr(&list.initial_arg)?);
        for (_, expr) in &list.rest {
             args.push(Self::build_expr(expr)?);
        }
        Ok(args)
    }

    fn map_bin_op(token: &Token) -> Result<ast::BinOp, SemanticError> {
        match token.value.to_lowercase().as_str() {
            "+" => Ok(ast::BinOp::Add),
            "-" => Ok(ast::BinOp::Sub),
            "*" => Ok(ast::BinOp::Mul),
            "/" => Ok(ast::BinOp::DivReal),
            "div" | "bagi" => Ok(ast::BinOp::DivInt),
            "mod" => Ok(ast::BinOp::Mod),
            "and" | "dan" => Ok(ast::BinOp::And),
            "or" | "atau" => Ok(ast::BinOp::Or),
            "=" => Ok(ast::BinOp::Eq),
            "<>" => Ok(ast::BinOp::Neq),
            "<" => Ok(ast::BinOp::Lt),
            "<=" => Ok(ast::BinOp::Lte),
            ">" => Ok(ast::BinOp::Gt),
            ">=" => Ok(ast::BinOp::Gte),
            _ => Err(SemanticError::new(
                SemanticErrorKind::GenericError(format!("Unknown binary operator: {}", token.value)),
                token.line,
                token.column,
            )),
        }
    }
}