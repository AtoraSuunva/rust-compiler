use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        nodes::{CodeNode, NodeValue, SymbolData, SymbolTable, VarType},
        tree_node::TreeNode,
    },
    compiler_error::CompilerError,
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

        self.global.get("main()").ok_or_else(|| {
            CompilerError::new(
                "No main function found! (Did you make a typo or add parameters?)".to_string(),
                node_ref.token.clone(),
            )
        })?;

        Ok(())
    }

    fn visit_class(
        &mut self,
        node: &CodeNode,
        id: Type,
        _inherits: CodeNode,
        _members: CodeNode,
    ) -> VisitorResult {
        let class_name = match id {
            Type::Id(id) => id,
            _ => {
                return Err(CompilerError::new(
                    format!("Expected identifier at '{}'!", node.borrow().value),
                    node.borrow().token.clone(),
                )
                .into())
            }
        };

        let node_ref = node.borrow();
        let mut table_ref = node_ref.symbol_table.borrow_mut();
        let table = table_ref.get_or_insert_with(Default::default);

        if (self.global).contains_key(&class_name) {
            return Err(CompilerError::new(
                format!(
                    "Class '{}' already defined! Defined again at {}",
                    class_name, node
                ),
                node.borrow().token.clone(),
            )
            .into());
        }

        let size = table.values().fold(0, |acc, x| acc + x.borrow().size);

        table.insert(
            "..".to_string(),
            Rc::new(RefCell::new(SymbolData::new_with_table(
                0,
                0,
                VarType::Global,
                self.global.clone(),
            ))),
        );

        // Then add the class to the global table:
        self.global.insert(
            class_name.clone(),
            Rc::new(RefCell::new(SymbolData::new_with_table(
                size,
                0,
                VarType::Class(class_name),
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

        let mut errors: Vec<CompilerError> = vec![];

        let (func_name, _) = head_table
            .as_ref()
            .unwrap()
            .iter()
            .find(|(_, value)| value.borrow().var_type == VarType::Function)
            .ok_or(CompilerError::new(
                "Function not found in head table".to_string(),
                node_ref.token.clone(),
            ))?;

        let (return_name, return_type) = head_table
            .as_ref()
            .unwrap()
            .iter()
            .find(|(k, _)| k == &"_return")
            .ok_or(CompilerError::new(
                "Function return type not found in head table".to_string(),
                node_ref.token.clone(),
            ))?;

        table.insert(return_name.clone(), return_type.clone());
        table.insert(
            "_ret_addr".to_string(),
            Rc::new(RefCell::new(SymbolData::new(4, 0, VarType::Void))),
        );

        if (self.global).contains_key(func_name) {
            errors.push(CompilerError::new(
                format!(
                    "Function '{}' already defined! Defined again at {}",
                    func_name, node
                ),
                node_ref.token.clone(),
            ));
        }

        let func_split = func_name.split_once('(').map(|v| v.0);
        for (key, value) in self.global.clone() {
            let val_type = value.borrow().var_type.clone();

            if val_type == VarType::Function
                && key.split_once('(').map(|v| v.0) == func_split
                && &key != func_name
            {
                errors.push(CompilerError::new(
                    format!("[WARN] Function '{func_name}' is overloading function '{key}'",),
                    node_ref.token.clone(),
                ));
            }
        }

        let param_list = head.children().nth(1).unwrap();
        let param_list_ref = param_list.borrow();

        if let Some(param_table) = param_list_ref.symbol_table.borrow().clone() {
            size += param_table.values().fold(0, |acc, x| acc + x.borrow().size);
            table.extend(param_table);
        }

        for child in body.children() {
            if let NodeValue::Tree(TreeNode::LocalVarDecl()) = &child.borrow().value {
                if let Some(var_table) = child.borrow().symbol_table.borrow().clone() {
                    let (key, value) = var_table.iter().next().unwrap();

                    if table.contains_key(key) {
                        errors.push(CompilerError::new(
                            format!(
                                "Variable '{}' already defined in '{}'! Defined again at {}",
                                key, func_name, child
                            ),
                            child.borrow().token.clone(),
                        ));
                    };

                    size += value.borrow().size;

                    // Add localvar decl info
                    table.insert(key.clone(), value.clone());
                }
            }
        }

        // Resolve class sizes
        for (key, value) in table.clone().iter() {
            let value_ref = value.borrow();
            let var_type = value_ref.var_type.clone();

            if let VarType::Class(class_name) = var_type.clone() {
                let class_data = self.global.get(&class_name).unwrap();
                let class_size = class_data.borrow().size;

                table.insert(
                    key.clone(),
                    Rc::new(RefCell::new(SymbolData::new_with_table(
                        class_size,
                        value_ref.offset,
                        var_type,
                        class_data.borrow().table.clone().unwrap(),
                    ))),
                );
            }
        }

        let head_id_node = head.children().next().unwrap();

        let (var_type, parent_table) =
            if let NodeValue::Tree(TreeNode::Scope()) = head_id_node.borrow().value.clone() {
                let scope_id = head_id_node.children().next().unwrap();
                let scope_id = match scope_id.borrow().value.clone() {
                    NodeValue::Leaf(Type::Id(id)) => id,
                    _ => {
                        return Err(CompilerError::new(
                            format!("Expected identifier at '{}'!", node.borrow().value),
                            node.borrow().token.clone(),
                        )
                        .into())
                    }
                };

                let var_data = self.global.get(&scope_id).unwrap();
                let var_type = VarType::Class(scope_id);
                (var_type, var_data.borrow().clone().table.unwrap())
            } else {
                (VarType::Global, self.global.clone())
            };

        table.insert(
            "..".to_string(),
            Rc::new(RefCell::new(SymbolData::new_with_table(
                0,
                0,
                var_type,
                parent_table,
            ))),
        );

        let mut symbol_data = SymbolData::new_with_table(size, 0, VarType::Function, table.clone());
        let func_label = head_ref.label.borrow();
        symbol_data.label = func_label.clone();

        // Then add the func to the global table:
        self.global
            .insert(func_name.to_string(), Rc::new(RefCell::new(symbol_data)));

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(())
    }
}
