pub use crate::parser::{Eatable, Parsable, Parser};
pub use crate::errors::Error;
pub use crate::tokenizer::{Token, TokenType, Tokenizer};
pub use crate::tree::Tree;

pub type Result<T> = core::result::Result<T, Error>;
