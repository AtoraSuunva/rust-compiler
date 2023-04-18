use crate::{
    ast::nodes::CodeNode, compiler_error::CompilerError, lexical::tokens::token_type::Type,
};

use super::{
    visitor::{Visitor, VisitorResult},
    visitor_utils::get_global_table,
};

pub struct SymbolGlobalResolverVisitor {}

impl SymbolGlobalResolverVisitor {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for SymbolGlobalResolverVisitor {
    fn default() -> Self {
        Self::new()
    }
}

impl Visitor for SymbolGlobalResolverVisitor {}
