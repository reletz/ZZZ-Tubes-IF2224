use super::ast::*;
use crate::lexer::token_types::{Token, TokenType};
use super::error::SyntaxError;

pub struct PascalParser {
	tokens: Vec<Token>,
	current: usize, 
}

impl PascalParser {
	pub fn new(tokens: Vec<Token>) -> Self {
		PascalParser {
			tokens,
			current: 0,
		}
	}

	fn peek(&self) -> &Token {
    if self.current >= self.tokens.len() {
      return &self.tokens[self.tokens.len() - 1]; // Harusnya token EOF
    }
    &self.tokens[self.current]
	}

	fn advance(&mut self) -> &Token {
		if !(self.is_at_end()) {
			self.current += 1;
		} &self.tokens[self.current - 1]
	}

	fn is_at_end(&self) -> bool {
    self.peek().token_type == TokenType::Eof
	}

	fn check(&self, token_type: TokenType) -> bool {
    if self.is_at_end() {
      return false;
    }
    self.peek().token_type == token_type
	}

	fn match_token(&mut self, types: &[TokenType]) -> bool {
    for token_type in types {
			if self.check(*token_type) {
				self.advance();
				return true;
			}
    }
    false
	}

	fn error(&self, message: &str) -> SyntaxError {
		let token = self.peek();
		SyntaxError::new(message.to_string(), token.line, token.column)
	}

	fn check_keyword(&self, value: &str) -> bool {
		if !self.check(TokenType::Keyword) {
			return false;
		}
		self.peek().value.to_lowercase() == value
	}

  fn match_keyword(&mut self, values: &[&str]) -> bool {
		for &value in values {
			if self.check_keyword(value) {
				self.advance();
				return true;
			}
		} false
  }

	fn consume_keyword(&mut self, value: &str, message: &str) -> Result<&Token, SyntaxError> {
		if self.check_keyword(value) {
			Ok(self.advance())
		} else {
			Err(self.error(message))
		}
	}

	fn consume_token(&mut self, token_type: TokenType, message: &str) -> Result<&Token, SyntaxError> {
		if self.check(token_type) {
			Ok(self.advance())
		} else {
			Err(self.error(message))
		}
	}

	pub fn parse(&mut self) -> Result<Program, SyntaxError> {
		self.parse_program()
	}

	// program -> program-header + declaration-part + compound-statement + DOT
	fn parse_program(&mut self) -> Result<Program, SyntaxError> {
		// 1. Parse program-header
		self.consume_keyword("program", "Mengharapkan keyword 'program'.")?; // "program"
		let program_name = self.consume_token(TokenType::Identifier, "Mengharapkan nama program.")?.value.clone();
		self.consume_token(TokenType::Semicolon, "Mengharapkan ';' setelah nama program.")?;

		// 2. Parse declaration-part
		let declarations = self.parse_declaration_part()?;

		// 3. Parse compound-statement
		let body = self.parse_compound_statement()?;

		// 4. Parse DOT
		self.consume_token(TokenType::Dot, "Mengharapkan '.' di akhir program.")?;

		// 5. Selesai! Kembalikan AST Node.
		Ok(Program {
			name: program_name,
			declarations: declarations,
			body: body,
		})
	}

	fn parse_declaration_part(&mut self) -> Result<Vec<Declaration>, SyntaxError> {
		// TODO: Implementasi parsing 'var', 'const', 'type'
		// Ini akan looping selama token-nya 'var', 'const', 'type'
		Ok(Vec::new())
	}

	fn parse_compound_statement(&mut self) -> Result<CompoundStatement, SyntaxError> {
		// TODO: Implementasi parsing 'mulai' ... 'selesai'
		// 1. consume 'mulai'
		// 2. panggil parse_statement_list()
		// 3. consume 'selesai'
		Ok(CompoundStatement { statements: Vec::new() })
	}
}