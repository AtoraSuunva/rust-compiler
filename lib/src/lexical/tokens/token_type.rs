use std::{
    fmt::Display,
    hash::{Hash, Hasher},
};

use super::error_type::ErrorType;

#[derive(Debug, Clone)]
pub enum Type {
    EndOfFile,
    Id(String),
    IntNum(isize),
    FloatNum(f64),
    Eq,
    NotEq,
    Lt,
    Gt,
    LEq,
    GEq,
    Plus,
    Minus,
    Mult,
    Div,
    Assign,
    OpenPar,
    ClosePar,
    OpenCubr,
    CloseCubr,
    OpenSqbr,
    CloseSqbr,
    Semi,
    Comma,
    Dot,
    Colon,
    ReturnType,
    ScopeOp,
    Or,
    And,
    Not,
    Integer,
    Float,
    Void,
    Class,
    SelfT,
    IsA,
    While,
    If,
    Then,
    Else,
    Read,
    Write,
    Return,
    LocalVar,
    Constructor,
    Attribute,
    Function,
    Public,
    Private,
    InlineCmt,
    BlockCmt,
    Invalid(ErrorType),
}

impl Type {
    pub fn from_alphanum(str: &str) -> Type {
        match str {
            "or" => Type::Or,
            "and" => Type::And,
            "not" => Type::Not,
            "integer" => Type::Integer,
            "float" => Type::Float,
            "void" => Type::Void,
            "class" => Type::Class,
            "self" => Type::SelfT,
            "isa" => Type::IsA,
            "while" => Type::While,
            "if" => Type::If,
            "then" => Type::Then,
            "else" => Type::Else,
            "read" => Type::Read,
            "write" => Type::Write,
            "return" => Type::Return,
            "localvar" => Type::LocalVar,
            "constructor" => Type::Constructor,
            "attribute" => Type::Attribute,
            "function" => Type::Function,
            "public" => Type::Public,
            "private" => Type::Private,
            _ => Type::Id(str.to_string()),
        }
    }

    pub fn empty_variant(&self) -> Type {
        match self {
            Type::Id(_) => Type::Id("".to_string()),
            Type::IntNum(_) => Type::IntNum(0),
            Type::FloatNum(_) => Type::FloatNum(0.0),
            _ => self.clone(),
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Type::EndOfFile => "$",
                Type::Id(_) => "id",
                Type::IntNum(_) => "intnum",
                Type::FloatNum(_) => "floatnum",
                Type::Eq => "eq",
                Type::NotEq => "noteq",
                Type::Lt => "lt",
                Type::Gt => "gt",
                Type::LEq => "leq",
                Type::GEq => "geq",
                Type::Plus => "plus",
                Type::Minus => "minus",
                Type::Mult => "mult",
                Type::Div => "div",
                Type::Assign => "assign",
                Type::OpenPar => "openpar",
                Type::ClosePar => "closepar",
                Type::OpenCubr => "opencubr",
                Type::CloseCubr => "closecubr",
                Type::OpenSqbr => "opensqbr",
                Type::CloseSqbr => "closesqbr",
                Type::Semi => "semi",
                Type::Comma => "comma",
                Type::Dot => "dot",
                Type::Colon => "colon",
                Type::ReturnType => "returntype",
                Type::ScopeOp => "scopeop",
                Type::Or => "or",
                Type::And => "and",
                Type::Not => "not",
                Type::Integer => "integer",
                Type::Float => "float",
                Type::Void => "void",
                Type::Class => "class",
                Type::SelfT => "self",
                Type::IsA => "isa",
                Type::While => "while",
                Type::If => "if",
                Type::Then => "then",
                Type::Else => "else",
                Type::Read => "read",
                Type::Write => "write",
                Type::Return => "return",
                Type::LocalVar => "localvar",
                Type::Constructor => "constructor",
                Type::Attribute => "attribute",
                Type::Function => "function",
                Type::Public => "public",
                Type::Private => "private",
                Type::InlineCmt => "inlinecmt",
                Type::BlockCmt => "blockcmt",
                Type::Invalid(err) => {
                    return write!(f, "{}", err);
                }
            }
        )
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl Eq for Type {}

impl Hash for Type {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_string().hash(state);
    }
}
