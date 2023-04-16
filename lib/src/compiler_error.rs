use std::fmt::{Display, Formatter};

use crate::lexical::tokens::{location::Location, token::Token};

#[derive(Debug, Clone, Eq)]
pub struct CompilerError {
    pub message: String,
    pub location: Location,
    pub token: Option<Token>,
}

impl CompilerError {
    pub fn new(message: String, token: Token) -> Self {
        Self {
            message,
            location: token.location.clone(),
            token: Some(token),
        }
    }

    pub fn new_with_message(message: String) -> Self {
        Self {
            message,
            location: Location::new(0, 0),
            token: None,
        }
    }
}

impl Display for CompilerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let first = if let Some(t) = &self.token {
            format!("{}: ", t)
        } else {
            format!("{}: ", self.location)
        };

        write!(f, "{} {}", first, self.message)
    }
}

pub fn print_errors(errors: &[CompilerError]) {
    let mut e = errors.to_vec();
    // sort them by location
    e.sort();

    for error in e {
        eprintln!("{}", error);
    }
}

pub fn errors_to_string(errors: &[CompilerError]) -> String {
    let mut e = errors.to_vec();
    // sort them by location
    e.sort();

    e.iter()
        .map(|e| e.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

impl Ord for CompilerError {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.location.cmp(&other.location)
    }
}

impl PartialOrd for CompilerError {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for CompilerError {
    fn eq(&self, other: &Self) -> bool {
        self.location == other.location
    }
}

/// lol
impl From<CompilerError> for Vec<CompilerError> {
    fn from(error: CompilerError) -> Self {
        vec![error]
    }
}

pub type CompilerResult<T> = Result<T, Vec<CompilerError>>;
