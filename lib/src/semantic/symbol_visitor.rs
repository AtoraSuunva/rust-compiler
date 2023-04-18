use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
    sync::atomic::{AtomicIsize, AtomicUsize, Ordering},
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
    visitor::{Visitor, VisitorResult, FLOAT_SIZE, INT_SIZE},
    visitor_utils::get_global_table,
};

type InheritsMap = HashMap<String, Vec<String>>;

pub struct SymbolTableVisitor {
    inherits: InheritsMap,
    offset: AtomicIsize,
    label_count: AtomicUsize,
    current_table: Rc<RefCell<SymbolTable>>,
}

const BASE_OFFSET: isize = 4;

impl SymbolTableVisitor {
    pub fn new() -> Self {
        Self {
            inherits: InheritsMap::new(),
            offset: AtomicIsize::new(BASE_OFFSET),
            label_count: AtomicUsize::new(0),
            current_table: Rc::new(RefCell::new(SymbolTable::new())),
        }
    }

    pub fn new_func_label(&self) -> String {
        format!("f{}", self.label_count.fetch_add(1, Ordering::SeqCst))
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
        members: CodeNode,
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
                                    return Err(CompilerError::new(
                                        format!(
                                            "Expected identifier at '{}'!",
                                            node.borrow().value,
                                        ),
                                        node.borrow().token.clone(),
                                    )
                                    .into())
                                }
                            };
                        }
                    }
                }
                _ => {
                    return Err(CompilerError::new(
                        format!("Expected inherits node at '{}'!", node.borrow().value,),
                        node.borrow().token.clone(),
                    )
                    .into())
                }
            }
        }

        let node_ref = node.borrow();
        let mut table_ref = node_ref.symbol_table.borrow_mut();
        let table = table_ref.get_or_insert_with(Default::default);

        if let Some(members_table) = members.borrow().symbol_table.borrow().clone() {
            table.extend(members_table);
        }

        // Reset the offset counter
        self.offset.store(BASE_OFFSET, Ordering::SeqCst);
        Ok(())
    }

    fn visit_class_members(&mut self, node: &CodeNode, members: Vec<CodeNode>) -> VisitorResult {
        let node_ref = node.borrow();
        let mut table_ref = node_ref.symbol_table.borrow_mut();
        let table = table_ref.get_or_insert_with(Default::default);

        for member in members {
            let res: Result<(usize, String, VarType), CompilerError> = match member.borrow().value {
                NodeValue::Tree(TreeNode::Attribute()) => {
                    let mut children = member.children();
                    let _visibility: Type = children.next().unwrap().try_into()?;
                    let id: Type = children.next().unwrap().try_into()?;

                    let id = if let Type::Id(n) = id {
                        n
                    } else {
                        return Err(CompilerError::new(
                            format!("Expected identifier at '{}'!", node_ref.value),
                            node_ref.token.clone(),
                        )
                        .into());
                    };

                    let type_: Type = children.next().unwrap().try_into()?;

                    let indices = children
                        .next()
                        .unwrap()
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
                    let size = get_type_size(&type_) * indice_mult.max(1);

                    let var_type = if type_ == Type::Integer {
                        VarType::Integer(indices)
                    } else if type_ == Type::Float {
                        VarType::Float(indices)
                    } else {
                        match type_ {
                            Type::Id(class_name) => VarType::Class(class_name),
                            _ => {
                                return Err(CompilerError::new(
                                    format!("Expected class identifier at '{}'!", node_ref.value),
                                    node_ref.token.clone(),
                                )
                                .into());
                            }
                        }
                    };

                    Ok((size, id, var_type))
                }
                NodeValue::Tree(TreeNode::ConstructorFunc()) => {
                    let mut children = member.children();
                    let _visibility: Type = children.next().unwrap().try_into()?;
                    let id: Type = children.next().unwrap().try_into()?;
                    let _param_list: Vec<String> = children
                        .next()
                        .unwrap()
                        .children()
                        .map(|c| {
                            let mut children = c.children();
                            let type_: Type = children.nth(1).unwrap().try_into()?;
                            let indices = children.next().unwrap().children().count();

                            Ok(format!("{}[{}]", type_, indices))
                        })
                        .collect::<Result<Vec<_>, CompilerError>>()?;

                    Ok((0, id.to_string(), VarType::Function))
                }
                NodeValue::Tree(TreeNode::MemberFunc()) => {
                    let mut children = member.children();
                    let _visibility: Type = children.next().unwrap().try_into()?;
                    let id: Type = children.next().unwrap().try_into()?;
                    let id = if let Type::Id(n) = id {
                        n
                    } else {
                        return Err(CompilerError::new(
                            format!("Expected identifier at '{}'!", node_ref.value),
                            node_ref.token.clone(),
                        )
                        .into());
                    };
                    let _param_list: Vec<String> = children
                        .next()
                        .unwrap()
                        .children()
                        .map(|c| {
                            let mut children = c.children();
                            let type_: Type = children.nth(1).unwrap().try_into()?;
                            let indices = children.next().unwrap().children().count();

                            Ok(format!("{}[{}]", type_, indices))
                        })
                        .collect::<Result<Vec<_>, CompilerError>>()?;
                    let _return_type: Type = children.next().unwrap().try_into()?;

                    Ok((0, id, VarType::Function))
                }
                _ => {
                    return Err(CompilerError::new(
                        format!(
                            "Expected Attribute, ConstructorFunc, or MemberFunc at '{}'!",
                            member.borrow().value,
                        ),
                        member.borrow().token.clone(),
                    )
                    .into())
                }
            };

            let (size, key, var_type) = res?;
            table.insert(
                key,
                Rc::new(RefCell::new(SymbolData::new(size, 0, var_type))),
            );
        }

        Ok(())
    }

    fn visit_function(
        &mut self,
        _node: &CodeNode,
        _head: CodeNode,
        _body: CodeNode,
    ) -> VisitorResult {
        self.offset.store(BASE_OFFSET, Ordering::SeqCst);
        self.current_table.replace(SymbolTable::new());
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

        fmt_params.sort_by(|a, b| b.borrow().offset.cmp(&a.borrow().offset));
        let fmt_params = fmt_params
            .iter()
            .map(|s| s.borrow().var_type.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        let func_signature = format!("{func_name}({fmt_params})");

        table.insert(
            func_signature.clone(),
            Rc::new(RefCell::new(SymbolData::new(size, 0, VarType::Function))),
        );

        let func_label = if func_signature == "main()" {
            String::from("main")
        } else {
            let func_label = self.new_func_label();
            format!("{func_label}_{func_name}")
        };

        node_ref.label.borrow_mut().replace(func_label);

        Ok(())
    }

    fn visit_factor(&mut self, node: &CodeNode, factor: Type) -> VisitorResult {
        let node_ref = node.borrow();

        let var_type = match factor {
            Type::IntNum(_) => VarType::Integer(vec![]),
            Type::FloatNum(_) => VarType::Float(vec![]),
            _ => {
                return Err(CompilerError::new(
                    "Expected literal value for factor!".into(),
                    node_ref.token.clone(),
                )
                .into())
            }
        };

        node_ref.var_type.borrow_mut().replace(var_type);

        Ok(())
    }

    fn visit_variable(
        &mut self,
        node: &CodeNode,
        id: Type,
        indices: Option<CodeNode>,
    ) -> VisitorResult {
        // Check for the var type, then resolve it using the indices (if needed)
        let node_ref = node.borrow();

        // These next two checks should *never* fail, ideally
        let id = match id {
            Type::Id(id) => id,
            _ => {
                return Err(CompilerError::new(
                    format!("Expected identifier at '{}'!", node_ref.value),
                    node_ref.token.clone(),
                )
                .into())
            }
        };

        let func_table = self.current_table.borrow();

        let var_data = match func_table.get(&id) {
            Some(var_data) => var_data,
            None => {
                return Err(CompilerError::new(
                    format!("Unknown variable '{id}'!"),
                    node_ref.token.clone(),
                )
                .into())
            }
        };

        let var_type = var_data.borrow().var_type.clone();

        // Set it even before checking so we can get more errors later (and not panic)
        node_ref.var_type.borrow_mut().replace(var_type.clone());

        let check_dimensions = |dimensions: usize, indexes: &Vec<usize>| -> VisitorResult {
            if dimensions > indexes.len() {
                return Err(CompilerError::new(
                    format!(
                        "Expected max {} dimensions for array '{id}', got {dimensions}!",
                        indexes.len(),
                    ),
                    node_ref.token.clone(),
                )
                .into());
            }

            Ok(())
        };

        // Check if our variable is indexed
        // If it is, then we have to "resolve" the type using the indices
        // Basically, if we have VarType::Integer([7, 8, 9]) representing arr: integer[7][8][9]
        // then arr[1]    -> VarType::Integer([8, 9])
        // and  arr[1][2] -> VarType::Integer([9])
        // "popping off" dimensions from the start
        if let Some(idx) = indices {
            let new_type = match *idx.borrow().var_type.borrow() {
                Some(VarType::IndiceList(dimensions)) => match var_type {
                    VarType::Integer(indexes) => {
                        check_dimensions(dimensions, &indexes)?;
                        VarType::Integer(indexes[dimensions..].to_vec())
                    }
                    VarType::Float(indexes) => {
                        check_dimensions(dimensions, &indexes)?;
                        VarType::Float(indexes[dimensions..].to_vec())
                    }
                    _ => {
                        return Err(CompilerError::new(
                            format!("Indexed a non-integer or non-float array at '{id}'!"),
                            idx.borrow().token.clone(),
                        )
                        .into());
                    }
                },
                _ => {
                    return Err(CompilerError::new(
                        format!("Expected indice list for array '{id}'!"),
                        idx.borrow().token.clone(),
                    )
                    .into());
                }
            };

            node_ref.var_type.borrow_mut().replace(new_type);
        }

        Ok(())
    }

    fn visit_indice_list(&mut self, node: &CodeNode, indices: Vec<CodeNode>) -> VisitorResult {
        let node_ref = node.borrow();

        // indices are all (arith)expr or factors (literals)
        // get all their types and make sure they're all ints (since we can't index using anything else)

        // first always set a type, so we dont panic later and can check for more errors
        node_ref
            .var_type
            .borrow_mut()
            .replace(VarType::IndiceList(indices.len()));

        for index in indices {
            let index_ref = index.borrow();
            let index_type = index_ref.var_type.borrow().clone().unwrap_or_else(|| {
                if let NodeValue::Leaf(Type::IntNum(_)) = index_ref.value {
                    VarType::Integer(vec![])
                } else {
                    VarType::Void
                }
            });

            // *Only* integers, not arrays or anything else
            if index_type != VarType::Integer(vec![]) {
                return Err(CompilerError::new(
                    format!("Array indices must be integers ({index} is not)!"),
                    node_ref.token.clone(),
                )
                .into());
            }
        }

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
            _ => {
                return Err(CompilerError::new(
                    format!("Expected identifier at '{}'!", node.borrow().value),
                    node.borrow().token.clone(),
                )
                .into())
            }
        };

        let size: usize;
        let var_type: VarType;

        let var_key = match &indice_or_args.borrow().value {
            NodeValue::Tree(t) => match t {
                TreeNode::IndiceList() => {
                    let indices = indice_or_args
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
                TreeNode::ArgumentList() => {
                    size = 0;

                    if let Type::Id(_id) = &type_ {
                        var_type = VarType::Class(_id.clone());
                        var_name
                    } else {
                        return Err(CompilerError::new(
                            format!("Expected identifier at '{}'!", node.borrow().value),
                            node.borrow().token.clone(),
                        )
                        .into());
                    }
                }
                _ => {
                    return Err(CompilerError::new(
                        format!(
                            "Expected indice list or argument list node at '{}'!",
                            node.borrow().value
                        ),
                        node.borrow().token.clone(),
                    )
                    .into())
                }
            },
            _ => {
                return Err(CompilerError::new(
                    format!("Expected Tree at '{}'!", node.borrow().value),
                    node.borrow().token.clone(),
                )
                .into())
            }
        };

        let offset = -self.offset.fetch_add(size as isize, Ordering::SeqCst);

        let node_ref = node.borrow();
        let mut table_ref = node_ref.symbol_table.borrow_mut();
        let table = table_ref.get_or_insert_with(Default::default);
        table.insert(
            var_key.clone(),
            Rc::new(RefCell::new(SymbolData::new(
                size,
                offset,
                var_type.clone(),
            ))),
        );

        let mut func_table = self.current_table.borrow_mut();
        func_table.insert(
            var_key,
            Rc::new(RefCell::new(SymbolData::new(
                size,
                offset,
                var_type.clone(),
            ))),
        );

        node_ref.var_type.borrow_mut().replace(var_type);

        Ok(())
    }

    fn visit_expr(&mut self, node: &CodeNode, expr: Vec<CodeNode>) -> VisitorResult {
        let node_ref = node.borrow();
        let first = expr.first().unwrap().borrow();

        let label = first.label.borrow().clone();
        *node_ref.label.borrow_mut() = label;

        let code = first.code.borrow().clone();
        *node_ref.code.borrow_mut() = code;

        let var_type = first.var_type.borrow().clone();
        *node_ref.var_type.borrow_mut() = var_type;

        Ok(())
    }

    fn visit_arith_expr(
        &mut self,
        node: &CodeNode,
        left: CodeNode,
        op: CodeNode,
        right: CodeNode,
    ) -> VisitorResult {
        let node_ref = node.borrow();
        let left_ref = left.borrow();
        let right_ref = right.borrow();

        let left_type = left_ref.var_type.borrow().clone().ok_or_else(|| {
            CompilerError::new(
                "Failed to get type of left side of expression".to_string(),
                left_ref.token.clone(),
            )
        })?;
        let right_type = right_ref.var_type.borrow().clone().ok_or_else(|| {
            CompilerError::new(
                "Failed to get type of right side of expression".to_string(),
                right_ref.token.clone(),
            )
        })?;

        // Just set this already before checking, so in case of errors we're propagating *something*
        // Any messed up values will just cause more errors later lol
        node_ref.var_type.borrow_mut().replace(left_type.clone());

        // check if left/right type have any indexes (error if they do, since we can't add arrays)
        let check_type = |vt: &VarType| -> VisitorResult {
            match vt {
                VarType::Integer(indexes) | VarType::Float(indexes) => {
                    if !indexes.is_empty() {
                        return Err(CompilerError::new(
                            "Cannot do arithmetic on arrays!".into(),
                            node_ref.token.clone(),
                        )
                        .into());
                    }
                }
                _ => {
                    return Err(CompilerError::new(
                        "You can only do arithmetic on integers or floats!".into(),
                        node_ref.token.clone(),
                    )
                    .into())
                }
            }

            Ok(())
        };

        check_type(&left_type)?;
        check_type(&right_type)?;

        if !left_type.eq_variant(&right_type) {
            return Err(CompilerError::new(
                format!(
                    "Cannot do arithmetic on mixed types (Tried {left_type} {op} {right_type})!",
                ),
                node_ref.token.clone(),
            )
            .into());
        }

        Ok(())
    }

    fn visit_function_call(
        &mut self,
        node: &CodeNode,
        id: Type,
        param_list: CodeNode,
    ) -> VisitorResult {
        let node_ref = node.borrow();
        let func_name = match id {
            Type::Id(id) => id,
            _ => {
                return Err(CompilerError::new(
                    format!("Expected identifier at '{}'!", node.borrow().value),
                    node.borrow().token.clone(),
                )
                .into())
            }
        };

        let param_types = param_list
            .children()
            .map(|c| {
                let c_ref = c.borrow();
                let c_type = c_ref.var_type.borrow().clone();
                c_type.map_or_else(|| String::from("?"), |v| v.to_string())
            })
            .collect::<Vec<_>>()
            .join(", ");

        let func_signature = format!("{}({})", func_name, param_types);
        let global_table = get_global_table(node)?;

        let func_data = global_table.get(&func_signature).ok_or_else(|| {
            node_ref
                .code
                .borrow_mut()
                .replace("addi r13, r0 , 0".to_string());
            node_ref.label.borrow_mut().replace("r13".to_string());
            CompilerError::new(
                format!("Function '{}' not found!", func_signature),
                node_ref.token.clone(),
            )
        })?;

        let func_table = func_data.borrow();
        let (_, return_type) = func_table
            .table
            .as_ref()
            .unwrap()
            .iter()
            .find(|(k, _)| k == &"_return")
            .ok_or(CompilerError::new(
                "Function return type not found in head table".to_string(),
                node_ref.token.clone(),
            ))?;

        let return_type = return_type.borrow().clone().var_type;
        node_ref.var_type.borrow_mut().replace(return_type);

        Ok(())
    }
}

pub fn get_type_size(type_: &Type) -> usize {
    if type_.eq_variant(&Type::IntNum(0)) || type_.eq(&Type::Integer) {
        INT_SIZE
    } else if type_.eq_variant(&Type::FloatNum(0.0)) || type_.eq(&Type::Float) {
        FLOAT_SIZE
    } else {
        0 // TODO: handle classes?
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
