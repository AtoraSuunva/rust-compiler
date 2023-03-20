use std::collections::HashMap;

use crate::{
    ast::{
        nodes::{CodeNode, NodeValue},
        tree_node::TreeNode,
    },
    lexical::tokens::token_type::Type,
};

use super::visitor::{Visitor, VisitorResult};

// enum Symbols {
//     Class {
//         name: String,
//         inherits: Vec<String>,
//     },
//     Function {
//         name: String,
//         params: Vec<String>,
//         return_type: Option<Type>,
//     },
//     Variable {
//         name: String,
//         type_: Type,
//     },
// }

type InheritsMap = HashMap<String, Vec<String>>;

pub struct SymbolTableVisitor {
    inherits: InheritsMap,
}

impl SymbolTableVisitor {
    pub fn new() -> Self {
        Self {
            inherits: InheritsMap::new(),
        }
    }
}

impl Default for SymbolTableVisitor {
    fn default() -> Self {
        Self::new()
    }
}

impl Visitor for SymbolTableVisitor {
    fn visit_class(
        &mut self,
        node: &CodeNode,
        id: Type,
        inherits: CodeNode,
        _members: CodeNode,
    ) -> VisitorResult {
        let class_name = match id {
            Type::Id(id) => id,
            _ => return Err(format!("Expected identifier at '{}'!", node.borrow().value)),
        };

        if let NodeValue::Tree(t) = &inherits.borrow().value {
            match t {
                TreeNode::InheritsList() => {
                    for child in inherits.children() {
                        if let NodeValue::Leaf(l) = &child.borrow().value {
                            match l {
                                Type::Id(id) => {
                                    self.inherits
                                        .entry(class_name.clone())
                                        .or_insert(Vec::new())
                                        .push(id.clone());
                                }
                                _ => {
                                    return Err(format!(
                                        "Expected identifier at '{}'!",
                                        node.borrow().value
                                    ))
                                }
                            };
                        }
                    }
                }
                _ => {
                    return Err(format!(
                        "Expected inherits node at '{}'!",
                        node.borrow().value
                    ))
                }
            }
        }

        if node.borrow().symbol_table.borrow().is_some() {
            return Err(format!("Symbol table already exists for '{}'!", class_name));
        }

        Ok(())
    }

    fn visit_class_members(&mut self, node: &CodeNode, members: Vec<CodeNode>) -> VisitorResult {
        let node_ref = node.borrow();
        let mut table_ref = node_ref.symbol_table.borrow_mut();
        let table = table_ref.get_or_insert_with(Default::default);

        for member in members {
            let key: Result<String, String> = match member.borrow().value {
                NodeValue::Tree(TreeNode::Attribute()) => {
                    let mut children = member.children();
                    let visibility: Type = children.next().unwrap().try_into()?;
                    let id: Type = children.next().unwrap().try_into()?;
                    let type_: Type = children.next().unwrap().try_into()?;
                    let indices = children.next().unwrap().children().count();

                    Ok(format!("{visibility} {id}: {type_}[{indices}]"))
                }
                NodeValue::Tree(TreeNode::ConstructorFunc()) => {
                    let mut children = member.children();
                    let visibility: Type = children.next().unwrap().try_into()?;
                    let id: Type = children.next().unwrap().try_into()?;
                    let param_list: Vec<String> = children
                        .next()
                        .unwrap()
                        .children()
                        .map(|c| {
                            let mut children = c.children();
                            let type_: Type = children.nth(1).unwrap().try_into()?;
                            let indices = children.next().unwrap().children().count();

                            Ok(format!("{}[{}]", type_, indices))
                        })
                        .collect::<Result<Vec<_>, String>>()?;

                    Ok(format!("{visibility} {id}({})", param_list.join(", ")))
                }
                NodeValue::Tree(TreeNode::MemberFunc()) => {
                    let mut children = member.children();
                    let visibility: Type = children.next().unwrap().try_into()?;
                    let id: Type = children.next().unwrap().try_into()?;
                    let param_list: Vec<String> = children
                        .next()
                        .unwrap()
                        .children()
                        .map(|c| {
                            let mut children = c.children();
                            let type_: Type = children.nth(1).unwrap().try_into()?;
                            let indices = children.next().unwrap().children().count();

                            Ok(format!("{}[{}]", type_, indices))
                        })
                        .collect::<Result<Vec<_>, String>>()?;
                    let return_type: Type = children.next().unwrap().try_into()?;

                    if let Type::Id(id) = id {
                        Ok(format!(
                            "{visibility} {id}({}) -> {return_type}",
                            param_list.join(", ")
                        ))
                    } else {
                        Err(format!(
                            "Expected identifier at '{}'!",
                            member.borrow().value
                        ))
                    }
                }
                _ => {
                    return Err(format!(
                        "Expected Attribute, ConstructorFunc, or MemberFunc at '{}'!",
                        member.borrow().value
                    ))
                }
            };

            table.0.insert(key?, None);
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
                _ => return Err(format!("Expected scope node at '{}'!", node.borrow().value)),
            },
            _ => return Err(format!("Expected identifier at '{}'!", node.borrow().value)),
        };

