use crate::lexer::dfa;
use crate::lexer::token_types::{Token, TokenType};
use std::iter::Peekable;

pub struct PascalLexer<'a> {
    chars: Peekable<std::str::Chars<'a>>,
    dfa: dfa::DfaConfig,
    line: usize,
    column: usize,
    pending_token: Option<Token>,
}

impl<'a> PascalLexer<'a> {
    pub fn new(source: &'a str) -> Self {
        let dfa_config =
            dfa::load_dfa_config("config/dfa.json")
						.expect("Gagal memuat atau mem-parsing dfa.json");

        PascalLexer {
            chars: source.chars().peekable(),
            dfa: dfa_config,
            line: 1,
            column: 1,
            pending_token: None,
        }
    }

    pub fn get_all_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
				// Loop sampai semua karakter habis
        while let Some(token) = self.next_token() {
            let is_eof = token.token_type == TokenType::Eof;
            tokens.push(token);
            if is_eof {
                break;
            }
        }
        tokens
    }

    fn advance(&mut self) -> Option<char> {
        match self.chars.next() {
            Some(char) => {
                if char == '\n' { //Bro gonna use LF
                    self.line += 1;
                    self.column = 1;
                } else {
                    self.column += 1;
                }
                Some(char)
            }
            None => None,
        }
    }

    fn get_char_category(c: char) -> String {
        if c.is_alphabetic() {
            "letter".to_string()
        } else if c.is_digit(10) {
            "digit".to_string()
        } else {
            c.to_string()
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.chars.peek() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn matches_transition_input(input: &str, c: char) -> bool {
        let category = Self::get_char_category(c);
        
        if input == category || input == c.to_string() {
            true
        } else if input == "any_other" && c != '\'' {
            true
        } else {
            false
        }
    }

    fn resolve_token_type(&self, state_name: &str, lexeme: &str) -> TokenType {
        let final_state = self.dfa.states.get(state_name).unwrap();
        let mut token_type_str = final_state.token_type.as_ref().unwrap().clone();

        if token_type_str == "Identifier" {
            if let Some(reserved_type) = self.dfa.reserved_words.get(&lexeme.to_lowercase()) {
                token_type_str = reserved_type.clone();
            }
        }
        
        let token_type = TokenType::from_string(&token_type_str);
        
        token_type
    }

    fn consume_lexeme(&mut self, lexeme: &str) {
        for _ in 0..lexeme.len() {
            self.advance();
        }
    }

    fn create_unknown_token(&mut self, start_line: usize, start_col: usize) -> Token {
        let unknown_char = self.advance().unwrap();
        Token::new(TokenType::Unknown, unknown_char.to_string(), start_line, start_col)
    }

    //
    fn skip_comment_content_and_get_end(&mut self, lexeme: &str) -> Option<Token> {
        if lexeme == "{" {
            // skip sampe '}'
            while let Some(ch) = self.advance() {
                if ch == '}' {
                    return Some(Token::new(
                        TokenType::CommentEnd, 
                        "}".to_string(), 
                        self.line, 
                        self.column - 1
                    ));
                }
            }
        } else if lexeme == "(*" {
            // skip sampe '*)'
            while let Some(ch) = self.advance() {
                if ch == '*' {
                    if let Some(&')') = self.chars.peek() {
                        let end_line = self.line;
                        let end_col = self.column;
                        self.advance(); // ambil ')'
                        return Some(Token::new(
                            TokenType::CommentEnd, 
                            "*)".to_string(), 
                            end_line, 
                            end_col
                        ));
                    }
                }
            }
        }
        None // unclosed comment
    }

    fn next_token(&mut self) -> Option<Token> {
        if let Some(token) = self.pending_token.take() {
            return Some(token);
        }

        self.skip_whitespace();

        if self.chars.peek().is_none() {
            return Some(Token::new(TokenType::Eof, "".to_string(), self.line, self.column));
        }

        let start_line = self.line;
        let start_col = self.column;

        if let Some((final_state_name, lexeme)) = self.dfa.dfa(
            &mut self.chars,
            Self::get_char_category,
            Self::matches_transition_input
        ) {
            let token_type = self.resolve_token_type(&final_state_name, &lexeme);
            
            self.consume_lexeme(&lexeme);
            
            if token_type == TokenType::CommentStart {
                let comment_start_token = Token::new(token_type, lexeme.clone(), start_line, start_col);
                
                if let Some(comment_end_token) = self.skip_comment_content_and_get_end(&lexeme) {
                    self.pending_token = Some(comment_end_token);
                }
                
                return Some(comment_start_token);
            }
            
            return Some(Token::new(token_type, lexeme, start_line, start_col));
        } else {
            return Some(self.create_unknown_token(start_line, start_col));
        }
    }
}
