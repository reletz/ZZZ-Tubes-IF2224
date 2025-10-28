use super::parser::PascalParser;
use super::ast::*;
use crate::lexer::token_types::{Token, TokenType};
use super::error::SyntaxError;

impl PascalParser {
	/// Placeholder untuk parsing ekspresi
	pub(super) fn parse_expression(&mut self) -> Result<Expression, SyntaxError> {
		// TODO: Implementasi ini (Pratt Parser atau Operator Precedence)
		println!("Not implemented yet. Returned Literal Integer 0.");

		// Kembalikan nilai dummy
		Ok(Expression::Literal(LiteralValue::Integer(0)))
	}
}
