use rctree::Node;

use crate::lexical::tokens::token_type::Type;

pub type CodeNode = Node<NodeValue>;

#[derive(Debug)]
pub enum NodeValue {
    Leaf(usize, Type),
    Tree(usize, String),
    Marker,
}

trait CoolNode {
    fn id(&self) -> usize;
    fn as_string(&self) -> String;
}

impl CoolNode for CodeNode {
    fn id(&self) -> usize {
        match *self.borrow() {
            NodeValue::Leaf(id, _) => id,
            NodeValue::Tree(id, _) => id,
            NodeValue::Marker => 0,
        }
    }

    fn as_string(&self) -> String {
        let id = self.id();

        let name = match &*self.borrow() {
            NodeValue::Leaf(_, t) => format!("{:?}", t).replace('\"', "'"),
            NodeValue::Tree(_, s) => s.clone(),
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
