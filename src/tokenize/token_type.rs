
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Single-character tokens

    LeftParen,   // is '('
    RightParen,  // is ')'
    LeftBrace,   // is '{'
    RightBrace,  // is '}' 
    Comma,       // is ','
    Dot,         // is '.'
    Minus,       // is '-'
    Plus,        // is '+'
    Semicolon,   // is ';'
    Slash,       // is '/'
    Star,        // is '*'

    // Operator tokens

    Bang,           // is '!'
    BangEqual,      // is '!='
    Equal,          // is '='
    EqualEqual,     // is '=='
    Greater,        // is '>'
    GreaterEqual,   // is '>='
    Less,           // is '<'
    LessEqual,      // is '<='

    // Literals.

    /*
    IDENTIFIER,
    STRING,
    NUMBER,
    */

    // Keywords.

    /*
    AND,     // is 'and',
    CLASS,   // is 'class',
    ELSE,    // is 'else',
    FALSE,   // is 'false',
    FUN,     // is 'fun',
    FOR,     // is 'for',
    IF,      // is 'if',
    NIL,     // is 'nil' value,
    OR,      // is 'or',
    PRINT,   // is 'print',
    RETURN,  // is 'return',
    SUPER,   // is 'super',
    THIS,    // is 'this',
    TRUE,    // is 'true',
    VAR,     // is 'var',
    WHILE,   // is 'while',
    */

    Eof // is end of file
}

use std::fmt;

impl fmt::Display for TokenType {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::LeftParen => write!(f, "LEFT_PAREN"),
            TokenType::RightParen => write!(f, "RIGHT_PAREN"),
            TokenType::LeftBrace => write!(f, "LEFT_BRACE"),
            TokenType::RightBrace => write!(f, "RIGHT_BRACE"),
            TokenType::Comma => write!(f, "COMMA"),
            TokenType::Dot => write!(f, "DOT"),
            TokenType::Minus => write!(f, "MINUS"),
            TokenType::Plus => write!(f, "PLUS"),
            TokenType::Semicolon => write!(f, "SEMICOLON"),
            TokenType::Slash => write!(f, "SLASH"),
            TokenType::Star => write!(f, "STAR"),

            TokenType::Bang => write!(f, "BANG"),
            TokenType::BangEqual => write!(f, "BANG_EQUAL"),
            TokenType::Equal => write!(f, "EQUAL"),
            TokenType::EqualEqual => write!(f, "EQUAL_EQUAL"),
            TokenType::Greater => write!(f, "GREATER"),
            TokenType::GreaterEqual => write!(f, "GREATER_EQUAL"),
            TokenType::Less => write!(f, "LESS"),
            TokenType::LessEqual => write!(f, "LESS_EQUAL"),

            TokenType::Eof => write!(f, "EOF"),
        }
    }
}
