use super::parser::PascalParser; // Impor struct utama
use super::ast::*;
use crate::lexer::token_types::{Token, TokenType};
use super::error::SyntaxError;

impl PascalParser {
	/// compound-statement -> 'mulai' statement-list 'selesai'
	pub(super) fn parse_compound_statement(&mut self) -> Result<Statement, SyntaxError> {
		self.consume_keyword("mulai", "Mengharapkan 'mulai' (begin).")?;
		let statements = self.parse_statement_list()?;
		self.consume_keyword("selesai", "Mengharapkan 'selesai' (end) untuk menutup blok 'mulai'.")?;
		Ok(Statement::Compound(CompoundStatement { statements }))
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
			condition: condition,
			then_branch: then_branch, 
			else_branch: else_branch
		}))
	}

	fn parse_while_statement(&mut self) -> Result<Statement, SyntaxError> {
		self.consume_keyword("selama", "Expected 'selama'.")?;
		let condition = self.parse_expression()?;
		self.consume_keyword("lakukan", "Expected 'lakukan' after condition.")?;
		let body = Box::new(self.parse_statement()?);

		Ok(Statement::While(WhileStatement {
			condition: condition,
			body: body
		}))
	}

	fn parse_for_statement(&mut self) -> Result<Statement, SyntaxError> {
		self.consume_keyword("untuk", "Expected 'untuk'.")?;
		let counter = self.consume_token(TokenType::Identifier, "Expected control variable.")?.value.clone();
		self.consume_token(TokenType::AssignOperator, "Expected ':=' inside for loop.")?;
		let start = self.parse_expression()?;

		let direction = if self.check_keyword("ke") {
			self.advance();
			ForDirection::To
		} else if self.check_keyword("turun-ke") {
			self.advance();
			ForDirection::DownTo
		} else {
			return Err(self.error("Expected 'ke' or 'turun-ke' inside for loop."));
		};

		let end = self.parse_expression()?;
		self.consume_keyword("lakukan", "Expected 'lakukan' inside for loop.")?;
		
		let body = Box::new(self.parse_statement()?);

		Ok(Statement::For(ForStatement{
			counter_variable: counter,
			start_value: start,
			end_value: end,
			direction: direction,
			body: body
		}))
	}

	fn parse_repeat_statement(&mut self) -> Result<Statement, SyntaxError> {
		//TODO: Implement this
	}

	fn parse_case_statement(&mut self) -> Result<Statement, SyntaxError> {
		//TODO: implement this
	}
}
