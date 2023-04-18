use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::{Display, Formatter, Write},
    mem::discriminant,
    rc::Rc,
    sync::atomic::{AtomicUsize, Ordering},
};

use rctree::Node;

use crate::{
    compiler_error::CompilerError,
    lexical::tokens::{token::Token, token_type::Type},
};

use super::tree_node::TreeNode;

#[derive(Debug, Clone)]
pub enum NodeValue {
    Leaf(Type),
    Tree(TreeNode),
    Marker,
}

impl NodeValue {
    pub fn eq_variant(&self, other: &NodeValue) -> bool {
        discriminant(self) == discriminant(other)
    }
}

impl Display for NodeValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeValue::Leaf(t) => write!(f, "{}", t),
            NodeValue::Tree(t) => write!(f, "{}", t),
            NodeValue::Marker => write!(f, "Marker"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VarType {
    Integer(Vec<usize>),
    Float(Vec<usize>),
    Class(String),
    Function,
    Void,
    Global,
    IndiceList(usize),
    ArgumentList(Vec<VarType>),
    Inherits(Vec<String>),
}

impl VarType {
    pub fn eq_variant(&self, other: &VarType) -> bool {
        discriminant(self) == discriminant(other)
    }
}

impl Display for VarType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            VarType::Integer(idx) => write!(f, "Integer{}", indexes_to_string(idx)),
            VarType::Float(idx) => write!(f, "Float{}", indexes_to_string(idx)),
            VarType::Class(c) => write!(f, "Class({})", c),
            VarType::Function => write!(f, "Function"),
            VarType::Void => write!(f, "Void"),
            VarType::Global => write!(f, "Global"),
            VarType::IndiceList(i) => write!(f, "IndiceList({})", i),
            VarType::ArgumentList(args) => {
                write!(
                    f,
                    "ArgumentList({})",
                    args.iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            VarType::Inherits(inherits) => {
                write!(
                    f,
                    "Inherits({})",
                    inherits
                        .iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
        }
    }
}

fn indexes_to_string(indexes: &[usize]) -> String {
    if indexes.is_empty() {
        String::new()
    } else {
        format!(
            "[{}]",
            indexes
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

#[derive(Debug, Clone)]
pub struct SymbolData {
    pub size: usize,
    pub offset: isize,
    pub label: Option<String>,
    pub table: Option<SymbolTable>,
    pub var_type: VarType,
}

impl SymbolData {
    pub fn new(size: usize, offset: isize, var_type: VarType) -> Self {
        Self {
            size,
            offset,
            label: None,
            table: None,
            var_type,
        }
    }

    pub fn new_with_table(
        size: usize,
        offset: isize,
        var_type: VarType,
        table: SymbolTable,
    ) -> Self {
        Self {
            size,
            offset,
            label: None,
            table: Some(table),
            var_type,
        }
    }
}

impl Display for SymbolData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "size: {}, offset: {}, type: {}{}",
            self.size,
            self.offset,
            self.var_type,
            if self.table.is_some() {
                ", -> Symbol Table"
            } else {
                ""
            }
        )
    }
}

pub type SymbolTable = HashMap<String, Rc<RefCell<SymbolData>>>;

pub fn fmt_symbol_table(table: &SymbolTable) -> Result<String, std::fmt::Error> {
    let longest_key = table
        .keys()
        .map(|s| s.len())
        .max()
        .unwrap_or(0)
        .max(" Symbol ".len());

    let longest_value = table
        .values()
        .map(|s| s.borrow().to_string().len())
        .max()
        .unwrap_or(0)
        .max(" Data ".len());

    let mut output = String::new();

    writeln!(
        output,
        "| {:=^longest_key$} | {:=^longest_value$} |",
        " Symbol ", " Data "
    )?;

    let mut other_tables: Vec<(String, SymbolTable)> = Vec::new();

    let mut to_print = table.iter().collect::<Vec<_>>();

    to_print.sort_by(
        |(k1, v1), (k2, v2)| match v2.borrow().offset.cmp(&v1.borrow().offset) {
            std::cmp::Ordering::Equal => k1.cmp(k2),
            ord => ord,
        },
    );

    for (key, value) in to_print {
        writeln!(
            output,
            "| {:<longest_key$} | {:<longest_value$} |",
            key,
            value.borrow().to_string(),
        )?;

        if key != ".." && !key.starts_with("_in") {
            if let Some(other) = &value.borrow().table {
                other_tables.push((key.clone(), other.clone()));
            }
        }
    }

    for (key, other) in other_tables {
        let other_out = fmt_symbol_table(&other)?;
        write!(output, "\n{key}:\n{other_out}")?;
    }

    Ok(output)
}

#[derive(Debug)]
pub struct StructNode {
    pub id: usize,
    pub value: NodeValue,
    pub token: Token,
    pub symbol_table: RefCell<Option<SymbolTable>>,

    pub label: Rc<RefCell<Option<String>>>,
    pub code: Rc<RefCell<Option<String>>>,
    pub var_type: Rc<RefCell<Option<VarType>>>,
}

static ID_COUNT: AtomicUsize = AtomicUsize::new(1);

impl StructNode {
    pub fn new(value: NodeValue, token: Token) -> Self {
        let id = if NodeValue::Marker.eq_variant(&value) {
            0
        } else {
            ID_COUNT.fetch_add(1, Ordering::SeqCst)
        };

        Self {
            id,
            value,
            token,
            symbol_table: RefCell::new(None),
            label: Rc::new(RefCell::new(None)),
            code: Rc::new(RefCell::new(None)),
            var_type: Rc::new(RefCell::new(None)),
        }
    }

    pub fn new_node(value: NodeValue, token: Token) -> CodeNode {
        Node::new(Self::new(value, token))
    }
}

impl Display for StructNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.value, self.token)
    }
}

