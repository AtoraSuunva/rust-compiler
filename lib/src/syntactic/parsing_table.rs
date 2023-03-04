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
        ("START", vec![Type::EndOfFile, Type::Class, Type::Function]),
        ("ARRAYSIZE2", vec![Type::IntNum(0), Type::CloseSqbr]),
        ("CLASSDECL", vec![Type::Class]),
        (
            "EXPR2",
            vec![
                Type::Eq,
                Type::NotEq,
                Type::Lt,
                Type::Gt,
                Type::LEq,
                Type::GEq,
            ],
        ),
        ("FACTOR2", vec![Type::OpenPar, Type::OpenSqbr]),
        ("FUNCDEF", vec![Type::Function]),
        ("FUNCBODY", vec![Type::OpenCubr]),
        ("FUNCHEAD", vec![Type::Function]),
        ("FUNCHEADTAIL", vec![Type::ScopeOp, Type::OpenPar]),
        (
            "FUNCHEADMEMBERTAIL",
            vec![Type::Id("".to_owned()), Type::Constructor],
        ),
        ("IDNEST2", vec![Type::OpenPar, Type::OpenSqbr]),
        ("ARRAYOROBJECT", vec![Type::OpenPar, Type::OpenSqbr]),
        ("LOCALVARDECL", vec![Type::LocalVar]),
        ("MEMBERFUNCDECL", vec![Type::Function, Type::Constructor]),
        ("MEMBERFUNCHEAD", vec![Type::Function, Type::Constructor]),
        ("FPARAMS", vec![Type::Id("".to_owned())]),
        ("MEMBERVARDECL", vec![Type::Attribute]),
        ("OPTINHERITS", vec![Type::IsA]),
        ("PROG", vec![Type::Class, Type::Function]),
        (
            "ARITHEXPR",
            vec![
                Type::Id("".to_owned()),
                Type::IntNum(0),
                Type::FloatNum(0f64),
                Type::OpenPar,
                Type::Not,
                Type::Plus,
                Type::Minus,
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
        ("APARAMSTAIL", vec![Type::Comma]),
        ("REPTAPARAMS1", vec![Type::Comma]),
        ("REPTARRAYSIZE", vec![Type::OpenSqbr]),
        ("REPTFPARAMS3", vec![Type::OpenSqbr]),
        ("FPARAMSTAIL", vec![Type::Comma]),
        ("REPTFPARAMS4", vec![Type::Comma]),
        ("ARRAYSIZE", vec![Type::OpenSqbr]),
        ("REPTFPARAMSTAIL4", vec![Type::OpenSqbr]),
        ("REPTINHERITSLIST", vec![Type::Comma]),
        (
            "LOCALVARORSTAT",
            vec![
                Type::LocalVar,
                Type::Id("".to_owned()),
                Type::If,
                Type::While,
                Type::Read,
                Type::Write,
                Type::Return,
            ],
        ),
        (
            "REPTLOCALVARORSTAT",
            vec![
                Type::LocalVar,
                Type::Id("".to_owned()),
                Type::If,
                Type::While,
                Type::Read,
                Type::Write,
                Type::Return,
            ],
        ),
        (
            "MEMBERDECL",
            vec![Type::Attribute, Type::Function, Type::Constructor],
        ),
        ("REPTMEMBERDECL", vec![Type::Public, Type::Private]),
        ("CLASSDECLORFUNCDEF", vec![Type::Class, Type::Function]),
        ("REPTPROG0", vec![Type::Class, Type::Function]),
        ("IDNEST", vec![Type::Dot]),
        ("REPTVARIABLEORFUNCTIONCALL", vec![Type::Dot]),
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
                Type::Id("".to_owned()),
                Type::If,
                Type::While,
                Type::Read,
                Type::Write,
                Type::Return,
            ],
        ),
        (
            "STATEMENT",
            vec![
                Type::Id("".to_owned()),
                Type::If,
                Type::While,
                Type::Read,
                Type::Write,
                Type::Return,
            ],
        ),
        (
            "RELEXPR",
            vec![
                Type::Id("".to_owned()),
                Type::IntNum(0),
                Type::FloatNum(0f64),
                Type::OpenPar,
                Type::Not,
                Type::Plus,
                Type::Minus,
            ],
        ),
        (
            "STATBLOCK",
            vec![
                Type::OpenCubr,
                Type::Id("".to_owned()),
                Type::If,
                Type::While,
                Type::Read,
                Type::Write,
                Type::Return,
            ],
        ),
        ("INDICE", vec![Type::OpenSqbr]),
        ("STATEMENTIDNEST2", vec![Type::Dot]),
        ("STATEMENTIDNEST3", vec![Type::Dot, Type::Assign]),
        ("ASSIGNOP", vec![Type::Assign]),
        (
            "EXPR",
            vec![
                Type::Id("".to_owned()),
                Type::IntNum(0),
                Type::FloatNum(0f64),
                Type::OpenPar,
                Type::Not,
                Type::Plus,
                Type::Minus,
            ],
        ),
        (
            "STATEMENTIDNEST",
            vec![Type::Dot, Type::OpenPar, Type::OpenSqbr, Type::Assign],
        ),
        (
            "TERM",
            vec![
                Type::Id("".to_owned()),
                Type::IntNum(0),
                Type::FloatNum(0f64),
                Type::OpenPar,
                Type::Not,
                Type::Plus,
                Type::Minus,
            ],
        ),
        (
            "FACTOR",
            vec![
                Type::Id("".to_owned()),
                Type::IntNum(0),
                Type::FloatNum(0f64),
                Type::OpenPar,
                Type::Not,
                Type::Plus,
                Type::Minus,
            ],
        ),
        ("RIGHTRECTERM", vec![Type::Mult, Type::Div, Type::And]),
        (
            "TYPE",
            vec![Type::Integer, Type::Float, Type::Id("".to_owned())],
        ),
        ("VARIABLE", vec![Type::Id("".to_owned())]),
        ("VARIABLE2", vec![Type::OpenPar, Type::OpenSqbr, Type::Dot]),
        ("REPTVARIABLE", vec![Type::Dot]),
        ("VARIDNEST2", vec![Type::OpenPar, Type::OpenSqbr]),
        (
            "APARAMS",
            vec![
                Type::Id("".to_owned()),
                Type::IntNum(0),
                Type::FloatNum(0f64),
                Type::OpenPar,
                Type::Not,
                Type::Plus,
                Type::Minus,
            ],
        ),
        ("VARIDNEST", vec![Type::Dot]),
        ("REPTIDNEST1", vec![Type::OpenSqbr]),
        ("VISIBILITY", vec![Type::Public, Type::Private]),
    ])
}

