use std::fmt::format;

use regex::Regex;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Number,
    String,
}

impl TokenType {
    /**
     * Tokenizer spec.
     */
    const SPEC: [(Option<TokenType>, &str); 6] = [
        // ----- WHITESPACES -----
        (None, r"^\s+"),

        // ----- SINGLE-LINE COMMENTS -----
        (None, r"^(//.*)"),

        // ----- MULTI-LINE COMMENTS -----
        (None, r"^(/*[\s\S]*?\*/)"),

        // ----- NUMBERS -----
        (Some(Self::Number), r"^(\d+)"),

        // ----- STRINGS -----
        (Some(Self::String), r#"^".*""#),
        (Some(Self::String), r#"^'.*'"#),
    ];
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

/**
 * Tokenizer class.
 * 
 * Lazily pulls a token from a stream.
 */
pub struct Tokenizer {
    content_string: String,
    cursor: usize,
}

impl Tokenizer {

    pub fn new(content_string: String) -> Self {
        Self {
            content_string,
            cursor: 0,
        }
    }

    /**
     * Whether we still have more tokens.
     */
    fn has_tokens(&self) -> bool {
        self.cursor < self.content_string.len()
    }

    /**
     * Obtains next token.
     */
    pub fn get_next_token(&mut self) -> Result<Option<Token>, SyntaxError> {
        if !self.has_tokens() {
            return Ok(None);
        }

        for (token_type, regex) in TokenType::SPEC {
            let re = Regex::new(regex).unwrap();
            if let Some(caps) = re.captures(&self.content_string[self.cursor..]) {
                let cap = &caps[0];
                self.cursor += cap.len();

                // Should skip token, e.g. whitespaces
                return match token_type {
                    None => self.get_next_token(),
                    Some(token_type) => Ok(Some(Token {
                        token_type,
                        value: cap.to_string(),
                    }))
                };
            }
        }

        Err(SyntaxError {
            message: format(format_args!("Unexpected token: {}", self.content_string.chars().nth(self.cursor).unwrap())),
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SyntaxError {
    pub message: String,
}
