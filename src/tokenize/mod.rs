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
fn test_tokenize() {
    let file_contents = String::from("(()");
    let tokens = tokenize(file_contents);

    let tokens_msg = tokens
        .iter()
        .map(|token| token.to_string())
        .collect::<Vec<String>>()
        .join("\n");

    println!("AUII {}", tokens_msg);
}
