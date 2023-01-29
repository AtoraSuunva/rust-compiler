use std::{env, fs, path::Path, process};

use rust_compiler_lib::lexical::{lexer::LexerScanner, tokens::token_type::Type};

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
        lex_file(path);
        return;
    }

    // It's a dir
    for entry in path.read_dir().expect("Failed to read directory").flatten() {
        if entry.metadata().unwrap().is_dir() || entry.path().extension().unwrap() != "src" {
            continue;
        }
        println!("Lexing file: {}", entry.path().display());
        lex_file(entry.path());
        println!("\n");
    }
}

fn lex_file<P>(path: P)
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

    let lexer = LexerScanner::new(&content);

    let mut last_line = 1;
    let mut tokens: Vec<String> = vec![];
    let mut errors: Vec<String> = vec![];

    for token in lexer {
        if token.location.line != last_line {
            tokens.push("\n".to_owned());
            println!();
            last_line = token.location.line;
        }

        if let Type::Invalid(err) = &token.token_type {
            errors.push(format!(
                "Lexical error: {}: \"{}\": line {}\n",
                err.as_detailed(),
                token.lexeme.replace('\n', "\\n").replace('\r', "\\r"),
                token.location.line
            ));
        }

        tokens.push(token.to_string());
        tokens.push(" ".to_owned());

        print!("{} ", token);
    }

    let valid_tokens = tokens.join("");
    let invalid_tokens = errors.join("");

    let valid_path = path.with_extension("outlextokens");
    let invalid_path = path.with_extension("outlexerrors");

    match fs::write(valid_path, valid_tokens.trim_end()) {
        Ok(_) => (),
        Err(err) => {
            eprintln!("Error while writing valid tokens to file: {}", err);
            process::exit(1);
        }
    }

    match fs::write(invalid_path, invalid_tokens.trim_end()) {
        Ok(_) => (),
        Err(err) => {
            eprintln!("Error while writing invalid tokens to file: {}", err);
            process::exit(1);
        }
    }
}
