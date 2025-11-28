use std::fmt;
#[derive(Debug, Clone, PartialEq)] 
pub struct SemanticData {
    pub type_kind: Option<TypeKind>,
    pub tab_index: Option<usize>,
    pub is_const: bool, 
}

impl SemanticData {
    pub fn new() -> Self {
        Self { type_kind: None, tab_index: None, is_const: false }
    }
}

#[derive(Debug, Clone)]
pub struct ProgramAST {
    pub name: String,
    pub declarations: Vec<Decl>,
    pub main_body: BlockStmt,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Param {
    pub names: Vec<String>,
    pub type_kind: TypeKind,
    pub is_var: bool
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeKind {
    Integer,
    Real,
    Boolean,
    Char,
    String,
    Void,
    Custom(String),
    Subrange(Box<Expr>, Box<Expr>),
    Array {
        index_range: Box<TypeKind>,
        element_type: Box<TypeKind>
    }
}

impl fmt::Display for TypeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeKind::Integer => write!(f, "Integer"),
            TypeKind::Real => write!(f, "Real"),
            TypeKind::Boolean => write!(f, "Boolean"),
            TypeKind::Char => write!(f, "Char"),
            TypeKind::String => write!(f, "String"),
            TypeKind::Void => write!(f, "Void"),
            TypeKind::Custom(s) => write!(f, "Custom({})", s),
            TypeKind::Subrange(_, _) => write!(f, "Subrange"),
            TypeKind::Array { element_type, .. } => write!(f, "Array of {}", element_type),
        }
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct BlockStmt {
    pub statements: Vec<Stmt>
}

#[derive(Debug, Clone)]
pub struct CaseBranch {
    pub labels: Vec<Expr>,
    pub stmt: Stmt
}

#[derive(Debug, Clone, PartialEq)]
pub enum ForDirection {
    To,
    Downto,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    pub kind: ExprKind,
    pub annotation: SemanticData,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
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

impl Expr {
    pub fn new(kind: ExprKind) -> Self {
        Expr {
            kind,
            annotation: SemanticData::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)] // <-- Tambahkan ini
pub enum BinOp { Add, Sub, Mul, DivReal, DivInt, Mod, And, Or, Eq, Neq, Lt, Gt, Lte, Gte }

#[derive(Debug, Clone, PartialEq)] // <-- Tambahkan ini
pub enum UnOp { Plus, Neg, Not }