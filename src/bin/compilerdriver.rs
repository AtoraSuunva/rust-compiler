use std::{env, fs, path::Path, process};

use rust_compiler_lib::{
    ast::nodes::{fmt_symbol_table, string_tree},
    codegen::codegen_visitor::CodegenVisitor,
    lexical::lexer::LexerScanner,
    semantic::{
        symbol_collector::SymbolCollectorVisitor, symbol_visitor::SymbolTableVisitor,
        visitor::Visitor,
    },
    syntactic::predictive_parser,
};

fn main() -> Result<(), String> {
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
        return syn_parse_file(path);
    }

    // It's a dir
    for entry in path.read_dir().expect("Failed to read directory").flatten() {
        if entry.metadata().unwrap().is_dir() || entry.path().extension().unwrap() != "src" {
            continue;
        }
        println!("Parsing file: {}", entry.path().display());
        syn_parse_file(entry.path())?;
        println!("\n");
    }

    Ok(())
}

fn syn_parse_file<P>(path: P) -> Result<(), String>
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
    let semantic_tables = path.with_extension("outsymboltables");
    let semantic_err_path = path.with_extension("outsemanticerrors");

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

                println!("\nVisiting...\n");
                let mut visitor = SymbolTableVisitor::new();
                let res = visitor.visit(root);
                let mut visit_errors: Vec<String> = Vec::new();

                if let Err(e) = res {
                    visit_errors.push("First Pass Errors:".to_string());
                    visit_errors.extend(e);
                }

                let mut collector = SymbolCollectorVisitor::new();
                let res = collector.visit(root);

                if let Err(e) = res {
                    visit_errors.push("Collector Pass Errors:".to_string());
                    visit_errors.extend(e);
                }

                let tables = format!(
                    "Global table:\n{}",
                    fmt_symbol_table(&collector.global).unwrap()
                );
                println!("{}", tables);
                fs::write(semantic_tables, tables).expect("Failed to write to file");

                if !visit_errors.is_empty() {
                    println!("{}", visit_errors.join("\n"));
                    fs::write(semantic_err_path, visit_errors.join("\n"))
                        .expect("Failed to write to file");
                }

                let mut codegen_visitor = CodegenVisitor::new();
                let res = codegen_visitor.visit(root);

                if let Err(e) = res {
                    println!("Codegen Errors:\n{}", e.join("\n"));
                }

                let outcode = codegen_visitor.get_code().trim().to_string();
                let moon_out = path.with_extension("moon");

                // println!("Codegen:\n{}", outcode);
                println!("Code generated!");

                fs::write(moon_out, outcode).expect("Failed to write to file");
            } else {
                println!("No AST generated!");
            }

            fs::write(valid_path, derivations.join("\n")).expect("Failed to write to file");

            if !errors.is_empty() {
                fs::write(invalid_path, errors.join("\n")).expect("Failed to write to file");
            }
        }
        Err(err) => {
            eprintln!("Parsing failed: {}", err);
        }
    };

    Ok(())
}
