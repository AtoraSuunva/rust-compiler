use std::{collections::HashMap, fmt::Display};

use crate::lexical::tokens::token_type::Type;

#[derive(Debug)]
pub enum Production<'a> {
    Term(Type),
    NonTerm(&'a str),
}

impl Display for Production<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Production::Term(t) => write!(f, "'{}'", t),
            Production::NonTerm(nt) => write!(f, "<{}>", nt),
        }
    }
}

// This should be a compile-time static collection but I don't wanna install any crates to see how far I can go
pub fn get_first_set_table() -> HashMap<&'static str, Vec<Type>> {
    HashMap::from([
        ("START", vec![Type::Class, Type::Function]),
        ("ARRAYSIZETAIL", vec![Type::IntNum(0), Type::CloseSqbr]),
        ("CLASSDECL", vec![Type::Class]),
        (
            "EXPRTAIL",
            vec![
                Type::Eq,
                Type::NotEq,
                Type::Lt,
                Type::Gt,
                Type::LEq,
                Type::GEq,
            ],
        ),
        (
            "RELOP",
            vec![
                Type::Eq,
                Type::NotEq,
                Type::Lt,
                Type::Gt,
                Type::LEq,
                Type::GEq,
            ],
        ),
        ("VARIABLE_OR_FUNCTIONCALL", vec![Type::Id("".to_owned())]),
        ("FUNCDEF", vec![Type::Function]),
        ("FUNCBODY", vec![Type::OpenCubr]),
        ("FUNCHEAD", vec![Type::Function]),
        ("FUNCHEADTAIL", vec![Type::OpenPar, Type::ScopeOp]),
        ("FUNCHEADSR", vec![Type::ScopeOp]),
        (
            "FUNCHEADSRTAIL",
            vec![Type::Id("".to_owned()), Type::Constructor],
        ),
        ("FUNCHEADPARAMS", vec![Type::OpenPar]),
        ("OPT_CALL_OR_INDEXED", vec![Type::OpenPar, Type::OpenSqbr]),
        ("INDICE", vec![Type::OpenSqbr]),
        (
            "ARITHEXPR",
            vec![
                Type::IntNum(0),
                Type::FloatNum(0f64),
                Type::OpenPar,
                Type::Not,
                Type::Plus,
                Type::Minus,
                Type::Id("".to_owned()),
            ],
        ),
        (
            "LOCALVARDECLTAIL",
            vec![Type::Semi, Type::OpenPar, Type::OpenSqbr],
        ),
        (
            "APARAMS",
            vec![
                Type::IntNum(0),
                Type::FloatNum(0f64),
                Type::OpenPar,
                Type::Not,
                Type::Plus,
                Type::Minus,
                Type::Id("".to_owned()),
            ],
        ),
        ("LOCALVARDECL", vec![Type::LocalVar]),
        ("MEMBERFUNCDECL", vec![Type::Function, Type::Constructor]),
        ("FPARAMS", vec![Type::Id("".to_owned())]),
        ("MEMBERVARDECL", vec![Type::Attribute]),
        ("OPTCLASSDECL2", vec![Type::IsA]),
        ("APARAMSTAIL", vec![Type::Comma]),
        ("REPTAPARAMS1", vec![Type::Comma]),
        (
            "MEMBERDECL",
            vec![Type::Function, Type::Constructor, Type::Attribute],
        ),
        (
            "REPTCLASSDECL4",
            vec![
                Type::Public,
                Type::Private,
                Type::Function,
                Type::Constructor,
                Type::Attribute,
            ],
        ),
        ("REPTFPARAMS3", vec![Type::OpenSqbr]),
        ("FPARAMSTAIL", vec![Type::Comma]),
        ("REPTFPARAMS4", vec![Type::Comma]),
        ("REPTFPARAMSTAIL4", vec![Type::OpenSqbr]),
        (
            "LOCALVARDECLORSTMT",
            vec![
                Type::LocalVar,
                Type::If,
                Type::While,
                Type::Read,
                Type::Write,
                Type::Return,
                Type::Id("".to_owned()),
            ],
        ),
        (
            "REPTFUNCBODY1",
            vec![
                Type::LocalVar,
                Type::If,
                Type::While,
                Type::Read,
                Type::Write,
                Type::Return,
                Type::Id("".to_owned()),
            ],
        ),
        ("REPTLOCALVARDECL4", vec![Type::OpenSqbr]),
        ("ARRAYSIZE", vec![Type::OpenSqbr]),
        ("REPTMEMBERVARDECL4", vec![Type::OpenSqbr]),
        ("REPTOPTCLASSDECL22", vec![Type::Comma]),
        ("CLASSDECLORFUNCDEF", vec![Type::Class, Type::Function]),
        ("REPTSTART0", vec![Type::Class, Type::Function]),
        (
            "RETURNTYPE",
            vec![
                Type::Void,
                Type::Integer,
                Type::Float,
                Type::Id("".to_owned()),
            ],
        ),
        ("ADDOP", vec![Type::Plus, Type::Minus, Type::Or]),
        ("RIGHTRECARITHEXPR", vec![Type::Plus, Type::Minus, Type::Or]),
        ("MULTOP", vec![Type::Mult, Type::Div, Type::And]),
        ("SIGN", vec![Type::Plus, Type::Minus]),
        (
            "REPTSTATBLOCK1",
            vec![
                Type::If,
                Type::While,
                Type::Read,
                Type::Write,
                Type::Return,
                Type::Id("".to_owned()),
            ],
        ),
        (
            "STATEMENT",
            vec![
                Type::If,
                Type::While,
                Type::Read,
                Type::Write,
                Type::Return,
                Type::Id("".to_owned()),
            ],
        ),
        (
            "RELEXPR",
            vec![
                Type::IntNum(0),
                Type::FloatNum(0f64),
                Type::OpenPar,
                Type::Not,
                Type::Plus,
                Type::Minus,
                Type::Id("".to_owned()),
            ],
        ),
        (
            "STATBLOCK",
            vec![
                Type::OpenCubr,
                Type::If,
                Type::While,
                Type::Read,
                Type::Write,
                Type::Return,
                Type::Id("".to_owned()),
            ],
        ),
        (
            "ASSIGN_OR_FUNCTIONCALL_TAIL",
            vec![Type::Dot, Type::OpenPar, Type::OpenSqbr, Type::Assign],
        ),
        ("CALL", vec![Type::OpenPar]),
        ("AF_INDEXED_TAIL", vec![Type::Dot, Type::Assign]),
        ("ASSIGNOP", vec![Type::Assign]),
        (
            "EXPR",
            vec![
                Type::IntNum(0),
                Type::FloatNum(0f64),
                Type::OpenPar,
                Type::Not,
                Type::Plus,
                Type::Minus,
                Type::Id("".to_owned()),
            ],
        ),
        ("AF_CALL_TAIL", vec![Type::Dot]),
        ("ASSIGN_OR_FUNCTIONCALL", vec![Type::Id("".to_owned())]),
        (
            "TERM",
            vec![
                Type::IntNum(0),
                Type::FloatNum(0f64),
                Type::OpenPar,
                Type::Not,
                Type::Plus,
                Type::Minus,
                Type::Id("".to_owned()),
            ],
        ),
        (
            "FACTOR",
            vec![
                Type::IntNum(0),
                Type::FloatNum(0f64),
                Type::OpenPar,
                Type::Not,
                Type::Plus,
                Type::Minus,
                Type::Id("".to_owned()),
            ],
        ),
        ("RIGHTRECTERM", vec![Type::Mult, Type::Div, Type::And]),
        (
            "TYPE",
            vec![Type::Integer, Type::Float, Type::Id("".to_owned())],
        ),
        ("VARIABLE", vec![Type::Id("".to_owned())]),
        ("CALL_OR_INDEXED_ID", vec![Type::Id("".to_owned())]),
        ("REP_CI_ID0", vec![Type::Dot]),
        ("INDEXED", vec![Type::OpenSqbr]),
        ("VISIBILITY", vec![Type::Public, Type::Private]),
    ])
}

