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
		
		if self.check_keyword("jika") {
			self.parse_if_statement()
		} else if self.check_keyword("selama") {
			self.parse_while_statement()
		} else if self.check_keyword("untuk") {
			self.parse_for_statement()
		} else if self.check_keyword("ulangi") {
			self.parse_repeat_statement()
		} else if self.check_keyword("kasus") {
			self.parse_case_statement()
		} else if self.check_keyword("mulai") {
			self.parse_compound_statement()
		} else if self.check(TokenType::Identifier) { // For now, treat any expression as a statement (incomplete, but works for testing)
            let expr = self.parse_expression()?;
            return Ok(Statement::ExpressionStatement(expr));
        } else {
			Err(self.error("Expected statement"))
		}
    }

	fn parse_if_statement(&mut self) -> Result<Statement, SyntaxError> {
		self.consume_keyword("jika", "Expected 'jika'.")?;
		let condition = self.parse_expression()?;
		self.consume_keyword("maka", "Expected 'maka' after condition.")?;
		let then_branch = Box::new(self.parse_statement()?);

		let else_branch = if self.check_keyword("selain-itu") {
			self.advance();
			Some(Box::new(self.parse_statement()?))
		} else {
			None
		};

		Ok(Statement::If(IfStatement {
			condition,
			then_branch, 
			else_branch
		}))
	}

	fn parse_while_statement(&mut self) -> Result<Statement, SyntaxError> {
		self.consume_keyword("selama", "Expected 'selama'.")?;
		let condition = self.parse_expression()?;
		self.consume_keyword("lakukan", "Expected 'lakukan' after condition.")?;
		let body = Box::new(self.parse_statement()?);

		Ok(Statement::While(WhileStatement {
			condition,
			body
		}))
	}

	fn 
}
