use std::{
    cell::RefCell,
    rc::Rc,
    sync::atomic::{AtomicIsize, Ordering},
};

use crate::{
    ast::{
        nodes::{CodeNode, NodeValue, SymbolData, SymbolTable, VarType},
        tree_node::TreeNode,
    },
    compiler_error::CompilerError,
    lexical::tokens::token_type::Type,
};

use super::{
    symbol_visitor::get_type_size,
    visitor::{Visitor, VisitorResult},
};

const BASE_OFFSET: isize = 4;

pub struct SymbolGlobalResolverVisitor {
    global: SymbolTable,
    offset: AtomicIsize,
    current_table: Rc<RefCell<SymbolTable>>,
}

impl SymbolGlobalResolverVisitor {
    pub fn new() -> Self {
        Self {
            global: Default::default(),
            offset: AtomicIsize::new(BASE_OFFSET),
            current_table: Rc::new(RefCell::new(SymbolTable::new())),
        }
    }
}

impl Default for SymbolGlobalResolverVisitor {
    fn default() -> Self {
        Self::new()
    }
}

impl Visitor for SymbolGlobalResolverVisitor {
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

    fn visit_parameter(
        &mut self,
        node: &CodeNode,
        id: Type,
        type_: Type,
        indices: CodeNode,
    ) -> VisitorResult {
        let var_name = match id {
            Type::Id(id) => id,
            _ => {
                return Err(CompilerError::new(
                    format!("Expected identifier at '{}'!", node.borrow().value),
                    node.borrow().token.clone(),
                )
                .into())
            }
        };

        let size: usize;
        let offset: isize;
        let var_type: VarType;

        let var_key = match &indices.borrow().value {
            NodeValue::Tree(TreeNode::IndiceList()) => {
                let indices = indices
                    .children()
                    .map(|num| -> Result<usize, CompilerError> {
                        if let NodeValue::Leaf(Type::IntNum(n)) = &num.borrow().value {
                            usize::try_from(*n).map_err(|_| {
                                CompilerError::new(
                                    "Expected usize!".to_string(),
                                    num.borrow().token.clone(),
                                )
                            })
                        } else {
                            Err(CompilerError::new(
                                "Expected number!".to_string(),
                                num.borrow().token.clone(),
                            ))
                        }
                    })
                    .collect::<Result<Vec<_>, _>>()?;

                let indice_mult: usize = indices.iter().product();
                size = get_type_size(&type_) * indice_mult.max(1);
                offset = -self.offset.fetch_add(size as isize, Ordering::SeqCst);

                var_type = if type_ == Type::Integer {
                    VarType::Integer(indices)
                } else if type_ == Type::Float {
                    VarType::Float(indices)
                } else {
                    match type_ {
                        Type::Id(class_name) => VarType::Class(class_name),
                        _ => {
                            return Err(CompilerError::new(
                                format!("Expected class identifier at '{}'!", var_name),
                                node.borrow().token.clone(),
                            )
                            .into())
                        }
                    }
                };

                var_name
            }
            _ => {
                return Err(CompilerError::new(
                    format!("Expected indice list at '{}'!", node.borrow().value),
                    node.borrow().token.clone(),
                )
                .into())
            }
        };

        let mut symbol_data = SymbolData::new(size, offset, var_type);
        symbol_data.label.replace(format!("{offset}(r14)"));

        let node_ref = node.borrow();
        let mut table_ref = node_ref.symbol_table.borrow_mut();
        let table = table_ref.get_or_insert_with(Default::default);
        table.insert(var_key, Rc::new(RefCell::new(symbol_data)));

        Ok(())
    }

    fn visit_parameter_list(&mut self, node: &CodeNode, params: Vec<CodeNode>) -> VisitorResult {
        let node_ref = node.borrow();
        let mut table_ref = node_ref.symbol_table.borrow_mut();
        let table = table_ref.get_or_insert_with(Default::default);

        for param in params {
            let param_ref = param.borrow();
            let mut param_table_ref = param_ref.symbol_table.borrow_mut();
            let param_table = param_table_ref.get_or_insert_with(Default::default);
            let mut func_table = self.current_table.borrow_mut();

            for (key, value) in param_table.iter() {
                table.insert(key.clone(), value.clone());
                func_table.insert(key.clone(), value.clone());
            }
        }

        Ok(())
    }

    fn visit_function_head(
        &mut self,
        node: &CodeNode,
        id: CodeNode,
        param_list: CodeNode,
        return_type: Option<Type>,
    ) -> VisitorResult {
        let node_ref = node.borrow();
        let mut table_ref = node_ref.symbol_table.borrow_mut();
        let table = table_ref.get_or_insert_with(Default::default);

        let func_name = match &id.borrow().value {
            NodeValue::Leaf(Type::Id(id)) => id.clone(),
            NodeValue::Tree(t) => match t {
                TreeNode::Scope() => {
                    let mut id_str: Vec<String> = vec![];

                    for child in id.children() {
                        if let NodeValue::Leaf(Type::Id(id)) = &child.borrow().value {
                            id_str.push(id.clone());
                        }
                    }

                    id_str.join("::")
                }
                _ => {
                    return Err(CompilerError::new(
                        format!("Expected scope node at '{}'!", node.borrow().value),
                        node.borrow().token.clone(),
                    )
                    .into())
                }
            },
            _ => {
                return Err(CompilerError::new(
                    format!("Expected identifier at '{}'!", node.borrow().value),
                    node.borrow().token.clone(),
                )
                .into())
            }
        };

        let mut fmt_params: Vec<Rc<RefCell<SymbolData>>> = vec![];
        let mut size = 0;

        if let Some(param_table) = param_list.borrow().symbol_table.borrow().clone() {
            for (_, value) in param_table.iter() {
                fmt_params.push(value.clone());
                size += value.borrow().size;
            }
            table.extend(param_table);
        }

        fmt_params.sort_by(|a, b| b.borrow().offset.cmp(&a.borrow().offset));
        let fmt_params = fmt_params
            .iter()
            .map(|s| s.borrow().var_type.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        let func_signature = format!("{func_name}({fmt_params})");

        if let Some(t) = return_type {
            let return_size = get_type_size(&t);

            table.insert(
                String::from("_return"),
                Rc::new(RefCell::new(SymbolData::new(
                    return_size,
                    0,
                    match t {
                        Type::Integer => VarType::Integer(vec![]),
                        Type::Float => VarType::Float(vec![]),
                        Type::Id(id) => VarType::Class(id),
                        Type::Void => VarType::Void,
                        _ => {
                            return Err(CompilerError::new(
                                format!("Expected type at '{}'!", node.borrow().value),
                                node.borrow().token.clone(),
                            )
                            .into())
                        }
                    },
                ))),
            );
        }

        let symbol_data = SymbolData::new_with_table(size, 0, VarType::Function, table.clone());

        // Then add the func to the global table:
        self.global
            .insert(func_signature, Rc::new(RefCell::new(symbol_data)));

        Ok(())
    }
}
