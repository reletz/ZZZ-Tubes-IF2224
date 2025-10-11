use serde::{Deserialize, Serialize};
use std::collections::HashSet;
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

// Pascal-S Keywords
lazy_static::lazy_static! {
    pub static ref KEYWORDS: HashSet<&'static str> = {
        let mut set = HashSet::new();
        set.insert("program");
        set.insert("var");
        set.insert("begin");
        set.insert("end");
        set.insert("if");
        set.insert("then");
        set.insert("else");
        set.insert("while");
        set.insert("do");
        set.insert("for");
        set.insert("to");
        set.insert("downto");
        set.insert("integer");
        set.insert("real");
        set.insert("boolean");
        set.insert("char");
        set.insert("array");
        set.insert("of");
        set.insert("procedure");
        set.insert("function");
        set.insert("const");
        set.insert("type");
        set
    };
    
    pub static ref ARITHMETIC_OPERATORS: HashSet<&'static str> = {
        let mut set = HashSet::new();
        set.insert("+");
        set.insert("-");
        set.insert("*");
        set.insert("/");
        set.insert("div");
        set.insert("mod");
        set
    };
    
    pub static ref RELATIONAL_OPERATORS: HashSet<&'static str> = {
        let mut set = HashSet::new();
        set.insert("=");
        set.insert("<>");
        set.insert("<");
        set.insert("<=");
        set.insert(">");
        set.insert(">=");
        set
    };
    
    pub static ref LOGICAL_OPERATORS: HashSet<&'static str> = {
        let mut set = HashSet::new();
        set.insert("and");
        set.insert("or");
        set.insert("not");
        set
    };
}

// Helper functions
pub fn is_keyword(word: &str) -> bool {
    KEYWORDS.contains(&word.to_lowercase().as_str())
}