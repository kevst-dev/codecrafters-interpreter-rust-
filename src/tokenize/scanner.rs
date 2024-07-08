use crate::tokenize::{Token, TokenType, TokenizerError};

#[derive(Debug)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    token_errors: Vec<TokenizerError>,

    start: u32,
    current: u32,
    line: u32,


}

#[allow(dead_code)]
impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            token_errors: Vec::new(),

            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(
            Token::new(
                TokenType::Eof,
                "".to_string(),
                None,
                self.line
            )
        );

        self.tokens.clone()
    }

    pub fn token_errors(&self) -> Vec<TokenizerError> {
        self.token_errors.clone()
    }


    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() as u32
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {

            // Single-character tokens

            '(' => self.add_token(TokenType::LeftParen, None),
            ')' => self.add_token(TokenType::RightParen, None),
            '{' => self.add_token(TokenType::LeftBrace, None),
            '}' => self.add_token(TokenType::RightBrace, None),
            ',' => self.add_token(TokenType::Comma, None),
            '.' => self.add_token(TokenType::Dot, None),
            '-' => self.add_token(TokenType::Minus, None),
            '+' => self.add_token(TokenType::Plus, None),
            ';' => self.add_token(TokenType::Semicolon, None),
            '*' => self.add_token(TokenType::Star, None),

            // Operator tokens

            '!' => {
                let token_type = if self.match_char('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token_type, None);
            }
            '=' => {
                let token_type = if self.match_char('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token_type, None);
            }
            '<' => {
                let token_type = if self.match_char('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token_type, None);
            }
            '>' => {
                let token_type = if self.match_char('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token_type, None);
            }

            '\n' => self.line += 1,
            ' ' | '\r' | '\t' => { /* ignore whitespace */ },
            _ => {
                self.add_token_error(
                    format!("Unexpected character: {}", c)
                );
            }
        }
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current as usize).unwrap();
        self.current += 1;

        c
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<String>) {
        let text = self.source[self.start as usize..self.current as usize].to_string();

        self.tokens.push(
            Token::new(
                token_type,
                text,
                literal,
                self.line
            )
        );
    }

    fn add_token_error(&mut self, message: String) {
        self.token_errors.push(
            TokenizerError::new(
                self.line,
                message
            )
        );
    }

    fn match_char(&mut self, expected: char) -> bool {
        let index = self.current as usize;
        let c = self.source[index..index + 1].to_string();

        if self.is_at_end() { return false; }
        if c != expected.to_string() { return false; }

        self.current += 1;
        true
    }
}
