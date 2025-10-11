mod frontend;
mod backend;
mod global;
mod cli;

use colored::Colorize;
use std::{fs, process::{exit, Command}};
use clap::Parser;

fn main() {
    let cli = cli::Cli::parse();

    run(cli.input, cli.output, cli.debug)
}

fn run(input: String, output: Option<String>, debug: bool) {
    let path = &*input;
    println!("\t{:20} `{}`", "Compiling".green().bold(), path);
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
        eprintln!(
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

    let mut type_checker = frontend::TypeChecker::new(parsed.clone(), contents.clone(), path.to_string());
    let r = type_checker.check();
    if r.is_empty() {
        if debug { println!("{}", "\nType Checker finished without errors".cyan().bold()) }
    } else {
        eprintln!(
            "{}\n{}", 
            "Compilation stopped due to error(s):".red().bold(), 
            r
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join("\n")
        );
        exit(1)
    }

    let raw = backend::ASTCompiler::new(
        debug,
        parsed,
        contents,
        path.to_string()
    );

    let mut compiler = match raw {
        Ok(compiler) => compiler,
        Err(e) => {
            eprintln!("Could not create compiler due to:\n{}", e);
            exit(1)
        }
    };
    
    // Step 5: Compile to object file
    match compiler.compile_module() {
        Ok(()) => {},
        Err(e) => {
            eprintln!("Could not compiler due to:\n{}", e);
            exit(1)
        }
    }
    let output_name = if let Some(output) = output {
        if output.ends_with(".exe") { output } else { format!("{}.exe", output) }
    } else {
        "main.exe".to_string()
    };
    Command::new("clang")
        .arg("output.o")
        .arg("-o")
        .arg(&output_name)
        .output()
        .expect("Failed to link object file, please install clang first");
    println!("\t{:20}", "Finished compiling".green().bold());
    if std::path::Path::new("output.o").exists() {
        let _ = std::fs::remove_file("output.o");
    }
}