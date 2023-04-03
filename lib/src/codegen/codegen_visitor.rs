use std::{
    cell::RefCell,
    collections::HashSet,
    rc::Rc,
    sync::atomic::{AtomicUsize, Ordering},
};

use crate::{
    ast::nodes::{CodeNode, NodeValue, SymbolData},
    lexical::tokens::token_type::Type,
    semantic::visitor::{Visitor, VisitorResult, FLOAT_SIZE, INT_SIZE},
};

pub struct CodegenVisitor {
    pub code: String,
    label_count: AtomicUsize,
    literal_labels: HashSet<String>,
}

impl CodegenVisitor {
    pub fn new() -> Self {
        Self {
            code: String::from("entry\n"),
            label_count: AtomicUsize::new(0),
            literal_labels: HashSet::new(),
            // registers: (1..13).map(|r| format!("r{r}",)).collect(),
        }
    }

    pub fn new_temp_label(&self) -> String {
        format!("t{}", self.label_count.fetch_add(1, Ordering::SeqCst))
    }
}

impl Default for CodegenVisitor {
    fn default() -> Self {
        Self::new()
    }
}

fn get_symbol_data(start: &CodeNode, id: &str) -> Option<Rc<RefCell<SymbolData>>> {
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

impl Visitor for CodegenVisitor {
    fn visit_local_var_decl(
        &mut self,
        node: &CodeNode,
        id: Type,
        _type: Type,
        _indice_or_args: CodeNode,
    ) -> VisitorResult {
        let id = match id {
            Type::Id(ref id) => id,
            _ => return Err(format!("Expected identifier at '{}'!", node.borrow().value)),
        };

        let symbol_data = get_symbol_data(node, id)
            .unwrap_or_else(|| panic!("Found no symbol table entry for '{}'!", id));

        let temp_label = self.new_temp_label();
        let size = symbol_data.borrow().size;

        self.code.push_str(&format!(
            "% space for variable {id}\n{temp_label} res {size}\n\n"
        ));

        symbol_data.borrow_mut().label = Some(temp_label.clone());
        node.borrow().label.borrow_mut().replace(temp_label);
        Ok(())
    }

    fn visit_variable(
        &mut self,
        node: &CodeNode,
        id: CodeNode,
        _indices: Option<CodeNode>,
    ) -> VisitorResult {
        let id_ref = id.borrow();
        let id = match id_ref.value {
            NodeValue::Leaf(Type::Id(ref id)) => id,
            _ => return Err(format!("Expected identifier at '{}'!", node.borrow().value)),
        };

        let symbol_data = get_symbol_data(node, id)
            .unwrap_or_else(|| panic!("Found no symbol table entry for '{}'!", id));

        let label = symbol_data.borrow().label.clone().unwrap();

        node.borrow().label.borrow_mut().replace(label);
        Ok(())
    }

    fn visit_factor(&mut self, node: &CodeNode, factor: Type) -> VisitorResult {
        let (key, size, lit, bytes) = get_literal_label(&factor);

        if !self.literal_labels.contains(&key) {
            self.code.push_str(&format!("% space for literal {lit}\n"));

            if size == INT_SIZE {
                self.code.push_str(&format!("{key} res {size}\n"));
                self.code.push_str(&format!("addi r14,r0,{lit}\n"));
                self.code.push_str(&format!("sw {key}(r0),r14\n\n"));
            } else {
                let bytes_str = bytes
                    .iter()
                    .map(|b| format!("{}", b))
                    .collect::<Vec<String>>()
                    .join(",");

                self.code.push_str(&format!("{key} db {}\n\n", bytes_str));
            }

            self.literal_labels.insert(key.clone());
        }

        node.borrow().label.borrow_mut().replace(key);

        Ok(())
    }

    fn visit_arith_expr(
        &mut self,
        node: &CodeNode,
        left: CodeNode,
        op: CodeNode,
        right: CodeNode,
    ) -> VisitorResult {
        let left_label = left.borrow().label.borrow().clone().unwrap();
        let right_label = right.borrow().label.borrow().clone().unwrap();

        let operand = match op.borrow().value {
            NodeValue::Leaf(ref leaf) => match leaf {
                Type::Plus => "add",
                Type::Minus => "sub",
                Type::Mult => "mul",
                Type::Div => "div",
                _ => panic!("Expected operator!"),
            },
            _ => panic!("Expected operator!"),
        };

        let temp_label = self.new_temp_label();

        self.code.push_str("% arith expression\n");
        self.code.push_str(&format!("lw r1,{left_label}(r0)\n"));
        self.code.push_str(&format!("lw r2,{right_label}(r0)\n"));
        self.code.push_str(&format!("{temp_label} res 4\n"));
        self.code.push_str(&format!("{operand} r3,r1,r2\n"));
        self.code.push_str(&format!("sw {temp_label}(r0),r3\n\n"));

        node.borrow().label.borrow_mut().replace(temp_label);
        Ok(())
    }
}

fn get_literal_label(lit_type: &Type) -> (String, usize, String, Vec<u8>) {
    let (key, size, bytes) = match lit_type {
        Type::IntNum(ref key) => (key.to_string(), INT_SIZE, key.to_le_bytes()),
        Type::FloatNum(ref key) => (key.to_string(), FLOAT_SIZE, key.to_le_bytes()),
        _ => panic!("Expected literal!"),
    };

    // Prefix with L to avoid conflicts with labels
    (
        format!("l{key}").replace('.', "_"),
        size,
        key,
        Vec::from(bytes),
    )
}