pub fn get_follow_set_table() -> HashMap<&'static str, Vec<Type>> {
    HashMap::from([
        ("START", vec![]),
        (
            "ARRAYSIZETAIL",
            vec![Type::OpenSqbr, Type::Semi, Type::ClosePar, Type::Comma],
        ),
        ("CLASSDECL", vec![Type::Class, Type::Function]),
        ("EXPRTAIL", vec![Type::Semi, Type::Comma, Type::ClosePar]),
        (
            "RELOP",
            vec![
                Type::IntNum(0),
                Type::FloatNum(0f64),
                Type::OpenPar,
                Type::Not,
                Type::Plus,
                Type::Minus,
                Type::Id("".to_owned()),
            ],
        ),
        (
            "VARIABLE_OR_FUNCTIONCALL",
            vec![
                Type::Semi,
                Type::Mult,
                Type::Div,
                Type::And,
                Type::Eq,
                Type::NotEq,
                Type::Lt,
                Type::Gt,
                Type::LEq,
                Type::GEq,
                Type::CloseSqbr,
                Type::Plus,
                Type::Minus,
                Type::Or,
                Type::Comma,
                Type::ClosePar,
            ],
        ),
        ("FUNCDEF", vec![Type::Class, Type::Function]),
        ("FUNCBODY", vec![Type::Class, Type::Function]),
        ("FUNCHEAD", vec![Type::OpenCubr]),
        ("FUNCHEADTAIL", vec![Type::OpenCubr]),
        ("FUNCHEADSR", vec![Type::OpenCubr]),
        ("FUNCHEADSRTAIL", vec![Type::OpenCubr]),
        ("FUNCHEADPARAMS", vec![Type::ReturnType, Type::OpenCubr]),
        (
            "OPT_CALL_OR_INDEXED",
            vec![
                Type::Semi,
                Type::Mult,
                Type::Div,
                Type::And,
                Type::Dot,
                Type::Id("".to_owned()),
                Type::Eq,
                Type::NotEq,
                Type::Lt,
                Type::Gt,
                Type::LEq,
                Type::GEq,
                Type::CloseSqbr,
                Type::Plus,
                Type::Minus,
                Type::Or,
                Type::Comma,
                Type::ClosePar,
            ],
        ),
        (
            "INDICE",
            vec![
                Type::Assign,
                Type::Semi,
                Type::Mult,
                Type::Div,
                Type::And,
                Type::OpenSqbr,
                Type::Dot,
                Type::Id("".to_owned()),
                Type::Eq,
                Type::NotEq,
                Type::Lt,
                Type::Gt,
                Type::LEq,
                Type::GEq,
                Type::CloseSqbr,
                Type::Plus,
                Type::Minus,
                Type::Or,
                Type::Comma,
                Type::ClosePar,
            ],
        ),
        (
            "ARITHEXPR",
            vec![
                Type::Semi,
                Type::Eq,
                Type::NotEq,
                Type::Lt,
                Type::Gt,
                Type::LEq,
                Type::GEq,
                Type::CloseSqbr,
                Type::Comma,
                Type::ClosePar,
            ],
        ),
        (
            "LOCALVARDECLTAIL",
            vec![
                Type::LocalVar,
                Type::If,
                Type::While,
                Type::Read,
                Type::Write,
                Type::Return,
                Type::Id("".to_owned()),
                Type::CloseCubr,
            ],
        ),
        ("APARAMS", vec![Type::ClosePar]),
        (
            "LOCALVARDECL",
            vec![
                Type::LocalVar,
                Type::If,
                Type::While,
                Type::Read,
                Type::Write,
                Type::Return,
                Type::Id("".to_owned()),
                Type::CloseCubr,
            ],
        ),
        (
            "MEMBERFUNCDECL",
            vec![
                Type::Public,
                Type::Private,
                Type::Function,
                Type::Constructor,
                Type::Attribute,
                Type::CloseCubr,
            ],
        ),
        ("FPARAMS", vec![Type::ClosePar]),
        (
            "MEMBERVARDECL",
            vec![
                Type::Public,
                Type::Private,
                Type::Function,
                Type::Constructor,
                Type::Attribute,
                Type::CloseCubr,
            ],
        ),
        ("OPTCLASSDECL2", vec![Type::OpenCubr]),
        ("APARAMSTAIL", vec![Type::Comma, Type::ClosePar]),
        ("REPTAPARAMS1", vec![Type::ClosePar]),
        (
            "MEMBERDECL",
            vec![
                Type::Public,
                Type::Private,
                Type::Function,
                Type::Constructor,
                Type::Attribute,
                Type::CloseCubr,
            ],
        ),
        ("REPTCLASSDECL4", vec![Type::CloseCubr]),
        ("REPTFPARAMS3", vec![Type::ClosePar, Type::Comma]),
        ("FPARAMSTAIL", vec![Type::Comma, Type::ClosePar]),
        ("REPTFPARAMS4", vec![Type::ClosePar]),
        ("REPTFPARAMSTAIL4", vec![Type::Comma, Type::ClosePar]),
        (
            "LOCALVARDECLORSTMT",
            vec![
                Type::LocalVar,
                Type::If,
                Type::While,
                Type::Read,
                Type::Write,
                Type::Return,
                Type::Id("".to_owned()),
                Type::CloseCubr,
            ],
        ),
        ("REPTFUNCBODY1", vec![Type::CloseCubr]),
        ("REPTLOCALVARDECL4", vec![Type::Semi]),
        (
            "ARRAYSIZE",
            vec![Type::OpenSqbr, Type::Semi, Type::ClosePar, Type::Comma],
        ),
        ("REPTMEMBERVARDECL4", vec![Type::Semi]),
        ("REPTOPTCLASSDECL22", vec![Type::OpenCubr]),
        ("CLASSDECLORFUNCDEF", vec![Type::Class, Type::Function]),
        ("REPTSTART0", vec![]),
        ("RETURNTYPE", vec![Type::Semi, Type::OpenCubr]),
        (
            "ADDOP",
            vec![
                Type::IntNum(0),
                Type::FloatNum(0f64),
                Type::OpenPar,
                Type::Not,
                Type::Plus,
                Type::Minus,
                Type::Id("".to_owned()),
            ],
        ),
        (
            "RIGHTRECARITHEXPR",
            vec![
                Type::Semi,
                Type::Eq,
                Type::NotEq,
                Type::Lt,
                Type::Gt,
                Type::LEq,
                Type::GEq,
                Type::CloseSqbr,
                Type::Comma,
                Type::ClosePar,
            ],
        ),
        (
            "MULTOP",
            vec![
                Type::IntNum(0),
                Type::FloatNum(0f64),
                Type::OpenPar,
                Type::Not,
                Type::Plus,
                Type::Minus,
                Type::Id("".to_owned()),
            ],
        ),
        (
            "SIGN",
            vec![
                Type::IntNum(0),
                Type::FloatNum(0f64),
                Type::OpenPar,
                Type::Not,
                Type::Plus,
                Type::Minus,
                Type::Id("".to_owned()),
            ],
        ),
        ("REPTSTATBLOCK1", vec![Type::CloseCubr]),
        (
            "STATEMENT",
            vec![
                Type::Else,
                Type::Semi,
                Type::LocalVar,
                Type::If,
                Type::While,
                Type::Read,
                Type::Write,
                Type::Return,
                Type::Id("".to_owned()),
                Type::CloseCubr,
            ],
        ),
        ("RELEXPR", vec![Type::ClosePar]),
        ("STATBLOCK", vec![Type::Else, Type::Semi]),
        ("ASSIGN_OR_FUNCTIONCALL_TAIL", vec![Type::Semi]),
        (
            "CALL",
            vec![
                Type::Semi,
                Type::Mult,
                Type::Div,
                Type::And,
                Type::Dot,
                Type::Id("".to_owned()),
                Type::Eq,
                Type::NotEq,
                Type::Lt,
                Type::Gt,
                Type::LEq,
                Type::GEq,
                Type::CloseSqbr,
                Type::Plus,
                Type::Minus,
                Type::Or,
                Type::Comma,
                Type::ClosePar,
            ],
        ),
        ("AF_INDEXED_TAIL", vec![Type::Semi]),
        (
            "ASSIGNOP",
            vec![
                Type::IntNum(0),
                Type::FloatNum(0f64),
                Type::OpenPar,
                Type::Not,
                Type::Plus,
                Type::Minus,
                Type::Id("".to_owned()),
            ],
        ),
        ("EXPR", vec![Type::Semi, Type::Comma, Type::ClosePar]),
        ("AF_CALL_TAIL", vec![Type::Semi]),
        ("ASSIGN_OR_FUNCTIONCALL", vec![Type::Semi]),
        (
            "TERM",
            vec![
                Type::Semi,
                Type::Eq,
                Type::NotEq,
                Type::Lt,
                Type::Gt,
                Type::LEq,
                Type::GEq,
                Type::CloseSqbr,
                Type::Plus,
                Type::Minus,
                Type::Or,
                Type::Comma,
                Type::ClosePar,
            ],
        ),
        (
            "FACTOR",
            vec![
                Type::Semi,
                Type::Mult,
                Type::Div,
                Type::And,
                Type::Eq,
                Type::NotEq,
                Type::Lt,
                Type::Gt,
                Type::LEq,
                Type::GEq,
                Type::CloseSqbr,
                Type::Plus,
                Type::Minus,
                Type::Or,
                Type::Comma,
                Type::ClosePar,
            ],
        ),
        (
            "RIGHTRECTERM",
            vec![
                Type::Semi,
                Type::Eq,
                Type::NotEq,
                Type::Lt,
                Type::Gt,
                Type::LEq,
                Type::GEq,
                Type::CloseSqbr,
                Type::Plus,
                Type::Minus,
                Type::Or,
                Type::Comma,
                Type::ClosePar,
            ],
        ),
        (
            "TYPE",
            vec![
                Type::ClosePar,
                Type::OpenCubr,
                Type::Comma,
                Type::OpenPar,
                Type::OpenSqbr,
                Type::Semi,
            ],
        ),
        ("VARIABLE", vec![Type::ClosePar]),
        (
            "CALL_OR_INDEXED_ID",
            vec![
                Type::Semi,
                Type::Mult,
                Type::Div,
                Type::And,
                Type::Dot,
                Type::Id("".to_owned()),
                Type::Eq,
                Type::NotEq,
                Type::Lt,
                Type::Gt,
                Type::LEq,
                Type::GEq,
                Type::CloseSqbr,
                Type::Plus,
                Type::Minus,
                Type::Or,
                Type::Comma,
                Type::ClosePar,
            ],
        ),
        (
            "REP_CI_ID0",
            vec![
                Type::Id("".to_owned()),
                Type::Semi,
                Type::Mult,
                Type::Div,
                Type::And,
                Type::Eq,
                Type::NotEq,
                Type::Lt,
                Type::Gt,
                Type::LEq,
                Type::GEq,
                Type::CloseSqbr,
                Type::Plus,
                Type::Minus,
                Type::Or,
                Type::Comma,
                Type::ClosePar,
            ],
        ),
        (
            "INDEXED",
            vec![
                Type::Assign,
                Type::Semi,
                Type::Mult,
                Type::Div,
                Type::And,
                Type::Dot,
                Type::Id("".to_owned()),
                Type::Eq,
                Type::NotEq,
                Type::Lt,
                Type::Gt,
                Type::LEq,
                Type::GEq,
                Type::CloseSqbr,
                Type::Plus,
                Type::Minus,
                Type::Or,
                Type::Comma,
                Type::ClosePar,
            ],
        ),
        (
            "VISIBILITY",
            vec![Type::Function, Type::Constructor, Type::Attribute],
        ),
    ])
}

