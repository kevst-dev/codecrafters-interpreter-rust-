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
            '/' => {
                if self.match_char('/') {
                    // A comment goes until the end of the line.
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, None);
                }
            }

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

            ' ' | '\r' | '\t' => { /* ignore whitespace */ },
            '\n' => self.line += 1,

            // Literals

            '"' => self.string(),
            '0'..='9' => self.number(),
            'a'..='z' | 'A'..='Z' | '_' => self.identifier(),

            _ => {
                self.add_token_error(
                    format!("Unexpected character: {}", c)
                );
            }
        }
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current as usize).unwrap_or('\0');
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

    fn add_token_number(&mut self, token_type: TokenType, literal: Option<f64>) {
        let text = self.source[self.start as usize..self.current as usize].to_string();

        self.tokens.push(
            Token::new_number(
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

        if self.is_at_end() { return false; }

        let c = self.source[index..index + 1].to_string();
        if c != expected.to_string() { return false; }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() { return '\0'; }

        self.source.chars().nth(self.current as usize).unwrap_or('\0')
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() as u32 { return '\0'; }

        self.source.chars().nth((self.current + 1) as usize).unwrap_or('\0')
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' { self.line += 1; }

            self.advance();
        }

        if self.is_at_end() {
            self.add_token_error(
                "Unterminated string.".to_string()
            );

            return;
        }

        // The closing ".
        self.advance();

        // Trim the surrounding quotes.
        let value = self.source[
            (self.start as usize + 1)..(self.current as usize - 1)
        ].to_string();
        self.add_token(TokenType::String, Some(value));
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // Look for a fractional part.
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            // Consume the "."
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let value = self.source[
            (self.start as usize)..(self.current as usize)
        ].to_string();
        let float_value = value.parse::<f64>().unwrap();
        self.add_token_number(TokenType::Number, Some(float_value));
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        self.add_token(TokenType::Identifier, None);
    }
}
