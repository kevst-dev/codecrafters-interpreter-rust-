use super::token_type::TokenType;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<String>,

    #[allow(dead_code)]
    line: u32,
}

#[allow(dead_code)]
impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<String>, line: u32) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    pub fn to_string(&self) -> String {
        let literal = self.literal.clone();
        let text_literal = literal.unwrap_or("null".to_string());

        format!(
            "{} {} {:?}",
            self.token_type,
            self.lexeme,
            text_literal,
        )
    }
}

/*
#[test]
fn test_token_type() {
    println!("{}", TokenType::LeftParen);
    println!("{}", TokenType::RightParen);
    println!("{}", TokenType::Eof);
}

#[test]
fn test_token() {
    let token = Token::new(
        TokenType::LeftParen,
        "(".to_string(),
        None,
        0
    );
    println!("{}", token.to_string());
}
*/
