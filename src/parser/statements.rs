use super::parser::PascalParser; // Impor struct utama
use super::ast::*;
use crate::lexer::token_types::{TokenType};
use super::error::SyntaxError;

impl PascalParser {
	/// compound-statement -> 'mulai' statement-list 'selesai'
	pub(super) fn parse_compound_statement(&mut self) -> Result<Statement, SyntaxError> {
		self.consume_keyword("mulai", "Mengharapkan 'mulai' (begin).")?;
		let statements = self.parse_statement_list(|parser| parser.check_keyword("selesai"))?;
		self.consume_keyword("selesai", "Mengharapkan 'selesai' (end) untuk menutup blok 'mulai'.")?;
		Ok(Statement::Compound(CompoundStatement { statements }))
	}

    /// statement-list -> (statement (';' statement)* )?
	fn parse_statement_list<F>(
		&mut self, 
		mut is_terminator: F
	,) -> Result<Vec<Statement>, SyntaxError>
	where F: FnMut(&mut Self) -> bool, {
		let mut statements = Vec::new();

		if is_terminator(self) {
			return Ok(statements);
		}

		statements.push(self.parse_statement()?);

		while self.match_token(&[TokenType::Semicolon]) {
			if is_terminator(self) {
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
		} else if self.check(TokenType::Identifier) {
            self.parse_assignment_or_call()
        } else {
			Err(self.error("Mengharapkan pernyataan"))
		}
    }

	fn parse_if_statement(&mut self) -> Result<Statement, SyntaxError> {
		self.consume_keyword("jika", "Mengharapkan 'jika'.")?;
		let condition = self.parse_expression()?;
		self.consume_keyword("maka", "Mengharapkan 'maka' setelah kondisi.")?;
		let then_branch = Box::new(self.parse_statement()?);

		let else_branch = if self.check_keyword("selain_itu") {
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
		self.consume_keyword("selama", "Mengharapkan 'selama'.")?;
		let condition = self.parse_expression()?;
		self.consume_keyword("lakukan", "Mengharapkan 'lakukan' setelah kondisi.")?;
		let body = Box::new(self.parse_statement()?);

		Ok(Statement::While(WhileStatement {
			condition: condition,
			body: body
		}))
	}

	fn parse_for_statement(&mut self) -> Result<Statement, SyntaxError> {
		self.consume_keyword("untuk", "Mengharapkan 'untuk'.")?;
		let counter = self.consume_token(TokenType::Identifier, "Mengharapkan variabel kontrol.")?.value.clone();
		self.consume_token(TokenType::AssignOperator, "Mengharapkan ':=' dalam pengulangan 'untuk'.")?;
		let start = self.parse_expression()?;

		let direction = if self.check_keyword("ke") {
			self.advance();
			ForDirection::To
		} else if self.check_keyword("turun_ke") {
			self.advance();
			ForDirection::DownTo
		} else {
			return Err(self.error("Mengharapkan 'ke' or 'turun_ke' dalam pengulangan 'untuk'."));
		};

		let end = self.parse_expression()?;
		self.consume_keyword("lakukan", "Mengharapkan 'lakukan' dalam pengulangan 'untuk'.")?;
		
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
		self.consume_keyword("ulangi", "Mengharapkan 'ulangi'.")?;
		let statements = self.parse_statement_list(|parser| parser.check_keyword("sampai"))?;
		self.consume_keyword("sampai", "Mengharapkan 'sampai' dalam 'ulangi-sampai'.")?;
		let condition = self.parse_expression()?;

		Ok(Statement::Repeat(RepeatStatement{
			statements: statements,
			condition: condition
		}))
	}

	fn parse_assignment_or_call(&mut self) -> Result<Statement, SyntaxError> {
		let var = self.parse_expression()?;

		if self.check(TokenType::AssignOperator) {
			self.advance();
			let expr = self.parse_expression()?;

			Ok(Statement::Assignment(AssignmentStatement{ 
				variable: var, 
				expression: expr
			}))
		} else if self.check(TokenType::LParenthesis) {
			let Expression::Identifier(name) = var else {
				return Err(self.error("Mengharapkan nama prosedur (pengenal)."))
			};
			self.advance();
			let args = self.parse_argument_list()?;

			if name == "readln" {
				Ok(Statement::Read(ReadStatement{
					variables: args
				}))
			} else if name == "writeln" {
				Ok(Statement::Write(WriteStatement{
					expressions: args
				}))
			} else {
				Ok(Statement::ProcedureCall(ProcedureCallStatement{
					procedure_name: name,
					arguments: args
				}))
			}
		} else {
            // If neither := nor (, treat as expression statement
            Ok(Statement::ExpressionStatement(var))
        }
	}

	pub(super) fn parse_argument_list(&mut self) -> Result<Vec<Expression>, SyntaxError> {
		let mut args = Vec::new();

		if !self.check(TokenType::RParenthesis) {
			args.push(self.parse_expression()?);
			while self.check(TokenType::Comma) {
				self.advance();
				args.push(self.parse_expression()?);
			}
		}

		Ok(args)
	}

	fn parse_case_statement(&mut self) -> Result<Statement, SyntaxError> {
		self.consume_keyword("kasus", "Mengharapkan 'kasus'.")?;
		let expr = self.parse_expression()?;
		self.consume_keyword("dari", "Mengharapkan 'dari' setelah ekspresi kasus.")?;

		let mut branches = Vec::new();
		while !self.check_keyword("selain_itu") && !self.check_keyword("selesai") {
			branches.push(self.parse_case_branch()?);
		}

		let else_branch = if self.check_keyword("selain_itu") {
			self.advance();
			let mut statements = Vec::new();
			while !self.check_keyword("selesai") {
				statements.push(self.parse_statement()?);
				if self.check(TokenType::Semicolon) {
					self.advance();
				}	
			}
			Some(statements) 
			} else {
				None
		};
		self.consume_keyword("selesai", "Mengharapkan 'selesai' pada akhir pernyataan kasus.")?;

		Ok(Statement::Case(CaseStatement{
			expression: expr,
			branches: branches,
			else_branch: else_branch
		}))
	}
	
	fn parse_case_branch(&mut self) -> Result<CaseBranch, SyntaxError> {
		let mut labels = vec![self.parse_expression()?];
		while self.check(TokenType::Comma) {
			self.advance();
			labels.push(self.parse_expression()?);
		}

		self.consume_token(TokenType::Colon, "Mengharapkan ':' setelah label kasus.")?;
		let statement = self.parse_statement()?;
		self.consume_token(TokenType::Semicolon, "Mengharapkan ';' setelah pernyataan kasus.")?;

		Ok(CaseBranch {
			labels: labels,
			statement: statement
		})
	}
}
