use crate::lexer::token_types::Token;

#[derive(Debug)]
pub struct Program {
    pub header: ProgramHeader,
    pub declarations: DeclarationPart,
    pub body: CompoundStatement,
    pub dot: Token, // DOT(.)
}

#[derive(Debug)]
pub struct ProgramHeader {
    pub program_kw: Token, // KEYWORD(program)
    pub name: Token,       // IDENTIFIER(Hello)
    pub semicolon: Token,  // SEMICOLON(;)
}

// --- Declarations ---

#[derive(Debug)]
pub struct DeclarationPart {
    pub const_declarations: Vec<ConstantDeclaration>,
    pub type_declarations: Vec<TypeDeclaration>,
    pub var_declarations: Vec<VariableDeclaration>,
    pub subprogram_declarations: Vec<SubprogramDeclaration>,
}

#[derive(Debug)]
pub struct ConstantDeclaration {
    pub const_kw: Token, // KEYWORD(konstanta)
    pub constants: Vec<ConstantDefinition>,
}

#[derive(Debug)]
pub struct ConstantDefinition {
    pub name: Token,      // IDENTIFIER(MAX)
    pub equals_op: Token, // OPERATOR(=)
    pub value: Expression,
    pub semicolon: Token, // SEMICOLON(;)
}

#[derive(Debug)]
pub struct TypeDeclaration {
    pub type_kw: Token, // KEYWORD(tipe)
    pub definitions: Vec<TypeDefinition>,
}

#[derive(Debug)]
pub struct TypeDefinition {
    pub name: Token,      // IDENTIFIER(MyArray)
    pub equals_op: Token, // OPERATOR(=)
    pub type_def: Type,
    pub semicolon: Token, // SEMICOLON(;)
}

#[derive(Debug)]
pub struct VariableDeclaration {
    pub var_kw: Token, // KEYWORD(variabel)
    pub groups: Vec<VariableGroup>,
}

#[derive(Debug)]
pub struct VariableGroup {
    pub identifiers: IdentifierList,
    pub colon: Token,     // COLON(:)
    pub var_type: Type,
    pub semicolon: Token, // SEMICOLON(;)
}

#[derive(Debug)]
pub struct IdentifierList {
    pub initial_id: Token, // IDENTIFIER(a)
    pub rest: Vec<(Token, Token)>, // (COMMA, IDENTIFIER)
}

#[derive(Debug)]
pub enum SubprogramDeclaration {
    Procedure(ProcedureDeclaration),
    Function(FunctionDeclaration),
}

#[derive(Debug)]
pub struct ProcedureDeclaration {
    pub proc_kw: Token,          // KEYWORD(prosedur)
    pub name: Token,             // IDENTIFIER(Cetak)
    pub parameters: FormalParameterList,
    pub header_semicolon: Token, // SEMICOLON(;)
    pub declarations: DeclarationPart,
    pub body: CompoundStatement,
    pub block_semicolon: Token,  // SEMICOLON(;)
}

#[derive(Debug)]
pub struct FunctionDeclaration {
    pub func_kw: Token,          // KEYWORD(fungsi)
    pub name: Token,             // IDENTIFIER(Hitung)
    pub parameters: FormalParameterList,
    pub colon: Token,            // COLON(:)
    pub return_type: Type,
    pub header_semicolon: Token, // SEMICOLON(;)
    pub declarations: DeclarationPart,
    pub body: CompoundStatement,
    pub block_semicolon: Token,  // SEMICOLON(;)
}

#[derive(Debug)]
pub struct FormalParameterList {
    pub l_paren: Token, // LPARENTHESIS(()
    // (param_group (SEMICOLON param_group)*)?
    pub initial_param: Option<FormalParameterGroup>,
    pub rest: Vec<(Token, FormalParameterGroup)>, // (SEMICOLON, FormalParameterGroup)
    pub r_paren: Token, // RPARENTHESIS())
}

