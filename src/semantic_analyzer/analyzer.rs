use crate::semantic_analyzer::ast::ast::{
    ProgramAST, Decl, Stmt, Expr, ExprKind, TypeKind, BinOp, UnOp, BlockStmt, Param
};
use crate::semantic_analyzer::tab::{SymbolTable, ObjectKind, TYP_INT, TYP_BOOL, TYP_REAL, TYP_CHAR, TYP_STRING, TYP_NOTYPE};
use crate::semantic_analyzer::error::{SemanticError, SemanticErrorKind};

pub struct SemanticAnalyzer {
    pub symbol_table: SymbolTable,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            symbol_table: SymbolTable::new(),
        }
    }

    /// Entry point
    pub fn analyze(&mut self, program: &mut ProgramAST) -> Result<(), SemanticError> {
        self.visit_program(program)
    }

    // ==========================================
    // Anggota 1: Declarations & Scope
    // Fokus: Mengisi tabel simbol, scope management, hitung address
    // ==========================================
    fn visit_program(&mut self, program: &mut ProgramAST) -> Result<(), SemanticError> {
        // 1. Init Global Scope
        // Identifier program terlebih dahulu supaya masuk ke level 0
        self.symbol_table.enter(program.name.clone(), ObjectKind::Program, TYP_NOTYPE, 0, true);
        self.symbol_table.enter_scope();
        
        // 2. Masukkan identifier program ke tabel
        // Kita masukkan sebagai Constant dengan tipe Void karena tidak punya nilai runtime.
        
        // 3. Visit semua deklarasi global
        self.visit_decls(&mut program.declarations)?;
        
        // 4. Visit main body
        self.visit_block(&mut program.main_body)?;
        
        // 5. Exit Scope
        self.symbol_table.exit_scope();
        
        Ok(())
    }

    fn visit_decls(&mut self, decls: &mut [Decl]) -> Result<(), SemanticError> {
        for decl in decls {
            self.visit_decl(decl)?;
        }
        Ok(())
    }

    fn visit_decl(&mut self, decl: &mut Decl) -> Result<(), SemanticError> {
        match decl {
            Decl::Constant { name, value, .. } => {
                // 1. Evaluasi nilai konstanta untuk dapat tipenya
                let type_kind = self.visit_expr(value)?;
                let type_idx = self.kind_to_typ_idx(&type_kind);
                // 2. Masukkan ke tabel
                self.symbol_table.enter(name.clone(), ObjectKind::Constant, type_idx, 0, true);
                Ok(())
            },
            Decl::Type { name, wrapped_type, .. } => {
                let type_idx = self.kind_to_typ_idx(wrapped_type);
                // 1. Masukkan ke tabel sebagai Type Alias
                self.symbol_table.enter(name.clone(), ObjectKind::Type, type_idx, 0, true);
                Ok(())
            },
            Decl::Variable { name, type_kind, line, column } => {
                // 1. Validasi tipe data
                if *type_kind == TypeKind::Void {
                    return Err(SemanticError::new(
                        SemanticErrorKind::GenericError("Variable cannot be Void".to_string()), 
                        *line, *column
                    ));
                }

                // 2. Loop vector 'name' dan masukkan ke tabel
                let type_idx = self.kind_to_typ_idx(type_kind);
                for var_name in name {
                    self.symbol_table.enter(var_name.clone(), ObjectKind::Variable, type_idx.clone(), 0, true);
                }
                Ok(())
            },
            Decl::Procedure { name, params, local_decls, body, line: _, column: _ } => {
                // 1. Masukkan nama prosedur ke tabel parent
                self.symbol_table.enter(name.clone(), ObjectKind::Procedure, TYP_NOTYPE, 0, true);

                // 2. Naik level (Scope Baru)
                self.symbol_table.enter_scope();

                // 3. Visit parameters
                for param in params {
                    self.visit_param(param)?;
                }

                // Ambil identifier terakhir yang baru saja dimasukkan
                let current_btab_idx = self.symbol_table.display[self.symbol_table.level];
                let last_param_idx = self.symbol_table.btab[current_btab_idx].last;
                self.symbol_table.btab[current_btab_idx].lpar = last_param_idx;
                
                // 4. Visit local_decls & body
                self.visit_decls(local_decls)?;
                self.visit_block(body)?;

                // 6. Exit Scope
                self.symbol_table.exit_scope();
                Ok(())
            },
            Decl::Function { name, params, return_type, local_decls, body, line: _, column: _ } => {
                // 1. Masukkan nama fungsi ke tabel parent
                let ret_idx = self.kind_to_typ_idx(return_type);
                self.symbol_table.enter(name.clone(), ObjectKind::Function, ret_idx, 0, true);

                // 2. Naik level (Scope Baru)
                self.symbol_table.enter_scope();

                // 3. Visit parameters
                for param in params {
                    self.visit_param(param)?;
                }

                // Masukkan nama fungsi sebagai variabel lokal untuk return value assignment
                self.symbol_table.enter(name.clone(), ObjectKind::Variable, ret_idx, 0, true);

                // 4. Visit local_decls & body
                self.visit_decls(local_decls)?;
                self.visit_block(body)?;

                // 6. Exit Scope
                self.symbol_table.exit_scope();
                Ok(())
            }
        }
    }

    fn visit_param(&mut self, param: &Param) -> Result<(), SemanticError> {
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
        Ok(())
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
                let left_type = self.visit_expr(left)?;
                let right_type = self.visit_expr(right)?;

                match op {
                    // Operasi aritmatika
                    BinOp::Add | BinOp::Sub | BinOp::Mul => {
                        if left_type == TypeKind::Integer && right_type == TypeKind::Integer {
                            Ok(TypeKind::Integer)
                        } else if (left_type == TypeKind::Integer || left_type == TypeKind::Real) &&
                                  (right_type == TypeKind::Integer || right_type == TypeKind::Real) {
                            Ok(TypeKind::Real)
                        } else {
                            Err(SemanticError::new(
                                SemanticErrorKind::InvalidOperation {
                                    op: format!("{:?}", op),
                                    left_type: left_type.to_string(),
                                    right_type: right_type.to_string()
                                },
                                current_line, current_col
                            ))
                        }
                    },
                    BinOp::DivReal => {
                        if (left_type == TypeKind::Integer || left_type == TypeKind::Real) &&
                           (right_type == TypeKind::Integer || right_type == TypeKind::Real) {
                            Ok(TypeKind::Real)
                        } else {
                            Err(SemanticError::new(
                                SemanticErrorKind::TypeMismatch { expected: "Numeric".into(), found: "Non-numeric".into() },
                                current_line, current_col
                            ))
                        }
                    },
                    BinOp::DivInt | BinOp::Mod => {
                        if left_type == TypeKind::Integer && right_type == TypeKind::Integer {
                            Ok(TypeKind::Integer)
                        } else {
                            Err(SemanticError::new(
                                SemanticErrorKind::TypeMismatch { expected: "Integer".into(), found: "Non-Integer".into() },
                                current_line, current_col
                            ))
                        }
                    },
                    // Operasi perbandingan
                    BinOp::Eq | BinOp::Neq | BinOp::Lt | BinOp::Lte | BinOp::Gt | BinOp::Gte => {
                        if left_type == right_type {
                            Ok(TypeKind::Boolean)
                        } else if (left_type == TypeKind::Integer || left_type == TypeKind::Real) &&
                                  (right_type == TypeKind::Integer || right_type == TypeKind::Real) {
                            Ok(TypeKind::Boolean)
                        } else {
                             Err(SemanticError::new(
                                SemanticErrorKind::TypeMismatch { expected: "Comparable Types".into(), found: "Incompatible Types".into() },
                                current_line, current_col
                            ))
                        }
                    },
                    // Operasi logika
                    BinOp::And | BinOp::Or => {
                        if left_type == TypeKind::Boolean && right_type == TypeKind::Boolean {
                            Ok(TypeKind::Boolean)
                        } else {
                            Err(SemanticError::new(
                                SemanticErrorKind::TypeMismatch { expected: "Boolean".into(), found: "Non-Boolean".into() },
                                current_line, current_col
                            ))
                        }
                    }
                }
            },
            ExprKind::Unary { op, operand } => {
                let op_type = self.visit_expr(operand)?;

                match op {
                    // Operasi Not
                    UnOp::Not => {
                        if op_type == TypeKind::Boolean {
                            Ok(TypeKind::Boolean)
                        } else { 
                            Err(SemanticError::new(
                                SemanticErrorKind::TypeMismatch { expected: "Boolean".into(), found: op_type.to_string() },
                                current_line, current_col
                            ))
                        }
                    },
                    // Operasi Negasi
                    UnOp::Neg | UnOp::Plus => {
                        if op_type == TypeKind::Integer {
                            Ok(TypeKind::Integer)
                        } else if op_type == TypeKind::Real {
                            Ok(TypeKind::Real)
                        } else { 
                            Err(SemanticError::new(
                                SemanticErrorKind::TypeMismatch { expected: "Numeric".into(), found: op_type.to_string() },
                                current_line, current_col
                            ))
                        }
                    },
                }
            },
            ExprKind::LiteralInt(_) => Ok(TypeKind::Integer),
            ExprKind::LiteralReal(_) => Ok(TypeKind::Real),
            ExprKind::LiteralString(_) => Ok(TypeKind::String),
            ExprKind::LiteralChar(_) => Ok(TypeKind::Char),
            ExprKind::LiteralBool(_) => Ok(TypeKind::Boolean),
            
            ExprKind::Variable(name) => {
                if let Some(idx) = self.symbol_table.find(name) {
                    let entry = self.symbol_table.tab.get(idx).unwrap();
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
                    TypeKind::Array { element_type, .. } => {
                        let index_type = self.visit_expr(index)?;
                        if index_type == TypeKind::Integer {
                            Ok(*element_type)
                        } else {
                            return Err(SemanticError::new(
                                SemanticErrorKind::IndexTypeMismatch(index_type.to_string()),
                                current_line, current_col
                            ));
                        }
                    },
                    _ => {
                        return Err(SemanticError::new(
                         SemanticErrorKind::NotArray("Variable".into()), 
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

    // ==========================================
    // Anggota 3: Statements & Flow Control
    // Fokus: Validasi alur (if/while butuh bool), assignment compatibility
    // ==========================================
    fn visit_block(&mut self, block: &mut BlockStmt) -> Result<(), SemanticError> {
        for stmt in &mut block.statements {
            self.visit_stmt(stmt)?;
        }
        Ok(())
    }

    fn visit_stmt(&mut self, stmt: &mut Stmt) -> Result<(), SemanticError> {
        match stmt {
            Stmt::Assignment { target, value, line, column } => {
                // Evaluasi Tipe
                let target_type = self.visit_expr(target)?;
                let value_type = self.visit_expr(value)?;

                // Cek apakah target adalah variabel?
                if let Some(idx) = target.annotation.tab_index {
                    if let Some(entry) = self.symbol_table.tab.get(idx) {
                        let entry = &self.symbol_table.tab[idx];
                        if entry.obj == ObjectKind::Constant {
                            return Err(SemanticError::new(
                                SemanticErrorKind::AssignmentToConstant(entry.name.clone()), 
                                *line, *column
                            ));
                        }
                    }
                }

                if target_type != value_type {
                    // Allow Int to Real
                    if target_type == TypeKind::Real && value_type == TypeKind::Integer {
                        return Ok(());
                    }
                    return Err(SemanticError::new(
                        SemanticErrorKind::TypeMismatch { 
                            expected: target_type.to_string(), 
                            found: value_type.to_string() 
                        },
                        *line, *column
                    ));
                }
                Ok(())
            },
            Stmt::If { condition, then_branch, else_branch, .. } => {
                // Evaluasi kondisi
                let condition_type = self.visit_expr(condition)?;

                // Validasi tipe kondisi harus boolean
                if condition_type != TypeKind::Boolean {
                    return Err(SemanticError::new(
                        SemanticErrorKind::TypeMismatch { expected: "Boolean".into(), found: condition_type.to_string() },
                        condition.line, condition.column
                    ));
                }

                // Visit branch
                self.visit_stmt(then_branch)?;

                if let Some(else_stmt) = else_branch {
                    self.visit_stmt(else_stmt)?;
                }
                Ok(())
            },
            Stmt::While { condition, body, .. } => {
                // Evaluasi kondisi
                let condition_type = self.visit_expr(condition)?;

                // Validasi tipe kondisi harus boolean
                if condition_type != TypeKind::Boolean {
                    return Err(SemanticError::new(
                        SemanticErrorKind::TypeMismatch { expected: "Boolean".into(), found: condition_type.to_string() },
                        condition.line, condition.column
                    ));
                }

                // Visit body
                self.visit_stmt(body)?;
                Ok(())
            },
            Stmt::For { iterator, start, end, direction: _, body, line, column } => {
                // Search iterator variable di symbol table
                if let Some(idx) = self.symbol_table.find(iterator) {
                    let entry = &self.symbol_table.tab[idx];
                    let iter_type = self.typ_idx_to_kind(entry.typ);
                    if iter_type != TypeKind::Integer {
                         return Err(SemanticError::new(
                             SemanticErrorKind::InvalidIterator(iterator.clone()), 
                             *line, *column
                        ));
                    }
                } else {
                     return Err(SemanticError::new(
                         SemanticErrorKind::UndefinedIdentifier(iterator.clone()), 
                         *line, *column
                    ));
                }

                let start_type = self.visit_expr(start)?;
                let end_type = self.visit_expr(end)?;

                if start_type != TypeKind::Integer || end_type != TypeKind::Integer {
                    return Err(SemanticError::new(
                        SemanticErrorKind::TypeMismatch { expected: "Integer".into(), found: "Non-Integer".into() },
                        *line, *column
                    ));
                }

                self.visit_stmt(body)?;
                Ok(())
            },
            Stmt::Repeat { body, condition, .. } => {
                for s in body {
                    self.visit_stmt(s)?;
                }
                let condition_type = self.visit_expr(condition)?;
                if condition_type != TypeKind::Boolean {
                     return Err(SemanticError::new(
                        SemanticErrorKind::TypeMismatch { expected: "Boolean".into(), found: condition_type.to_string() },
                        condition.line, condition.column
                    ));
                }
                Ok(())
            },
            Stmt::Case { operand, branches, else_branch, line, column } => {
                let op_type = self.visit_expr(operand)?;
                
                // Cek branches
                for branch in branches {
                    for label in &mut branch.labels {
                         let label_type = self.visit_expr(label)?;
                         if label_type != op_type {
                             return Err(SemanticError::new(
                                 SemanticErrorKind::TypeMismatch { expected: op_type.to_string(), found: label_type.to_string() },
                                 *line, *column
                            ));
                         }
                    }
                    self.visit_stmt(&mut branch.stmt)?;
                }

                if let Some(else_stmts) = else_branch {
                    for stmt in else_stmts {
                        self.visit_stmt(stmt)?;
                    }
                }
                Ok(())
            },
            Stmt::ProcedureCall { name, args, line, column } => {
                // Delegasi ke helper function
                self.visit_proc_call(name, args, *line, *column)
            },
            Stmt::Compound(block) => self.visit_block(block),
        }
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
            for arg in args { self.visit_expr(arg)?; }
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
            let is_compat = arg_type == param_type || (arg_type == TypeKind::Integer && param_type == TypeKind::Real);
            if !is_compat {
                 return Err(SemanticError::new(
                     SemanticErrorKind::TypeMismatch { expected: param_type.to_string(), found: arg_type.to_string() },
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
        if btab_idx >= self.symbol_table.btab.len() { return Vec::new(); }
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
        // Cek apakah tipe adalah Array (disimpan di ATAB)
        let entry = &self.symbol_table.tab[idx];
        if entry.obj == ObjectKind::Type && entry.ref_idx > 0 {
             // ref_idx 1-based ke ATAB
             let atab_idx = entry.ref_idx - 1;
             if let Some(arr_info) = self.symbol_table.atab.get(atab_idx) {
                 let element_kind = self.typ_idx_to_kind(arr_info.etyp);
                 // Rekonstruksi subrange dummy (kita hanya butuh TypeKind, bukan nilai eksaknya untuk display)
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
                
                // ref_idx -> atab_idx + 1
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
            _ => None
        }
    }

    pub fn print_tables(&self) {
        self.symbol_table.print_tables();
    }
}