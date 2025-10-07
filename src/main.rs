mod frontend;
mod backend;
mod global;
mod cli;

use colored::Colorize;
use std::{fs, process::exit};
use clap::Parser;

fn main() {
    let cli = cli::Cli::parse();

    run(cli.input, cli.output, cli.debug)
}

fn run(input: String, _output: Option<String>, debug: bool) {
    let path = &*input;
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

    let mut parser = frontend::Parser::new(tokens, &contents, &path.to_string());
    let (parsed, errors) = parser.parse_program();

    if !errors.is_empty() {
        println!(
            "{}\n{}", 
            "Compilation stopped due to error(s):".red().bold(), 
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