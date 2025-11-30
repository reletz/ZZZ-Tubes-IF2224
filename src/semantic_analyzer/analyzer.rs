use crate::semantic_analyzer::ast::ast::{
    ProgramAST, Decl, Stmt, Expr, ExprKind, TypeKind, BinOp, UnOp, BlockStmt, Param
};
use crate::semantic_analyzer::tab::{SymbolTable, ObjectKind, TYP_INT, TYP_BOOL, TYP_REAL, TYP_CHAR, TYP_STRING, TYP_NOTYPE};
use crate::semantic_analyzer::error::{SemanticError, SemanticErrorKind};

pub struct SemanticAnalyzer {
    pub symbol_table: SymbolTable,
    errors: Vec<SemanticError>,
    max_errors: usize, // ini buat max error yg bakal di show (klo ngga bisa infinite scroll)
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            symbol_table: SymbolTable::new(),
            errors: Vec::new(),
            max_errors: 50 // 50 aja dlu buat starting (might be better klo pake const cm yauda)
        }
    }

    /// Entry point
    pub fn analyze(&mut self, program: &mut ProgramAST) -> Result<(), Vec<SemanticError>> {
        self.visit_program(program);

        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(std::mem::take(&mut self.errors))
        }
    }

    // ==========================================
    // Anggota 1: Declarations & Scope
    // Fokus: Mengisi tabel simbol, scope management, hitung address
    // ==========================================
    fn visit_program(&mut self, program: &mut ProgramAST) -> () {
        // 1. Init Global Scope
        // Identifier program terlebih dahulu supaya masuk ke level 0
        self.symbol_table.enter(program.name.clone(), ObjectKind::Program, TYP_NOTYPE, 0, true);
        self.symbol_table.enter_scope();
        
        // 2. Masukkan identifier program ke tabel
        // Kita masukkan sebagai Constant dengan tipe Void karena tidak punya nilai runtime.
        
        // 3. Visit semua deklarasi global
        self.visit_decls(&mut program.declarations);
        
        // 4. Visit main body
        if !self.should_bail() {
            self.visit_block(&mut program.main_body);
        }
        // 5. Exit Scope
        self.symbol_table.exit_scope();
    }

    fn visit_decls(&mut self, decls: &mut [Decl]) -> () {
        for decl in decls {
            if self.should_bail() {
                break;
            }
            self.visit_decl(decl);
        }
    }

    fn visit_decl(&mut self, decl: &mut Decl) -> () {
        match decl {
            Decl::Constant { name, value, line, column, .. } => {
                // 1. Evaluasi nilai konstanta untuk dapat tipenya
                match self.visit_expr(value) {
                    Ok(type_kind) => {
                        if !self.is_constant_expr(value) {
                            self.report_error(SemanticError::new(
                                SemanticErrorKind::GenericError(
                                    format!("Constant '{}' must be initialized with a constant expression", name)
                                ),
                                *line, *column
                            ));
                        }
                        let type_idx = self.kind_to_typ_idx(&type_kind);
                        // 2. Masukkan ke tabel
                        self.symbol_table.enter(name.clone(), ObjectKind::Constant, type_idx, 0, true);
                    }
                    Err(e) => {
                        self.report_error(e);
                        self.symbol_table.enter(name.clone(), ObjectKind::Constant, TYP_NOTYPE, 0, true);
                    }
                }
            },
            Decl::Type { name, wrapped_type, line, column, .. } => {
                if self.check_redeclaration(name, *line, *column) {
                        return;
                    }
                let type_idx = self.kind_to_typ_idx(wrapped_type);
                // 1. Masukkan ke tabel sebagai Type Alias
                self.symbol_table.enter(name.clone(), ObjectKind::Type, type_idx, 0, true);
            },
            Decl::Variable { name, type_kind, line, column } => {
                // 1. Validasi tipe data
                if *type_kind == TypeKind::Void {
                    self.report_error(SemanticError::new(
                        SemanticErrorKind::TypeMismatch { 
                            expected: "Valid Type".into(),
                            found: "Void".into() 
                        },
                        *line, *column
                    ));
                    return;
                }

                // 2. Loop vector 'name' dan masukkan ke tabel
                let type_idx = self.kind_to_typ_idx(type_kind);
                for var_name in name {
                    if self.check_redeclaration(var_name, *line, *column) {
                        continue;
                    }
                    self.symbol_table.enter(var_name.clone(), ObjectKind::Variable, type_idx.clone(), 0, true);
                }
            },
            Decl::Procedure { name, params, local_decls, body, line, column } => {
                // 1. Cek redeklarasi
                if self.check_redeclaration(name, *line, *column) {
                    return;
                }

                let btab_idx = self.symbol_table.make_block();

                // 2. Masukkan nama prosedur ke tabel entry
                self.symbol_table.enter(name.clone(), ObjectKind::Procedure, TYP_NOTYPE, btab_idx, true);

                // 2. Naik level (Scope Baru)
                self.symbol_table.enter_scope();

                // 3. Visit parameters
                for param in params {
                    self.visit_param(param);
                }

                // Ambil identifier terakhir yang baru saja dimasukkan
                let current_btab_idx = self.symbol_table.display[self.symbol_table.level];
                let last_param_idx = self.symbol_table.btab[current_btab_idx].last;
                self.symbol_table.btab[current_btab_idx].lpar = last_param_idx;
                
                // 4. Visit local_decls & body
                self.visit_decls(local_decls);
                self.visit_block(body);

                // 6. Exit Scope
                self.symbol_table.exit_scope();
            },
            Decl::Function { name, params, return_type, local_decls, body, line, column} => {
                // 1. cek redeklarasi
                if self.check_redeclaration(name, *line, *column) {
                    return;
                }

                // 2. type checking
                if *return_type == TypeKind::Void {
                    self.report_error(SemanticError::new(
                        SemanticErrorKind::TypeMismatch { 
                            expected: "Valid Return Type".into(),
                            found: "Void".into() 
                        },
                        *line, *column
                    ));
                }

                // 3. Masukkan nama fungsi ke tabel parent
                let ret_idx = self.kind_to_typ_idx(return_type);
                let btab_idx = self.symbol_table.make_block();
                self.symbol_table.enter(name.clone(), ObjectKind::Variable, ret_idx, btab_idx, true);

                // 4. Naik level (Scope Baru)
                self.symbol_table.enter_scope();

                // 5. Visit param
                for param in params {
                    self.visit_param(param);
                }
                
                // Masukkan nama fungsi sebagai variabel lokal untuk return value assignment
                self.symbol_table.enter(name.clone(), ObjectKind::Variable, ret_idx, 0, true);

                let current_btab_idx = self.symbol_table.display[self.symbol_table.level];
                let last_param_idx = self.symbol_table.btab[current_btab_idx].last;
                self.symbol_table.btab[current_btab_idx].lpar = last_param_idx;

                // 6. Visit local_decls & body
                self.visit_decls(local_decls);
                self.visit_block(body);

                // 7. Exit Scope
                self.symbol_table.exit_scope();
            }
        }
    }

    fn visit_param(&mut self, param: &Param) -> () {
        let type_idx = self.kind_to_typ_idx(&param.type_kind);

        let is_normal = !param.is_var;

        for param_name in &param.names {
            // Masukkan parameter sebagai variabel lokal
            self.symbol_table.enter(
                param_name.clone(), 
                ObjectKind::Variable, 
                type_idx.clone(), 
                0, 
                is_normal
            );
        }
    }

    /// Check if identifier is already declared in current scope
    fn check_redeclaration(&mut self, name: &str, line: usize, column: usize) -> bool {
        if let Some(idx) = self.symbol_table.find_in_current_scope(name) {
            self.report_error(SemanticError::new(
                SemanticErrorKind::RedeclaredIdentifier(name.to_string()),
                line, column
            ));
            true
        } else {
            false
        }
    }

    /// Check if expression is a compile-time constant
    fn is_constant_expr(&self, expr: &Expr) -> bool {
        match &expr.kind {
            ExprKind::LiteralInt(_) |
            ExprKind::LiteralReal(_) |
            ExprKind::LiteralString(_) |
            ExprKind::LiteralChar(_) |
            ExprKind::LiteralBool(_) => true,
            
            ExprKind::Unary { operand, .. } => self.is_constant_expr(operand),
            
            ExprKind::Binary { left, right, .. } => {
                self.is_constant_expr(left) && self.is_constant_expr(right)
            },
            
            ExprKind::Variable(name) => {
                // Check if it's a constant
                if let Some(idx) = self.symbol_table.find(name) {
                    self.symbol_table.tab[idx].obj == ObjectKind::Constant
                } else {
                    false
                }
            },
            
            _ => false,
        }
    }

    // ==========================================
    // Anggota 2: Expressions & Type Checking
    // Fokus: Validasi tipe data operand, return tipe hasil
    // ==========================================
    
    /// Mengembalikan Tipe Data (TypeKind) dari ekspresi tersebut
    /// Wajib mengisi expr.annotation.type_kind dan expr.annotation.tab_index (jika variabel)
    fn visit_expr(&mut self, expr: &mut Expr) -> Result<TypeKind, SemanticError> {
        let current_line = expr.line;
        let current_col = expr.column;

        let type_result = match &mut expr.kind {
            ExprKind::Binary { left, op, right } => {
                // Visit both sides, collecting errors
                let left_result = self.visit_expr(left);
                let right_result = self.visit_expr(right);
                
                // If either side failed, propagate first error
                let left_type = left_result?;
                let right_type = right_result?;

                self.check_binary_op(*op, &left_type, &right_type, current_line, current_col)
            },
            ExprKind::Unary { op, operand } => {
                let op_type = self.visit_expr(operand)?;
                self.check_unary_op(*op, &op_type, current_line, current_col)
            },
            ExprKind::LiteralInt(_) => Ok(TypeKind::Integer),
            ExprKind::LiteralReal(_) => Ok(TypeKind::Real),
            ExprKind::LiteralString(_) => Ok(TypeKind::String),
            ExprKind::LiteralChar(_) => Ok(TypeKind::Char),
            ExprKind::LiteralBool(_) => Ok(TypeKind::Boolean),
            
            ExprKind::Variable(name) => {
                if let Some(idx) = self.symbol_table.find(name) {
                    let entry = &self.symbol_table.tab[idx];
                    
                    // Validate usage based on object kind
                    match entry.obj {
                        ObjectKind::Program => {
                            return Err(SemanticError::new(
                                SemanticErrorKind::GenericError(
                                    format!("Cannot use program name '{}' as expression", name)
                                ),
                                current_line, current_col
                            ));
                        },
                        ObjectKind::Procedure => {
                            return Err(SemanticError::new(
                                SemanticErrorKind::GenericError(
                                    format!("Cannot use procedure '{}' as expression (use function instead)", name)
                                ),
                                current_line, current_col
                            ));
                        },
                        ObjectKind::Type => {
                            return Err(SemanticError::new(
                                SemanticErrorKind::GenericError(
                                    format!("Cannot use type '{}' as expression", name)
                                ),
                                current_line, current_col
                            ));
                        },
                        _ => {}
                    }
                    
                    let type_kind = self.typ_idx_to_kind(entry.typ);
                    expr.annotation.tab_index = Some(idx);
                    Ok(type_kind)
                } else {
                    Err(SemanticError::new(
                        SemanticErrorKind::UndefinedIdentifier(name.clone()), 
                        current_line, current_col
                    ))
                }
            },
            
            ExprKind::ArrayAccess { array, index } => {
                let array_type = self.visit_expr(array)?;
                
                match array_type {
                    TypeKind::Array { element_type, index_range } => {
                        let index_type = self.visit_expr(index)?;
                        
                        // Strict: index must be Integer
                        if index_type != TypeKind::Integer {
                            return Err(SemanticError::new(
                                SemanticErrorKind::IndexTypeMismatch(index_type.to_string()),
                                current_line, current_col
                            ));
                        }
                        
                        // Compile-time bounds check if index is constant
                        if let Some(idx_val) = self.eval_const_expr(index) {
                            if let TypeKind::Subrange(low_expr, high_expr) = *index_range {
                                if let (Some(low), Some(high)) = (
                                    self.eval_const_expr(&low_expr),
                                    self.eval_const_expr(&high_expr)
                                ) {
                                    if idx_val < low || idx_val > high {
                                        return Err(SemanticError::new(
                                            SemanticErrorKind::IndexOutOfBounds {
                                                index: idx_val,
                                                low,
                                                high
                                            },
                                            current_line, current_col
                                        ));
                                    }
                                }
                            }
                        }
                        
                        Ok(*element_type)
                    },
                    TypeKind::String => {
                        // String indexing returns Char
                        let index_type = self.visit_expr(index)?;
                        if index_type != TypeKind::Integer {
                            return Err(SemanticError::new(
                                SemanticErrorKind::IndexTypeMismatch(index_type.to_string()),
                                current_line, current_col
                            ));
                        }
                        Ok(TypeKind::Char)
                    },
                    _ => {
                        Err(SemanticError::new(
                            SemanticErrorKind::NotArray(array_type.to_string()), 
                            current_line, current_col
                        ))
                    }
                }
            },
            
            ExprKind::FunctionCall { name, args } => {
                self.visit_function_call(name, args, current_line, current_col)
            }
        }?;

        expr.annotation.type_kind = Some(type_result.clone());
        Ok(type_result)
    }

    /// Strict binary operator type checking
    fn check_binary_op(
        &self, 
        op: BinOp, 
        left: &TypeKind, 
        right: &TypeKind,
        line: usize, 
        col: usize
    ) -> Result<TypeKind, SemanticError> {
        match op {
            // Arithmetic: +, -, *
            BinOp::Add | BinOp::Sub | BinOp::Mul => {
                match (left, right) {
                    (TypeKind::Integer, TypeKind::Integer) => Ok(TypeKind::Integer),
                    (TypeKind::Integer, TypeKind::Real) |
                    (TypeKind::Real, TypeKind::Integer) |
                    (TypeKind::Real, TypeKind::Real) => Ok(TypeKind::Real),
                    // String concatenation (only for Add)
                    (TypeKind::String, TypeKind::String) if op == BinOp::Add => Ok(TypeKind::String),
                    (TypeKind::String, TypeKind::Char) if op == BinOp::Add => Ok(TypeKind::String),
                    (TypeKind::Char, TypeKind::String) if op == BinOp::Add => Ok(TypeKind::String),
                    _ => Err(SemanticError::new(
                        SemanticErrorKind::InvalidOperation {
                            op: format!("{:?}", op),
                            left_type: left.to_string(),
                            right_type: right.to_string()
                        },
                        line, col
                    ))
                }
            },
            
            // Real division: /
            BinOp::DivReal => {
                match (left, right) {
                    (TypeKind::Integer, TypeKind::Integer) |
                    (TypeKind::Integer, TypeKind::Real) |
                    (TypeKind::Real, TypeKind::Integer) |
                    (TypeKind::Real, TypeKind::Real) => Ok(TypeKind::Real),
                    _ => Err(SemanticError::new(
                        SemanticErrorKind::TypeMismatch { 
                            expected: "Numeric".into(), 
                            found: format!("{} and {}", left, right)
                        },
                        line, col
                    ))
                }
            },
            
            // Integer division and modulo: div, mod
            BinOp::DivInt | BinOp::Mod => {
                match (left, right) {
                    (TypeKind::Integer, TypeKind::Integer) => Ok(TypeKind::Integer),
                    _ => Err(SemanticError::new(
                        SemanticErrorKind::TypeMismatch { 
                            expected: "Integer".into(), 
                            found: format!("{} and {}", left, right)
                        },
                        line, col
                    ))
                }
            },
            
            // Relational: =, <>, <, <=, >, >=
            BinOp::Eq | BinOp::Neq => {
                // Allow comparison of same types, or numeric types
                if left == right {
                    Ok(TypeKind::Boolean)
                } else if self.is_numeric(left) && self.is_numeric(right) {
                    Ok(TypeKind::Boolean)
                } else {
                    Err(SemanticError::new(
                        SemanticErrorKind::TypeMismatch { 
                            expected: left.to_string(), 
                            found: right.to_string()
                        },
                        line, col
                    ))
                }
            },
            
            BinOp::Lt | BinOp::Lte | BinOp::Gt | BinOp::Gte => {
                match (left, right) {
                    // Numeric comparisons
                    (TypeKind::Integer, TypeKind::Integer) |
                    (TypeKind::Integer, TypeKind::Real) |
                    (TypeKind::Real, TypeKind::Integer) |
                    (TypeKind::Real, TypeKind::Real) => Ok(TypeKind::Boolean),
                    // Char comparisons
                    (TypeKind::Char, TypeKind::Char) => Ok(TypeKind::Boolean),
                    // String comparisons
                    (TypeKind::String, TypeKind::String) => Ok(TypeKind::Boolean),
                    _ => Err(SemanticError::new(
                        SemanticErrorKind::TypeMismatch { 
                            expected: "Comparable types".into(), 
                            found: format!("{} and {}", left, right)
                        },
                        line, col
                    ))
                }
            },
            
            // Logical: and, or
            BinOp::And | BinOp::Or => {
                match (left, right) {
                    (TypeKind::Boolean, TypeKind::Boolean) => Ok(TypeKind::Boolean),
                    _ => Err(SemanticError::new(
                        SemanticErrorKind::TypeMismatch { 
                            expected: "Boolean".into(), 
                            found: format!("{} and {}", left, right)
                        },
                        line, col
                    ))
                }
            },
        }
    }

    /// Strict unary operator type checking
    fn check_unary_op(
        &self,
        op: UnOp,
        operand_type: &TypeKind,
        line: usize,
        col: usize
    ) -> Result<TypeKind, SemanticError> {
        match op {
            UnOp::Not => {
                if *operand_type == TypeKind::Boolean {
                    Ok(TypeKind::Boolean)
                } else {
                    Err(SemanticError::new(
                        SemanticErrorKind::TypeMismatch { 
                            expected: "Boolean".into(), 
                            found: operand_type.to_string() 
                        },
                        line, col
                    ))
                }
            },
            UnOp::Neg | UnOp::Plus => {
                match operand_type {
                    TypeKind::Integer => Ok(TypeKind::Integer),
                    TypeKind::Real => Ok(TypeKind::Real),
                    _ => Err(SemanticError::new(
                        SemanticErrorKind::TypeMismatch { 
                            expected: "Numeric".into(), 
                            found: operand_type.to_string() 
                        },
                        line, col
                    ))
                }
            },
        }
    }

    fn is_numeric(&self, t: &TypeKind) -> bool {
        matches!(t, TypeKind::Integer | TypeKind::Real)
    }

    // ==========================================
    // Anggota 3: Statements & Flow Control
    // Fokus: Validasi alur (if/while butuh bool), assignment compatibility
    // ==========================================
    fn visit_block(&mut self, block: &mut BlockStmt) {
        for stmt in &mut block.statements {
            if self.should_bail() {
                break;
            }
            self.visit_stmt(stmt);
        }
    }

    fn visit_stmt(&mut self, stmt: &mut Stmt) {
        match stmt {
            Stmt::Assignment { target, value, line, column } => {
                // Evaluasi Tipe
                let target_type = self.visit_expr(target);
                let value_type = self.visit_expr(value);

                match (target_type, value_type) {
                    (Ok(target_type), Ok(value_type)) => {
                        // Check apakah target variabel
                        if let Some(idx) = target.annotation.tab_index {
                            if let Some(entry) = self.symbol_table.tab.get(idx) {
                                if entry.obj == ObjectKind::Constant {
                                    self.report_error(SemanticError::new(
                                        SemanticErrorKind::AssignmentToConstant(entry.name.clone()), 
                                        *line, *column
                                    ));
                                    return;
                                }
                            }
                        }

                        if target_type != value_type {
                            // Allow Int to Real coercion
                            if !(target_type == TypeKind::Real && value_type == TypeKind::Integer) {
                                self.report_error(SemanticError::new(
                                    SemanticErrorKind::TypeMismatch { 
                                        expected: target_type.to_string(), 
                                        found: value_type.to_string() 
                                    },
                                    *line, *column
                                ));
                            }
                        }
                    }
                    (Err(e), Ok(_)) | (Ok(_), Err(e)) => {
                        self.report_error(e);
                    }
                    (Err(e1), Err(e2)) => {
                        self.report_error(e1);
                        self.report_error(e2);
                    }
                }
            },
            Stmt::If { condition, then_branch, else_branch, .. } => {
                // Evaluasi kondisi
                match self.visit_expr(condition) {
                    Ok(condition_type) => {
                        // Validasi tipe kondisi harus boolean
                        if condition_type != TypeKind::Boolean {
                            self.report_error(SemanticError::new(
                                SemanticErrorKind::TypeMismatch { 
                                    expected: "Boolean".into(), 
                                    found: condition_type.to_string() 
                                },
                                condition.line, condition.column
                            ));
                        }
                    }
                    Err(e) => { self.report_error(e); }
                }

                // Visit branch
                self.visit_stmt(then_branch);
                                
                if let Some(else_stmt) = else_branch {
                    self.visit_stmt(else_stmt);
                }
            },
            Stmt::While { condition, body, .. } => {
                // Evaluasi kondisi
                match self.visit_expr(condition) {
                    Ok(condition_type) => {
                        // Validasi tipe kondisi harus boolean
                        if condition_type != TypeKind::Boolean {
                            self.report_error(SemanticError::new(
                                SemanticErrorKind::TypeMismatch { 
                                    expected: "Boolean".into(), 
                                    found: condition_type.to_string() 
                                },
                                condition.line, condition.column
                            ));
                        }
                    }
                    Err(e) => { self.report_error(e); }
                }

                // Visit body
                self.visit_stmt(body);
            },
            Stmt::For { iterator, start, end, direction: _, body, line, column } => {
                // Search iterator variable di symbol table
                match self.symbol_table.find(iterator) {
                    Some(idx) => {
                        let entry = &self.symbol_table.tab[idx];
                        let iter_type = self.typ_idx_to_kind(entry.typ);
                        if iter_type != TypeKind::Integer {
                            self.report_error(SemanticError::new(
                                SemanticErrorKind::InvalidIterator(iterator.clone()), 
                                *line, *column
                            ));
                        }
                    }
                    None => {
                        self.report_error(SemanticError::new(
                            SemanticErrorKind::UndefinedIdentifier(iterator.clone()), 
                            *line, *column
                        ));
                    }
                }

                // Validate start and end expressions
                let start_ok = match self.visit_expr(start) {
                    Ok(t) if t != TypeKind::Integer => {
                        self.report_error(SemanticError::new(
                            SemanticErrorKind::TypeMismatch { 
                                expected: "Integer".into(), 
                                found: t.to_string() 
                            },
                            start.line, start.column
                        ));
                        false
                    }
                    Err(e) => { self.report_error(e); false }
                    Ok(_) => true
                };

                let end_ok = match self.visit_expr(end) {
                    Ok(t) if t != TypeKind::Integer => {
                        self.report_error(SemanticError::new(
                            SemanticErrorKind::TypeMismatch { 
                                expected: "Integer".into(), 
                                found: t.to_string() 
                            },
                            end.line, end.column
                        ));
                        false
                    }
                    Err(e) => { self.report_error(e); false }
                    Ok(_) => true
                };

                // Still visit body even if bounds are wrong
                let _ = (start_ok, end_ok); // suppress unused warning
                self.visit_stmt(body);
            },
            Stmt::Repeat { body, condition, .. } => {
                for s in body {
                    if self.should_bail() { break; }
                    self.visit_stmt(s);
                }
                
                match self.visit_expr(condition) {
                    Ok(condition_type) => {
                        if condition_type != TypeKind::Boolean {
                            self.report_error(SemanticError::new(
                                SemanticErrorKind::TypeMismatch { 
                                    expected: "Boolean".into(), 
                                    found: condition_type.to_string() 
                                },
                                condition.line, condition.column
                            ));
                        }
                    }
                    Err(e) => { self.report_error(e); }
                }
            },
            Stmt::Case { operand, branches, else_branch, line, column } => {
                let op_type = match self.visit_expr(operand) {
                    Ok(t) => Some(t),
                    Err(e) => { self.report_error(e); None }
                };
                
                for branch in branches {
                    for label in &mut branch.labels {
                        match self.visit_expr(label) {
                            Ok(label_type) => {
                                if let Some(ref expected) = op_type {
                                    if &label_type != expected {
                                        self.report_error(SemanticError::new(
                                            SemanticErrorKind::TypeMismatch { 
                                                expected: expected.to_string(), 
                                                found: label_type.to_string() 
                                            },
                                            *line, *column
                                        ));
                                    }
                                }
                            }
                            Err(e) => { self.report_error(e); }
                        }
                    }
                    self.visit_stmt(&mut branch.stmt);
                }

                if let Some(else_stmts) = else_branch {
                    for stmt in else_stmts {
                        if self.should_bail() { break; }
                        self.visit_stmt(stmt);
                    }
                }
            },
            Stmt::ProcedureCall { name, args, line, column } => {
                // Delegasi ke helper function
                if let Err(e) = self.visit_proc_call(name, args, *line, *column) {
                    self.report_error(e);
                }
            },
            Stmt::Compound(block) => self.visit_block(block),
        }
    }

    fn is_ordinal_type(&self, t: &TypeKind) -> bool {
        matches!(t, TypeKind::Integer | TypeKind::Char | TypeKind::Boolean)
    }

    /// Check type compatibility for assignments
    fn is_type_compatible(&self, target: &TypeKind, value: &TypeKind) -> bool {
        if target == value {
            return true;
        }
        
        // Integer can be assigned to Real
        if *target == TypeKind::Real && *value == TypeKind::Integer {
            return true;
        }
        
        // Char can be assigned to String
        if *target == TypeKind::String && *value == TypeKind::Char {
            return true;
        }
        
        false
    }

    // ==========================================
    // Anggota 4: Array & Function/Procedure Calls
    // Fokus: Validasi argumen, parameter matching, array bounds
    // ==========================================
    
    fn visit_function_call(&mut self, name: &str, args: &mut Vec<Expr>, line: usize, col: usize) -> Result<TypeKind, SemanticError> {
        let func_idx = self.symbol_table.find(name).ok_or(
            SemanticError::new(SemanticErrorKind::UndefinedIdentifier(name.into()), line, col)
        )?;
        let func_entry = &self.symbol_table.tab[func_idx];
        if func_entry.obj != ObjectKind::Function {
             return Err(SemanticError::new(
                 SemanticErrorKind::NotCallable(name.into()), 
                 line, col
            ));
        }
        let ret_type = self.typ_idx_to_kind(func_entry.typ);
        let btab_idx = func_entry.ref_idx;
        if btab_idx == 0 {
            for arg in args { let _ = self.visit_expr(arg); }
            return Ok(ret_type);
        }
        let params_idx = self.get_parameters_from_btab(btab_idx);
        self.validate_args(args, &params_idx, line, col)?;
        Ok(ret_type)
    }

    fn visit_proc_call(&mut self, name: &str, args: &mut Vec<Expr>, line: usize, col: usize) -> Result<(), SemanticError> {
        let proc_idx = self.symbol_table.find(name).ok_or(
            SemanticError::new(SemanticErrorKind::UndefinedIdentifier(name.into()), line, col)
        )?;
        let proc_entry = &self.symbol_table.tab[proc_idx];
        if proc_entry.obj != ObjectKind::Procedure {
             return Err(SemanticError::new(
                 SemanticErrorKind::NotCallable(name.into()),
                 line, col
            ));
        }
        let btab_idx = proc_entry.ref_idx;
        if btab_idx == 0 {
            for arg in args { let _ = self.visit_expr(arg); }
            return Ok(());
        }
        let params_idx = self.get_parameters_from_btab(btab_idx);
        self.validate_args(args, &params_idx, line, col)?;
        Ok(())
    }

    fn validate_args(&mut self, args: &mut Vec<Expr>, params_idx: &[usize], line: usize, col: usize) -> Result<(), SemanticError> {
        if params_idx.len() != args.len() {
             return Err(SemanticError::new(
                 SemanticErrorKind::ArgumentCountMismatch { expected: params_idx.len(), found: args.len() },
                 line, col
            ));
        }
        for (i, (arg_expr, &param_idx)) in args.iter_mut().zip(params_idx.iter()).enumerate() {
            let arg_type = self.visit_expr(arg_expr)?;
            let param_entry = &self.symbol_table.tab[param_idx];
            let param_type = self.typ_idx_to_kind(param_entry.typ);
            
            if !self.is_type_compatible(&param_type, &arg_type) {
                return Err(SemanticError::new(
                    SemanticErrorKind::TypeMismatch { 
                        expected: param_type.to_string(), 
                        found: arg_type.to_string() 
                    },
                    arg_expr.line, arg_expr.column
                ));
            }

            if !param_entry.normal && arg_expr.annotation.tab_index.is_none() {
                 return Err(SemanticError::new(
                     SemanticErrorKind::GenericError(format!("Argument {} must be a variable", i+1)),
                     arg_expr.line, arg_expr.column
                ));
            }
        }
        Ok(())
    }

    // Helper functions
    fn get_parameters_from_btab(&self, btab_idx: usize) -> Vec<usize> {
        if btab_idx >= self.symbol_table.btab.len() { 
            return Vec::new(); 
        }
        
        let mut params = Vec::new();
        let mut curr = self.symbol_table.btab[btab_idx].lpar;
        
        while curr != 0 {
            params.push(curr);
            curr = self.symbol_table.tab[curr].link;
        }
        
        params.reverse();
        params
    }

    fn typ_idx_to_kind(&self, idx: usize) -> TypeKind {
        if idx < self.symbol_table.tab.len() {
            let entry = &self.symbol_table.tab[idx];
            if entry.obj == ObjectKind::Type && entry.ref_idx > 0 {
                let atab_idx = entry.ref_idx - 1;
                if let Some(arr_info) = self.symbol_table.atab.get(atab_idx) {
                    let element_kind = self.typ_idx_to_kind(arr_info.etyp);
                    let range_kind = TypeKind::Subrange(
                        Box::new(Expr::new(ExprKind::LiteralInt(arr_info.low), 0, 0)),
                        Box::new(Expr::new(ExprKind::LiteralInt(arr_info.high), 0, 0))
                    );
                    return TypeKind::Array {
                        index_range: Box::new(range_kind),
                        element_type: Box::new(element_kind)
                    };
                }
            }
        }

        match idx {
            TYP_INT => TypeKind::Integer,
            TYP_REAL => TypeKind::Real,
            TYP_BOOL => TypeKind::Boolean,
            TYP_CHAR => TypeKind::Char,
            TYP_STRING => TypeKind::String,
            _ => TypeKind::Void,
        }
    }

    fn kind_to_typ_idx(&mut self, kind: &TypeKind) -> usize {
        match kind {
            TypeKind::Integer => TYP_INT,
            TypeKind::Real => TYP_REAL,
            TypeKind::Boolean => TYP_BOOL,
            TypeKind::Char => TYP_CHAR,
            TypeKind::String => TYP_STRING,
            TypeKind::Array { index_range, element_type } => {
                let el_idx = self.kind_to_typ_idx(element_type);
                let (low, high, idx_typ) = match &**index_range {
                    TypeKind::Subrange(start, end) => {
                        let l = self.eval_const_expr(start).unwrap_or(0);
                        let h = self.eval_const_expr(end).unwrap_or(0);
                        (l, h, TYP_INT)
                    },
                    _ => (0, 0, TYP_INT),
                };

                let el_size = self.symbol_table.tab[el_idx].adr; 
                let atab_idx = self.symbol_table.make_array(idx_typ, el_idx, 0, low, high, el_size);
                
                let type_idx = self.symbol_table.enter("".to_string(), ObjectKind::Type, TYP_NOTYPE, 0, true); 
                
                let total_size = self.symbol_table.atab[atab_idx].size;
                let last = self.symbol_table.tab.len() - 1;
                
                self.symbol_table.tab[last].ref_idx = atab_idx + 1;
                self.symbol_table.tab[last].typ = last;
                self.symbol_table.tab[last].adr = total_size;

                last
            },
            TypeKind::Subrange(_, _) => TYP_INT, 
            TypeKind::Custom(name) => {
                if let Some(idx) = self.symbol_table.find(name) {
                    if let Some(entry) = self.symbol_table.tab.get(idx) {
                        if entry.obj == ObjectKind::Type {
                            return entry.typ;
                        }
                    }
                }
                TYP_NOTYPE
            },
            _ => TYP_NOTYPE,
        }
    }

    fn eval_const_expr(&self, expr: &Expr) -> Option<i32> {
        match &expr.kind {
            ExprKind::LiteralInt(val) => Some(*val),
            ExprKind::Unary { op: UnOp::Neg, operand } => {
                self.eval_const_expr(operand).map(|v| -v)
            },
            ExprKind::Unary { op: UnOp::Plus, operand } => {
                self.eval_const_expr(operand)
            },
            ExprKind::Binary { left, op, right } => {
                let l = self.eval_const_expr(left)?;
                let r = self.eval_const_expr(right)?;
                match op {
                    BinOp::Add => Some(l + r),
                    BinOp::Sub => Some(l - r),
                    BinOp::Mul => Some(l * r),
                    BinOp::DivInt => if r != 0 { Some(l / r) } else { None },
                    BinOp::Mod => if r != 0 { Some(l % r) } else { None },
                    _ => None,
                }
            },
            ExprKind::Variable(name) => {
                // Look up constant value
                if let Some(idx) = self.symbol_table.find(name) {
                    let entry = &self.symbol_table.tab[idx];
                    if entry.obj == ObjectKind::Constant {
                        // Would need to store constant values in table
                        // For now, return None
                    }
                }
                None
            },
            _ => None
        }
    }

    /// Helper buat record error sama cek harus stop reporting (based on max)
    fn report_error(&mut self, error: SemanticError) -> bool {
        self.errors.push(error);
        self.should_bail()
    }

    /// Helper buat cek kalo udh masuk limit
    fn should_bail(&self) -> bool {
        self.errors.len() >= self.max_errors
    }

    pub fn print_tables(&self) {
        self.symbol_table.print_tables();
    }
}