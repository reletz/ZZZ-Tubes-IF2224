#[derive(Debug, Clone, PartialEq)]
pub enum Factor {
    Literal(LiteralValue),
    Identifier(String),
    FunctionCall(FunctionCallNode),
    ArrayAccess(ArrayAccess),
    Parenthesized(Box<Expression>),
    Not(Box<Factor>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Term {
    pub initial_factor: Box<Factor>,
    pub rest: Vec<(String, Box<Factor>)>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SimpleExpression {
    pub unary_op: Option<String>,
    pub initial_term: Box<Term>,
    pub rest: Vec<(String, Box<Term>)>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub initial_simple_expr: Box<SimpleExpression>,
    pub rest: Vec<(String, Box<SimpleExpression>)>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LiteralValue {
    pub value: Box<Literal>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Integer(i64),
    Real(f64),
    String(String),
    Boolean(bool),
    Char(char),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayAccess {
    // Sisi kiri assignment BISA jadi array access
    // Jadi ini harus Expression, bukan cuma String
    pub array: Box<Expression>,
    pub index: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCallNode {
    pub function_name: String,
    pub arguments: ParameterList,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParameterList {
    pub expressions: Vec<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Range {
    pub start: Box<Expression>,
    pub end: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayType {
    pub range: Box<Range>,
    pub base_type: Box<Type>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Integer,
    Real,
    Boolean,
    String,
    Char,
    Array(ArrayType),
    Subrange(Range),
    TypeIdentifier(String),
}

#[derive(Debug)]
pub struct IdentifierList {
    pub identifiers: Vec<String>,
}

#[derive(Debug)]
pub struct VariableGroup {
    pub identifiers: IdentifierList,
    pub var_type: Type,
}

#[derive(Debug)]
pub struct VariableDeclaration {
    pub groups: Vec<VariableGroup>,
}

#[derive(Debug)]
pub struct ConstantDefinition {
    pub name: String,
    pub value: Expression,
}

#[derive(Debug)]
pub struct ConstantDeclaration {
    pub constants: Vec<ConstantDefinition>,
}

#[derive(Debug)]
pub struct TypeDefinition {
    pub name: String,
    pub type_def: Type,
}

#[derive(Debug)]
pub struct TypeDeclaration {
    pub definitions: Vec<TypeDefinition>,
}

#[derive(Debug)]
pub struct FormalParameterGroup {
    pub identifiers: IdentifierList,
    pub var_type: Type,
}

#[derive(Debug)]
pub struct FormalParameterList {
    pub parameters: Vec<FormalParameterGroup>,
}

#[derive(Debug)]
pub struct ProcedureDeclaration {
    pub name: String,
    pub parameters: FormalParameterList,
    pub declarations: DeclarationPart,
    pub body: CompoundStatement,
}

#[derive(Debug)]
pub struct FunctionDeclaration {
    pub name: String,
    // Diubah: () wajib ada, jadi bukan Option
    pub parameters: FormalParameterList,
    pub return_type: Type,
    pub declarations: DeclarationPart,
    pub body: CompoundStatement,
}

#[derive(Debug)]
pub enum SubprogramDeclaration {
    Procedure(ProcedureDeclaration),
    Function(FunctionDeclaration),
}

#[derive(Debug)]
pub struct DeclarationPart {
    pub const_declarations: Vec<ConstantDeclaration>,
    pub type_declarations: Vec<TypeDeclaration>,
    pub var_declarations: Vec<VariableDeclaration>,
    pub subprogram_declarations: Vec<SubprogramDeclaration>,
}

#[derive(Debug)]
pub struct ProgramHeader {
    pub name: String,
}

#[derive(Debug)]
pub struct Program {
    pub header: ProgramHeader,
    pub declarations: DeclarationPart,
    pub body: CompoundStatement,
}

#[derive(Debug)]
pub struct StatementList {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub struct CompoundStatement {
    pub statement_list: StatementList,
}

#[derive(Debug)]
pub struct AssignmentStatement {
    pub variable: Expression,
    pub expression: Expression,
}

#[derive(Debug)]
pub struct IfStatement {
    pub condition: Expression,
    pub then_branch: Box<Statement>,
    pub else_branch: Option<Box<Statement>>,
}

#[derive(Debug)]
pub struct WhileStatement {
    pub condition: Expression,
    pub body: Box<Statement>,
}

#[derive(Debug, Clone, Copy)]
pub enum ForDirection {
    To,
    DownTo,
}

#[derive(Debug)]
pub struct ForStatement {
    pub counter_variable: String,
    pub start_value: Expression,
    pub end_value: Expression,
    pub direction: ForDirection,
    pub body: Box<Statement>,
}

#[derive(Debug)]
pub struct ProcedureCallStatement {
    pub procedure_name: String,
    pub arguments: ParameterList,
}

#[derive(Debug)]
pub struct RepeatStatement {
    pub statement_list: StatementList,
    pub condition: Expression,
}

#[derive(Debug)]
pub struct CaseBranch {
    pub labels: Vec<Expression>,
    pub statement: Box<Statement>,
}

#[derive(Debug)]
pub struct CaseStatement {
    pub expression: Expression,
    pub branches: Vec<CaseBranch>,
    pub else_branch: Option<StatementList>,
}

#[derive(Debug)]
pub enum Statement {
    Compound(CompoundStatement),
    Assignment(AssignmentStatement),
    If(IfStatement),
    While(WhileStatement),
    For(ForStatement),
    Repeat(RepeatStatement),
    Case(CaseStatement),
    ProcedureCall(ProcedureCallStatement),
}