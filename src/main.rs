use std::env;
use std::fs;
use std::process;

mod lexer;

use lexer::lexer::PascalLexer;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run -- <path/to/source_file.pas>");
        process::exit(1);
    }

    let file_path = &args[1];

    let source_code = match fs::read_to_string(file_path) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", file_path, e);
            process::exit(1);
        }
    };

    let mut lexer = PascalLexer::new(&source_code);

    let tokens = lexer.get_all_tokens();
    for token in tokens {
        println!("{}", token);
    }
}