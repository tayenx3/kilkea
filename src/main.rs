mod frontend;
mod backend;
mod global;

use colored::Colorize;
use std::{fs, process::exit};

fn main() {
    let debug = false;

    let path = "tests/main.kl";
    let contents = fs::read_to_string(path)
    .expect(&format!("Unable to read from: {}", path))
    .replace("\r\n", "\n");
    if debug {
        println!("{}", " === DEBUG ===".cyan().bold());
        println!("{}\n{}", "Contents:".cyan().bold(), contents)
    }

    let tokens = frontend::tokenize(&contents);
    if debug {
        println!();
        println!("{}\n{:#?}", "Tokens:".cyan().bold(), tokens)
    }

    let mut parser = frontend::Parser::new(tokens);
    let (parsed, errors) = parser.parse_program();

    if !errors.is_empty() {
        println!(
            "{}\n{}", 
            "Error!".red().bold(), 
            errors
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join("\n")
        );
        exit(1)
    }
    if debug {
        println!();
        println!("{}\n{:#?}", "AST:".cyan().bold(), parsed)
    }
}