pub fn get_parsing_table() -> HashMap<(&'static str, Type), Vec<Production<'static>>> {
    HashMap::from([
        (
            ("START", Type::EndOfFile),
            vec![Production::NonTerm("REPTSTART0")],
        ),
        (
            ("START", Type::Function),
            vec![Production::NonTerm("REPTSTART0")],
        ),
        (
            ("START", Type::Class),
            vec![Production::NonTerm("REPTSTART0")],
        ),
        (("ADDOP", Type::Minus), vec![Production::Term(Type::Minus)]),
        (("ADDOP", Type::Plus), vec![Production::Term(Type::Plus)]),
        (("ADDOP", Type::Or), vec![Production::Term(Type::Or)]),
        (
            ("APARAMS", Type::Id("".to_owned())),
            vec![
                Production::NonTerm("EXPR"),
                Production::NonTerm("REPTAPARAMS1"),
            ],
        ),
        (("APARAMS", Type::ClosePar), vec![]),
        (
            ("APARAMS", Type::OpenPar),
            vec![
                Production::NonTerm("EXPR"),
                Production::NonTerm("REPTAPARAMS1"),
            ],
        ),
        (
            ("APARAMS", Type::Minus),
            vec![
                Production::NonTerm("EXPR"),
                Production::NonTerm("REPTAPARAMS1"),
            ],
        ),
        (
            ("APARAMS", Type::Plus),
            vec![
                Production::NonTerm("EXPR"),
                Production::NonTerm("REPTAPARAMS1"),
            ],
        ),
        (
            ("APARAMS", Type::Not),
            vec![
                Production::NonTerm("EXPR"),
                Production::NonTerm("REPTAPARAMS1"),
            ],
        ),
        (
            ("APARAMS", Type::FloatNum(0f64)),
            vec![
                Production::NonTerm("EXPR"),
                Production::NonTerm("REPTAPARAMS1"),
            ],
        ),
        (
            ("APARAMS", Type::IntNum(0)),
            vec![
                Production::NonTerm("EXPR"),
                Production::NonTerm("REPTAPARAMS1"),
            ],
        ),
        (
            ("APARAMSTAIL", Type::Comma),
            vec![Production::Term(Type::Comma), Production::NonTerm("EXPR")],
        ),
        (
            ("ARITHEXPR", Type::Id("".to_owned())),
            vec![
                Production::NonTerm("TERM"),
                Production::NonTerm("RIGHTRECARITHEXPR"),
            ],
        ),
        (
            ("ARITHEXPR", Type::OpenPar),
            vec![
                Production::NonTerm("TERM"),
                Production::NonTerm("RIGHTRECARITHEXPR"),
            ],
        ),
        (
            ("ARITHEXPR", Type::Minus),
            vec![
                Production::NonTerm("TERM"),
                Production::NonTerm("RIGHTRECARITHEXPR"),
            ],
        ),
        (
            ("ARITHEXPR", Type::Plus),
            vec![
                Production::NonTerm("TERM"),
                Production::NonTerm("RIGHTRECARITHEXPR"),
            ],
        ),
        (
            ("ARITHEXPR", Type::Not),
            vec![
                Production::NonTerm("TERM"),
                Production::NonTerm("RIGHTRECARITHEXPR"),
            ],
        ),
        (
            ("ARITHEXPR", Type::FloatNum(0f64)),
            vec![
                Production::NonTerm("TERM"),
                Production::NonTerm("RIGHTRECARITHEXPR"),
            ],
        ),
        (
            ("ARITHEXPR", Type::IntNum(0)),
            vec![
                Production::NonTerm("TERM"),
                Production::NonTerm("RIGHTRECARITHEXPR"),
            ],
        ),
        (
            ("ARRAYSIZE", Type::OpenSqbr),
            vec![
                Production::Term(Type::OpenSqbr),
                Production::NonTerm("ARRAYSIZETAIL"),
            ],
        ),
        (
            ("ARRAYSIZETAIL", Type::CloseSqbr),
            vec![Production::Term(Type::CloseSqbr)],
        ),
        (
            ("ARRAYSIZETAIL", Type::IntNum(0)),
            vec![
                Production::Term(Type::IntNum(0)),
                Production::Term(Type::CloseSqbr),
            ],
        ),
        (
            ("ASSIGNOP", Type::Assign),
            vec![Production::Term(Type::Assign)],
        ),
        (
            ("CLASSDECL", Type::Class),
            vec![
                Production::Term(Type::Class),
                Production::Term(Type::Id("".to_owned())),
                Production::NonTerm("OPTCLASSDECL2"),
                Production::Term(Type::OpenCubr),
                Production::NonTerm("REPTCLASSDECL4"),
                Production::Term(Type::CloseCubr),
                Production::Term(Type::Semi),
            ],
        ),
        (
            ("CLASSDECLORFUNCDEF", Type::Function),
            vec![Production::NonTerm("FUNCDEF")],
        ),
        (
            ("CLASSDECLORFUNCDEF", Type::Class),
            vec![Production::NonTerm("CLASSDECL")],
        ),
        (
            ("EXPR", Type::Id("".to_owned())),
            vec![
                Production::NonTerm("ARITHEXPR"),
                Production::NonTerm("EXPRTAIL"),
            ],
        ),
        (
            ("EXPR", Type::OpenPar),
            vec![
                Production::NonTerm("ARITHEXPR"),
                Production::NonTerm("EXPRTAIL"),
            ],
        ),
        (
            ("EXPR", Type::Minus),
            vec![
                Production::NonTerm("ARITHEXPR"),
                Production::NonTerm("EXPRTAIL"),
            ],
        ),
        (
            ("EXPR", Type::Plus),
            vec![
                Production::NonTerm("ARITHEXPR"),
                Production::NonTerm("EXPRTAIL"),
            ],
        ),
        (
            ("EXPR", Type::Not),
            vec![
                Production::NonTerm("ARITHEXPR"),
                Production::NonTerm("EXPRTAIL"),
            ],
        ),
        (
            ("EXPR", Type::FloatNum(0f64)),
            vec![
                Production::NonTerm("ARITHEXPR"),
                Production::NonTerm("EXPRTAIL"),
            ],
        ),
        (
            ("EXPR", Type::IntNum(0)),
            vec![
                Production::NonTerm("ARITHEXPR"),
                Production::NonTerm("EXPRTAIL"),
            ],
        ),
        (("EXPRTAIL", Type::Semi), vec![]),
        (("EXPRTAIL", Type::ClosePar), vec![]),
        (("EXPRTAIL", Type::Comma), vec![]),
        (
            ("EXPRTAIL", Type::GEq),
            vec![
                Production::NonTerm("RELOP"),
                Production::NonTerm("ARITHEXPR"),
            ],
        ),
        (
            ("EXPRTAIL", Type::LEq),
            vec![
                Production::NonTerm("RELOP"),
                Production::NonTerm("ARITHEXPR"),
            ],
        ),
        (
            ("EXPRTAIL", Type::Gt),
            vec![
                Production::NonTerm("RELOP"),
                Production::NonTerm("ARITHEXPR"),
            ],
        ),
        (
            ("EXPRTAIL", Type::Lt),
            vec![
                Production::NonTerm("RELOP"),
                Production::NonTerm("ARITHEXPR"),
            ],
        ),
        (
            ("EXPRTAIL", Type::NotEq),
            vec![
                Production::NonTerm("RELOP"),
                Production::NonTerm("ARITHEXPR"),
            ],
        ),
        (
            ("EXPRTAIL", Type::Eq),
            vec![
                Production::NonTerm("RELOP"),
                Production::NonTerm("ARITHEXPR"),
            ],
        ),
        (
            ("RELEXPR", Type::Id("".to_owned())),
            vec![
                Production::NonTerm("ARITHEXPR"),
                Production::NonTerm("RELOP"),
                Production::NonTerm("ARITHEXPR"),
            ],
        ),
        (
            ("RELEXPR", Type::OpenPar),
            vec![
                Production::NonTerm("ARITHEXPR"),
                Production::NonTerm("RELOP"),
                Production::NonTerm("ARITHEXPR"),
            ],
        ),
        (
            ("RELEXPR", Type::Minus),
            vec![
                Production::NonTerm("ARITHEXPR"),
                Production::NonTerm("RELOP"),
                Production::NonTerm("ARITHEXPR"),
            ],
        ),
        (
            ("RELEXPR", Type::Plus),
            vec![
                Production::NonTerm("ARITHEXPR"),
                Production::NonTerm("RELOP"),
                Production::NonTerm("ARITHEXPR"),
            ],
        ),
        (
            ("RELEXPR", Type::Not),
            vec![
                Production::NonTerm("ARITHEXPR"),
                Production::NonTerm("RELOP"),
                Production::NonTerm("ARITHEXPR"),
            ],
        ),
        (
            ("RELEXPR", Type::FloatNum(0f64)),
            vec![
                Production::NonTerm("ARITHEXPR"),
                Production::NonTerm("RELOP"),
                Production::NonTerm("ARITHEXPR"),
            ],
        ),
        (
            ("RELEXPR", Type::IntNum(0)),
            vec![
                Production::NonTerm("ARITHEXPR"),
                Production::NonTerm("RELOP"),
                Production::NonTerm("ARITHEXPR"),
            ],
        ),
        (("RELOP", Type::GEq), vec![Production::Term(Type::GEq)]),
        (("RELOP", Type::LEq), vec![Production::Term(Type::LEq)]),
        (("RELOP", Type::Gt), vec![Production::Term(Type::Gt)]),
        (("RELOP", Type::Lt), vec![Production::Term(Type::Lt)]),
        (("RELOP", Type::NotEq), vec![Production::Term(Type::NotEq)]),
        (("RELOP", Type::Eq), vec![Production::Term(Type::Eq)]),
        (
            ("VARIABLE_OR_FUNCTIONCALL", Type::Id("".to_owned())),
            vec![
                Production::NonTerm("CALL_OR_INDEXED_ID"),
                Production::NonTerm("REP_CI_ID0"),
            ],
        ),
        (
            ("FACTOR", Type::Id("".to_owned())),
            vec![Production::NonTerm("VARIABLE_OR_FUNCTIONCALL")],
        ),
        (
            ("FACTOR", Type::OpenPar),
            vec![
                Production::Term(Type::OpenPar),
                Production::NonTerm("ARITHEXPR"),
                Production::Term(Type::ClosePar),
            ],
        ),
        (
            ("FACTOR", Type::Minus),
            vec![Production::NonTerm("SIGN"), Production::NonTerm("FACTOR")],
        ),
        (
            ("FACTOR", Type::Plus),
            vec![Production::NonTerm("SIGN"), Production::NonTerm("FACTOR")],
        ),
        (
            ("FACTOR", Type::Not),
            vec![Production::Term(Type::Not), Production::NonTerm("FACTOR")],
        ),
        (
            ("FACTOR", Type::FloatNum(0f64)),
            vec![Production::Term(Type::FloatNum(0f64))],
        ),
        (
            ("FACTOR", Type::IntNum(0)),
            vec![Production::Term(Type::IntNum(0))],
        ),
        (
            ("FPARAMS", Type::Id("".to_owned())),
            vec![
                Production::Term(Type::Id("".to_owned())),
                Production::Term(Type::Colon),
                Production::NonTerm("TYPE"),
                Production::NonTerm("REPTFPARAMS3"),
                Production::NonTerm("REPTFPARAMS4"),
            ],
        ),
        (("FPARAMS", Type::ClosePar), vec![]),
        (
            ("FPARAMSTAIL", Type::Comma),
            vec![
                Production::Term(Type::Comma),
                Production::Term(Type::Id("".to_owned())),
                Production::Term(Type::Colon),
                Production::NonTerm("TYPE"),
                Production::NonTerm("REPTFPARAMSTAIL4"),
            ],
        ),
        (
            ("FUNCBODY", Type::OpenCubr),
            vec![
                Production::Term(Type::OpenCubr),
                Production::NonTerm("REPTFUNCBODY1"),
                Production::Term(Type::CloseCubr),
            ],
        ),
        (
            ("FUNCDEF", Type::Function),
            vec![
                Production::NonTerm("FUNCHEAD"),
                Production::NonTerm("FUNCBODY"),
            ],
        ),
        (
            ("FUNCHEAD", Type::Function),
            vec![
                Production::Term(Type::Function),
                Production::Term(Type::Id("".to_owned())),
                Production::NonTerm("FUNCHEADTAIL"),
            ],
        ),
        (
            ("FUNCHEADTAIL", Type::OpenPar),
            vec![
                Production::NonTerm("FUNCHEADPARAMS"),
                Production::Term(Type::ReturnType),
                Production::NonTerm("RETURNTYPE"),
            ],
        ),
        (
            ("FUNCHEADTAIL", Type::ScopeOp),
            vec![Production::NonTerm("FUNCHEADSR")],
        ),
        (
            ("FUNCHEADPARAMS", Type::OpenPar),
            vec![
                Production::Term(Type::OpenPar),
                Production::NonTerm("FPARAMS"),
                Production::Term(Type::ClosePar),
            ],
        ),
        (
            ("FUNCHEADSR", Type::ScopeOp),
            vec![
                Production::Term(Type::ScopeOp),
                Production::NonTerm("FUNCHEADSRTAIL"),
            ],
        ),
        (
            ("FUNCHEADSRTAIL", Type::Id("".to_owned())),
            vec![
                Production::Term(Type::Id("".to_owned())),
                Production::NonTerm("FUNCHEADPARAMS"),
                Production::Term(Type::ReturnType),
                Production::NonTerm("RETURNTYPE"),
            ],
        ),
        (
            ("FUNCHEADSRTAIL", Type::Constructor),
            vec![
                Production::Term(Type::Constructor),
                Production::NonTerm("FUNCHEADPARAMS"),
            ],
        ),
        (("REP_CI_ID0", Type::Id("".to_owned())), vec![]),
        (
            ("REP_CI_ID0", Type::Dot),
            vec![
                Production::Term(Type::Dot),
                Production::NonTerm("CALL_OR_INDEXED_ID"),
                Production::NonTerm("REP_CI_ID0"),
            ],
        ),
        (("REP_CI_ID0", Type::Semi), vec![]),
        (("REP_CI_ID0", Type::ClosePar), vec![]),
        (("REP_CI_ID0", Type::Minus), vec![]),
        (("REP_CI_ID0", Type::Plus), vec![]),
        (("REP_CI_ID0", Type::Comma), vec![]),
        (("REP_CI_ID0", Type::And), vec![]),
        (("REP_CI_ID0", Type::Div), vec![]),
        (("REP_CI_ID0", Type::Mult), vec![]),
        (("REP_CI_ID0", Type::CloseSqbr), vec![]),
        (("REP_CI_ID0", Type::GEq), vec![]),
        (("REP_CI_ID0", Type::LEq), vec![]),
        (("REP_CI_ID0", Type::Gt), vec![]),
        (("REP_CI_ID0", Type::Lt), vec![]),
        (("REP_CI_ID0", Type::NotEq), vec![]),
        (("REP_CI_ID0", Type::Eq), vec![]),
        (("REP_CI_ID0", Type::Or), vec![]),
        (
            ("CALL", Type::OpenPar),
            vec![
                Production::Term(Type::OpenPar),
                Production::NonTerm("APARAMS"),
                Production::Term(Type::ClosePar),
            ],
        ),
        (("INDEXED", Type::Id("".to_owned())), vec![]),
        (("INDEXED", Type::Dot), vec![]),
        (("INDEXED", Type::Semi), vec![]),
        (("INDEXED", Type::ClosePar), vec![]),
        (("INDEXED", Type::Minus), vec![]),
        (("INDEXED", Type::Plus), vec![]),
        (("INDEXED", Type::Comma), vec![]),
        (("INDEXED", Type::And), vec![]),
        (("INDEXED", Type::Div), vec![]),
        (("INDEXED", Type::Mult), vec![]),
        (("INDEXED", Type::CloseSqbr), vec![]),
        (
            ("INDEXED", Type::OpenSqbr),
            vec![
                Production::NonTerm("INDICE"),
                Production::NonTerm("INDEXED"),
            ],
        ),
        (("INDEXED", Type::GEq), vec![]),
        (("INDEXED", Type::LEq), vec![]),
        (("INDEXED", Type::Gt), vec![]),
        (("INDEXED", Type::Lt), vec![]),
        (("INDEXED", Type::NotEq), vec![]),
        (("INDEXED", Type::Eq), vec![]),
        (("INDEXED", Type::Assign), vec![]),
        (("INDEXED", Type::Or), vec![]),
        (
            ("OPT_CALL_OR_INDEXED", Type::Id("".to_owned())),
            vec![Production::NonTerm("INDEXED")],
        ),
        (
            ("OPT_CALL_OR_INDEXED", Type::Dot),
            vec![Production::NonTerm("INDEXED")],
        ),
        (
            ("OPT_CALL_OR_INDEXED", Type::Semi),
            vec![Production::NonTerm("INDEXED")],
        ),
        (
            ("OPT_CALL_OR_INDEXED", Type::ClosePar),
            vec![Production::NonTerm("INDEXED")],
        ),
        (
            ("OPT_CALL_OR_INDEXED", Type::OpenPar),
            vec![Production::NonTerm("CALL")],
        ),
        (
            ("OPT_CALL_OR_INDEXED", Type::Minus),
            vec![Production::NonTerm("INDEXED")],
        ),
        (
            ("OPT_CALL_OR_INDEXED", Type::Plus),
            vec![Production::NonTerm("INDEXED")],
        ),
        (
            ("OPT_CALL_OR_INDEXED", Type::Comma),
            vec![Production::NonTerm("INDEXED")],
        ),
        (
            ("OPT_CALL_OR_INDEXED", Type::And),
            vec![Production::NonTerm("INDEXED")],
        ),
        (
            ("OPT_CALL_OR_INDEXED", Type::Div),
            vec![Production::NonTerm("INDEXED")],
        ),
        (
            ("OPT_CALL_OR_INDEXED", Type::Mult),
            vec![Production::NonTerm("INDEXED")],
        ),
        (
            ("OPT_CALL_OR_INDEXED", Type::CloseSqbr),
            vec![Production::NonTerm("INDEXED")],
        ),
        (
            ("OPT_CALL_OR_INDEXED", Type::OpenSqbr),
            vec![Production::NonTerm("INDEXED")],
        ),
        (
            ("OPT_CALL_OR_INDEXED", Type::GEq),
            vec![Production::NonTerm("INDEXED")],
        ),
        (
            ("OPT_CALL_OR_INDEXED", Type::LEq),
            vec![Production::NonTerm("INDEXED")],
        ),
        (
            ("OPT_CALL_OR_INDEXED", Type::Gt),
            vec![Production::NonTerm("INDEXED")],
        ),
        (
            ("OPT_CALL_OR_INDEXED", Type::Lt),
            vec![Production::NonTerm("INDEXED")],
        ),
        (
            ("OPT_CALL_OR_INDEXED", Type::NotEq),
            vec![Production::NonTerm("INDEXED")],
        ),
        (
            ("OPT_CALL_OR_INDEXED", Type::Eq),
            vec![Production::NonTerm("INDEXED")],
        ),
        (
            ("OPT_CALL_OR_INDEXED", Type::Or),
            vec![Production::NonTerm("INDEXED")],
        ),
        (
            ("CALL_OR_INDEXED_ID", Type::Id("".to_owned())),
            vec![
                Production::Term(Type::Id("".to_owned())),
                Production::NonTerm("OPT_CALL_OR_INDEXED"),
            ],
        ),
        (
            ("INDICE", Type::OpenSqbr),
            vec![
                Production::Term(Type::OpenSqbr),
                Production::NonTerm("ARITHEXPR"),
                Production::Term(Type::CloseSqbr),
            ],
        ),
        (
            ("LOCALVARDECL", Type::LocalVar),
            vec![
                Production::Term(Type::LocalVar),
                Production::Term(Type::Id("".to_owned())),
                Production::Term(Type::Colon),
                Production::NonTerm("TYPE"),
                Production::NonTerm("LOCALVARDECLTAIL"),
            ],
        ),
        (
            ("LOCALVARDECLTAIL", Type::Semi),
            vec![
                Production::NonTerm("REPTLOCALVARDECL4"),
                Production::Term(Type::Semi),
            ],
        ),
        (
            ("LOCALVARDECLTAIL", Type::OpenPar),
            vec![
                Production::Term(Type::OpenPar),
                Production::NonTerm("APARAMS"),
                Production::Term(Type::ClosePar),
                Production::Term(Type::Semi),
            ],
        ),
        (
            ("LOCALVARDECLTAIL", Type::OpenSqbr),
            vec![
                Production::NonTerm("REPTLOCALVARDECL4"),
                Production::Term(Type::Semi),
            ],
        ),
        (
            ("LOCALVARDECLORSTMT", Type::Id("".to_owned())),
            vec![Production::NonTerm("STATEMENT")],
        ),
        (
            ("LOCALVARDECLORSTMT", Type::Return),
            vec![Production::NonTerm("STATEMENT")],
        ),
        (
            ("LOCALVARDECLORSTMT", Type::Write),
            vec![Production::NonTerm("STATEMENT")],
        ),
        (
            ("LOCALVARDECLORSTMT", Type::Read),
            vec![Production::NonTerm("STATEMENT")],
        ),
        (
            ("LOCALVARDECLORSTMT", Type::While),
            vec![Production::NonTerm("STATEMENT")],
        ),
        (
            ("LOCALVARDECLORSTMT", Type::If),
            vec![Production::NonTerm("STATEMENT")],
        ),
        (
            ("LOCALVARDECLORSTMT", Type::LocalVar),
            vec![Production::NonTerm("LOCALVARDECL")],
        ),
        (
            ("MEMBERDECL", Type::Attribute),
            vec![Production::NonTerm("MEMBERVARDECL")],
        ),
        (
            ("MEMBERDECL", Type::Constructor),
            vec![Production::NonTerm("MEMBERFUNCDECL")],
        ),
        (
            ("MEMBERDECL", Type::Function),
            vec![Production::NonTerm("MEMBERFUNCDECL")],
        ),
        (
            ("MEMBERFUNCDECL", Type::Constructor),
            vec![
                Production::Term(Type::Constructor),
                Production::Term(Type::Colon),
                Production::Term(Type::OpenPar),
                Production::NonTerm("FPARAMS"),
                Production::Term(Type::ClosePar),
                Production::Term(Type::Semi),
            ],
        ),
        (
            ("MEMBERFUNCDECL", Type::Function),
            vec![
                Production::Term(Type::Function),
                Production::Term(Type::Id("".to_owned())),
                Production::Term(Type::Colon),
                Production::Term(Type::OpenPar),
                Production::NonTerm("FPARAMS"),
                Production::Term(Type::ClosePar),
                Production::Term(Type::ReturnType),
                Production::NonTerm("RETURNTYPE"),
                Production::Term(Type::Semi),
            ],
        ),
        (
            ("MEMBERVARDECL", Type::Attribute),
            vec![
                Production::Term(Type::Attribute),
                Production::Term(Type::Id("".to_owned())),
                Production::Term(Type::Colon),
                Production::NonTerm("TYPE"),
                Production::NonTerm("REPTMEMBERVARDECL4"),
                Production::Term(Type::Semi),
            ],
        ),
        (("MULTOP", Type::And), vec![Production::Term(Type::And)]),
        (("MULTOP", Type::Div), vec![Production::Term(Type::Div)]),
        (("MULTOP", Type::Mult), vec![Production::Term(Type::Mult)]),
        (("OPTCLASSDECL2", Type::OpenCubr), vec![]),
        (
            ("OPTCLASSDECL2", Type::IsA),
            vec![
                Production::Term(Type::IsA),
                Production::Term(Type::Id("".to_owned())),
                Production::NonTerm("REPTOPTCLASSDECL22"),
            ],
        ),
        (("REPTAPARAMS1", Type::ClosePar), vec![]),
        (
            ("REPTAPARAMS1", Type::Comma),
            vec![
                Production::NonTerm("APARAMSTAIL"),
                Production::NonTerm("REPTAPARAMS1"),
            ],
        ),
        (
            ("REPTCLASSDECL4", Type::Private),
            vec![
                Production::NonTerm("VISIBILITY"),
                Production::NonTerm("MEMBERDECL"),
                Production::NonTerm("REPTCLASSDECL4"),
            ],
        ),
        (
            ("REPTCLASSDECL4", Type::Public),
            vec![
                Production::NonTerm("VISIBILITY"),
                Production::NonTerm("MEMBERDECL"),
                Production::NonTerm("REPTCLASSDECL4"),
            ],
        ),
        (("REPTCLASSDECL4", Type::CloseCubr), vec![]),
        (
            ("REPTCLASSDECL4", Type::Attribute),
            vec![
                Production::NonTerm("VISIBILITY"),
                Production::NonTerm("MEMBERDECL"),
                Production::NonTerm("REPTCLASSDECL4"),
            ],
        ),
        (
            ("REPTCLASSDECL4", Type::Constructor),
            vec![
                Production::NonTerm("VISIBILITY"),
                Production::NonTerm("MEMBERDECL"),
                Production::NonTerm("REPTCLASSDECL4"),
            ],
        ),
        (
            ("REPTCLASSDECL4", Type::Function),
            vec![
                Production::NonTerm("VISIBILITY"),
                Production::NonTerm("MEMBERDECL"),
                Production::NonTerm("REPTCLASSDECL4"),
            ],
        ),
        (("REPTFPARAMS3", Type::ClosePar), vec![]),
        (("REPTFPARAMS3", Type::Comma), vec![]),
        (
            ("REPTFPARAMS3", Type::OpenSqbr),
            vec![
                Production::NonTerm("ARRAYSIZE"),
                Production::NonTerm("REPTFPARAMS3"),
            ],
        ),
        (("REPTFPARAMS4", Type::ClosePar), vec![]),
        (
            ("REPTFPARAMS4", Type::Comma),
            vec![
                Production::NonTerm("FPARAMSTAIL"),
                Production::NonTerm("REPTFPARAMS4"),
            ],
        ),
        (("REPTFPARAMSTAIL4", Type::ClosePar), vec![]),
        (("REPTFPARAMSTAIL4", Type::Comma), vec![]),
        (
            ("REPTFPARAMSTAIL4", Type::OpenSqbr),
            vec![
                Production::NonTerm("ARRAYSIZE"),
                Production::NonTerm("REPTFPARAMSTAIL4"),
            ],
        ),
        (
            ("REPTFUNCBODY1", Type::Id("".to_owned())),
            vec![
                Production::NonTerm("LOCALVARDECLORSTMT"),
                Production::NonTerm("REPTFUNCBODY1"),
            ],
        ),
        (
            ("REPTFUNCBODY1", Type::Return),
            vec![
                Production::NonTerm("LOCALVARDECLORSTMT"),
                Production::NonTerm("REPTFUNCBODY1"),
            ],
        ),
        (
            ("REPTFUNCBODY1", Type::Write),
            vec![
                Production::NonTerm("LOCALVARDECLORSTMT"),
                Production::NonTerm("REPTFUNCBODY1"),
            ],
        ),
        (
            ("REPTFUNCBODY1", Type::Read),
            vec![
                Production::NonTerm("LOCALVARDECLORSTMT"),
                Production::NonTerm("REPTFUNCBODY1"),
            ],
        ),
        (
            ("REPTFUNCBODY1", Type::While),
            vec![
                Production::NonTerm("LOCALVARDECLORSTMT"),
                Production::NonTerm("REPTFUNCBODY1"),
            ],
        ),
        (
            ("REPTFUNCBODY1", Type::If),
            vec![
                Production::NonTerm("LOCALVARDECLORSTMT"),
                Production::NonTerm("REPTFUNCBODY1"),
            ],
        ),
        (("REPTFUNCBODY1", Type::CloseCubr), vec![]),
        (
            ("REPTFUNCBODY1", Type::LocalVar),
            vec![
                Production::NonTerm("LOCALVARDECLORSTMT"),
                Production::NonTerm("REPTFUNCBODY1"),
            ],
        ),
        (("REPTLOCALVARDECL4", Type::Semi), vec![]),
        (
            ("REPTLOCALVARDECL4", Type::OpenSqbr),
            vec![
                Production::NonTerm("ARRAYSIZE"),
                Production::NonTerm("REPTLOCALVARDECL4"),
            ],
        ),
        (("REPTMEMBERVARDECL4", Type::Semi), vec![]),
        (
            ("REPTMEMBERVARDECL4", Type::OpenSqbr),
            vec![
                Production::NonTerm("ARRAYSIZE"),
                Production::NonTerm("REPTMEMBERVARDECL4"),
            ],
        ),
        (("REPTOPTCLASSDECL22", Type::OpenCubr), vec![]),
        (
            ("REPTOPTCLASSDECL22", Type::Comma),
            vec![
                Production::Term(Type::Comma),
                Production::Term(Type::Id("".to_owned())),
                Production::NonTerm("REPTOPTCLASSDECL22"),
            ],
        ),
        (("REPTSTART0", Type::EndOfFile), vec![]),
        (
            ("REPTSTART0", Type::Function),
            vec![
                Production::NonTerm("CLASSDECLORFUNCDEF"),
                Production::NonTerm("REPTSTART0"),
            ],
        ),
        (
            ("REPTSTART0", Type::Class),
            vec![
                Production::NonTerm("CLASSDECLORFUNCDEF"),
                Production::NonTerm("REPTSTART0"),
            ],
        ),
        (
            ("REPTSTATBLOCK1", Type::Id("".to_owned())),
            vec![
                Production::NonTerm("STATEMENT"),
                Production::NonTerm("REPTSTATBLOCK1"),
            ],
        ),
        (
            ("REPTSTATBLOCK1", Type::Return),
            vec![
                Production::NonTerm("STATEMENT"),
                Production::NonTerm("REPTSTATBLOCK1"),
            ],
        ),
        (
            ("REPTSTATBLOCK1", Type::Write),
            vec![
                Production::NonTerm("STATEMENT"),
                Production::NonTerm("REPTSTATBLOCK1"),
            ],
        ),
        (
            ("REPTSTATBLOCK1", Type::Read),
            vec![
                Production::NonTerm("STATEMENT"),
                Production::NonTerm("REPTSTATBLOCK1"),
            ],
        ),
        (
            ("REPTSTATBLOCK1", Type::While),
            vec![
                Production::NonTerm("STATEMENT"),
                Production::NonTerm("REPTSTATBLOCK1"),
            ],
        ),
        (
            ("REPTSTATBLOCK1", Type::If),
            vec![
                Production::NonTerm("STATEMENT"),
                Production::NonTerm("REPTSTATBLOCK1"),
            ],
        ),
        (("REPTSTATBLOCK1", Type::CloseCubr), vec![]),
        (
            ("RETURNTYPE", Type::Id("".to_owned())),
            vec![Production::NonTerm("TYPE")],
        ),
        (
            ("RETURNTYPE", Type::Float),
            vec![Production::NonTerm("TYPE")],
        ),
        (
            ("RETURNTYPE", Type::Integer),
            vec![Production::NonTerm("TYPE")],
        ),
        (
            ("RETURNTYPE", Type::Void),
            vec![Production::Term(Type::Void)],
        ),
        (("RIGHTRECARITHEXPR", Type::Semi), vec![]),
        (("RIGHTRECARITHEXPR", Type::ClosePar), vec![]),
        (
            ("RIGHTRECARITHEXPR", Type::Minus),
            vec![
                Production::NonTerm("ADDOP"),
                Production::NonTerm("TERM"),
                Production::NonTerm("RIGHTRECARITHEXPR"),
            ],
        ),
        (
            ("RIGHTRECARITHEXPR", Type::Plus),
            vec![
                Production::NonTerm("ADDOP"),
                Production::NonTerm("TERM"),
                Production::NonTerm("RIGHTRECARITHEXPR"),
            ],
        ),
        (("RIGHTRECARITHEXPR", Type::Comma), vec![]),
        (("RIGHTRECARITHEXPR", Type::CloseSqbr), vec![]),
        (("RIGHTRECARITHEXPR", Type::GEq), vec![]),
        (("RIGHTRECARITHEXPR", Type::LEq), vec![]),
        (("RIGHTRECARITHEXPR", Type::Gt), vec![]),
        (("RIGHTRECARITHEXPR", Type::Lt), vec![]),
        (("RIGHTRECARITHEXPR", Type::NotEq), vec![]),
        (("RIGHTRECARITHEXPR", Type::Eq), vec![]),
        (
            ("RIGHTRECARITHEXPR", Type::Or),
            vec![
                Production::NonTerm("ADDOP"),
                Production::NonTerm("TERM"),
                Production::NonTerm("RIGHTRECARITHEXPR"),
            ],
        ),
        (("RIGHTRECTERM", Type::Semi), vec![]),
        (("RIGHTRECTERM", Type::ClosePar), vec![]),
        (("RIGHTRECTERM", Type::Minus), vec![]),
        (("RIGHTRECTERM", Type::Plus), vec![]),
        (("RIGHTRECTERM", Type::Comma), vec![]),
        (
            ("RIGHTRECTERM", Type::And),
            vec![
                Production::NonTerm("MULTOP"),
                Production::NonTerm("FACTOR"),
                Production::NonTerm("RIGHTRECTERM"),
            ],
        ),
        (
            ("RIGHTRECTERM", Type::Div),
            vec![
                Production::NonTerm("MULTOP"),
                Production::NonTerm("FACTOR"),
                Production::NonTerm("RIGHTRECTERM"),
            ],
        ),
        (
            ("RIGHTRECTERM", Type::Mult),
            vec![
                Production::NonTerm("MULTOP"),
                Production::NonTerm("FACTOR"),
                Production::NonTerm("RIGHTRECTERM"),
            ],
        ),
        (("RIGHTRECTERM", Type::CloseSqbr), vec![]),
        (("RIGHTRECTERM", Type::GEq), vec![]),
        (("RIGHTRECTERM", Type::LEq), vec![]),
        (("RIGHTRECTERM", Type::Gt), vec![]),
        (("RIGHTRECTERM", Type::Lt), vec![]),
        (("RIGHTRECTERM", Type::NotEq), vec![]),
        (("RIGHTRECTERM", Type::Eq), vec![]),
        (("RIGHTRECTERM", Type::Or), vec![]),
        (("SIGN", Type::Minus), vec![Production::Term(Type::Minus)]),
        (("SIGN", Type::Plus), vec![Production::Term(Type::Plus)]),
        (
            ("STATBLOCK", Type::Id("".to_owned())),
            vec![Production::NonTerm("STATEMENT")],
        ),
        (("STATBLOCK", Type::Semi), vec![]),
        (
            ("STATBLOCK", Type::Return),
            vec![Production::NonTerm("STATEMENT")],
        ),
        (
            ("STATBLOCK", Type::Write),
            vec![Production::NonTerm("STATEMENT")],
        ),
        (
            ("STATBLOCK", Type::Read),
            vec![Production::NonTerm("STATEMENT")],
        ),
        (
            ("STATBLOCK", Type::While),
            vec![Production::NonTerm("STATEMENT")],
        ),
        (("STATBLOCK", Type::Else), vec![]),
        (
            ("STATBLOCK", Type::If),
            vec![Production::NonTerm("STATEMENT")],
        ),
        (
            ("STATBLOCK", Type::OpenCubr),
            vec![
                Production::Term(Type::OpenCubr),
                Production::NonTerm("REPTSTATBLOCK1"),
                Production::Term(Type::CloseCubr),
            ],
        ),
        (
            ("STATEMENT", Type::Id("".to_owned())),
            vec![
                Production::NonTerm("ASSIGN_OR_FUNCTIONCALL"),
                Production::Term(Type::Semi),
            ],
        ),
        (
            ("STATEMENT", Type::Return),
            vec![
                Production::Term(Type::Return),
                Production::Term(Type::OpenPar),
                Production::NonTerm("EXPR"),
                Production::Term(Type::ClosePar),
                Production::Term(Type::Semi),
            ],
        ),
        (
            ("STATEMENT", Type::Write),
            vec![
                Production::Term(Type::Write),
                Production::Term(Type::OpenPar),
                Production::NonTerm("EXPR"),
                Production::Term(Type::ClosePar),
                Production::Term(Type::Semi),
            ],
        ),
        (
            ("STATEMENT", Type::Read),
            vec![
                Production::Term(Type::Read),
                Production::Term(Type::OpenPar),
                Production::NonTerm("VARIABLE"),
                Production::Term(Type::ClosePar),
                Production::Term(Type::Semi),
            ],
        ),
        (
            ("STATEMENT", Type::While),
            vec![
                Production::Term(Type::While),
                Production::Term(Type::OpenPar),
                Production::NonTerm("RELEXPR"),
                Production::Term(Type::ClosePar),
                Production::NonTerm("STATBLOCK"),
                Production::Term(Type::Semi),
            ],
        ),
        (
            ("STATEMENT", Type::If),
            vec![
                Production::Term(Type::If),
                Production::Term(Type::OpenPar),
                Production::NonTerm("RELEXPR"),
                Production::Term(Type::ClosePar),
                Production::Term(Type::Then),
                Production::NonTerm("STATBLOCK"),
                Production::Term(Type::Else),
                Production::NonTerm("STATBLOCK"),
                Production::Term(Type::Semi),
            ],
        ),
        (
            ("ASSIGN_OR_FUNCTIONCALL", Type::Id("".to_owned())),
            vec![
                Production::Term(Type::Id("".to_owned())),
                Production::NonTerm("ASSIGN_OR_FUNCTIONCALL_TAIL"),
            ],
        ),
        (
            ("ASSIGN_OR_FUNCTIONCALL_TAIL", Type::Dot),
            vec![
                Production::NonTerm("INDEXED"),
                Production::NonTerm("AF_INDEXED_TAIL"),
            ],
        ),
        (
            ("ASSIGN_OR_FUNCTIONCALL_TAIL", Type::OpenPar),
            vec![
                Production::NonTerm("CALL"),
                Production::NonTerm("AF_CALL_TAIL"),
            ],
        ),
        (
            ("ASSIGN_OR_FUNCTIONCALL_TAIL", Type::OpenSqbr),
            vec![
                Production::NonTerm("INDEXED"),
                Production::NonTerm("AF_INDEXED_TAIL"),
            ],
        ),
        (
            ("ASSIGN_OR_FUNCTIONCALL_TAIL", Type::Assign),
            vec![
                Production::NonTerm("INDEXED"),
                Production::NonTerm("AF_INDEXED_TAIL"),
            ],
        ),
        (
            ("AF_INDEXED_TAIL", Type::Dot),
            vec![
                Production::Term(Type::Dot),
                Production::NonTerm("ASSIGN_OR_FUNCTIONCALL"),
            ],
        ),
        (
            ("AF_INDEXED_TAIL", Type::Assign),
            vec![Production::NonTerm("ASSIGNOP"), Production::NonTerm("EXPR")],
        ),
        (
            ("AF_CALL_TAIL", Type::Dot),
            vec![
                Production::Term(Type::Dot),
                Production::NonTerm("ASSIGN_OR_FUNCTIONCALL"),
            ],
        ),
        (("AF_CALL_TAIL", Type::Semi), vec![]),
        (
            ("TERM", Type::Id("".to_owned())),
            vec![
                Production::NonTerm("FACTOR"),
                Production::NonTerm("RIGHTRECTERM"),
            ],
        ),
        (
            ("TERM", Type::OpenPar),
            vec![
                Production::NonTerm("FACTOR"),
                Production::NonTerm("RIGHTRECTERM"),
            ],
        ),
        (
            ("TERM", Type::Minus),
            vec![
                Production::NonTerm("FACTOR"),
                Production::NonTerm("RIGHTRECTERM"),
            ],
        ),
        (
            ("TERM", Type::Plus),
            vec![
                Production::NonTerm("FACTOR"),
                Production::NonTerm("RIGHTRECTERM"),
            ],
        ),
        (
            ("TERM", Type::Not),
            vec![
                Production::NonTerm("FACTOR"),
                Production::NonTerm("RIGHTRECTERM"),
            ],
        ),
        (
            ("TERM", Type::FloatNum(0f64)),
            vec![
                Production::NonTerm("FACTOR"),
                Production::NonTerm("RIGHTRECTERM"),
            ],
        ),
        (
            ("TERM", Type::IntNum(0)),
            vec![
                Production::NonTerm("FACTOR"),
                Production::NonTerm("RIGHTRECTERM"),
            ],
        ),
        (
            ("TYPE", Type::Id("".to_owned())),
            vec![Production::Term(Type::Id("".to_owned()))],
        ),
        (("TYPE", Type::Float), vec![Production::Term(Type::Float)]),
        (
            ("TYPE", Type::Integer),
            vec![Production::Term(Type::Integer)],
        ),
        (
            ("VARIABLE", Type::Id("".to_owned())),
            vec![
                Production::NonTerm("CALL_OR_INDEXED_ID"),
                Production::NonTerm("REP_CI_ID0"),
                Production::Term(Type::Id("".to_owned())),
                Production::NonTerm("INDEXED"),
            ],
        ),
        (
            ("VISIBILITY", Type::Private),
            vec![Production::Term(Type::Private)],
        ),
        (
            ("VISIBILITY", Type::Public),
            vec![Production::Term(Type::Public)],
        ),
        (("VISIBILITY", Type::Attribute), vec![]),
        (("VISIBILITY", Type::Constructor), vec![]),
        (("VISIBILITY", Type::Function), vec![]),
    ])
}