#[derive(Debug)]
pub struct FormalParameterGroup {
    pub identifiers: IdentifierList,
    pub colon: Token, // COLON(:)
    pub var_type: Type,
}

// --- Types ---

#[derive(Debug)]
pub enum Type {
    Integer(Token),        // KEYWORD(integer)
    Real(Token),           // KEYWORD(real)
    Boolean(Token),        // KEYWORD(boolean)
    String(Token),         // KEYWORD(string)
    Char(Token),           // KEYWORD(char)
    Array(ArrayType),
    Subrange(Range),
    TypeIdentifier(Token), // IDENTIFIER(MyType)
}

#[derive(Debug)]
pub struct ArrayType {
    pub larik_kw: Token,    // KEYWORD(larik)
    pub l_bracket: Token,   // LBRACKET([)
    pub index_type: Box<Type>,
    pub r_bracket: Token,   // RBRACKET(])
    pub dari_kw: Token,     // KEYWORD(dari)
    pub base_type: Box<Type>,
}

#[derive(Debug)]
pub struct Range {
    pub start: Box<Expression>,
    pub range_op: Token, // RANGE_OPERATOR(..)
    pub end: Box<Expression>,
}

// --- Statements ---

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

#[derive(Debug)]
pub struct StatementList {
    // (statement (SEMICOLON statement)*)?
    pub initial_stmt: Option<Box<Statement>>,
    pub rest: Vec<(Token, Box<Statement>)>, // (SEMICOLON, Statement)
    pub trailing_semicolon: Option<Token>,
}

#[derive(Debug)]
pub struct CompoundStatement {
    pub begin_kw: Token, // KEYWORD(mulai)
    pub statement_list: StatementList,
    pub end_kw: Token,   // KEYWORD(selesai)
}

#[derive(Debug)]
pub struct AssignmentStatement {
    pub variable: Expression, // Bisa jadi Identifier, ArrayAccess, dll.
    pub assign_op: Token,   // ASSIGN_OPERATOR(:=)
    pub expression: Expression,
}

#[derive(Debug)]
pub struct IfStatement {
    pub if_kw: Token,      // KEYWORD(jika)
    pub condition: Expression,
    pub then_kw: Token,    // KEYWORD(maka)
    pub then_branch: Box<Statement>,
    pub else_clause: Option<ElseClause>,
}

#[derive(Debug)]
pub struct ElseClause {
    pub else_kw: Token, // KEYWORD(selain-itu)
    pub statement: Box<Statement>,
}

#[derive(Debug)]
pub struct WhileStatement {
    pub while_kw: Token, // KEYWORD(selama)
    pub condition: Expression,
    pub do_kw: Token,    // KEYWORD(lakukan)
    pub body: Box<Statement>,
}

#[derive(Debug)]
pub struct ForStatement {
    pub for_kw: Token,         // KEYWORD(untuk)
    pub counter_variable: Token, // IDENTIFIER(i)
    pub assign_op: Token,      // ASSIGN_OPERATOR(:=)
    pub start_value: Expression,
    pub direction_kw: Token,   // KEYWORD(ke) atau KEYWORD(turun-ke)
    pub end_value: Expression,
    pub do_kw: Token,          // KEYWORD(lakukan)
    pub body: Box<Statement>,
}

#[derive(Debug)]
pub struct ProcedureCallStatement {
    // Ini sebenarnya adalah <factor> -> FunctionCall
    pub call: FunctionCallNode,
}

#[derive(Debug)]
pub struct RepeatStatement {
    pub repeat_kw: Token, // KEYWORD(ulangi)
    pub statement_list: StatementList,
    pub until_kw: Token,  // KEYWORD(sampai)
    pub condition: Expression,
}

