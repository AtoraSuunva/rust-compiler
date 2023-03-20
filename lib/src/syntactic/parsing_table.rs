use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};

use crate::{
    ast::{
        actions::{
            create_leaf, create_marker, create_subtree_from_n_nodes, create_subtree_until_marker,
            SemanticAction,
        },
        tree_node::TreeNode,
    },
    lexical::tokens::token_type::Type,
};

#[derive(Debug)]
pub enum Production<'a> {
    Term(Type),
    NonTerm(&'a str),
    Action(SemanticAction),
}

impl Display for Production<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Production::Term(t) => write!(f, "'{}'", t),
            Production::NonTerm(nt) => write!(f, "<{}>", nt),
            Production::Action(a) => a.fmt(f),
        }
    }
}

pub fn get_parsing_table() -> HashMap<(&'static str, Type), Vec<Production<'static>>> {
    HashMap::from([
        (
            ("START", Type::Function),
            vec![
                Production::Action(create_marker()),
                Production::NonTerm("PROG"),
                // Production::Term(Type::EndOfFile),
                Production::Action(create_subtree_until_marker(TreeNode::Program)),
            ],
        ),
        (
            ("START", Type::Class),
            vec![
                Production::Action(create_marker()),
                Production::NonTerm("PROG"),
                // Production::Term(Type::EndOfFile),
                Production::Action(create_subtree_until_marker(TreeNode::Program)),
            ],
        ),
        (
            ("START", Type::EndOfFile),
            vec![
                Production::Action(create_marker()),
                Production::NonTerm("PROG"),
                Production::Term(Type::EndOfFile),
                Production::Action(create_subtree_until_marker(TreeNode::Program)),
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
            ("APARAMS", Type::Id(String::from(""))),
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
                Production::Action(create_marker()),
                Production::NonTerm("TERM"),
                Production::NonTerm("RIGHTRECARITHEXPR"),
                Production::Action(create_subtree_until_marker(TreeNode::ArithExpr)),
            ],
        ),
        (
            ("ARITHEXPR", Type::Id(String::from(""))),
            vec![
                Production::Action(create_marker()),
                Production::NonTerm("TERM"),
                Production::NonTerm("RIGHTRECARITHEXPR"),
                Production::Action(create_subtree_until_marker(TreeNode::ArithExpr)),
            ],
        ),
        (
            ("ARITHEXPR", Type::Minus),
            vec![
                Production::Action(create_marker()),
                Production::NonTerm("TERM"),
                Production::NonTerm("RIGHTRECARITHEXPR"),
                Production::Action(create_subtree_until_marker(TreeNode::ArithExpr)),
            ],
        ),
        (
            ("ARITHEXPR", Type::Plus),
            vec![
                Production::Action(create_marker()),
                Production::NonTerm("TERM"),
                Production::NonTerm("RIGHTRECARITHEXPR"),
                Production::Action(create_subtree_until_marker(TreeNode::ArithExpr)),
            ],
        ),
        (
            ("ARITHEXPR", Type::Not),
            vec![
                Production::Action(create_marker()),
                Production::NonTerm("TERM"),
                Production::NonTerm("RIGHTRECARITHEXPR"),
                Production::Action(create_subtree_until_marker(TreeNode::ArithExpr)),
            ],
        ),
        (
            ("ARITHEXPR", Type::FloatNum(0f64)),
            vec![
                Production::Action(create_marker()),
                Production::NonTerm("TERM"),
                Production::NonTerm("RIGHTRECARITHEXPR"),
                Production::Action(create_subtree_until_marker(TreeNode::ArithExpr)),
            ],
        ),
        (
            ("ARITHEXPR", Type::IntNum(0)),
            vec![
                Production::Action(create_marker()),
                Production::NonTerm("TERM"),
                Production::NonTerm("RIGHTRECARITHEXPR"),
                Production::Action(create_subtree_until_marker(TreeNode::ArithExpr)),
            ],
        ),
        (
            ("ARRAYOROBJECT", Type::OpenPar),
            vec![
                Production::Term(Type::OpenPar),
                Production::Action(create_marker()),
                Production::NonTerm("APARAMS"),
                Production::Term(Type::ClosePar),
                Production::Action(create_subtree_until_marker(TreeNode::ArgumentList)),
            ],
        ),
        (
            ("ARRAYOROBJECT", Type::Semi),
            vec![
                Production::Action(create_marker()),
                Production::NonTerm("REPTARRAYSIZE"),
                Production::Action(create_subtree_until_marker(TreeNode::IndiceList)),
            ],
        ),
        (
            ("ARRAYOROBJECT", Type::OpenSqbr),
            vec![
                Production::Action(create_marker()),
                Production::NonTerm("REPTARRAYSIZE"),
                Production::Action(create_subtree_until_marker(TreeNode::IndiceList)),
            ],
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
            vec![
                // Production::Action(create_marker()), TODO: is `private attribute z: float[];` okay?
                Production::Term(Type::CloseSqbr),
            ],
        ),
        (
            ("ARRAYSIZE2", Type::IntNum(0)),
            vec![
                Production::Term(Type::IntNum(0)),
                Production::Action(create_leaf()),
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
                Production::Term(Type::Id(String::from(""))),
                Production::Action(create_leaf()),
                Production::Action(create_marker()),
                Production::NonTerm("OPTINHERITS"),
                Production::Action(create_subtree_until_marker(TreeNode::InheritsList)),
                Production::Term(Type::OpenCubr),
                Production::Action(create_marker()),
                Production::NonTerm("REPTMEMBERDECL"),
                Production::Action(create_subtree_until_marker(TreeNode::ClassMembers)),
                Production::Term(Type::CloseCubr),
                Production::Term(Type::Semi),
                Production::Action(create_subtree_from_n_nodes(TreeNode::Class, 3)),
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
                Production::Action(create_marker()),
                Production::NonTerm("ARITHEXPR"),
                Production::NonTerm("EXPR2"),
                Production::Action(create_subtree_until_marker(TreeNode::Expr)),
            ],
        ),
        (
            ("EXPR", Type::Id(String::from(""))),
            vec![
                Production::Action(create_marker()),
                Production::NonTerm("ARITHEXPR"),
                Production::NonTerm("EXPR2"),
                Production::Action(create_subtree_until_marker(TreeNode::Expr)),
            ],
        ),
        (
            ("EXPR", Type::Minus),
            vec![
                Production::Action(create_marker()),
                Production::NonTerm("ARITHEXPR"),
                Production::NonTerm("EXPR2"),
                Production::Action(create_subtree_until_marker(TreeNode::Expr)),
            ],
        ),
        (
            ("EXPR", Type::Plus),
            vec![
                Production::Action(create_marker()),
                Production::NonTerm("ARITHEXPR"),
                Production::NonTerm("EXPR2"),
                Production::Action(create_subtree_until_marker(TreeNode::Expr)),
            ],
        ),
        (
            ("EXPR", Type::Not),
            vec![
                Production::Action(create_marker()),
                Production::NonTerm("ARITHEXPR"),
                Production::NonTerm("EXPR2"),
                Production::Action(create_subtree_until_marker(TreeNode::Expr)),
            ],
        ),
        (
            ("EXPR", Type::FloatNum(0f64)),
            vec![
                Production::Action(create_marker()),
                Production::NonTerm("ARITHEXPR"),
                Production::NonTerm("EXPR2"),
                Production::Action(create_subtree_until_marker(TreeNode::Expr)),
            ],
        ),
        (
            ("EXPR", Type::IntNum(0)),
            vec![
                Production::Action(create_marker()),
                Production::NonTerm("ARITHEXPR"),
                Production::NonTerm("EXPR2"),
                Production::Action(create_subtree_until_marker(TreeNode::Expr)),
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
                Production::Action(create_subtree_from_n_nodes(TreeNode::Factor, 1)),
            ],
        ),
        (
            ("FACTOR", Type::Id(String::from(""))),
            vec![
                Production::Action(create_marker()),
                Production::Term(Type::Id(String::from(""))),
                Production::Action(create_leaf()),
                Production::NonTerm("FACTOR2"),
                Production::NonTerm("REPTVARIABLEORFUNCTIONCALL"),
                Production::Action(create_subtree_until_marker(TreeNode::Variable)),
            ],
        ),
        (
            ("FACTOR", Type::Minus),
            vec![
                Production::NonTerm("SIGN"),
                Production::Action(create_leaf()),
                Production::NonTerm("FACTOR"),
                Production::Action(create_subtree_from_n_nodes(TreeNode::Factor, 2)),
            ],
        ),
        (
            ("FACTOR", Type::Plus),
            vec![
                Production::NonTerm("SIGN"),
                Production::Action(create_leaf()),
                Production::NonTerm("FACTOR"),
                Production::Action(create_subtree_from_n_nodes(TreeNode::Factor, 2)),
            ],
        ),
        (
            ("FACTOR", Type::Not),
            vec![
                Production::Term(Type::Not),
                Production::Action(create_leaf()),
                Production::NonTerm("FACTOR"),
                Production::Action(create_subtree_from_n_nodes(TreeNode::Factor, 2)),
            ],
        ),
        (
            ("FACTOR", Type::FloatNum(0f64)),
            vec![
                Production::Term(Type::FloatNum(0f64)),
                Production::Action(create_leaf()),
                Production::Action(create_subtree_from_n_nodes(TreeNode::Factor, 1)),
            ],
        ),
        (
            ("FACTOR", Type::IntNum(0)),
            vec![
                Production::Term(Type::IntNum(0)),
                Production::Action(create_leaf()),
                Production::Action(create_subtree_from_n_nodes(TreeNode::Factor, 1)),
            ],
        ),
        (
            ("FACTOR2", Type::ClosePar),
            vec![
                Production::Action(create_marker()),
                Production::NonTerm("REPTIDNEST1"),
                Production::Action(create_subtree_until_marker(TreeNode::IndiceList)),
            ],
        ),
        (
            ("FACTOR2", Type::OpenPar),
            vec![
                Production::Term(Type::OpenPar),
                Production::Action(create_marker()),
                Production::NonTerm("APARAMS"),
                Production::Action(create_subtree_until_marker(TreeNode::ParameterList)),
                Production::Term(Type::ClosePar),
            ],
        ),
        (
            ("FACTOR2", Type::Dot),
            vec![
                Production::Action(create_marker()),
                Production::NonTerm("REPTIDNEST1"),
                Production::Action(create_subtree_until_marker(TreeNode::IndiceList)),
            ],
        ),
        (
            ("FACTOR2", Type::Semi),
            vec![
                Production::Action(create_marker()),
                Production::NonTerm("REPTIDNEST1"),
                Production::Action(create_subtree_until_marker(TreeNode::IndiceList)),
            ],
        ),
        (
            ("FACTOR2", Type::Minus),
            vec![
                Production::Action(create_marker()),
                Production::NonTerm("REPTIDNEST1"),
                Production::Action(create_subtree_until_marker(TreeNode::IndiceList)),
            ],
        ),
        (
            ("FACTOR2", Type::Plus),
            vec![
                Production::Action(create_marker()),
                Production::NonTerm("REPTIDNEST1"),
                Production::Action(create_subtree_until_marker(TreeNode::IndiceList)),
            ],
        ),
        (
            ("FACTOR2", Type::Comma),
            vec![
                Production::Action(create_marker()),
                Production::NonTerm("REPTIDNEST1"),
                Production::Action(create_subtree_until_marker(TreeNode::IndiceList)),
            ],
        ),
        (
            ("FACTOR2", Type::GEq),
            vec![
                Production::Action(create_marker()),
                Production::NonTerm("REPTIDNEST1"),
                Production::Action(create_subtree_until_marker(TreeNode::IndiceList)),
            ],
        ),
        (
            ("FACTOR2", Type::LEq),
            vec![
                Production::Action(create_marker()),
                Production::NonTerm("REPTIDNEST1"),
                Production::Action(create_subtree_until_marker(TreeNode::IndiceList)),
            ],
        ),
        (
            ("FACTOR2", Type::Gt),
            vec![
                Production::Action(create_marker()),
                Production::NonTerm("REPTIDNEST1"),
                Production::Action(create_subtree_until_marker(TreeNode::IndiceList)),
            ],
        ),
        (
            ("FACTOR2", Type::Lt),
            vec![
                Production::Action(create_marker()),
                Production::NonTerm("REPTIDNEST1"),
                Production::Action(create_subtree_until_marker(TreeNode::IndiceList)),
            ],
        ),
        (
            ("FACTOR2", Type::NotEq),
            vec![
                Production::Action(create_marker()),
                Production::NonTerm("REPTIDNEST1"),
                Production::Action(create_subtree_until_marker(TreeNode::IndiceList)),
            ],
        ),
        (
            ("FACTOR2", Type::Eq),
            vec![
                Production::Action(create_marker()),
                Production::NonTerm("REPTIDNEST1"),
                Production::Action(create_subtree_until_marker(TreeNode::IndiceList)),
            ],
        ),
        (
            ("FACTOR2", Type::And),
            vec![
                Production::Action(create_marker()),
                Production::NonTerm("REPTIDNEST1"),
                Production::Action(create_subtree_until_marker(TreeNode::IndiceList)),
            ],
        ),
        (
            ("FACTOR2", Type::Div),
            vec![
                Production::Action(create_marker()),
                Production::NonTerm("REPTIDNEST1"),
                Production::Action(create_subtree_until_marker(TreeNode::IndiceList)),
            ],
        ),
        (
            ("FACTOR2", Type::Mult),
            vec![
                Production::Action(create_marker()),
                Production::NonTerm("REPTIDNEST1"),
                Production::Action(create_subtree_until_marker(TreeNode::IndiceList)),
            ],
        ),
        (
            ("FACTOR2", Type::CloseSqbr),
            vec![
                Production::Action(create_marker()),
                Production::NonTerm("REPTIDNEST1"),
                Production::Action(create_subtree_until_marker(TreeNode::IndiceList)),
            ],
        ),
        (
            ("FACTOR2", Type::OpenSqbr),
            vec![
                Production::Action(create_marker()),
                Production::NonTerm("REPTIDNEST1"),
                Production::Action(create_subtree_until_marker(TreeNode::IndiceList)),
            ],
        ),
        (
            ("FACTOR2", Type::Or),
            vec![
                Production::Action(create_marker()),
                Production::NonTerm("REPTIDNEST1"),
                Production::Action(create_subtree_until_marker(TreeNode::IndiceList)),
            ],
        ),
        (("FPARAMS", Type::ClosePar), vec![]),
        (
            ("FPARAMS", Type::Id(String::from(""))),
            vec![
                Production::Term(Type::Id(String::from(""))),
                Production::Action(create_leaf()),
                Production::Term(Type::Colon),
                Production::NonTerm("TYPE"),
                Production::Action(create_leaf()),
                Production::Action(create_marker()),
                Production::NonTerm("REPTFPARAMS3"),
                Production::Action(create_subtree_until_marker(TreeNode::IndiceList)),
                Production::Action(create_subtree_from_n_nodes(TreeNode::Parameter, 3)),
                Production::NonTerm("REPTFPARAMS4"),
            ],
        ),
        (
            ("FPARAMSTAIL", Type::Comma),
            vec![
                Production::Term(Type::Comma),
                Production::Term(Type::Id(String::from(""))),
                Production::Action(create_leaf()),
                Production::Term(Type::Colon),
                Production::NonTerm("TYPE"),
                Production::Action(create_leaf()),
                Production::Action(create_marker()),
                Production::NonTerm("REPTFPARAMSTAIL4"),
                Production::Action(create_subtree_until_marker(TreeNode::IndiceList)),
                Production::Action(create_subtree_from_n_nodes(TreeNode::Parameter, 3)),
            ],
        ),
        (
            ("FUNCBODY", Type::OpenCubr),
            vec![
                Production::Term(Type::OpenCubr),
                Production::Action(create_marker()),
                Production::NonTerm("REPTLOCALVARORSTAT"),
                Production::Term(Type::CloseCubr),
            ],
        ),
        (
            ("FUNCDEF", Type::Function),
            vec![
                Production::Action(create_marker()),
                Production::NonTerm("FUNCHEAD"),
                Production::Action(create_subtree_until_marker(TreeNode::FunctionHead)),
                Production::NonTerm("FUNCBODY"),
                Production::Action(create_subtree_until_marker(TreeNode::FunctionBody)),
                Production::Action(create_subtree_from_n_nodes(TreeNode::Function, 2)),
            ],
        ),
        (
            ("FUNCHEAD", Type::Function),
            vec![
                Production::Term(Type::Function),
                Production::Term(Type::Id(String::from(""))),
                Production::Action(create_leaf()),
                Production::NonTerm("FUNCHEADTAIL"),
            ],
        ),
        (
            ("FUNCHEADMEMBERTAIL", Type::Id(String::from(""))),
            vec![
                Production::Term(Type::Id(String::from(""))),
                Production::Action(create_leaf()),
                Production::Action(create_subtree_from_n_nodes(TreeNode::Scope, 2)),
                Production::Term(Type::OpenPar),
                Production::Action(create_marker()),
                Production::NonTerm("FPARAMS"),
                Production::Action(create_subtree_until_marker(TreeNode::ParameterList)),
                Production::Term(Type::ClosePar),
                Production::Term(Type::ReturnType),
                Production::NonTerm("RETURNTYPE"),
                Production::Action(create_leaf()),
            ],
        ),
        (
            ("FUNCHEADMEMBERTAIL", Type::Constructor),
            vec![
                Production::Term(Type::Constructor),
                Production::Action(create_leaf()),
                Production::Action(create_subtree_from_n_nodes(TreeNode::Scope, 2)),
                Production::Term(Type::OpenPar),
                Production::Action(create_marker()),
                Production::NonTerm("FPARAMS"),
                Production::Action(create_subtree_until_marker(TreeNode::ParameterList)),
                Production::Term(Type::ClosePar),
            ],
        ),
        (
            ("FUNCHEADTAIL", Type::OpenPar),
            vec![
                Production::Term(Type::OpenPar),
                Production::Action(create_marker()),
                Production::NonTerm("FPARAMS"),
                Production::Action(create_subtree_until_marker(TreeNode::ParameterList)),
                Production::Term(Type::ClosePar),
                Production::Term(Type::ReturnType),
                Production::NonTerm("RETURNTYPE"),
                Production::Action(create_leaf()),
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
                Production::Term(Type::Id(String::from(""))),
                Production::Action(create_leaf()),
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
                Production::Action(create_marker()),
                Production::NonTerm("APARAMS"),
                Production::Action(create_subtree_until_marker(TreeNode::ArgumentList)),
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
                Production::Term(Type::Id(String::from(""))),
                Production::Action(create_leaf()),
                Production::Term(Type::Colon),
                Production::NonTerm("TYPE"),
                Production::Action(create_leaf()),
                Production::NonTerm("ARRAYOROBJECT"),
                Production::Term(Type::Semi),
                Production::Action(create_subtree_from_n_nodes(TreeNode::LocalVarDecl, 3)),
            ],
        ),
        (
            ("LOCALVARORSTAT", Type::Id(String::from(""))),
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
                Production::Action(create_leaf()),
                Production::Term(Type::Colon),
                Production::Term(Type::OpenPar),
                Production::Action(create_marker()),
                Production::NonTerm("FPARAMS"),
                Production::Action(create_subtree_until_marker(TreeNode::ParameterList)),
                Production::Term(Type::ClosePar),
                Production::Action(create_subtree_from_n_nodes(TreeNode::ConstructorFunc, 3)),
            ],
        ),
        (
            ("MEMBERFUNCHEAD", Type::Function),
            vec![
                Production::Term(Type::Function),
                Production::Term(Type::Id(String::from(""))),
                Production::Action(create_leaf()),
                Production::Term(Type::Colon),
                Production::Term(Type::OpenPar),
                Production::Action(create_marker()),
                Production::NonTerm("FPARAMS"),
                Production::Action(create_subtree_until_marker(TreeNode::ParameterList)),
                Production::Term(Type::ClosePar),
                Production::Term(Type::ReturnType),
                Production::NonTerm("RETURNTYPE"),
                Production::Action(create_leaf()),
                Production::Action(create_subtree_from_n_nodes(TreeNode::MemberFunc, 4)),
            ],
        ),
        (
            ("MEMBERVARDECL", Type::Attribute),
            vec![
                Production::Term(Type::Attribute),
                Production::Term(Type::Id(String::from(""))),
                Production::Action(create_leaf()),
                Production::Term(Type::Colon),
                Production::NonTerm("TYPE"),
                Production::Action(create_leaf()),
                Production::Action(create_marker()),
                Production::NonTerm("REPTARRAYSIZE"),
                Production::Action(create_subtree_until_marker(TreeNode::IndiceList)),
                Production::Term(Type::Semi),
                Production::Action(create_subtree_from_n_nodes(TreeNode::Attribute, 4)),
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
                Production::Term(Type::Id(String::from(""))),
                Production::Action(create_leaf()),
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
                Production::Action(create_leaf()),
                Production::NonTerm("ARITHEXPR"),
                Production::Action(create_subtree_from_n_nodes(TreeNode::RelExpr, 3)),
            ],
        ),
        (
            ("RELEXPR", Type::Id(String::from(""))),
            vec![
                Production::NonTerm("ARITHEXPR"),
                Production::NonTerm("RELOP"),
                Production::Action(create_leaf()),
                Production::NonTerm("ARITHEXPR"),
                Production::Action(create_subtree_from_n_nodes(TreeNode::RelExpr, 3)),
            ],
        ),
        (
            ("RELEXPR", Type::Minus),
            vec![
                Production::NonTerm("ARITHEXPR"),
                Production::NonTerm("RELOP"),
                Production::Action(create_leaf()),
                Production::NonTerm("ARITHEXPR"),
                Production::Action(create_subtree_from_n_nodes(TreeNode::RelExpr, 3)),
            ],
        ),
        (
            ("RELEXPR", Type::Plus),
            vec![
                Production::NonTerm("ARITHEXPR"),
                Production::NonTerm("RELOP"),
                Production::Action(create_leaf()),
                Production::NonTerm("ARITHEXPR"),
                Production::Action(create_subtree_from_n_nodes(TreeNode::RelExpr, 3)),
            ],
        ),
        (
            ("RELEXPR", Type::Not),
            vec![
                Production::NonTerm("ARITHEXPR"),
                Production::NonTerm("RELOP"),
                Production::Action(create_leaf()),
                Production::NonTerm("ARITHEXPR"),
                Production::Action(create_subtree_from_n_nodes(TreeNode::RelExpr, 3)),
            ],
        ),
        (
            ("RELEXPR", Type::FloatNum(0f64)),
            vec![
                Production::NonTerm("ARITHEXPR"),
                Production::NonTerm("RELOP"),
                Production::Action(create_leaf()),
                Production::NonTerm("ARITHEXPR"),
                Production::Action(create_subtree_from_n_nodes(TreeNode::RelExpr, 3)),
            ],
        ),
        (
            ("RELEXPR", Type::IntNum(0)),
            vec![
                Production::NonTerm("ARITHEXPR"),
                Production::NonTerm("RELOP"),
                Production::Action(create_leaf()),
                Production::NonTerm("ARITHEXPR"),
                Production::Action(create_subtree_from_n_nodes(TreeNode::RelExpr, 3)),
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
                Production::Term(Type::Id(String::from(""))),
                Production::Action(create_leaf()),
                Production::NonTerm("REPTINHERITSLIST"),
            ],
        ),
        (
            ("REPTLOCALVARORSTAT", Type::Id(String::from(""))),
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
                Production::Action(create_leaf()),
                Production::NonTerm("MEMBERDECL"),
                Production::NonTerm("REPTMEMBERDECL"),
            ],
        ),
        (
            ("REPTMEMBERDECL", Type::Public),
            vec![
                Production::NonTerm("VISIBILITY"),
                Production::Action(create_leaf()),
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
            ("REPTSTATBLOCK1", Type::Id(String::from(""))),
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
            ("RETURNTYPE", Type::Id(String::from(""))),
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
                Production::Action(create_leaf()),
                Production::NonTerm("TERM"),
                Production::Action(create_subtree_from_n_nodes(TreeNode::RightRecArithExpr, 2)),
                Production::NonTerm("RIGHTRECARITHEXPR"),
            ],
        ),
        (
            ("RIGHTRECARITHEXPR", Type::Plus),
            vec![
                Production::NonTerm("ADDOP"),
                Production::Action(create_leaf()),
                Production::NonTerm("TERM"),
                Production::Action(create_subtree_from_n_nodes(TreeNode::RightRecArithExpr, 2)),
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
                Production::Action(create_leaf()),
                Production::NonTerm("TERM"),
                Production::Action(create_subtree_from_n_nodes(TreeNode::RightRecArithExpr, 2)),
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
                Production::Action(create_leaf()),
                Production::NonTerm("FACTOR"),
                Production::Action(create_subtree_from_n_nodes(TreeNode::RightRecTerm, 2)),
                Production::NonTerm("RIGHTRECTERM"),
            ],
        ),
        (
            ("RIGHTRECTERM", Type::Div),
            vec![
                Production::NonTerm("MULTOP"),
                Production::Action(create_leaf()),
                Production::NonTerm("FACTOR"),
                Production::Action(create_subtree_from_n_nodes(TreeNode::RightRecTerm, 2)),
                Production::NonTerm("RIGHTRECTERM"),
            ],
        ),
        (
            ("RIGHTRECTERM", Type::Mult),
            vec![
                Production::NonTerm("MULTOP"),
                Production::Action(create_leaf()),
                Production::NonTerm("FACTOR"),
                Production::Action(create_subtree_from_n_nodes(TreeNode::RightRecTerm, 2)),
                Production::NonTerm("RIGHTRECTERM"),
            ],
        ),
        (("RIGHTRECTERM", Type::CloseSqbr), vec![]),
        (("RIGHTRECTERM", Type::Or), vec![]),
        (("SIGN", Type::Minus), vec![Production::Term(Type::Minus)]),
        (("SIGN", Type::Plus), vec![Production::Term(Type::Plus)]),
        (
            ("STATBLOCK", Type::Id(String::from(""))),
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
            ("STATEMENT", Type::Id(String::from(""))),
            vec![
                Production::Action(create_marker()),
                Production::Term(Type::Id(String::from(""))),
                Production::Action(create_leaf()),
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
                Production::Action(create_subtree_from_n_nodes(TreeNode::Return, 1)),
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
                Production::Action(create_subtree_from_n_nodes(TreeNode::Write, 1)),
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
                Production::Action(create_subtree_from_n_nodes(TreeNode::Read, 1)),
            ],
        ),
        (
            ("STATEMENT", Type::While),
            vec![
                Production::Term(Type::While),
                Production::Term(Type::OpenPar),
                Production::NonTerm("RELEXPR"),
                Production::Term(Type::ClosePar),
                Production::Action(create_marker()),
                Production::NonTerm("STATBLOCK"),
                Production::Action(create_subtree_until_marker(TreeNode::WhileBlock)),
                Production::Action(create_subtree_from_n_nodes(TreeNode::While, 2)),
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
                Production::Action(create_marker()),
                Production::NonTerm("STATBLOCK"),
                Production::Action(create_subtree_until_marker(TreeNode::IfBlock)),
                Production::Term(Type::Else),
                Production::Action(create_marker()),
                Production::NonTerm("STATBLOCK"),
                Production::Action(create_subtree_until_marker(TreeNode::ElseBlock)),
                Production::Term(Type::Semi),
                Production::Action(create_subtree_from_n_nodes(TreeNode::If, 3)),
            ],
        ),
        (
            ("STATEMENTIDNEST", Type::OpenPar),
            vec![
                Production::Term(Type::OpenPar),
                Production::Action(create_marker()),
                Production::NonTerm("APARAMS"),
                Production::Action(create_subtree_until_marker(TreeNode::ParameterList)),
                Production::Term(Type::ClosePar),
                Production::Action(create_subtree_until_marker(TreeNode::FunctionCall)),
                Production::NonTerm("STATEMENTIDNEST2"),
            ],
        ),
        (
            ("STATEMENTIDNEST", Type::Dot),
            vec![
                Production::Term(Type::Dot),
                Production::Term(Type::Id(String::from(""))),
                Production::Action(create_leaf()),
                Production::NonTerm("STATEMENTIDNEST"),
            ],
        ),
        (
            ("STATEMENTIDNEST", Type::OpenSqbr),
            vec![
                Production::Action(create_marker()),
                Production::NonTerm("INDICE"),
                Production::NonTerm("REPTIDNEST1"),
                Production::Action(create_subtree_until_marker(TreeNode::IndiceList)),
                Production::NonTerm("STATEMENTIDNEST3"),
            ],
        ),
        (
            ("STATEMENTIDNEST", Type::Assign),
            vec![
                Production::NonTerm("ASSIGNOP"),
                Production::Action(create_subtree_until_marker(TreeNode::Variable)),
                Production::NonTerm("EXPR"),
                Production::Action(create_subtree_from_n_nodes(TreeNode::Assignment, 2)),
            ],
        ),
        (
            ("STATEMENTIDNEST2", Type::Dot),
            vec![
                Production::Term(Type::Dot),
                Production::Term(Type::Id(String::from(""))),
                Production::Action(create_leaf()),
                Production::NonTerm("STATEMENTIDNEST"),
            ],
        ),
        (("STATEMENTIDNEST2", Type::Semi), vec![]),
        (
            ("STATEMENTIDNEST3", Type::Dot),
            vec![
                Production::Term(Type::Dot),
                Production::Term(Type::Id(String::from(""))),
                Production::Action(create_leaf()),
                Production::NonTerm("STATEMENTIDNEST"),
            ],
        ),
        (
            ("STATEMENTIDNEST3", Type::Assign),
            vec![
                Production::NonTerm("ASSIGNOP"),
                Production::Action(create_subtree_until_marker(TreeNode::Variable)),
                Production::NonTerm("EXPR"),
                Production::Action(create_subtree_from_n_nodes(TreeNode::Assignment, 2)),
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
            ("TERM", Type::Id(String::from(""))),
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
            ("TYPE", Type::Id(String::from(""))),
            vec![Production::Term(Type::Id(String::from("")))],
        ),
        (("TYPE", Type::Float), vec![Production::Term(Type::Float)]),
        (
            ("TYPE", Type::Integer),
            vec![Production::Term(Type::Integer)],
        ),
        (
            ("VARIABLE", Type::Id(String::from(""))),
            vec![
                Production::Term(Type::Id(String::from(""))),
                Production::Action(create_marker()),
                Production::Action(create_leaf()),
                Production::NonTerm("VARIABLE2"),
                Production::Action(create_subtree_until_marker(TreeNode::Variable)),
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
                Production::Action(create_marker()),
                Production::NonTerm("APARAMS"),
                Production::Term(Type::ClosePar),
                Production::Action(create_subtree_until_marker(TreeNode::ParameterList)),
                Production::Action(create_subtree_from_n_nodes(TreeNode::FunctionCall, 2)),
                Production::NonTerm("VARIDNEST"),
            ],
        ),
        (
            ("VARIABLE2", Type::Dot),
            vec![
                Production::Action(create_marker()),
                Production::NonTerm("REPTIDNEST1"),
                Production::Action(create_subtree_until_marker(TreeNode::IndexedVar)),
                Production::NonTerm("REPTVARIABLE"),
            ],
        ),
        (
            ("VARIABLE2", Type::OpenSqbr),
            vec![
                Production::Action(create_marker()),
                Production::NonTerm("REPTIDNEST1"),
                Production::Action(create_subtree_until_marker(TreeNode::IndexedVar)),
                Production::NonTerm("REPTVARIABLE"),
            ],
        ),
        (
            ("VARIDNEST", Type::Dot),
            vec![
                Production::Term(Type::Dot),
                Production::Term(Type::Id(String::from(""))),
                Production::Action(create_marker()),
                Production::Action(create_leaf()),
                Production::NonTerm("VARIDNEST2"),
                Production::Action(create_subtree_until_marker(TreeNode::NestedVar)),
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
                Production::Action(create_marker()),
                Production::NonTerm("APARAMS"),
                Production::Term(Type::ClosePar),
                Production::Action(create_subtree_until_marker(TreeNode::ParameterList)),
                Production::NonTerm("VARIDNEST"),
            ],
        ),
        (
            ("VARIDNEST2", Type::Dot),
            vec![
                Production::Action(create_marker()),
                Production::NonTerm("REPTIDNEST1"),
                Production::Action(create_subtree_until_marker(TreeNode::IndiceList)),
            ],
        ),
        (
            ("VARIDNEST2", Type::OpenSqbr),
            vec![
                Production::Action(create_marker()),
                Production::NonTerm("REPTIDNEST1"),
                Production::Action(create_subtree_until_marker(TreeNode::IndiceList)),
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
    ])
}
