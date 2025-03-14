mod token_type;
use token_type::TokenType;

mod token;
pub use token::Token;

mod token_error;
pub use token_error::TokenizerError;

mod scanner;
use scanner::Scanner;

#[allow(dead_code)]
pub fn tokenize(file_contents: String) -> (Vec<Token>, Vec<TokenizerError>) {
    let mut scanner = Scanner::new(file_contents);

    let tokens = scanner.scan_tokens();
    let token_errors = scanner.token_errors();

    (tokens, token_errors)
}

use pretty_assertions::assert_eq;

#[test]
fn test_tokenize_parentheses() {
    let file_contents = String::from("(()");
    let (tokens, token_errors) = tokenize(file_contents);

    let expected_tokens = vec![
        Token::new(TokenType::LeftParen, "(".to_string(), None, 1),
        Token::new(TokenType::LeftParen, "(".to_string(), None, 1),
        Token::new(TokenType::RightParen, ")".to_string(), None, 1),
        Token::new(TokenType::Eof, "".to_string(), None, 1),
    ];

    assert_eq!(tokens, expected_tokens);
    assert_eq!(token_errors, vec![]);
}

#[test]
fn test_tokenize_brackets() {
    let file_contents = String::from("{{}}");
    let (tokens, token_errors) = tokenize(file_contents);

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
    assert_eq!(token_errors, vec![]);
}

#[test]
fn test_tokenize_single_chars() {
    let file_contents = String::from("({*.,+*})");
    let (tokens, token_errors) = tokenize(file_contents);

    let expected_tokens = vec![
        Token::new(TokenType::LeftParen, "(".to_string(), None, 1),
        Token::new(TokenType::LeftBrace, "{".to_string(), None, 1),
        Token::new(TokenType::Star, "*".to_string(), None, 1),
        Token::new(TokenType::Dot, ".".to_string(), None, 1),
        Token::new(TokenType::Comma, ",".to_string(), None, 1),
        Token::new(TokenType::Plus, "+".to_string(), None, 1),
        Token::new(TokenType::Star, "*".to_string(), None, 1),
        Token::new(TokenType::RightBrace, "}".to_string(), None, 1),
        Token::new(TokenType::RightParen, ")".to_string(), None, 1),
        Token::new(TokenType::Eof, "".to_string(), None, 1),
    ];

    assert_eq!(tokens, expected_tokens);
    assert_eq!(token_errors, vec![]);
}

#[test]
fn test_tokenize_single_chars_with_unexpected_chars() {
    let file_contents = String::from(",.$(#");
    let (tokens, token_errors) = tokenize(file_contents);

    let expected_tokens = vec![
        Token::new(TokenType::Comma, ",".to_string(), None, 1),
        Token::new(TokenType::Dot, ".".to_string(), None, 1),
        Token::new(TokenType::LeftParen, "(".to_string(), None, 1),
        Token::new(TokenType::Eof, "".to_string(), None, 1),
    ];

    let expected_token_errors = vec![
        TokenizerError::new(1, "Unexpected character: $".to_string()),
        TokenizerError::new(1, "Unexpected character: #".to_string()),
    ];

    assert_eq!(tokens, expected_tokens);
    assert_eq!(token_errors, expected_token_errors);
}

#[test]
fn test_tokenize_single_chars_with_unexpected_chars_multi_lines() {
    let file_contents = String::from("# (\n)\t@");
    let (tokens, token_errors) = tokenize(file_contents);

    let expected_tokens = vec![
        Token::new(TokenType::LeftParen, "(".to_string(), None, 1),
        Token::new(TokenType::RightParen, ")".to_string(), None, 2),
        Token::new(TokenType::Eof, "".to_string(), None, 2),
    ];

    let expected_token_errors = vec![
        TokenizerError::new(1, "Unexpected character: #".to_string()),
        TokenizerError::new(2, "Unexpected character: @".to_string()),
    ];

    assert_eq!(tokens, expected_tokens);
    assert_eq!(token_errors, expected_token_errors);
}

#[test]
fn test_tokenize_operators_single_equal() {
    let file_contents = String::from("=");
    let (tokens, token_errors) = tokenize(file_contents);

    let expected_tokens = vec![
        Token::new(TokenType::Equal, "=".to_string(), None, 1),
        Token::new(TokenType::Eof, "".to_string(), None, 1),
    ];

    assert_eq!(tokens, expected_tokens);
    assert_eq!(token_errors, vec![]);
}

#[test]
fn test_tokenize_operators_chars_1() {
    let file_contents = String::from("={===}");
    let (tokens, token_errors) = tokenize(file_contents);

    let expected_tokens = vec![
        Token::new(TokenType::Equal, "=".to_string(), None, 1),
        Token::new(TokenType::LeftBrace, "{".to_string(), None, 1),
        Token::new(TokenType::EqualEqual, "==".to_string(), None, 1),
        Token::new(TokenType::Equal, "=".to_string(), None, 1),
        Token::new(TokenType::RightBrace, "}".to_string(), None, 1),
        Token::new(TokenType::Eof, "".to_string(), None, 1),
    ];

    assert_eq!(tokens, expected_tokens);
    assert_eq!(token_errors, vec![]);
}