#[derive(Debug)]
pub struct CaseStatement {
    pub case_kw: Token, // KEYWORD(kasus)
    pub expression: Expression,
    pub of_kw: Token,   // KEYWORD(dari)
    pub branches: Vec<CaseBranch>,
    pub else_clause: Option<CaseElseClause>,
    pub end_kw: Token,  // KEYWORD(selesai)
}

#[derive(Debug)]
pub struct CaseBranch {
    pub labels: CaseLabelList,
    pub colon: Token,     // COLON(:)
    pub statement: Box<Statement>,
    pub semicolon: Token, // SEMICOLON(;)
}

#[derive(Debug)]
pub struct CaseLabelList {
    // expression (COMMA expression)*
    pub initial_label: Expression,
    pub rest: Vec<(Token, Expression)>, // (COMMA, Expression)
}

#[derive(Debug)]
pub struct CaseElseClause {
    pub else_kw: Token, // KEYWORD(selain-itu)
    pub statement_list: StatementList,
    // (Tidak ada semicolon setelah statement list 'selain-itu' sebelum 'selesai')
}

// --- Expressions (Hierarki) ---

#[derive(Debug)]
pub struct Expression {
    pub initial_simple_expr: Box<SimpleExpression>,
    // (RELATIONAL_OPERATOR SimpleExpression)*
    pub rest: Vec<(Token, Box<SimpleExpression>)>, 
}

#[derive(Debug)]
pub struct SimpleExpression {
    pub initial_term: Box<Term>,
    pub rest: Vec<(Token, Box<Term>)>, // (+, -, atau)
}

#[derive(Debug)]
pub struct Term {
    pub initial_factor: Box<Factor>,
    // (MULTIPLICATIVE_OPERATOR Factor)*
    pub rest: Vec<(Token, Box<Factor>)>, // (*, /, div, mod, dan)
}

#[derive(Debug)]
pub struct ArithmeticUnaryFactor {
    pub op: Token, // ARITHMETIC_OPERATOR(+ atau -)
    pub factor: Box<Factor>,
}

#[derive(Debug)]
pub enum Factor {
    Literal(LiteralValue),
    Identifier(Token), // IDENTIFIER(x)
    FunctionCall(FunctionCallNode),
    ArrayAccess(ArrayAccess),
    Parenthesized(ParenthesizedExpression),
    Not(NotFactor),
    ArithmeticUnary(ArithmeticUnaryFactor),
}

// --- Expression Components ---

#[derive(Debug)]
pub struct LiteralValue {
    // Wrapper untuk token literal
    // INT_LITERAL, REAL_LITERAL, STRING_LITERAL, CHAR_LITERAL,
    // KEYWORD(benar), KEYWORD(salah)
    pub token: Token,
}

#[derive(Debug)]
pub struct FunctionCallNode {
    pub function_name: Token, // IDENTIFIER(MyFunc)
    pub l_paren: Token,     // LPARENTHESIS(()
    pub arguments: Option<ActualParameterList>, // Bisa kosong: MyFunc()
    pub r_paren: Token,     // RPARENTHESIS())
}

#[derive(Debug)]
pub struct ActualParameterList {
    // expression (COMMA expression)*
    pub initial_arg: Box<Expression>,
    pub rest: Vec<(Token, Box<Expression>)>, // (COMMA, Expression)
}

#[derive(Debug)]
pub struct ArrayAccess {
    pub array: Box<Expression>, // IDENTIFIER(arr) atau FuncCall()
    pub l_bracket: Token,       // LBRACKET([)
    pub index: Box<Expression>,
    pub r_bracket: Token,       // RBRACKET(])
}

#[derive(Debug)]
pub struct ParenthesizedExpression {
    pub l_paren: Token, // LPARENTHESIS(()
    pub expr: Box<Expression>,
    pub r_paren: Token, // RPARENTHESIS())
}

#[derive(Debug)]
pub struct NotFactor {
    pub not_token: Token, // LOGICAL_OPERATOR(tidak)
    pub factor: Box<Factor>,
}