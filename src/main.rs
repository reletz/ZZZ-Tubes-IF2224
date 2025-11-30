use std::env;
use std::fs;
use std::process;

mod lexer;
mod parser;
mod semantic_analyzer;

use lexer::lexer::PascalLexer;
use parser::parser::PascalParser;

use semantic_analyzer::ast::ast_builder::ASTBuilder;
use semantic_analyzer::ast::ast_printer::ASTPrinter;
use semantic_analyzer::analyzer::SemanticAnalyzer;

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
    // for token in &tokens {
    //     println!("{}", token);
    // }

    // 4. PARSER
    let mut parser = PascalParser::new(tokens);
    let parse_tree = match parser.parse() {
        Ok(pt) => {
            println!(">> Parsing Berhasil!");
            pt
        }
        Err(e) => {
            eprintln!(">> Parsing Gagal: {}", e);
            process::exit(1);
        }
    };

    let mut ast = match ASTBuilder::build(&parse_tree) {
        Ok(ast) => {
            println!(">> AST Berhasil dibangun.");
            ast
        },
        Err(e) => {
            eprintln!(">> Gagal membangun AST (Semantic Error di Builder): {}", e);
            process::exit(1);
        }
    };

    // 5. SEMANTIC ANALYZER (Type Check & Symbol Table)
    let mut analyzer = SemanticAnalyzer::new();
    
    match analyzer.analyze(&mut ast) {
        Ok(_) => {
            println!(">> Analisis Semantik BERHASIL!");
            
            println!(">> Symbol Table State:");
            analyzer.print_tables();

            println!(">> Decorated AST:");
            ASTPrinter::print(&ast);
        },
        Err(errors) => {
            eprintln!(">> Analisis Semantik GAGAL dengan {} error:", errors.len());
            for (i, e) in errors.iter().enumerate() {
                eprintln!("  [{}] {}", i + 1, e);
            }
            process::exit(1);
        }
    }
}