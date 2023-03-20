use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::{Display, Formatter},
    mem::discriminant,
    sync::atomic::{AtomicUsize, Ordering},
};

use rctree::Node;

use crate::lexical::tokens::{token::Token, token_type::Type};

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

#[derive(Debug, Clone, Default)]
pub struct SymbolTable(pub HashMap<String, Option<SymbolTable>>);

impl Display for SymbolTable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let longest_key = self
            .0
            .keys()
            .map(|s| s.len())
            .max()
            .unwrap_or(0)
            .max("Symbol".len());

        let longest_value = self
            .0
            .values()
            .map(|s| s.as_ref().map(|s| s.0.len()).unwrap_or(0))
            .max()
            .unwrap_or(0)
            .max("-> Symbol Table".len());

        writeln!(
            f,
            "| {:=^longest_key$} | {:=^longest_value$} |",
            "Symbol", "Pointer"
        )?;

        let mut other_tables: Vec<(String, SymbolTable)> = Vec::new();

        for (key, value) in &self.0 {
            write!(f, "| {:<longest_key$} | ", key)?;
            if let Some(table) = value {
                other_tables.push((key.clone(), table.clone()));
                write!(f, "{:^longest_value$}", "-> Symbol Table")?;
            } else {
                write!(f, "{:^longest_value$}", "")?;
            }
            writeln!(f, " |")?;
        }

        for (key, table) in other_tables {
            write!(f, "\n{key}:\n{table}")?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct StructNode {
    pub id: usize,
    pub value: NodeValue,
    pub token: Token,
    pub symbol_table: RefCell<Option<SymbolTable>>,
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
    type Error = String;

    fn try_from(value: NodeValue) -> Result<Self, Self::Error> {
        match value {
            NodeValue::Leaf(t) => Ok(t),
            _ => Err(format!("Expected {} to be a leaf", value)),
        }
    }
}

impl TryFrom<NodeValue> for TreeNode {
    type Error = String;

    fn try_from(value: NodeValue) -> Result<Self, Self::Error> {
        match value {
            NodeValue::Tree(t) => Ok(t),
            _ => Err(format!("Expected {} to be a tree", value)),
        }
    }
}

impl TryFrom<StructNode> for Type {
    type Error = String;

    fn try_from(node: StructNode) -> Result<Self, Self::Error> {
        node.value
            .clone()
            .try_into()
            .map_err(|e| -> String { format!("{}: {}", e, node) })
    }
}

impl TryFrom<StructNode> for TreeNode {
    type Error = String;

    fn try_from(node: StructNode) -> Result<Self, Self::Error> {
        node.value
            .clone()
            .try_into()
            .map_err(|e| -> String { format!("{}: {}", e, node) })
    }
}

impl TryFrom<CodeNode> for Type {
    type Error = String;

    fn try_from(node: CodeNode) -> Result<Self, Self::Error> {
        node.borrow()
            .value
            .clone()
            .try_into()
            .map_err(|e| -> String { format!("{}: {}", e, node) })
    }
}
