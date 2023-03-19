use std::fmt;

use crate::{lexical::tokens::token::Token, syntactic::parsing_table::Production};

use super::{
    nodes::{CodeNode, NodeValue, StructNode},
    tree_node::TreeNode,
};

pub trait SemanticActionTrait: Fn(&mut Vec<CodeNode>, &Production, &Token) {}
impl<F> SemanticActionTrait for F where F: Fn(&mut Vec<CodeNode>, &Production, &Token) {}

pub type SemanticAction = Box<dyn SemanticActionTrait>;

impl fmt::Debug for SemanticAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[Action]")
    }
}

pub fn test() {
    println!("Hello from actions!");
    let actions: Vec<SemanticAction> = vec![
        create_leaf(),
        create_subtree_from_n_nodes(TreeNode::InheritsList, 2),
        create_subtree_until_marker(TreeNode::InheritsList),
        create_marker(),
    ];

    let mut stack: Vec<CodeNode> = Vec::new();

    for action in actions {
        action(&mut stack, &Production::NonTerm("test"), &Token::empty());
    }
}

/**
 * Create a new leaf using the last production as a value, if it's a terminal
 */
pub fn create_leaf() -> SemanticAction {
    Box::new(
        move |stack: &mut Vec<CodeNode>, prev: &Production, token: &Token| match prev {
            Production::Term(_t) => {
                stack.push(StructNode::new_node(NodeValue::Leaf(
                    token.token_type.clone(),
                )));
            }

            Production::NonTerm(nt) => {
                panic!(
                    "Non-terminal found in stack while creating leaf node: {}",
                    nt
                );
            }

            Production::Action(_) => {
                panic!("Action found in stack while creating leaf node!");
            }
        },
    )
}

/**
 * Pop the last n nodes and create a new subtree using them
 */
pub fn create_subtree_from_n_nodes<F>(name: F, count: usize) -> SemanticAction
where
    F: 'static + Fn() -> TreeNode,
{
    Box::new(
        move |stack: &mut Vec<CodeNode>, _prev: &Production, _token: &Token| {
            let subtree = StructNode::new_node(NodeValue::Tree(name()));

            for _ in 0..count {
                match stack.pop() {
                    Some(node) => subtree.prepend(node),
                    None => panic!("Stack is empty while creating subtree!"),
                }
            }

            stack.push(subtree);
        },
    )
}

/**
 * Create a new subtree using all the previous nodes until we reach a Marker node, naming it after the last production
 */
pub fn create_subtree_until_marker<F>(name: F) -> SemanticAction
where
    F: 'static + Fn() -> TreeNode,
{
    Box::new(
        move |stack: &mut Vec<CodeNode>, _prev: &Production, _token: &Token| {
            let subtree = StructNode::new_node(NodeValue::Tree(name()));

            loop {
                match stack.pop() {
                    Some(node) => {
                        if let NodeValue::Marker = node.borrow().value {
                            break;
                        }

                        subtree.prepend(node);
                    }
                    None => panic!("Stack is empty while creating subtree!"),
                }
            }

            stack.push(subtree);
        },
    )
}

/**
 * Push a marker node to the stack, like pushing an epsilon ε but actually typeable on a keyboard
 */
pub fn create_marker() -> SemanticAction {
    Box::new(
        move |stack: &mut Vec<CodeNode>, _prev: &Production, _token: &Token| {
            stack.push(StructNode::new_node(NodeValue::Marker));
        },
    )
}
