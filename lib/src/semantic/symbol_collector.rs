use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        nodes::{CodeNode, NodeValue, SymbolData, SymbolTable, VarType},
        tree_node::TreeNode,
    },
    lexical::tokens::token_type::Type,
};

use super::visitor::{Visitor, VisitorResult};

#[derive(Default)]
pub struct SymbolCollectorVisitor {
    pub global: SymbolTable,
}

impl SymbolCollectorVisitor {
    pub fn new() -> Self {
        Self {
            global: Default::default(),
        }
    }
}

impl Visitor for SymbolCollectorVisitor {
    fn visit_program(
        &mut self,
        node: &CodeNode,
        _classes_or_funcs: Vec<CodeNode>,
    ) -> VisitorResult {
        let node_ref = node.borrow();
        let mut table_ref = node_ref.symbol_table.borrow_mut();
        let table = table_ref.get_or_insert_with(Default::default);
        table.extend(self.global.clone());
        Ok(())
    }

    fn visit_class(
        &mut self,
        node: &CodeNode,
        id: Type,
        _inherits: CodeNode,
        members: CodeNode,
    ) -> VisitorResult {
        let class_name = match id {
            Type::Id(id) => id,
            _ => return Err(format!("Expected identifier at '{}'!", node.borrow().value)),
        };

        let node_ref = node.borrow();
        let mut table_ref = node_ref.symbol_table.borrow_mut();
        let table = table_ref.get_or_insert_with(Default::default);

        if let Some(members_table) = members.borrow().symbol_table.borrow().clone() {
            table.extend(members_table);
        }

        if (self.global).contains_key(&class_name) {
            return Err(format!(
                "Class '{}' already defined! Defined again at {}",
                class_name, node
            ));
        }

        let size = table.values().fold(0, |acc, x| acc + x.borrow().size);

        // Then add the class to the global table:
        self.global.insert(
            class_name,
            Rc::new(RefCell::new(SymbolData::new_with_table(
                size,
                0,
                VarType::Class,
                table.clone(),
            ))),
        );

        Ok(())
    }

    fn visit_function(&mut self, node: &CodeNode, head: CodeNode, body: CodeNode) -> VisitorResult {
        let node_ref = node.borrow();
        let mut table_ref = node_ref.symbol_table.borrow_mut();
        let table = table_ref.get_or_insert_with(Default::default);
        let head_ref = head.borrow();
        let head_table = head_ref.symbol_table.borrow();

        let mut size: usize = 0;

        for child in body.children() {
            if let NodeValue::Tree(TreeNode::LocalVarDecl()) = &child.borrow().value {
                if let Some(var_table) = child.borrow().symbol_table.borrow().clone() {
                    let key = var_table
                        .keys()
                        .next()
                        .unwrap()
                        .clone()
                        .split(':')
                        .next()
                        .unwrap()
                        .to_string();

                    if table.keys().any(|k| *k.split(':').next().unwrap() == key) {
                        return Err(format!(
                            "Variable '{}' already defined! Defined again at {}",
                            key, child
                        ));
                    };

                    size += var_table.values().fold(0, |acc, x| acc + x.borrow().size);

                    // Add localvar decl info
                    table.extend(var_table);
                }
            }
        }

        let (func_name, func_data) = head_table.as_ref().unwrap().iter().next().unwrap();

        if (self.global).contains_key(func_name) {
            return Err(format!(
                "Function '{}' already defined! Defined again at {}",
                func_name, node
            ));
        }

        size += func_data.borrow().size;

        // Then add the func to the global table:
        self.global.insert(
            func_name.to_string(),
            Rc::new(RefCell::new(SymbolData::new_with_table(
                size,
                0,
                VarType::Function,
                table.clone(),
            ))),
        );

        Ok(())
    }
}
