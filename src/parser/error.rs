use std::fmt;

pub struct SyntaxError{
	pub message: String,
	pub line: usize,
  pub column: usize,
}

impl SyntaxError {
	pub fn new(message: impl Into<String>, line: usize, column: usize) -> Self {
		SyntaxError {
			message: message.into(),
			line,
			column,
		}
	}
}

impl fmt::Display for SyntaxError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "SyntaxError at {}:{}: {}", self.line, self.column, self.message)
	}
}