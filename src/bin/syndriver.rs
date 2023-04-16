use std::{env, fs, path::Path, process};

use rust_compiler_lib::{
    ast::nodes::string_tree, compiler_error::errors_to_string, lexical::lexer::LexerScanner,
    syntactic::predictive_parser,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = match args.get(1) {
        Some(filename) => filename,
        None => {
            eprintln!("No path to file provided");
            process::exit(1);
        }
    };

    let path = Path::new(&filename);

    if !path.exists() {
        eprintln!("File or directory does not exist");
        process::exit(1);
    }

    if path.is_file() {
        syn_parse_file(path);
        return;
    }

    // It's a dir
    for entry in path.read_dir().expect("Failed to read directory").flatten() {
        if entry.metadata().unwrap().is_dir() || entry.path().extension().unwrap() != "src" {
            continue;
        }
        println!("Parsing file: {}", entry.path().display());
        syn_parse_file(entry.path());
        println!("\n");
    }
}

fn syn_parse_file<P>(path: P)
where
    P: AsRef<Path>,
{
    let path: &Path = path.as_ref();
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error while reading file: {}", err);
            process::exit(1);
        }
    };

    let mut lexer = LexerScanner::new(&content);

    let ast_path = path.with_extension("outast");
    let valid_path = path.with_extension("outderivation");
    let invalid_path = path.with_extension("outsyntaxerrors");

    match predictive_parser::parse(&mut lexer) {
        Ok((derivations, errors, ast_stack)) => {
            println!("Parsing successful!");
            println!("Errors: {}", errors.len());
            println!("Derivations: {}", derivations.len());
            println!("Last Derivation:\n{}", derivations.last().unwrap());

            println!("\n\nFinal AST:");
            println!("ast_stack: {:?}", ast_stack);

            if let Some(root) = ast_stack.first() {
                fs::write(ast_path, string_tree(root)).expect("Failed to write to file");
            } else {
                println!("No AST generated!");
            }

            fs::write(valid_path, derivations.join("\n")).expect("Failed to write to file");
            fs::write(invalid_path, errors_to_string(&errors)).expect("Failed to write to file");
        }
        Err(err) => {
            eprintln!("Parsing failed:\n{}", errors_to_string(&err));
        }
    };
}
