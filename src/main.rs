use std::env;
use std::fs;
use std::process;

mod lexer;
mod parser;

use lexer::lexer::PascalLexer;
use parser::parser::PascalParser;
use parser::tree_printer::ParseTreePrinter;

fn main() {
    // 1. Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run -- <path/to/source_file.pas>");
        process::exit(1);
    }

    // Get the .pas file path
    let file_path = &args[1];

    // 2. Read source code from file
    let source_code = match fs::read_to_string(file_path) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", file_path, e);
            process::exit(1);
        }
    };

    // 3. LEXER
    let mut lexer = PascalLexer::new(&source_code);
    let tokens = lexer.get_all_tokens();
    // for token in tokens {
    //     println!("{}", token);
    // }

    // 4. PARSER
    let mut parser = PascalParser::new(tokens);
    match parser.parse() {
        Ok(parse_tree) => {
            println!("Parsing Berhasil!\n");
            
            // Print AST
            let mut printer = ParseTreePrinter::new();
            let tree_output = printer.print_program(&parse_tree);
            println!("{}", tree_output);

            // println!("{:#?}", parse_tree);
        }
        Err(e) => {
            eprintln!("Parsing Gagal: {}", e);
            process::exit(1);
        }
    }

    // TODO: Milestone 3
    // pass ast_tree ke Semantic Analyzer
}