pub fn get_follow_set_table() -> HashMap<&'static str, Vec<Type>> {
    HashMap::from([
        ("START", vec![]),
        (
            "ARRAYSIZE2",
            vec![Type::OpenSqbr, Type::Semi, Type::ClosePar, Type::Comma],
        ),
        (
            "CLASSDECL",
            vec![Type::Class, Type::Function, Type::EndOfFile],
        ),
        ("EXPR2", vec![Type::Semi, Type::Comma, Type::ClosePar]),
        (
            "FACTOR2",
            vec![
                Type::Semi,
                Type::Mult,
                Type::Div,
                Type::And,
                Type::Dot,
                Type::CloseSqbr,
                Type::Eq,
                Type::NotEq,
                Type::Lt,
                Type::Gt,
                Type::LEq,
                Type::GEq,
                Type::Plus,
                Type::Minus,
                Type::Or,
                Type::Comma,
                Type::ClosePar,
            ],
        ),
        (
            "FUNCDEF",
            vec![Type::Class, Type::Function, Type::EndOfFile],
        ),
        (
            "FUNCBODY",
            vec![Type::Class, Type::Function, Type::EndOfFile],
        ),
        ("FUNCHEAD", vec![Type::OpenCubr]),
        ("FUNCHEADTAIL", vec![Type::OpenCubr]),
        ("FUNCHEADMEMBERTAIL", vec![Type::OpenCubr]),
        (
            "IDNEST2",
            vec![
                Type::Semi,
                Type::Mult,
                Type::Div,
                Type::And,
                Type::Dot,
                Type::CloseSqbr,
                Type::Eq,
                Type::NotEq,
                Type::Lt,
                Type::Gt,
                Type::LEq,
                Type::GEq,
                Type::Plus,
                Type::Minus,
                Type::Or,
                Type::Comma,
                Type::ClosePar,
            ],
        ),
        ("ARRAYOROBJECT", vec![Type::Semi]),
        (
            "LOCALVARDECL",
            vec![
                Type::LocalVar,
                Type::Id("".to_owned()),
                Type::If,
                Type::While,
                Type::Read,
                Type::Write,
                Type::Return,
                Type::CloseCubr,
            ],
        ),
        (
            "MEMBERFUNCDECL",
            vec![Type::Public, Type::Private, Type::CloseCubr],
        ),
        ("MEMBERFUNCHEAD", vec![Type::Semi]),
        ("FPARAMS", vec![Type::ClosePar]),
        (
            "MEMBERVARDECL",
            vec![Type::Public, Type::Private, Type::CloseCubr],
        ),
        ("OPTINHERITS", vec![Type::OpenCubr]),
        ("PROG", vec![Type::EndOfFile]),
        (
            "ARITHEXPR",
            vec![
                Type::Semi,
                Type::CloseSqbr,
                Type::Eq,
                Type::NotEq,
                Type::Lt,
                Type::Gt,
                Type::LEq,
                Type::GEq,
                Type::Comma,
                Type::ClosePar,
            ],
        ),
        (
            "RELOP",
            vec![
                Type::Id("".to_owned()),
                Type::IntNum(0),
                Type::FloatNum(0f64),
                Type::OpenPar,
                Type::Not,
                Type::Plus,
                Type::Minus,
            ],
        ),
        ("APARAMSTAIL", vec![Type::Comma, Type::ClosePar]),
        ("REPTAPARAMS1", vec![Type::ClosePar]),
        ("REPTARRAYSIZE", vec![Type::Semi]),
        ("REPTFPARAMS3", vec![Type::ClosePar, Type::Comma]),
        ("FPARAMSTAIL", vec![Type::Comma, Type::ClosePar]),
        ("REPTFPARAMS4", vec![Type::ClosePar]),
        (
            "ARRAYSIZE",
            vec![Type::OpenSqbr, Type::Semi, Type::ClosePar, Type::Comma],
        ),
        ("REPTFPARAMSTAIL4", vec![Type::Comma, Type::ClosePar]),
        ("REPTINHERITSLIST", vec![Type::OpenCubr]),
        (
            "LOCALVARORSTAT",
            vec![
                Type::LocalVar,
                Type::Id("".to_owned()),
                Type::If,
                Type::While,
                Type::Read,
                Type::Write,
                Type::Return,
                Type::CloseCubr,
            ],
        ),
        ("REPTLOCALVARORSTAT", vec![Type::CloseCubr]),
        (
            "MEMBERDECL",
            vec![Type::Public, Type::Private, Type::CloseCubr],
        ),
        ("REPTMEMBERDECL", vec![Type::CloseCubr]),
        (
            "CLASSDECLORFUNCDEF",
            vec![Type::Class, Type::Function, Type::EndOfFile],
        ),
        ("REPTPROG0", vec![Type::EndOfFile]),
        (
            "IDNEST",
            vec![
                Type::Semi,
                Type::Mult,
                Type::Div,
                Type::And,
                Type::Dot,
                Type::CloseSqbr,
                Type::Eq,
                Type::NotEq,
                Type::Lt,
                Type::Gt,
                Type::LEq,
                Type::GEq,
                Type::Plus,
                Type::Minus,
                Type::Or,
                Type::Comma,
                Type::ClosePar,
            ],
        ),
        (
            "REPTVARIABLEORFUNCTIONCALL",
            vec![
                Type::Semi,
                Type::Mult,
                Type::Div,
                Type::And,
                Type::CloseSqbr,
                Type::Eq,
                Type::NotEq,
                Type::Lt,
                Type::Gt,
                Type::LEq,
                Type::GEq,
                Type::Plus,
                Type::Minus,
                Type::Or,
                Type::Comma,
                Type::ClosePar,
            ],
        ),
        ("RETURNTYPE", vec![Type::Semi, Type::OpenCubr]),
        (
            "ADDOP",
            vec![
                Type::Id("".to_owned()),
                Type::IntNum(0),
                Type::FloatNum(0f64),
                Type::OpenPar,
                Type::Not,
                Type::Plus,
                Type::Minus,
            ],
        ),
        (
            "RIGHTRECARITHEXPR",
            vec![
                Type::Semi,
                Type::CloseSqbr,
                Type::Eq,
                Type::NotEq,
                Type::Lt,
                Type::Gt,
                Type::LEq,
                Type::GEq,
                Type::Comma,
                Type::ClosePar,
            ],
        ),
        (
            "MULTOP",
            vec![
                Type::Id("".to_owned()),
                Type::IntNum(0),
                Type::FloatNum(0f64),
                Type::OpenPar,
                Type::Not,
                Type::Plus,
                Type::Minus,
            ],
        ),
        (
            "SIGN",
            vec![
                Type::Id("".to_owned()),
                Type::IntNum(0),
                Type::FloatNum(0f64),
                Type::OpenPar,
                Type::Not,
                Type::Plus,
                Type::Minus,
            ],
        ),
        ("REPTSTATBLOCK1", vec![Type::CloseCubr]),
        (
            "STATEMENT",
            vec![
                Type::Else,
                Type::Semi,
                Type::LocalVar,
                Type::Id("".to_owned()),
                Type::If,
                Type::While,
                Type::Read,
                Type::Write,
                Type::Return,
                Type::CloseCubr,
            ],
        ),
        ("RELEXPR", vec![Type::ClosePar]),
        ("STATBLOCK", vec![Type::Else, Type::Semi]),
        (
            "INDICE",
            vec![
                Type::Semi,
                Type::Mult,
                Type::Div,
                Type::And,
                Type::OpenSqbr,
                Type::Assign,
                Type::Dot,
                Type::CloseSqbr,
                Type::Eq,
                Type::NotEq,
                Type::Lt,
                Type::Gt,
                Type::LEq,
                Type::GEq,
                Type::Plus,
                Type::Minus,
                Type::Or,
                Type::Comma,
                Type::ClosePar,
            ],
        ),
        ("STATEMENTIDNEST2", vec![Type::Semi]),
        ("STATEMENTIDNEST3", vec![Type::Semi]),
        (
            "ASSIGNOP",
            vec![
                Type::Id("".to_owned()),
                Type::IntNum(0),
                Type::FloatNum(0f64),
                Type::OpenPar,
                Type::Not,
                Type::Plus,
                Type::Minus,
            ],
        ),
        ("EXPR", vec![Type::Semi, Type::Comma, Type::ClosePar]),
        ("STATEMENTIDNEST", vec![Type::Semi]),
        (
            "TERM",
            vec![
                Type::Semi,
                Type::CloseSqbr,
                Type::Eq,
                Type::NotEq,
                Type::Lt,
                Type::Gt,
                Type::LEq,
                Type::GEq,
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
                Type::CloseSqbr,
                Type::Eq,
                Type::NotEq,
                Type::Lt,
                Type::Gt,
                Type::LEq,
                Type::GEq,
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
                Type::CloseSqbr,
                Type::Eq,
                Type::NotEq,
                Type::Lt,
                Type::Gt,
                Type::LEq,
                Type::GEq,
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
        ("VARIABLE2", vec![Type::ClosePar]),
        ("REPTVARIABLE", vec![Type::ClosePar]),
        ("VARIDNEST2", vec![Type::ClosePar, Type::Dot]),
        ("APARAMS", vec![Type::ClosePar]),
        ("VARIDNEST", vec![Type::ClosePar, Type::Dot]),
        (
            "REPTIDNEST1",
            vec![
                Type::Assign,
                Type::Semi,
                Type::Mult,
                Type::Div,
                Type::And,
                Type::Dot,
                Type::CloseSqbr,
                Type::Eq,
                Type::NotEq,
                Type::Lt,
                Type::Gt,
                Type::LEq,
                Type::GEq,
                Type::Plus,
                Type::Minus,
                Type::Or,
                Type::Comma,
                Type::ClosePar,
            ],
        ),
        (
            "VISIBILITY",
            vec![Type::Attribute, Type::Function, Type::Constructor],
        ),
    ])
}

pub fn get_parsing_table() -> HashMap<(&'static str, Type), Vec<Production<'static>>> {
    HashMap::from([
        (
            ("START", Type::Function),
            vec![
                Production::NonTerm("PROG"),
                // Production::Term(Type::EndOfFile),
            ],
        ),
        (
            ("START", Type::Class),
            vec![
                Production::NonTerm("PROG"),
                // Production::Term(Type::EndOfFile),
            ],
        ),
        (
            ("START", Type::EndOfFile),
            vec![
                Production::NonTerm("PROG"),
                Production::Term(Type::EndOfFile),
            ],
        ),
        (("ADDOP", Type::Minus), vec![Production::Term(Type::Minus)]),
        (("ADDOP", Type::Plus), vec![Production::Term(Type::Plus)]),
        (("ADDOP", Type::Or), vec![Production::Term(Type::Or)]),
        (("APARAMS", Type::ClosePar), vec![]),
        (
            ("APARAMS", Type::OpenPar),
            vec![
                Production::NonTerm("EXPR"),
                Production::NonTerm("REPTAPARAMS1"),
            ],
        ),
        (
            ("APARAMS", Type::Id("".to_owned())),
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
            ("ARITHEXPR", Type::OpenPar),
            vec![
                Production::NonTerm("TERM"),
                Production::NonTerm("RIGHTRECARITHEXPR"),
            ],
        ),
        (
            ("ARITHEXPR", Type::Id("".to_owned())),
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
            ("ARRAYOROBJECT", Type::OpenPar),
            vec![
                Production::Term(Type::OpenPar),
                Production::NonTerm("APARAMS"),
                Production::Term(Type::ClosePar),
            ],
        ),
        (
            ("ARRAYOROBJECT", Type::Semi),
            vec![Production::NonTerm("REPTARRAYSIZE")],
        ),
        (
            ("ARRAYOROBJECT", Type::OpenSqbr),
            vec![Production::NonTerm("REPTARRAYSIZE")],
        ),
        (
            ("ARRAYSIZE", Type::OpenSqbr),
            vec![
                Production::Term(Type::OpenSqbr),
                Production::NonTerm("ARRAYSIZE2"),
            ],
        ),
        (
            ("ARRAYSIZE2", Type::CloseSqbr),
            vec![Production::Term(Type::CloseSqbr)],
        ),
        (
            ("ARRAYSIZE2", Type::IntNum(0)),
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
                Production::NonTerm("OPTINHERITS"),
                Production::Term(Type::OpenCubr),
                Production::NonTerm("REPTMEMBERDECL"),
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
            ("EXPR", Type::OpenPar),
            vec![
                Production::NonTerm("ARITHEXPR"),
                Production::NonTerm("EXPR2"),
            ],
        ),
        (
            ("EXPR", Type::Id("".to_owned())),
            vec![
                Production::NonTerm("ARITHEXPR"),
                Production::NonTerm("EXPR2"),
            ],
        ),
        (
            ("EXPR", Type::Minus),
            vec![
                Production::NonTerm("ARITHEXPR"),
                Production::NonTerm("EXPR2"),
            ],
        ),
        (
            ("EXPR", Type::Plus),
            vec![
                Production::NonTerm("ARITHEXPR"),
                Production::NonTerm("EXPR2"),
            ],
        ),
        (
            ("EXPR", Type::Not),
            vec![
                Production::NonTerm("ARITHEXPR"),
                Production::NonTerm("EXPR2"),
            ],
        ),
        (
            ("EXPR", Type::FloatNum(0f64)),
            vec![
                Production::NonTerm("ARITHEXPR"),
                Production::NonTerm("EXPR2"),
            ],
        ),
        (
            ("EXPR", Type::IntNum(0)),
            vec![
                Production::NonTerm("ARITHEXPR"),
                Production::NonTerm("EXPR2"),
            ],
        ),
        (("EXPR2", Type::ClosePar), vec![]),
        (("EXPR2", Type::Semi), vec![]),
        (("EXPR2", Type::Comma), vec![]),
        (
            ("EXPR2", Type::GEq),
            vec![
                Production::NonTerm("RELOP"),
                Production::NonTerm("ARITHEXPR"),
            ],
        ),
        (
            ("EXPR2", Type::LEq),
            vec![
                Production::NonTerm("RELOP"),
                Production::NonTerm("ARITHEXPR"),
            ],
        ),
        (
            ("EXPR2", Type::Gt),
            vec![
                Production::NonTerm("RELOP"),
                Production::NonTerm("ARITHEXPR"),
            ],
        ),
        (
            ("EXPR2", Type::Lt),
            vec![
                Production::NonTerm("RELOP"),
                Production::NonTerm("ARITHEXPR"),
            ],
        ),
        (
            ("EXPR2", Type::NotEq),
            vec![
                Production::NonTerm("RELOP"),
                Production::NonTerm("ARITHEXPR"),
            ],
        ),
        (
            ("EXPR2", Type::Eq),
            vec![
                Production::NonTerm("RELOP"),
                Production::NonTerm("ARITHEXPR"),
            ],
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
            ("FACTOR", Type::Id("".to_owned())),
            vec![
                Production::Term(Type::Id("".to_owned())),
                Production::NonTerm("FACTOR2"),
                Production::NonTerm("REPTVARIABLEORFUNCTIONCALL"),
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
            ("FACTOR2", Type::ClosePar),
            vec![Production::NonTerm("REPTIDNEST1")],
        ),
        (
            ("FACTOR2", Type::OpenPar),
            vec![
                Production::Term(Type::OpenPar),
                Production::NonTerm("APARAMS"),
                Production::Term(Type::ClosePar),
            ],
        ),
        (
            ("FACTOR2", Type::Dot),
            vec![Production::NonTerm("REPTIDNEST1")],
        ),
        (
            ("FACTOR2", Type::Semi),
            vec![Production::NonTerm("REPTIDNEST1")],
        ),
        (
            ("FACTOR2", Type::Minus),
            vec![Production::NonTerm("REPTIDNEST1")],
        ),
        (
            ("FACTOR2", Type::Plus),
            vec![Production::NonTerm("REPTIDNEST1")],
        ),
        (
            ("FACTOR2", Type::Comma),
            vec![Production::NonTerm("REPTIDNEST1")],
        ),
        (
            ("FACTOR2", Type::GEq),
            vec![Production::NonTerm("REPTIDNEST1")],
        ),
        (
            ("FACTOR2", Type::LEq),
            vec![Production::NonTerm("REPTIDNEST1")],
        ),
        (
            ("FACTOR2", Type::Gt),
            vec![Production::NonTerm("REPTIDNEST1")],
        ),
        (
            ("FACTOR2", Type::Lt),
            vec![Production::NonTerm("REPTIDNEST1")],
        ),
        (
            ("FACTOR2", Type::NotEq),
            vec![Production::NonTerm("REPTIDNEST1")],
        ),
        (
            ("FACTOR2", Type::Eq),
            vec![Production::NonTerm("REPTIDNEST1")],
        ),
        (
            ("FACTOR2", Type::And),
            vec![Production::NonTerm("REPTIDNEST1")],
        ),
        (
            ("FACTOR2", Type::Div),
            vec![Production::NonTerm("REPTIDNEST1")],
        ),
        (
            ("FACTOR2", Type::Mult),
            vec![Production::NonTerm("REPTIDNEST1")],
        ),
        (
            ("FACTOR2", Type::CloseSqbr),
            vec![Production::NonTerm("REPTIDNEST1")],
        ),
        (
            ("FACTOR2", Type::OpenSqbr),
            vec![Production::NonTerm("REPTIDNEST1")],
        ),
        (
            ("FACTOR2", Type::Or),
            vec![Production::NonTerm("REPTIDNEST1")],
        ),
        (("FPARAMS", Type::ClosePar), vec![]),
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
                Production::NonTerm("REPTLOCALVARORSTAT"),
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
            ("FUNCHEADMEMBERTAIL", Type::Id("".to_owned())),
            vec![
                Production::Term(Type::Id("".to_owned())),
                Production::Term(Type::OpenPar),
                Production::NonTerm("FPARAMS"),
                Production::Term(Type::ClosePar),
                Production::Term(Type::ReturnType),
                Production::NonTerm("RETURNTYPE"),
            ],
        ),
        (
            ("FUNCHEADMEMBERTAIL", Type::Constructor),
            vec![
                Production::Term(Type::Constructor),
                Production::Term(Type::OpenPar),
                Production::NonTerm("FPARAMS"),
                Production::Term(Type::ClosePar),
            ],
        ),
        (
            ("FUNCHEADTAIL", Type::OpenPar),
            vec![
                Production::Term(Type::OpenPar),
                Production::NonTerm("FPARAMS"),
                Production::Term(Type::ClosePar),
                Production::Term(Type::ReturnType),
                Production::NonTerm("RETURNTYPE"),
            ],
        ),
        (
            ("FUNCHEADTAIL", Type::ScopeOp),
            vec![
                Production::Term(Type::ScopeOp),
                Production::NonTerm("FUNCHEADMEMBERTAIL"),
            ],
        ),
        (
            ("IDNEST", Type::Dot),
            vec![
                Production::Term(Type::Dot),
                Production::Term(Type::Id("".to_owned())),
                Production::NonTerm("IDNEST2"),
            ],
        ),
        (
            ("IDNEST2", Type::ClosePar),
            vec![Production::NonTerm("REPTIDNEST1")],
        ),
        (
            ("IDNEST2", Type::OpenPar),
            vec![
                Production::Term(Type::OpenPar),
                Production::NonTerm("APARAMS"),
                Production::Term(Type::ClosePar),
            ],
        ),
        (
            ("IDNEST2", Type::Dot),
            vec![Production::NonTerm("REPTIDNEST1")],
        ),
        (
            ("IDNEST2", Type::Semi),
            vec![Production::NonTerm("REPTIDNEST1")],
        ),
        (
            ("IDNEST2", Type::Minus),
            vec![Production::NonTerm("REPTIDNEST1")],
        ),
        (
            ("IDNEST2", Type::Plus),
            vec![Production::NonTerm("REPTIDNEST1")],
        ),
        (
            ("IDNEST2", Type::Comma),
            vec![Production::NonTerm("REPTIDNEST1")],
        ),
        (
            ("IDNEST2", Type::GEq),
            vec![Production::NonTerm("REPTIDNEST1")],
        ),
        (
            ("IDNEST2", Type::LEq),
            vec![Production::NonTerm("REPTIDNEST1")],
        ),
        (
            ("IDNEST2", Type::Gt),
            vec![Production::NonTerm("REPTIDNEST1")],
        ),
        (
            ("IDNEST2", Type::Lt),
            vec![Production::NonTerm("REPTIDNEST1")],
        ),
        (
            ("IDNEST2", Type::NotEq),
            vec![Production::NonTerm("REPTIDNEST1")],
        ),
        (
            ("IDNEST2", Type::Eq),
            vec![Production::NonTerm("REPTIDNEST1")],
        ),
        (
            ("IDNEST2", Type::And),
            vec![Production::NonTerm("REPTIDNEST1")],
        ),
        (
            ("IDNEST2", Type::Div),
            vec![Production::NonTerm("REPTIDNEST1")],
        ),
        (
            ("IDNEST2", Type::Mult),
            vec![Production::NonTerm("REPTIDNEST1")],
        ),
        (
            ("IDNEST2", Type::CloseSqbr),
            vec![Production::NonTerm("REPTIDNEST1")],
        ),
        (
            ("IDNEST2", Type::OpenSqbr),
            vec![Production::NonTerm("REPTIDNEST1")],
        ),
        (
            ("IDNEST2", Type::Or),
            vec![Production::NonTerm("REPTIDNEST1")],
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
                Production::NonTerm("ARRAYOROBJECT"),
                Production::Term(Type::Semi),
            ],
        ),
        (
            ("LOCALVARORSTAT", Type::Id("".to_owned())),
            vec![Production::NonTerm("STATEMENT")],
        ),
        (
            ("LOCALVARORSTAT", Type::Return),
            vec![Production::NonTerm("STATEMENT")],
        ),
        (
            ("LOCALVARORSTAT", Type::Write),
            vec![Production::NonTerm("STATEMENT")],
        ),
        (
            ("LOCALVARORSTAT", Type::Read),
            vec![Production::NonTerm("STATEMENT")],
        ),
        (
            ("LOCALVARORSTAT", Type::While),
            vec![Production::NonTerm("STATEMENT")],
        ),
        (
            ("LOCALVARORSTAT", Type::If),
            vec![Production::NonTerm("STATEMENT")],
        ),
        (
            ("LOCALVARORSTAT", Type::LocalVar),
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
                Production::NonTerm("MEMBERFUNCHEAD"),
                Production::Term(Type::Semi),
            ],
        ),
        (
            ("MEMBERFUNCDECL", Type::Function),
            vec![
                Production::NonTerm("MEMBERFUNCHEAD"),
                Production::Term(Type::Semi),
            ],
        ),
        (
            ("MEMBERFUNCHEAD", Type::Constructor),
            vec![
                Production::Term(Type::Constructor),
                Production::Term(Type::Colon),
                Production::Term(Type::OpenPar),
                Production::NonTerm("FPARAMS"),
                Production::Term(Type::ClosePar),
            ],
        ),
        (
            ("MEMBERFUNCHEAD", Type::Function),
            vec![
                Production::Term(Type::Function),
                Production::Term(Type::Id("".to_owned())),
                Production::Term(Type::Colon),
                Production::Term(Type::OpenPar),
                Production::NonTerm("FPARAMS"),
                Production::Term(Type::ClosePar),
                Production::Term(Type::ReturnType),
                Production::NonTerm("RETURNTYPE"),
            ],
        ),
        (
            ("MEMBERVARDECL", Type::Attribute),
            vec![
                Production::Term(Type::Attribute),
                Production::Term(Type::Id("".to_owned())),
                Production::Term(Type::Colon),
                Production::NonTerm("TYPE"),
                Production::NonTerm("REPTARRAYSIZE"),
                Production::Term(Type::Semi),
            ],
        ),
        (("MULTOP", Type::And), vec![Production::Term(Type::And)]),
        (("MULTOP", Type::Div), vec![Production::Term(Type::Div)]),
        (("MULTOP", Type::Mult), vec![Production::Term(Type::Mult)]),
        (("OPTINHERITS", Type::OpenCubr), vec![]),
        (
            ("OPTINHERITS", Type::IsA),
            vec![
                Production::Term(Type::IsA),
                Production::Term(Type::Id("".to_owned())),
                Production::NonTerm("REPTINHERITSLIST"),
            ],
        ),
        (
            ("PROG", Type::Function),
            vec![Production::NonTerm("REPTPROG0")],
        ),
        (
            ("PROG", Type::Class),
            vec![Production::NonTerm("REPTPROG0")],
        ),
        (
            ("PROG", Type::EndOfFile),
            vec![Production::NonTerm("REPTPROG0")],
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
            ("RELEXPR", Type::Id("".to_owned())),
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
        (("REPTAPARAMS1", Type::ClosePar), vec![]),
        (
            ("REPTAPARAMS1", Type::Comma),
            vec![
                Production::NonTerm("APARAMSTAIL"),
                Production::NonTerm("REPTAPARAMS1"),
            ],
        ),
        (("REPTARRAYSIZE", Type::Semi), vec![]),
        (
            ("REPTARRAYSIZE", Type::OpenSqbr),
            vec![
                Production::NonTerm("ARRAYSIZE"),
                Production::NonTerm("REPTARRAYSIZE"),
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
        (("REPTIDNEST1", Type::ClosePar), vec![]),
        (("REPTIDNEST1", Type::Dot), vec![]),
        (("REPTIDNEST1", Type::Semi), vec![]),
        (("REPTIDNEST1", Type::Minus), vec![]),
        (("REPTIDNEST1", Type::Plus), vec![]),
        (("REPTIDNEST1", Type::Comma), vec![]),
        (("REPTIDNEST1", Type::GEq), vec![]),
        (("REPTIDNEST1", Type::LEq), vec![]),
        (("REPTIDNEST1", Type::Gt), vec![]),
        (("REPTIDNEST1", Type::Lt), vec![]),
        (("REPTIDNEST1", Type::NotEq), vec![]),
        (("REPTIDNEST1", Type::Eq), vec![]),
        (("REPTIDNEST1", Type::And), vec![]),
        (("REPTIDNEST1", Type::Div), vec![]),
        (("REPTIDNEST1", Type::Mult), vec![]),
        (("REPTIDNEST1", Type::CloseSqbr), vec![]),
        (
            ("REPTIDNEST1", Type::OpenSqbr),
            vec![
                Production::NonTerm("INDICE"),
                Production::NonTerm("REPTIDNEST1"),
            ],
        ),
        (("REPTIDNEST1", Type::Assign), vec![]),
        (("REPTIDNEST1", Type::Or), vec![]),
        (("REPTINHERITSLIST", Type::OpenCubr), vec![]),
        (
            ("REPTINHERITSLIST", Type::Comma),
            vec![
                Production::Term(Type::Comma),
                Production::Term(Type::Id("".to_owned())),
                Production::NonTerm("REPTINHERITSLIST"),
            ],
        ),
        (
            ("REPTLOCALVARORSTAT", Type::Id("".to_owned())),
            vec![
                Production::NonTerm("LOCALVARORSTAT"),
                Production::NonTerm("REPTLOCALVARORSTAT"),
            ],
        ),
        (
            ("REPTLOCALVARORSTAT", Type::Return),
            vec![
                Production::NonTerm("LOCALVARORSTAT"),
                Production::NonTerm("REPTLOCALVARORSTAT"),
            ],
        ),
        (
            ("REPTLOCALVARORSTAT", Type::Write),
            vec![
                Production::NonTerm("LOCALVARORSTAT"),
                Production::NonTerm("REPTLOCALVARORSTAT"),
            ],
        ),
        (
            ("REPTLOCALVARORSTAT", Type::Read),
            vec![
                Production::NonTerm("LOCALVARORSTAT"),
                Production::NonTerm("REPTLOCALVARORSTAT"),
            ],
        ),
        (
            ("REPTLOCALVARORSTAT", Type::While),
            vec![
                Production::NonTerm("LOCALVARORSTAT"),
                Production::NonTerm("REPTLOCALVARORSTAT"),
            ],
        ),
        (
            ("REPTLOCALVARORSTAT", Type::If),
            vec![
                Production::NonTerm("LOCALVARORSTAT"),
                Production::NonTerm("REPTLOCALVARORSTAT"),
            ],
        ),
        (("REPTLOCALVARORSTAT", Type::CloseCubr), vec![]),
        (
            ("REPTLOCALVARORSTAT", Type::LocalVar),
            vec![
                Production::NonTerm("LOCALVARORSTAT"),
                Production::NonTerm("REPTLOCALVARORSTAT"),
            ],
        ),
        (
            ("REPTMEMBERDECL", Type::Private),
            vec![
                Production::NonTerm("VISIBILITY"),
                Production::NonTerm("MEMBERDECL"),
                Production::NonTerm("REPTMEMBERDECL"),
            ],
        ),
        (
            ("REPTMEMBERDECL", Type::Public),
            vec![
                Production::NonTerm("VISIBILITY"),
                Production::NonTerm("MEMBERDECL"),
                Production::NonTerm("REPTMEMBERDECL"),
            ],
        ),
        (("REPTMEMBERDECL", Type::CloseCubr), vec![]),
        (
            ("REPTPROG0", Type::Function),
            vec![
                Production::NonTerm("CLASSDECLORFUNCDEF"),
                Production::NonTerm("REPTPROG0"),
            ],
        ),
        (
            ("REPTPROG0", Type::Class),
            vec![
                Production::NonTerm("CLASSDECLORFUNCDEF"),
                Production::NonTerm("REPTPROG0"),
            ],
        ),
        (("REPTPROG0", Type::EndOfFile), vec![]),
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
        (("REPTVARIABLE", Type::ClosePar), vec![]),
        (
            ("REPTVARIABLE", Type::Dot),
            vec![
                Production::NonTerm("VARIDNEST"),
                Production::NonTerm("REPTVARIABLE"),
            ],
        ),
        (("REPTVARIABLEORFUNCTIONCALL", Type::ClosePar), vec![]),
        (
            ("REPTVARIABLEORFUNCTIONCALL", Type::Dot),
            vec![
                Production::NonTerm("IDNEST"),
                Production::NonTerm("REPTVARIABLEORFUNCTIONCALL"),
            ],
        ),
        (("REPTVARIABLEORFUNCTIONCALL", Type::Semi), vec![]),
        (("REPTVARIABLEORFUNCTIONCALL", Type::Minus), vec![]),
        (("REPTVARIABLEORFUNCTIONCALL", Type::Plus), vec![]),
        (("REPTVARIABLEORFUNCTIONCALL", Type::Comma), vec![]),
        (("REPTVARIABLEORFUNCTIONCALL", Type::GEq), vec![]),
        (("REPTVARIABLEORFUNCTIONCALL", Type::LEq), vec![]),
        (("REPTVARIABLEORFUNCTIONCALL", Type::Gt), vec![]),
        (("REPTVARIABLEORFUNCTIONCALL", Type::Lt), vec![]),
        (("REPTVARIABLEORFUNCTIONCALL", Type::NotEq), vec![]),
        (("REPTVARIABLEORFUNCTIONCALL", Type::Eq), vec![]),
        (("REPTVARIABLEORFUNCTIONCALL", Type::And), vec![]),
        (("REPTVARIABLEORFUNCTIONCALL", Type::Div), vec![]),
        (("REPTVARIABLEORFUNCTIONCALL", Type::Mult), vec![]),
        (("REPTVARIABLEORFUNCTIONCALL", Type::CloseSqbr), vec![]),
        (("REPTVARIABLEORFUNCTIONCALL", Type::Or), vec![]),
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
        (("RIGHTRECARITHEXPR", Type::ClosePar), vec![]),
        (("RIGHTRECARITHEXPR", Type::Semi), vec![]),
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
        (("RIGHTRECARITHEXPR", Type::GEq), vec![]),
        (("RIGHTRECARITHEXPR", Type::LEq), vec![]),
        (("RIGHTRECARITHEXPR", Type::Gt), vec![]),
        (("RIGHTRECARITHEXPR", Type::Lt), vec![]),
        (("RIGHTRECARITHEXPR", Type::NotEq), vec![]),
        (("RIGHTRECARITHEXPR", Type::Eq), vec![]),
        (("RIGHTRECARITHEXPR", Type::CloseSqbr), vec![]),
        (
            ("RIGHTRECARITHEXPR", Type::Or),
            vec![
                Production::NonTerm("ADDOP"),
                Production::NonTerm("TERM"),
                Production::NonTerm("RIGHTRECARITHEXPR"),
            ],
        ),
        (("RIGHTRECTERM", Type::ClosePar), vec![]),
        (("RIGHTRECTERM", Type::Semi), vec![]),
        (("RIGHTRECTERM", Type::Minus), vec![]),
        (("RIGHTRECTERM", Type::Plus), vec![]),
        (("RIGHTRECTERM", Type::Comma), vec![]),
        (("RIGHTRECTERM", Type::GEq), vec![]),
        (("RIGHTRECTERM", Type::LEq), vec![]),
        (("RIGHTRECTERM", Type::Gt), vec![]),
        (("RIGHTRECTERM", Type::Lt), vec![]),
        (("RIGHTRECTERM", Type::NotEq), vec![]),
        (("RIGHTRECTERM", Type::Eq), vec![]),
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
                Production::Term(Type::Id("".to_owned())),
                Production::NonTerm("STATEMENTIDNEST"),
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
            ("STATEMENTIDNEST", Type::OpenPar),
            vec![
                Production::Term(Type::OpenPar),
                Production::NonTerm("APARAMS"),
                Production::Term(Type::ClosePar),
                Production::NonTerm("STATEMENTIDNEST2"),
            ],
        ),
        (
            ("STATEMENTIDNEST", Type::Dot),
            vec![
                Production::Term(Type::Dot),
                Production::Term(Type::Id("".to_owned())),
                Production::NonTerm("STATEMENTIDNEST"),
            ],
        ),
        (
            ("STATEMENTIDNEST", Type::OpenSqbr),
            vec![
                Production::NonTerm("INDICE"),
                Production::NonTerm("REPTIDNEST1"),
                Production::NonTerm("STATEMENTIDNEST3"),
            ],
        ),
        (
            ("STATEMENTIDNEST", Type::Assign),
            vec![Production::NonTerm("ASSIGNOP"), Production::NonTerm("EXPR")],
        ),
        (
            ("STATEMENTIDNEST2", Type::Dot),
            vec![
                Production::Term(Type::Dot),
                Production::Term(Type::Id("".to_owned())),
                Production::NonTerm("STATEMENTIDNEST"),
            ],
        ),
        (("STATEMENTIDNEST2", Type::Semi), vec![]),
        (
            ("STATEMENTIDNEST3", Type::Dot),
            vec![
                Production::Term(Type::Dot),
                Production::Term(Type::Id("".to_owned())),
                Production::NonTerm("STATEMENTIDNEST"),
            ],
        ),
        (
            ("STATEMENTIDNEST3", Type::Assign),
            vec![Production::NonTerm("ASSIGNOP"), Production::NonTerm("EXPR")],
        ),
        (
            ("TERM", Type::OpenPar),
            vec![
                Production::NonTerm("FACTOR"),
                Production::NonTerm("RIGHTRECTERM"),
            ],
        ),
        (
            ("TERM", Type::Id("".to_owned())),
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
                Production::Term(Type::Id("".to_owned())),
                Production::NonTerm("VARIABLE2"),
            ],
        ),
        (
            ("VARIABLE2", Type::ClosePar),
            vec![
                Production::NonTerm("REPTIDNEST1"),
                Production::NonTerm("REPTVARIABLE"),
            ],
        ),
        (
            ("VARIABLE2", Type::OpenPar),
            vec![
                Production::Term(Type::OpenPar),
                Production::NonTerm("APARAMS"),
                Production::Term(Type::ClosePar),
                Production::NonTerm("VARIDNEST"),
            ],
        ),
        (
            ("VARIABLE2", Type::Dot),
            vec![
                Production::NonTerm("REPTIDNEST1"),
                Production::NonTerm("REPTVARIABLE"),
            ],
        ),
        (
            ("VARIABLE2", Type::OpenSqbr),
            vec![
                Production::NonTerm("REPTIDNEST1"),
                Production::NonTerm("REPTVARIABLE"),
            ],
        ),
        (
            ("VARIDNEST", Type::Dot),
            vec![
                Production::Term(Type::Dot),
                Production::Term(Type::Id("".to_owned())),
                Production::NonTerm("VARIDNEST2"),
            ],
        ),
        (
            ("VARIDNEST2", Type::ClosePar),
            vec![Production::NonTerm("REPTIDNEST1")],
        ),
        (
            ("VARIDNEST2", Type::OpenPar),
            vec![
                Production::Term(Type::OpenPar),
                Production::NonTerm("APARAMS"),
                Production::Term(Type::ClosePar),
                Production::NonTerm("VARIDNEST"),
            ],
        ),
        (
            ("VARIDNEST2", Type::Dot),
            vec![Production::NonTerm("REPTIDNEST1")],
        ),
        (
            ("VARIDNEST2", Type::OpenSqbr),
            vec![Production::NonTerm("REPTIDNEST1")],
        ),
        (
            ("VISIBILITY", Type::Private),
            vec![Production::Term(Type::Private)],
        ),
        (
            ("VISIBILITY", Type::Public),
            vec![Production::Term(Type::Public)],
        ),
    ])
}
