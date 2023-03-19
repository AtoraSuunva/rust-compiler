use std::{
    mem::discriminant,
    sync::atomic::{AtomicUsize, Ordering},
};

use rctree::Node;

use crate::lexical::tokens::token_type::Type;

use super::tree_node::TreeNode;

#[derive(Debug)]
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

#[derive(Debug)]
pub struct StructNode {
    pub id: usize,
    pub value: NodeValue,
}

static ID_COUNT: AtomicUsize = AtomicUsize::new(1);

impl StructNode {
    pub fn new(value: NodeValue) -> Self {
        let id = if NodeValue::Marker.eq_variant(&value) {
            0
        } else {
            ID_COUNT.fetch_add(1, Ordering::SeqCst)
        };

        Self { id, value }
    }

    pub fn new_node(value: NodeValue) -> CodeNode {
        Node::new(Self::new(value))
    }
}

pub type CodeNode = Node<StructNode>;

trait CoolNode {
    fn id(&self) -> usize;
    fn as_string(&self) -> String;
}

impl CoolNode for CodeNode {
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
