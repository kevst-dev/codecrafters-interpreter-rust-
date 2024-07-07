mod token_type;
use token_type::TokenType;

mod token;
pub use token::Token;

mod scanner;
use scanner::Scanner;

#[allow(dead_code)]
pub fn tokenize(file_contents: String) -> Vec<Token> {
    let mut scanner = Scanner::new(file_contents);
    let tokens = scanner.scan_tokens();

    tokens
}

#[test]
fn test_tokenize_parentheses() {
    let file_contents = String::from("(()");
    let tokens = tokenize(file_contents);

    let expected_tokens = vec![
        Token::new(TokenType::LeftParen, "(".to_string(), None, 1),
        Token::new(TokenType::LeftParen, "(".to_string(), None, 1),
        Token::new(TokenType::RightParen, ")".to_string(), None, 1),
        Token::new(TokenType::Eof, "".to_string(), None, 1),
    ];

    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_tokenize_brackets() {
    let file_contents = String::from("{{}}");
    let tokens = tokenize(file_contents);

    /*
    let tokens_msg = tokens
        .iter()
        .map(|token| token.to_string())
        .collect::<Vec<String>>()
        .join("\n");
    */

    let expected_tokens = vec![
        Token::new(TokenType::LeftBrace, "{".to_string(), None, 1),
        Token::new(TokenType::LeftBrace, "{".to_string(), None, 1),
        Token::new(TokenType::RightBrace, "}".to_string(), None, 1),
        Token::new(TokenType::RightBrace, "}".to_string(), None, 1),
        Token::new(TokenType::Eof, "".to_string(), None, 1),
    ];

    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_tokenize_single_chars() {
    let file_contents = String::from("({*.,+*})");
    let tokens = tokenize(file_contents);

    let expected_tokens = vec![
        Token::new(TokenType::LeftParen, "(".to_string(), None, 1),
        Token::new(TokenType::LeftBrace, "{".to_string(), None, 1),
        Token::new(TokenType::Star, "*".to_string(), None, 1),
        Token::new(TokenType::Dot, ".".to_string(), None, 1),
        Token::new(TokenType::Comma, ",".to_string(), None, 1),
        Token::new(TokenType::Plus, "+".to_string(), None, 1),
        Token::new(TokenType::Star, "*".to_string(), None, 1),
        // Token::new(TokenType::RightBrace, "}".to_string(), None, 1),
        Token::new(TokenType::RightParen, ")".to_string(), None, 1),
        Token::new(TokenType::Eof, "".to_string(), None, 1),
    ];

    assert_eq!(tokens, expected_tokens);
}
