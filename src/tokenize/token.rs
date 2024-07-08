use super::token_type::TokenType;

#[derive(Debug, Clone, PartialEq)]
enum LiteralType {
    String(String),
    Number(f64),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<LiteralType>,

    #[allow(dead_code)]
    line: u32,
}

#[allow(dead_code)]
impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<String>, line: u32) -> Self {
        let literal = match literal {
            Some(literal) => Some(
                LiteralType::String(literal),
            ),
            None => None
        };

        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    pub fn new_number(token_type: TokenType, lexeme: String, literal: Option<f64>, line: u32) -> Self {
        let literal = match literal {
            Some(literal) => Some(
                LiteralType::Number(literal),
            ),
            None => None
        };

        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    pub fn to_string(&self) -> String {
        let text_literal = match &self.literal {
            Some(LiteralType::String(literal)) => literal.clone(),
            Some(LiteralType::Number(literal)) => format!("{:?}", literal),
            None => "".to_string(),
        };

        format!(
            "{} {} {}",
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
