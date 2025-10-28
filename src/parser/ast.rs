#[derive(Debug)]
pub struct Program {
	pub name: String, // Nama dari 'program <name>;'
	pub declarations: Vec<Declaration>,
	pub body: CompoundStatement, // Blok 'begin ... end.'
}

// === DEKLARASI ===
#[derive(Debug)]
pub enum Declaration {
	Variable(VariableDeclaration),
	// Procedure(ProcedureDeclaration),
	// Function(FunctionDeclaration),
}

#[derive(Debug)]
pub struct VariableDeclaration {
	// Satu blok 'var' bisa berisi beberapa grup var
	//   x, y: integer;
	//   z: real;
	pub groups: Vec<VariableGroup>,
}

// 'ident1, ident2: type;'
#[derive(Debug)]
pub struct VariableGroup {
	pub identifiers: Vec<String>,
	pub var_type: Type,
}

// tipe data PASCAL-S
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
	Integer,
	Real,
	Boolean,
	String,
	Char,
	Array(Box<ArrayTypeDefinition>)
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayTypeDefinition {
	pub range_start: Expression,
	pub range_end: Expression,

	pub base_type: Box<Type>,
}

// === STATEMENT ===
#[derive(Debug)]
pub enum Statement {
		/// 'begin' ... 'end'
	Compound(CompoundStatement),
	/// 'variable := expression'
	Assignment(AssignmentStatement),
	/// 'if condition then ... else ...'
	If(IfStatement),
	/// 'while condition do ...'
	While(WhileStatement),
	/// 'for ... to/downto ... do ...'
	For(ForStatement),
	/// 'readln(var1, var2)'
	Read(ReadStatement),
	/// 'writeln(expr1, "hello")'
	Write(WriteStatement),
	/// 'MyProcedure(arg1, arg2)'
	ProcedureCall(ProcedureCallStatement),
	/// Statement kosong, misal karena ';;'
	Empty,
}

// 'begin' ... 'end'
#[derive(Debug)]
pub struct CompoundStatement {
	pub statements: Vec<Statement>,
}

// 'variable := expression'
#[derive(Debug)]
pub struct AssignmentStatement {
	pub variable: String, // Nama variabel di kiri
	pub expression: Expression,
}

// 'if condition then then_branch else else_branch'
#[derive(Debug)]
pub struct IfStatement {
	pub condition: Expression,
	// Kita pakai Box<Statement> karena Statement adalah enum rekursif
	// (misal, IfStatement bisa berisi CompoundStatement, yang berisi IfStatement lain)
	// Box<> menempatkan data di heap, jadi ukurannya diketahui saat kompilasi.
	pub then_branch: Box<Statement>,
	pub else_branch: Option<Box<Statement>>,
}

// 'while condition do body'
#[derive(Debug)]
pub struct WhileStatement {
	pub condition: Expression,
	pub body: Box<Statement>,
}

// 'for counter := start to/downto end do body'
#[derive(Debug)]
pub struct ForStatement {
	pub counter_variable: String,
	pub start_value: Expression,
	pub end_value: Expression,
	pub direction: ForDirection,
	pub body: Box<Statement>,
}

#[derive(Debug, Clone, Copy)]
pub enum ForDirection {
	To,
	DownTo,
}

// 'readln(var1, var2, ...)'
#[derive(Debug)]
pub struct ReadStatement {
	pub variables: Vec<String>,
}

// 'writeln(expr1, "hello", ...)'
#[derive(Debug)]
pub struct WriteStatement {
	pub expressions: Vec<Expression>,
}

// 'MyProcedure(arg1, arg2, ...)'
#[derive(Debug)]
pub struct ProcedureCallStatement {
	pub procedure_name: String,
	pub arguments: Vec<Expression>,
}

// === EXPRESSION ===
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
	/// 5, 3.14, "hello", true
	Literal(LiteralValue),
	/// 'x', 'myVariable'
	Variable(String),
	/// 'a + b', 'c > 10'
	Binary(BinaryExpression),
	/// '-x', 'not y'
	Unary(UnaryExpression),
	/// 'MyFunction(arg1, arg2)'
	FunctionCall(FunctionCallExpression),
	/// '(a + b)'
	Grouped(Box<Expression>),
}

#[derive(Debug, Clone, PartialEq)]

pub enum LiteralValue {
	Integer(i64),
	Real(f64),
	String(String),
	Boolean(bool),
	Char(char)
}

// 'left operator right'
#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpression {
	pub left: Box<Expression>,
	pub operator: BinaryOperator,
	pub right: Box<Expression>,
}

// 'operator operand'
#[derive(Debug, Clone, PartialEq)]
pub struct UnaryExpression {
	pub operator: UnaryOperator,
	pub operand: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCallExpression {
	pub function_name: String,
	pub arguments: Vec<Expression>,
}

// === OPERATOR ===
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BinaryOperator {
	Plus,       // +
	Minus,      // -
	Mult,       // *
	RealDiv,    // /
	IntDiv,     // div
	Mod,        // mod

	Eq,					// =
	Neq,				// <>
	Lt,					// <
	Le,					// <=
	Gt,					// >
	Ge,					// >=

	And,				// and
	Or					// or
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum UnaryOperator {
	Plus,
	Minus,
	Not
}
