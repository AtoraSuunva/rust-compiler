use crate::{
    ast::{
        nodes::{CodeNode, NodeValue, SymbolTable},
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

        let members_table = members.borrow().symbol_table.borrow().clone();

        if let Some(members_table) = members_table {
            table.0.extend(members_table.0);
        }

        if (self.global.0).contains_key(&class_name) {
            return Err(format!(
                "Class '{}' already defined! Defined again at {}",
                class_name, node
            ));
        }

        // Then add the class to the global table:
        self.global.0.insert(class_name, Some(table.clone()));

        Ok(())
    }

    fn visit_function(&mut self, node: &CodeNode, head: CodeNode, body: CodeNode) -> VisitorResult {
        let node_ref = node.borrow();
        let mut table_ref = node_ref.symbol_table.borrow_mut();
        let table = table_ref.get_or_insert_with(Default::default);

        let symbol = head.borrow().symbol_table.borrow().clone();

        if let Some(symbol) = symbol {
            for child in body.children() {
                if let NodeValue::Tree(TreeNode::LocalVarDecl()) = &child.borrow().value {
                    if let Some(var_table) = child.borrow().symbol_table.borrow().clone() {
                        let key = var_table
                            .0
                            .keys()
                            .next()
                            .unwrap()
                            .clone()
                            .split(':')
                            .next()
                            .unwrap()
                            .to_string();

                        if table.0.keys().any(|k| *k.split(':').next().unwrap() == key) {
                            return Err(format!(
                                "Variable '{}' already defined! Defined again at {}",
                                key, child
                            ));
                        };

                        // Add localvar decl info
                        table.0.extend(var_table.0);
                    }
                }
            }

            let func_name = symbol.0.iter().next().unwrap().0.clone();

            if (self.global.0).contains_key(&func_name) {
                return Err(format!(
                    "Function '{}' already defined! Defined again at {}",
                    func_name, node
                ));
            }

            // Then add the func to the global table:
            self.global.0.insert(func_name, Some(table.clone()));
        } else {
            return Err(format!("Symbol not found for {}!", node.borrow().value));
        }

        Ok(())
    }
}
