use std::env;
use std::fs;
use std::process;

mod lexer;

fn main() {
    let args:Vec<String> = env::args().collect();
    if args.len() < 2{
        eprintln!("Usage: cargo run -- <source_file.pas>");
        process::exit(1);
    }

    let file_path = &args[1];

    let source_code = match fs::read_to_string(file_path){
        Ok(code) => code,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", file_path, e);
            process::exit(1);
        }
    };

    println!("Source code:\n{}", source_code);
}