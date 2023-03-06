use std::{
    fmt,
    sync::atomic::{AtomicUsize, Ordering},
};

use rctree::Node;

use crate::{lexical::tokens::token::Token, syntactic::parsing_table::Production};

use super::nodes::{CodeNode, NodeValue};

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
        create_subtree_from_n_nodes(String::from("test tree"), 2),
        create_subtree_until_marker(String::from("test tree until marker")),
        create_marker(),
    ];

    let mut stack: Vec<CodeNode> = Vec::new();

    for action in actions {
        action(&mut stack, &Production::NonTerm("test"), &Token::empty());
    }
}

static ID_COUNT: AtomicUsize = AtomicUsize::new(1);

/**
 * Create a new leaf using the last production as a value, if it's a terminal
 */
pub fn create_leaf() -> SemanticAction {
    Box::new(
        move |stack: &mut Vec<CodeNode>, prev: &Production, token: &Token| match prev {
            Production::Term(_t) => {
                let id = ID_COUNT.fetch_add(1, Ordering::SeqCst);
                stack.push(Node::new(NodeValue::Leaf(id, token.token_type.clone())));
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
pub fn create_subtree_from_n_nodes(name: String, count: usize) -> SemanticAction {
    Box::new(
        move |stack: &mut Vec<CodeNode>, _prev: &Production, _token: &Token| {
            let id = ID_COUNT.fetch_add(1, Ordering::SeqCst);
            let subtree = Node::new(NodeValue::Tree(id, name.clone()));

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
pub fn create_subtree_until_marker(name: String) -> SemanticAction {
    Box::new(
        move |stack: &mut Vec<CodeNode>, _prev: &Production, _token: &Token| {
            let id = ID_COUNT.fetch_add(1, Ordering::SeqCst);
            let subtree = Node::new(NodeValue::Tree(id, name.clone()));

            loop {
                match stack.pop() {
                    Some(node) => {
                        if let NodeValue::Marker = *node.borrow() {
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
            stack.push(Node::new(NodeValue::Marker));
        },
    )
}
