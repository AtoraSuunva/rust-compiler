use std::fmt;

use super::{location::Location, token_type::Type};

#[derive(Debug)]
pub struct Token {
    pub token_type: Type,
    pub lexeme: String,
    pub location: Location,
}

impl Token {
    pub fn new(token_type: Type, lexeme: String, location: Location) -> Token {
        Token {
            token_type,
            lexeme,
            location,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}, {}, {}]",
            self.token_type,
            self.lexeme.replace('\n', "\\n").replace('\r', "\\r"),
            self.location
        )
    }
}
