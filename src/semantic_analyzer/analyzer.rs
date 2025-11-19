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
        // TODO: self.symbol_table.enter_scope() untuk level 0 (jika perlu)
        
        // 2. Masukkan identifier program ke tabel
        
        // 3. Visit semua deklarasi global
        self.visit_decls(&program.declarations)?;
        
        // 4. Visit main body
        self.visit_block(&program.main_body)?;
        
        // 5. Exit Scope
        // TODO: self.symbol_table.exit_scope()
        
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
                // TODO:
                // 1. Evaluasi nilai konstanta (harus constant expression)
                // 2. Masukkan ke tabel: self.symbol_table.enter(name, ObjectKind::Constant, type, value)
                Ok(())
            },
            Decl::Type { name, wrapped_type } => {
                // TODO:
                // 1. Validasi tipe data
                // 2. Masukkan ke tabel: ObjectKind::Type
                Ok(())
            },
            Decl::Variable { name, type_kind } => {
                // TODO:
                // 1. Validasi tipe data (pastikan tipe ada)
                // 2. Loop vector 'name'
                // 3. Untuk setiap nama: self.symbol_table.enter(n, ObjectKind::Variable, type_idx, address)
                // 4. Update address counter (adr)
                Ok(())
            },
            Decl::Procedure { name, params, local_decls, body } => {
                // TODO:
                // 1. Masukkan nama prosedur ke tabel (ObjectKind::Procedure) di scope parent
                // 2. self.symbol_table.enter_scope() -> Naik level
                // 3. Visit parameters (masukkan ke tabel sebagai variabel lokal/param)
                // 4. Visit local_decls
                // 5. Visit body (block stmt)
                // 6. self.symbol_table.exit_scope()
                Ok(())
            },
            Decl::Function { name, params, return_type, local_decls, body } => {
                // TODO:
                // Sama seperti Procedure, tapi set return type di tabel simbol
                // Pastikan tipe return valid
                Ok(())
            }
        }
    }

    fn visit_param(&mut self, param: &Param) -> Result<(), SemanticError> {
        // TODO:
        // 1. Resolve tipe data param
        // 2. Loop nama param
        // 3. Masukkan ke tabel. 
        // PENTING: Cek param.is_var
        // Jika is_var == true -> set normal = false (Pass by Reference)
        // Jika is_var == false -> set normal = true (Pass by Value)
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