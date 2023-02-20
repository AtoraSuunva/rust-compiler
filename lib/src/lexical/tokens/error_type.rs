use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum ErrorType {
    InvalidChar(),
    LeadingZero(),
    TrailingZero(),
    UnclosedBlockCmt(),
}

impl Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ErrorType::InvalidChar() => "invalidchar",
                ErrorType::LeadingZero() => "invalidnum",
                ErrorType::TrailingZero() => "invalidnum",
                ErrorType::UnclosedBlockCmt() => "invalidcmt",
            }
        )
    }
}

impl ErrorType {
    pub fn as_detailed(&self) -> String {
        (match self {
            ErrorType::InvalidChar() => "Invalid character",
            ErrorType::LeadingZero() => "Invalid leading zero",
            ErrorType::TrailingZero() => "Invalid trailing zero",
            ErrorType::UnclosedBlockCmt() => "Unclosed block comment",
        })
        .to_owned()
    }
}