#[test]
fn test_tokenize_operators_chars_2() {
    let file_contents = String::from("!!===");
    let (tokens, token_errors) = tokenize(file_contents);

    let expected_tokens = vec![
        Token::new(TokenType::Bang, "!".to_string(), None, 1),
        Token::new(TokenType::BangEqual, "!=".to_string(), None, 1),
        Token::new(TokenType::EqualEqual, "==".to_string(), None, 1),
        Token::new(TokenType::Eof, "".to_string(), None, 1),
    ];

    assert_eq!(tokens, expected_tokens);
    assert_eq!(token_errors, vec![]);
}

#[test]
fn test_tokenize_operators_chars_3() {
    let file_contents = String::from("<<=>>=");
    let (tokens, token_errors) = tokenize(file_contents);

    let expected_tokens = vec![
        Token::new(TokenType::Less, "<".to_string(), None, 1),
        Token::new(TokenType::LessEqual, "<=".to_string(), None, 1),
        Token::new(TokenType::Greater, ">".to_string(), None, 1),
        Token::new(TokenType::GreaterEqual, ">=".to_string(), None, 1),
        Token::new(TokenType::Eof, "".to_string(), None, 1),
    ];

    assert_eq!(tokens, expected_tokens);
    assert_eq!(token_errors, vec![]);
}

#[test]
fn test_tokenize_slash() {
    let file_contents = String::from("/");
    let (tokens, token_errors) = tokenize(file_contents);

    let expected_tokens = vec![
        Token::new(TokenType::Slash, "/".to_string(), None, 1),
        Token::new(TokenType::Eof, "".to_string(), None, 1),
    ];

    assert_eq!(tokens, expected_tokens);
    assert_eq!(token_errors, vec![]);
}

#[test]
fn test_tokenize_slash_comment() {
    // let file_contents = String::from("// Comment\n");
    let file_contents = String::from("///Unicode:£§᯽☺♣/)");
    let (tokens, token_errors) = tokenize(file_contents);

    let expected_tokens = vec![
        Token::new(TokenType::Eof, "".to_string(), None, 1),
    ];

    assert_eq!(tokens, expected_tokens);
    assert_eq!(token_errors, vec![]);
}

#[test]
fn test_tokenize_whitespaces() {
    let file_contents = String::from("(\t\n)");
    let (tokens, token_errors) = tokenize(file_contents);

    let expected_tokens = vec![
        Token::new(TokenType::LeftParen, "(".to_string(), None, 1),
        Token::new(TokenType::RightParen, ")".to_string(), None, 2),
        Token::new(TokenType::Eof, "".to_string(), None, 2),
    ];

    assert_eq!(tokens, expected_tokens);
    assert_eq!(token_errors, vec![]);
}


#[test]
fn test_tokenize_string() {
    let file_contents = String::from("\"Hello, World!\"");
    let (tokens, token_errors) = tokenize(file_contents);

    let expected_tokens = vec![
        Token::new(
            TokenType::String,
            "\"Hello, World!\"".to_string(),
            Some("Hello, World!".to_string()),
            1
        ),
        Token::new(TokenType::Eof, "".to_string(), None, 1),
    ];

    assert_eq!(tokens, expected_tokens);
    assert_eq!(token_errors, vec![]);
}

#[test]
fn test_tokenize_number() {
    let file_contents = String::from("1234.1234");
    let (tokens, token_errors) = tokenize(file_contents);

    let expected_tokens = vec![
        Token::new_number(
            TokenType::Number,
            "1234.1234".to_string(),
            Some(1234.1234),
            1
        ),
        Token::new(TokenType::Eof, "".to_string(), None, 1),
    ];

    assert_eq!(tokens, expected_tokens);
    assert_eq!(token_errors, vec![]);
}

#[test]
fn test_tokenize_identifiers() {
    let file_contents = String::from("foo bar _hello");
    let (tokens, token_errors) = tokenize(file_contents);

    let expected_tokens = vec![
        Token::new(TokenType::Identifier, "foo".to_string(), None, 1),
        Token::new(TokenType::Identifier, "bar".to_string(), None, 1),
        Token::new(TokenType::Identifier, "_hello".to_string(), None, 1),
        Token::new(TokenType::Eof, "".to_string(), None, 1),
    ];

    assert_eq!(tokens, expected_tokens);
    assert_eq!(token_errors, vec![]);
}

#[test]
fn test_tokenize_keywords() {
    let file_contents = String::from("if else return");
    let (tokens, token_errors) = tokenize(file_contents);

    let expected_tokens = vec![
        Token::new(TokenType::If, "if".to_string(), None, 1),
        Token::new(TokenType::Else, "else".to_string(), None, 1),
        Token::new(TokenType::Return, "return".to_string(), None, 1),
        Token::new(TokenType::Eof, "".to_string(), None, 1),
    ];

    assert_eq!(tokens, expected_tokens);
    assert_eq!(token_errors, vec![]);
}
