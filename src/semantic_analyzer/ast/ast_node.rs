pub struct ProgramAST {
    pub name: String,
    pub declarations: Vec<Decl>, // Semua deklarasi (const, type, var, func)
    pub main_body: BlockStmt,    // Blok 'mulai' ... 'selesai' utama
}

pub enum Decl {
    Constant {
        name: String,
        value: Expr
    },
    Type {
        name: String,
        wrapped_type: TypeKind
    },
    Variable {
        name: Vec<String>,
        type_kind: TypeKind
    },
    Procedure { 
        name: String, 
        params: Vec<Param>, 
        local_decls: Vec<Decl>, 
        body: BlockStmt 
    },
    Function { 
        name: String, 
        params: Vec<Param>, 
        return_type: TypeKind, 
        local_decls: Vec<Decl>, 
        body: BlockStmt 
    }
}

pub struct Param {
    pub names: Vec<String>,
    pub type_kind: TypeKind,
    pub is_var: bool
}

#[derive(Clone, Debug, PartialEq)]
pub enum TypeKind {
    Integer,
    Real,
    Boolean,
    Char,
    String,
    Custom(String), 
    Array {
        index_range: (i32, i32), // low .. high
        element_type: Box<TypeKind>
    }
}

pub enum Stmt {
    Assignment {
        target: Expr,
        value: Expr
    },
    ProcedureCall {
        name: String,
        args: Vec<Expr>
    },
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>
    },
    While {
        condition: Expr,
        body: Box<Stmt>
    },
    For {
        iterator: String,
        start: Expr,
        end: Expr,
        direction: ForDirection,
        body: Box<Stmt>
    },
    Repeat {
        body: Vec<Stmt>,
        condition: Expr
    },
    Case {
        operand: Expr,
        branches: Vec<CaseBranch>,
        else_branch: Option<Vec<Stmt>>
    },
    Compound(BlockStmt)
}

pub struct BlockStmt {
    pub statements: Vec<Stmt>
}

pub struct CaseBranch {
    pub labels: Vec<Expr>,
    pub stmt: Stmt
}

pub enum ForDirection {
    To,
    Downto,
}

pub enum Expr {
    Binary {
        left: Box<Expr>,
        op: BinOp,
        right: Box<Expr>
    },
    Unary {
        op: UnOp,
        operand: Box<Expr>
    },

    LiteralInt(i32),
    LiteralReal(f64),
    LiteralString(String),
    LiteralChar(char),
    LiteralBool(bool),
    
    Variable(String),
    
    ArrayAccess {
        array: Box<Expr>,
        index: Box<Expr>
    },
    FunctionCall {
        name: String,
        args: Vec<Expr>
    }
}

pub enum BinOp { Add, Sub, Mul, DivReal, DivInt, Mod, And, Or, Eq, Neq, Lt, Gt, Lte, Gte }
pub enum UnOp { Plus, Neg, Not }