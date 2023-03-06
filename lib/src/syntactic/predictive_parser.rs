use std::{collections::HashMap, env};

use crate::{
    ast::nodes::CodeNode,
    lexical::{
        lexer::LexerScanner,
        tokens::{token::Token, token_type::Type},
    },
    syntactic::first_follow_sets,
};

use super::parsing_table::{self, Production};

type ParserResult = (Vec<String>, Vec<String>, Vec<CodeNode>);

pub fn parse(scanner: &mut LexerScanner) -> Result<ParserResult, String> {
    let debug = env::var("DEBUG").map_or(false, |_| true);
    let mut stack: Vec<&Production> = vec![&Production::NonTerm("START")];
    let mut token = match scanner.next_token() {
        Some(t) => t,
        None => return Err("Empty input!".to_owned()),
    };

    let parsing_table = parsing_table::get_parsing_table();
    let first_sets = first_follow_sets::get_first_set_table();
    let follow_sets = first_follow_sets::get_follow_set_table();

    let mut derivation: Vec<String> = vec![];
    let mut errors: Vec<String> = vec![];
    let mut parsed: Vec<String> = vec![];

    let mut ast_stack: Vec<CodeNode> = Vec::new();
    let mut last_production = &Production::NonTerm("START");
    let mut last_token = Token::empty();

    while !stack.is_empty() {
        // Skip comments
        if token.token_type == Type::InlineCmt || token.token_type == Type::BlockCmt {
            token = match scanner.next_token() {
                Some(t) => t,
                None => Token::empty(),
            };
            continue;
        };

        // Special-case `self` as a generic ID since the grammar doesn't care
        if token.token_type == Type::SelfT {
            token.token_type = Type::Id(String::new());
        }

        if debug {
            eprintln!("token: {:?}", token);
        }

        let top = match stack.last() {
            Some(x) => x,
            None => return Err("Parsing stack emptied too early!".to_owned()),
        };

        if debug {
            eprintln!("top  : {:?}", top);
        }

        match top {
            Production::Term(t) => {
                if debug {
                    eprintln!("t: {:?}", t);
                }

                if t == &token.token_type {
                    parsed.push(token.lexeme.clone());
                    last_production = top;
                    last_token = token.clone();
                    stack.pop();
                    token = match scanner.next_token() {
                        Some(t) => t,
                        None => Token::empty(),
                    };
                } else {
                    if debug {
                        eprintln!("Error: {:?} != {:?}", t, token.token_type);
                    }

                    match skip_error(scanner, &token, top, &mut stack, &first_sets, &follow_sets) {
                        Ok(msg) => errors.push(msg),
                        Err(err) => return Err(err),
                    }
                }
            }

            Production::NonTerm(nt) => {
                if debug {
                    eprintln!("nt: {:?} {:?}", nt, &(nt, token.token_type.clone()));
                    eprintln!("match: {:?}", &(nt, token.token_type.empty_variant()));
                }

                match parsing_table.get(&(nt, token.token_type.empty_variant())) {
                    Some(productions) => {
                        last_production = top;
                        stack.pop();
                        stack.extend(productions.iter().rev());
                        derivation.push(format!(
                            "-> {} {}",
                            parsed.join(" "),
                            stack
                                .iter()
                                .map(|v| v.to_string())
                                .rev()
                                .collect::<Vec<_>>()
                                .join(" ")
                        ));

                        if debug {
                            eprintln!("= {}\n", derivation.last().unwrap());
                        }
                    }
                    None => {
                        if debug {
                            eprintln!(
                                "Error: no production found for {:?} {:?}",
                                nt, token.token_type
                            );
                        }

                        match skip_error(
                            scanner,
                            &token,
                            top,
                            &mut stack,
                            &first_sets,
                            &follow_sets,
                        ) {
                            Ok(msg) => errors.push(msg),
                            Err(err) => return Err(err),
                        }
                    }
                }
            }

            Production::Action(action) => {
                if debug {
                    eprintln!("Running action:");
                    eprintln!("production: {:?}", last_production);
                    eprintln!("ast_stack: {:?}", ast_stack);
                }

                action(&mut ast_stack, last_production, &last_token);
                stack.pop();
            }
        }
    }

    if !stack.is_empty() {
        Err("Parsing stack not empty at the end!".to_owned())
    } else {
        Ok((derivation, errors, ast_stack))
    }
}

type SetTable = HashMap<&'static str, Vec<Type>>;

fn skip_error(
    scanner: &mut LexerScanner,
    lookahead: &Token,
    top: &Production,
    stack: &mut Vec<&Production>,
    first_set: &SetTable,
    follow_set: &SetTable,
) -> Result<String, String> {
    let error_message = format!(
        "Syntax error: Unexpected token '{}' at {}, expected {}",
        lookahead.lexeme,
        lookahead.location,
        stack.last().unwrap()
    );

    eprintln!("{}", error_message);
    // eprintln!("lookahead: {:?}", lookahead);
    // eprintln!("top: {:?}", top);
    // eprintln!("Stack: {:?}", stack);

    let nt = match top {
        Production::NonTerm(nt) => nt,
        _ => {
            stack.pop();
            return Ok(error_message);
        }
    };

    let first = match first_set.get(nt) {
        Some(first) => first,
        None => panic!("No FIRST set found for non-terminal '{}'!", nt),
    };

    let follow = match follow_set.get(nt) {
        Some(follow) => follow,
        None => panic!("No FOLLOW set found for non-terminal '{}'!", nt),
    };

    let mut top_type = lookahead.token_type.empty_variant();

    // Pop the stack if the next token is in the FOLLOW set of our current non-terminal on top of the stack.
    if lookahead.token_type == Type::EndOfFile || follow.contains(&top_type) {
        eprintln!("Skipping token '{}'", lookahead.lexeme);
        stack.pop();
        return Ok(error_message);
    }

    // Scan tokens until we get one with which we can resume the parse.
    while !first.contains(&top_type) || !follow.contains(&top_type) {
        let lookahead = match scanner.next_token() {
            Some(t) => t,
            None => {
                return Err("Unexpected end of input! We cannot recover...".to_owned());
            }
        };

        top_type = lookahead.token_type.empty_variant();

        eprintln!("Skipping token '{}'", lookahead.lexeme);
    }

    Ok(error_message)
}