pub type CodeNode = Node<StructNode>;

trait GraphNode {
    fn id(&self) -> usize;
    fn as_string(&self) -> String;
}

impl GraphNode for CodeNode {
    fn id(&self) -> usize {
        self.borrow().id
    }

    fn as_string(&self) -> String {
        let id = self.id();

        let name = match &self.borrow().value {
            NodeValue::Leaf(t) => format!("{:?}", t).replace('\"', "'"),
            NodeValue::Tree(t) => format!("{:?}", t).replace('\"', "'"),
            NodeValue::Marker => String::from("MARKER"),
        };

        let parent_id = match self.parent() {
            Some(p) => format!("{} -> {}", p.id(), id),
            None => String::new(),
        };

        format!("{id}[label=\"{name}\"]\n{parent_id}")
    }
}

pub fn string_tree(node: &CodeNode) -> String {
    let body = node
        .descendants()
        .map(|n| n.as_string())
        .collect::<Vec<String>>()
        .join("\n");

    format!("digraph AST {{\nnode [shape=record];\nnode [fontname=Sans];charset=\"UTF-8\" splines=true splines=spline rankdir =LR\n{}\n}}", body)
}

impl TryFrom<NodeValue> for Type {
    type Error = CompilerError;

    fn try_from(value: NodeValue) -> Result<Self, Self::Error> {
        match value {
            NodeValue::Leaf(t) => Ok(t),
            _ => Err(CompilerError::new_with_message(format!(
                "Expected {} to be a leaf",
                value
            ))),
        }
    }
}

impl TryFrom<NodeValue> for TreeNode {
    type Error = CompilerError;

    fn try_from(value: NodeValue) -> Result<Self, Self::Error> {
        match value {
            NodeValue::Tree(t) => Ok(t),
            _ => Err(CompilerError::new_with_message(format!(
                "Expected {} to be a tree",
                value
            ))),
        }
    }
}

impl TryFrom<StructNode> for Type {
    type Error = CompilerError;

    fn try_from(node: StructNode) -> Result<Self, Self::Error> {
        node.value
            .clone()
            .try_into()
            .map_err(|e: CompilerError| CompilerError::new(e.message, node.token))
    }
}

impl TryFrom<StructNode> for TreeNode {
    type Error = CompilerError;

    fn try_from(node: StructNode) -> Result<Self, Self::Error> {
        node.value
            .clone()
            .try_into()
            .map_err(|e: CompilerError| CompilerError::new(e.message, node.token))
    }
}

impl TryFrom<CodeNode> for Type {
    type Error = CompilerError;

    fn try_from(node: CodeNode) -> Result<Self, Self::Error> {
        node.borrow().value.clone().try_into()
    }
}
