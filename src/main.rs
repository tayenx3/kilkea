mod frontend;
mod backend;
mod global;
mod cli;

use colored::Colorize;
use std::{fs, process::{exit, Command}};
use clap::Parser;
use std::env;
use std::path::PathBuf;

fn get_bundled_lld() -> Option<PathBuf> {
    let dev_path = PathBuf::from("bundles/lld");
    if dev_path.exists() {
        return Some(dev_path);
    }
    
    if let Ok(bundle_dir) = env::var("BUNDLE_DIR") {
        let bundle_path = PathBuf::from(bundle_dir);
        if bundle_path.exists() {
            return Some(bundle_path);
        }
    }
    
    None
}

fn link_with_bundled_lld(object_files: &[&str], output: &str) -> Result<(), Box<dyn std::error::Error>> {
    let lld_dir = get_bundled_lld().ok_or("No bundled LLD found")?;
    
    let lld_binary = if cfg!(target_os = "windows") {
        "windows/lld-link.exe"
    } else if cfg!(target_os = "macos") {
        "macos/ld64.lld" 
    } else {
        "linux/ld.lld"
    };
    
    let lld_path = lld_dir.join(lld_binary);
    
    if !lld_path.exists() {
        return Err(format!("Bundled LLD not found at: {}", lld_path.display()).into());
    }
    
    
    let status = if cfg!(target_os = "windows") {
        Command::new(&lld_path)
            .arg(format!("-out:{}", output))
            .arg("-entry:main")
            .arg("-subsystem:console")
            .args(object_files)
            .status()?
    } else {
        Command::new(&lld_path)
            .args(object_files)
            .arg("-o")
            .arg(output)
            .status()?
    };
    
    if status.success() { 
        Ok(()) 
    } else { 
        Err("Linking with bundled LLD failed".into()) 
    }
}

const fn longest_string_length<const N: usize>(strings: &[&str; N]) -> usize {
    let mut max_len = 0;
    let mut i = 0;
    
    while i < N {
        if strings[i].len() > max_len {
            max_len = strings[i].len();
        }
        i += 1;
    }
    max_len
}


const MSGS: [&str; 4] = [
    "Compiling",
    "Compilation stopped due to error(s):",
    "Finished compiling with LLD",
    "Finished compiling with system Clang"
];

const COMPILING: usize = 0;
const ERROR: usize = 1;
const COMPILED_LLD: usize = 2;
const COMPILED_CLANG: usize = 3;

fn main() {
    let cli = cli::Cli::parse();

    run(cli.input, cli.output, cli.debug, cli.parse_only)
}

fn run(input: String, output: Option<String>, debug: bool, parse_only: bool) {
    let path = &*input;
    let width = longest_string_length(&MSGS) + 5;

    println!("{:>width$} `{}`", MSGS[COMPILING].green().bold(), path);
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
            "{:>width$}\n{}", 
            MSGS[ERROR].red().bold(), 
            errors
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join("\n\n")
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
            "{:>width$}\n{}", 
            MSGS[ERROR].red().bold(), 
            r
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join("\n\n")
        );
        exit(1)
    }

    if parse_only { exit(0) }

    let mut compiler = match backend::ASTCompiler::new(
        debug,
        parsed,
        contents,
        path.to_string()
    ) {
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
    let object_files = vec!["output.o"];
    match link_with_bundled_lld(&object_files, &output_name) {
        Ok(()) => println!("{:>width$}", MSGS[COMPILED_LLD].green().bold()),
        Err(e) => {
            eprintln!("{} Bundled LLD failed: {}", "WARNING:".yellow().bold(), e);
            eprintln!("{} Falling back to system Clang linker...", "WARNING:".yellow().bold());
            
            // Fallback to system linker (Clang)
            match link_with_system_linker(&object_files, &output_name) {
                Ok(()) => println!("{:>width$}", MSGS[COMPILED_CLANG].green().bold()),
                Err(e) => {
                    eprintln!("All linking methods failed: {}", e);
                },
            }
        }
    }
    if std::path::Path::new("output.o").exists() {
        let _ = std::fs::remove_file("output.o");
    }
}

fn link_with_system_linker(object_files: &[&str], output: &str) -> Result<(), Box<dyn std::error::Error>> {
    let status = std::process::Command::new("clang")
        .args(object_files)
        .arg("-o")
        .arg(output)
        .status()?;
    
    if status.success() { 
        Ok(()) 
    } else { 
        Err("System linking failed".into()) 
    }
}