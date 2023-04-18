use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        nodes::{CodeNode, NodeValue, SymbolData, SymbolTable},
        tree_node::TreeNode,
    },
    compiler_error::{CompilerError, CompilerResult},
};

pub fn get_symbol_data(start: &CodeNode, id: &str) -> Option<Rc<RefCell<SymbolData>>> {
    let mut parent = start.parent();

    while let Some(p) = parent {
        if let Some(table) = p.borrow().symbol_table.borrow().clone() {
            if let Some(info) = table.get(id) {
                return Some(info.clone());
            }
        }

        parent = p.parent();
    }

    None
}

pub fn get_current_function(start: &CodeNode) -> Option<CodeNode> {
    let mut parent = start.parent();

    while let Some(p) = parent {
        if let NodeValue::Tree(TreeNode::Function()) = p.borrow().value {
            return Some(p.clone());
        }

        parent = p.parent();
    }

    None
}

pub fn get_global_table(start: &CodeNode) -> CompilerResult<SymbolTable> {
    let mut parent = start.parent();

    while let Some(p) = parent {
        if let NodeValue::Tree(TreeNode::Program()) = p.borrow().value {
            if let Some(table) = p.borrow().symbol_table.borrow().clone() {
                return Ok(table);
            }
        }

        parent = p.parent();
    }

    Err(CompilerError::new(
        "No global symbol table found!".to_string(),
        start.borrow().token.clone(),
    )
    .into())
}
