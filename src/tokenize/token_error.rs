#[derive(Debug, Clone, PartialEq)]
pub struct TokenizerError {
    line: u32,
    // where: String,
    message: String,
}

#[allow(dead_code)]
impl TokenizerError {
    pub fn new(line: u32, message: String) -> Self {
        Self {
            line,
            message,
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "[line {}] Error: {}",
            self.line,
            self.message
        )
    }
}

#[test]
fn test_tokenizer_error_unexpected_character_1() {
    let message = "Unexpected character: $".to_string();
    let token_error = TokenizerError::new(1, message);

    let expected = "[line 1] Error: Unexpected character: $".to_string();

    assert_eq!(token_error.to_string(), expected);
}

#[test]
fn test_tokenizer_error_unexpected_character_2() {
    let message = "Unexpected character: #".to_string();
    let token_error = TokenizerError::new(23, message);

    let expected = "[line 23] Error: Unexpected character: #".to_string();

    assert_eq!(token_error.to_string(), expected);
}