        let param_list = param_list
            .children()
            .map(|param| -> Result<String, String> {
                if let NodeValue::Tree(TreeNode::Parameter()) = &param.borrow().value {
                    let mut children = param.children();
                    let type_ = match children.nth(1).unwrap().borrow().value.clone() {
                        NodeValue::Leaf(l) => Ok(l),
                        _ => Err("Expected Leaf!"),
                    }?;
                    let indices = children.next().unwrap().children().count();

                    Ok(format!("{}[{}]", type_, indices))
                } else {
                    Err("Expected parameter node!".to_string())
                }
            })
            .collect::<Result<Vec<_>, _>>()?;

        let param_list = param_list.join(", ");

        let return_type = match return_type {
            Some(t) => format!(" => {}", t),
            None => "".to_string(),
        };

        let func_signature = format!("function: {func_name}({param_list}){return_type}");

        if node.borrow().symbol_table.borrow().is_some() {
            return Err(format!(
                "Symbol table already exists for '{}'!",
                func_signature
            ));
        }

        node.borrow()
            .symbol_table
            .borrow_mut()
            .get_or_insert_with(Default::default)
            .0
            .insert(func_signature, None);

        Ok(())
    }

    fn visit_local_var_decl(
        &mut self,
        node: &CodeNode,
        id: Type,
        type_: Type,
        indice_or_args: CodeNode,
    ) -> VisitorResult {
        let var_name = match id {
            Type::Id(id) => id,
            _ => return Err(format!("Expected identifier at '{}'!", node.borrow().value)),
        };

        let var_key = match &indice_or_args.borrow().value {
            NodeValue::Tree(t) => match t {
                TreeNode::IndiceList() => {
                    let indices = indice_or_args.children().count();
                    format!("localvar {}: {}[{}]", var_name, type_, indices)
                }
                TreeNode::ArgumentList() => {
                    // let args = indice_or_args
                    //     .children()
                    //     .map(|arg| {
                    //         if let NodeValue::Tree(TreeNode::Expr()) = &arg.borrow().value {
                    //             let type_ = get_expr_type(&arg)?;
                    //             Ok(format!("{}", type_))
                    //         } else {
                    //             Err(format!(
                    //                 "Expected expr node at '{}'!",
                    //                 node.borrow().value
                    //             ))
                    //         }
                    //     })
                    //     .collect::<Result<Vec<_>, _>>()?;

                    // let args = args.join(", ");
                    // println!("args: {}", args);

                    if let Type::Id(id) = type_ {
                        format!("localvar {}: {}", var_name, id)
                    } else {
                        return Err(format!("Expected identifier at '{}'!", node.borrow().value));
                    }
                }
                _ => {
                    return Err(format!(
                        "Expected indice list or argument list node at '{}'!",
                        node.borrow().value
                    ))
                }
            },
            _ => return Err(format!("Expected Tree at '{}'!", node.borrow().value)),
        };

        let node_ref = node.borrow();
        let mut table_ref = node_ref.symbol_table.borrow_mut();
        let table = table_ref.get_or_insert_with(Default::default);
        table.0.insert(var_key, None);

        Ok(())
    }
}

pub fn get_expr_type(node: &CodeNode) -> Result<Type, String> {
    match &node.borrow().value {
        // TODO: handle ID
        NodeValue::Leaf(t) => Ok(t.clone()),
        NodeValue::Marker => Err(format!("Unexpected marker at '{}'!", node.borrow().value)),
        NodeValue::Tree(_) => {
            let mut type_: Option<Type> = None;

            let children = node.children();

            for child in children {
                let child_type = get_expr_type(&child)?;

                if !child_type.is_literal() {
                    // ignore mult/plus/etc types
                    continue;
                }

                if let Some(old_type) = type_ {
                    if old_type != child_type {
                        return Err(format!(
                            "Expected type '{:?}' at '{}'!",
                            old_type,
                            node.borrow().value
                        ));
                    }
                }

                type_ = Some(child_type);
            }

            type_.ok_or_else(|| format!("Found no expr type at '{}'!", node.borrow().value))
        }
    }
}
