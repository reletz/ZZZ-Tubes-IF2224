use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TokenType {
    // Keywords (22 total)
    Keyword,
    
    // Identifiers
    Identifier,
    
    // Operators
    ArithmeticOperator,
    RelationalOperator,
    LogicalOperator,
    AssignOperator,
    RangeOperator,
    
    // Literals
    Number,
    CharLiteral,
    StringLiteral,
    IntegerLiteral,
    RealLiteral,
    
    // Punctuation
    Semicolon,
    Comma,
    Colon,
    Dot,
    LParenthesis,
    RParenthesis,
    LBracket,
    RBracket,
    
    // Comments
    CommentStart,
    CommentEnd,
    
    // Special tokens
    Eof,
    Unknown,
}

impl TokenType {
    pub fn from_string(s: &str) -> TokenType {
        match s {
            "Identifier" => TokenType::Identifier,
            "IntegerLiteral" => TokenType::IntegerLiteral,
            "RealLiteral" => TokenType::RealLiteral,
            "StringLiteral" => TokenType::StringLiteral,
            "CharLiteral" => TokenType::CharLiteral,
            "Semicolon" => TokenType::Semicolon,
            "Comma" => TokenType::Comma,
            "LParenthesis" => TokenType::LParenthesis,
            "RParenthesis" => TokenType::RParenthesis,
            "LBracket" => TokenType::LBracket,
            "RBracket" => TokenType::RBracket,
            "ArithmeticOperator" => TokenType::ArithmeticOperator,
            "RelationalOperator" => TokenType::RelationalOperator,
            "LogicalOperator" => TokenType::LogicalOperator,
            "AssignOperator" => TokenType::AssignOperator,
            "RangeOperator" => TokenType::RangeOperator,
            "Colon" => TokenType::Colon,
            "Dot" => TokenType::Dot,
            "Keyword" => TokenType::Keyword,
            "CommentStart" => TokenType::CommentStart,
            "CommentEnd" => TokenType::CommentEnd,
            _ => TokenType::Unknown,
        }
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::Keyword => write!(f, "KEYWORD"),
            TokenType::Identifier => write!(f, "IDENTIFIER"),
            TokenType::ArithmeticOperator => write!(f, "ARITHMETIC_OPERATOR"),
            TokenType::RelationalOperator => write!(f, "RELATIONAL_OPERATOR"),
            TokenType::LogicalOperator => write!(f, "LOGICAL_OPERATOR"),
            TokenType::AssignOperator => write!(f, "ASSIGN_OPERATOR"),
            TokenType::RangeOperator => write!(f, "RANGE_OPERATOR"),
            TokenType::Number => write!(f, "NUMBER"),
            TokenType::CharLiteral => write!(f, "CHAR_LITERAL"),
            TokenType::StringLiteral => write!(f, "STRING_LITERAL"),
            TokenType::IntegerLiteral => write!(f, "INT_LITERAL"),
            TokenType::RealLiteral => write!(f, "REAL_LITERAL"),
            TokenType::Semicolon => write!(f, "SEMICOLON"),
            TokenType::Comma => write!(f, "COMMA"),
            TokenType::Colon => write!(f, "COLON"),
            TokenType::Dot => write!(f, "DOT"),
            TokenType::LParenthesis => write!(f, "LPARENTHESIS"),
            TokenType::RParenthesis => write!(f, "RPARENTHESIS"),
            TokenType::LBracket => write!(f, "LBRACKET"),
            TokenType::RBracket => write!(f, "RBRACKET"),
            TokenType::CommentStart => write!(f, "COMMENT_START"),
            TokenType::CommentEnd => write!(f, "COMMENT_END"),
            TokenType::Eof => write!(f, "EOF"),
            TokenType::Unknown => write!(f, "UNKNOWN"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(token_type: TokenType, value: String, line: usize, column: usize) -> Self {
        Token {
            token_type,
            value,
            line,
            column,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}({})", self.token_type, self.value)
    }
}