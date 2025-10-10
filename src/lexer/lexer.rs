use crate::lexer::token_types::{Token, TokenType};

pub struct PascalLexer<'a> {
    source: &'a str,                                    // String yang diproses
    chars: std::iter::Peekable<std::str::Chars<'a>>,    // Char sekarang (yang udah jadi iterator)
    current_pos: usize,                                 // Pointer ke char di string
    line: usize,
    column: usize,
}

impl<'a> PascalLexer<'a> {
    pub fn new(source: &'a str) -> Self {
        PascalLexer {
            source,
            chars: source.chars().peekable(),
            current_pos: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn get_all_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        // Loop sampai semua karakter habis
        while let Some(token) = self.next_token() {
            if token.token_type == TokenType::Eof {
                tokens.push(token);
                break;
            }
            tokens.push(token);
        }
        tokens
    }

    fn advance(&mut self) -> Option<char> {
        let char = self.chars.next()?;
        self.current_pos += 1;
        if char == '\n'{ //Bro gonna use LF
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        Some(char)
    }

    fn read_identifier_or_keyword(&mut self, first_char: char, start_col: usize) -> Token {
        let mut value = String::new();
        value.push(first_char);

        while let Some(&next_c) = self.chars.peek() {
            if next_c.is_alphanumeric() {
                value.push(self.advance().unwrap());
            } else {
                break;
            }
        }

        if crate::lexer::token_types::is_keyword(&value) {
            Token::new(TokenType::Keyword, value, self.line, start_col)
        } else {
            Token::new(TokenType::Identifier, value, self.line, start_col)
        }
    }

    fn read_number(&mut self, first_char: char, start_col: usize) -> Token {
        let mut value = String::new();
        value.push(first_char);

        while let Some(&next_c) = self.chars.peek(){
            if next_c.is_digit(10){
                value.push(self.advance().unwrap());
            } else {
                break;
            }
        }

        Token::new(TokenType::IntegerLiteral, value, self.line, start_col)
    }

    // Fungsi untuk mendapatkan token berikutnya
    fn next_token(&mut self) -> Option<Token> {
        while let Some(&c) = self.chars.peek(){
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }

        let start_char = match self.advance() {
            Some(c) => c,
            None => return Some(Token::new(TokenType::Eof, "".to_string(), self.line, self.column)),
        };

        let start_col = self.column - 1;

        let token = match start_char {
            ';' => Token::new(TokenType::Semicolon, ";".to_string(), self.line, start_col),
            '.' => Token::new(TokenType::Dot, ".".to_string(), self.line, start_col),
            '+' => Token::new(TokenType::ArithmeticOperator, "+".to_string(), self.line, start_col),
            '(' => Token::new(TokenType::LParenthesis, "(".to_string(), self.line, start_col),
            ')' => Token::new(TokenType::RParenthesis, ")".to_string(), self.line, start_col),
            ':' => {
                if let Some('=') = self.chars.peek() {
                    self.advance();
                    Token::new(TokenType::AssignOperator, ":=".to_string(), self.line, start_col)
                } else {
                    Token::new(TokenType::Colon, ":".to_string(), self.line, start_col)
                }
            },

            c if c.is_alphabetic() => {
                let mut value = String::new();
                value.push(c); 
                while let Some(&next_c) = self.chars.peek() {
                    if next_c.is_alphanumeric() {
                        value.push(self.advance().unwrap());
                    } else {
                        break;
                    }
                }
                
                if crate::lexer::token_types::is_keyword(&value) {
                    Token::new(TokenType::Keyword, value, self.line, start_col)
                } else {
                    Token::new(TokenType::Identifier, value, self.line, start_col)
                }
            },

            c if c.is_alphanumeric() => self.read_identifier_or_keyword(c, start_col),
            c if c.is_digit(10) => self.read_number(c, start_col),

            _ => Token::new(TokenType::Unknown, start_char.to_string(), self.line, start_col),
        };

        Some(token)
    }
}
