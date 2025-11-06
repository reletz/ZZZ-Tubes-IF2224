use super::parser::PascalParser; // Impor struct utama
use super::ast::*;
use crate::lexer::token_types::{Token, TokenType};
use super::error::SyntaxError;

impl PascalParser {
	/// compound-statement -> 'mulai' statement-list 'selesai'
	pub(super) fn parse_compound_statement(&mut self) -> Result<CompoundStatement, SyntaxError> {
		self.consume_keyword("mulai", "Mengharapkan 'mulai' (begin).")?;
		let statements = self.parse_statement_list()?;
		self.consume_keyword("selesai", "Mengharapkan 'selesai' (end) untuk menutup blok 'mulai'.")?;
		Ok(CompoundStatement { statements })
	}

    /// statement-list -> (statement (';' statement)* )?
	fn parse_statement_list(&mut self) -> Result<Vec<Statement>, SyntaxError> {
		let mut statements = Vec::new();

		if self.check_keyword("selesai") {
			return Ok(statements);
		}

		statements.push(self.parse_statement()?);

		while self.match_token(&[TokenType::Semicolon]) {
			if self.check_keyword("selesai") {
				break;
			}
			statements.push(self.parse_statement()?);
		}

		Ok(statements)
	}

	/// Placeholder
	pub(super) fn parse_statement(&mut self) -> Result<Statement, SyntaxError> {
		// TODO: Implementasi ini
		// if self.check_keyword("jika") { self.parse_if_statement() } ...
		
		// For now, treat any expression as a statement (incomplete, but works for testing)
        if self.check(TokenType::Identifier) {
            let expr = self.parse_expression()?;
            return Ok(Statement::ExpressionStatement(expr));
        }
        
        Err(self.error("Expected statement"))
    }
}
