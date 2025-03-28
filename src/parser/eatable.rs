use crate::prelude::*;

use std::fmt::format;

pub trait Eatable {
    /**
     * Eats a token and advances the lookahead token.
     * Throws a Syntax error if lookahead doesn't match supplied token.
     */
    fn eat(&mut self, token_type: TokenType) -> Result<Token>;
}

impl Eatable for Parser {
    fn eat(&mut self, token_type: TokenType) -> Result<Token> {
        if self.lookahead.token_type != token_type {
            return Err(Error::Syntax(
                format(format_args!("Unexpected token {:?}, expected {:?}!", self.lookahead.token_type, token_type))
            ));
        }
        
        // Advance to the next token.
        let token = self.lookahead.clone();
        self.lookahead = self.tokenizer.get_next_token()?;
        Ok(token)
    }
}
