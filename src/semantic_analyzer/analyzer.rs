use crate::semantic_analyzer::ast::ast::{
    ProgramAST, Decl, Stmt, Expr, ExprKind, TypeKind, BinOp, UnOp, BlockStmt, Param
};
use crate::semantic_analyzer::tab::{SymbolTable, ObjectKind, TYP_INT, TYP_BOOL, TYP_REAL, TYP_CHAR, TYP_STRING};
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
    pub fn analyze(&mut self, program: &ProgramAST) -> Result<(), SemanticError> {
        self.visit_program(program)
    }

    // ==========================================
    // Anggota 1: Declarations & Scope
    // Fokus: Mengisi tabel simbol, scope management, hitung address
    // ==========================================
    fn visit_program(&mut self, program: &ProgramAST) -> Result<(), SemanticError> {
        // 1. Init Global Scope
        self.symbol_table.enter_scope();
        
        // 2. Masukkan identifier program ke tabel
        // Kita masukkan sebagai Constant dengan tipe Void karena tidak punya nilai runtime.
        self.symbol_table.enter(&program.name, ObjectKind::Constant, TypeKind::Void, 0)?;
        
        // 3. Visit semua deklarasi global
        self.visit_decls(&program.declarations)?;
        
        // 4. Visit main body
        self.visit_block(&program.main_body)?;
        
        // 5. Exit Scope
        self.symbol_table.exit_scope();
        
        Ok(())
    }

    fn visit_decls(&mut self, decls: &[Decl]) -> Result<(), SemanticError> {
        for decl in decls {
            self.visit_decl(decl)?;
        }
        Ok(())
    }

    fn visit_decl(&mut self, decl: &Decl) -> Result<(), SemanticError> {
        match decl {
            Decl::Constant { name, value } => {
                // 1. Evaluasi nilai konstanta untuk dapat tipenya
                let mut expr_clone = value.clone();
                let type_kind = self.visit_expr(&mut expr_clone)?;

                // 2. Masukkan ke tabel
                self.symbol_table.enter(name, ObjectKind::Constant, type_kind, 0)?;
                Ok(())
            },
            Decl::Type { name, wrapped_type } => {
                // 1. Masukkan ke tabel sebagai Type Alias
                self.symbol_table.enter(name, ObjectKind::Type, wrapped_type.clone(), 0)?;
                Ok(())
            },
            Decl::Variable { name, type_kind } => {
                // 1. Validasi tipe data
                if *type_kind == TypeKind::Void {
                    return Err(SemanticError::new(SemanticErrorKind::TypeMismatch, "Variable cannot be Void"));
                }

                // 2. Loop vector 'name' dan masukkan ke tabel
                for var_name in name {
                    self.symbol_table.enter(var_name, ObjectKind::Variable, type_kind.clone(), 0)?;
                }
                Ok(())
            },
            Decl::Procedure { name, params, local_decls, body } => {
                // 1. Masukkan nama prosedur ke tabel parent
                self.symbol_table.enter(name, ObjectKind::Procedure, TypeKind::Void, 0)?;

                // 2. Naik level (Scope Baru)
                self.symbol_table.enter_scope();

                // 3. Visit parameters
                for param in params {
                    self.visit_param(param)?;
                }

                // 4. Visit local_decls & body
                self.visit_decls(local_decls)?;
                self.visit_block(body)?;

                // 6. Exit Scope
                self.symbol_table.exit_scope();
                Ok(())
            },
            Decl::Function { name, params, return_type, local_decls, body } => {
                // 1. Masukkan nama fungsi ke tabel parent
                self.symbol_table.enter(name, ObjectKind::Function, return_type.clone(), 0)?;

                // 2. Naik level (Scope Baru)
                self.symbol_table.enter_scope();

                // 3. Visit parameters
                for param in params {
                    self.visit_param(param)?;
                }

                // PENTING: Masukkan nama fungsi sebagai variabel lokal untuk return value assignment
                self.symbol_table.enter(name, ObjectKind::Variable, return_type.clone(), 0)?;

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
        let type_kind = &param.type_kind;

        for param_name in &param.names {
            // Masukkan parameter sebagai variabel lokal
            self.symbol_table.enter(param_name, ObjectKind::Variable, type_kind.clone(), 0)?;
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
        let type_result = match &mut expr.kind {
            ExprKind::Binary { left, op, right } => {
                let left_type = self.visit_expr(left)?;
                let right_type = self.visit_expr(right)?;
                
                // TODO: Logic Type Checking
                // Misal: if left == Int && right == Int && op == Add -> Return Int
                // Misal: if left == Int && right == Int && op == Eq -> Return Bool
                // Else -> return SemanticError::TypeMismatch
                
                Ok(TypeKind::Integer) // Placeholder
            },
            ExprKind::Unary { op, operand } => {
                let op_type = self.visit_expr(operand)?;
                
                // TODO: Logic Unary
                // Misal: if op == Not && op_type == Bool -> Return Bool
                // Else -> Error
                
                Ok(op_type) // Placeholder
            },
            ExprKind::LiteralInt(_) => Ok(TypeKind::Integer),
            ExprKind::LiteralReal(_) => Ok(TypeKind::Real),
            ExprKind::LiteralString(_) => Ok(TypeKind::String),
            ExprKind::LiteralChar(_) => Ok(TypeKind::Char),
            ExprKind::LiteralBool(_) => Ok(TypeKind::Boolean),
            
            ExprKind::Variable(name) => {
                // TODO:
                // 1. self.symbol_table.find(name)
                // 2. Jika None -> Error UndefinedIdentifier
                // 3. Jika Some(idx) -> 
                //    - Ambil tipe data dari tab[idx].typ
                //    - Simpan idx ke expr.annotation.tab_index (PENTING BUAT CODE GEN)
                //    - Return TypeKind yang sesuai
                
                Ok(TypeKind::Integer) // Placeholder
            },
            
            ExprKind::ArrayAccess { array, index } => {
                // TODO:
                // 1. visit_expr(array) -> Pastikan tipenya Array
                // 2. visit_expr(index) -> Pastikan tipenya Integer/Subrange yang sesuai
                // 3. Return tipe elemen array
                
                Ok(TypeKind::Integer) // Placeholder
            },
            
            ExprKind::FunctionCall { name, args } => {
                // Delegasi ke helper function khusus
                self.visit_function_call(name, args)
            }
        }?;

        // Simpan hasil tipe ke dalam node AST untuk M4
        expr.annotation.type_kind = Some(type_result.clone());
        
        Ok(type_result)
    }

    // ==========================================
    // Anggota 3: Statements & Flow Control
    // Fokus: Validasi alur (if/while butuh bool), assignment compatibility
    // ==========================================
    fn visit_block(&mut self, block: &BlockStmt) -> Result<(), SemanticError> {
        for stmt in &block.statements {
            self.visit_stmt(stmt)?;
        }
        Ok(())
    }

    fn visit_stmt(&mut self, stmt: &Stmt) -> Result<(), SemanticError> {
        match stmt {
            Stmt::Assignment { target, value } => {
                // TODO:
                // 1. visit_expr(target) & visit_expr(value)
                // 2. Cek apakah target adalah l-value (bisa diisi, bukan konstanta)
                //    - Cek target.annotation.tab_index -> lihat di tab -> pastikan obj != Constant
                // 3. Cek Compatibility: Tipe target == Tipe value
                // 4. Error jika beda tipe
                Ok(())
            },
            Stmt::If { condition, then_branch, else_branch } => {
                // TODO:
                // 1. visit_expr(condition)
                // 2. Cek condition harus BOOLEAN. Error jika Integer/lainnya.
                // 3. visit_stmt(then_branch)
                // 4. if else_branch exists -> visit_stmt
                Ok(())
            },
            Stmt::While { condition, body } => {
                // TODO:
                // 1. visit_expr(condition)
                // 2. Cek condition harus BOOLEAN
                // 3. visit_stmt(body)
                Ok(())
            },
            Stmt::For { iterator, start, end, direction, body } => {
                // TODO:
                // 1. Lookup iterator di tabel. Harus variabel lokal & tipe ordinal (Int/Char).
                // 2. visit_expr(start) & visit_expr(end). Pastikan tipe sama dengan iterator.
                // 3. visit_stmt(body)
                Ok(())
            },
            Stmt::Repeat { body, condition } => {
                // TODO:
                // 1. Visit semua stmt di body
                // 2. visit_expr(condition) -> Harus BOOLEAN
                Ok(())
            },
            Stmt::Case { operand, branches, else_branch } => {
                // TODO:
                // 1. visit_expr(operand) -> Ambil tipenya (misal Int)
                // 2. Loop branches:
                //    - Loop labels: visit_expr(label) -> Pastikan tipenya SAMA dengan operand
                //    - visit_stmt(stmt)
                Ok(())
            },
            Stmt::ProcedureCall { name, args } => {
                // Delegasi ke helper function
                self.visit_proc_call(name, args)
            },
            Stmt::Compound(block) => self.visit_block(block),
        }
    }

    // ==========================================
    // Anggota 4: Array & Function/Procedure Calls
    // Fokus: Validasi argumen, parameter matching, array bounds
    // ==========================================
    
    fn visit_function_call(&mut self, name: &str, args: &mut Vec<Expr>) -> Result<TypeKind, SemanticError> {
        // TODO:
        // 1. Lookup 'name' di Symbol Table
        // 2. Cek obj == Function. Jika Variable/Procedure -> Error NotCallable
        // 3. Ambil daftar parameter dari btab (linked list params)
        // 4. Cek jumlah argumen == jumlah parameter
        // 5. Loop argumen vs parameter:
        //    - visit_expr(arg)
        //    - Cek tipe arg == tipe param
        //    - (Opsional) Handle var parameter check (arg harus l-value)
        // 6. Return tipe return function tersebut
        
        Ok(TypeKind::Integer) // Placeholder
    }

    fn visit_proc_call(&mut self, name: &str, args: &Vec<Expr>) -> Result<(), SemanticError> {
        // TODO:
        // Mirip function call, tapi return void/unit
        // 1. Lookup 'name'
        // 2. Cek obj == Procedure
        // 3. Validasi argumen vs parameter
        Ok(())
    }
}