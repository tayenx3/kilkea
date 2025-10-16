use std::fmt;
use std::collections::HashMap;
use colored::Colorize;
use cranelift::prelude::Value;
use once_cell::unsync::Lazy;
pub fn prec(op: &String) -> Option<(i32, i32)> {
    match &**op {
        "+" | "-" => Some((20, 21)),
        "*" | "/" => Some((30, 31)),
        "==" | ">" | "<"
        | ">=" | "<=" | "!=" => Some((10, 11)),
        "++" => Some((40, 41)),
        ":=" => Some((50, 51)),
        _ => None
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub line: usize,
    pub column: usize,
    pub start_pos: usize,
    pub end_pos: usize
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum ECode {
    UnexpectedEOF, // E1000
    UnexpectedToken, // E1001
    ExpectedToken, // E1002
    UndefinedIdentifier, // E1003

    MismatchedTypes, // E1004
    MutationError, // E1005
}

pub const ERR_MAP: Lazy<HashMap<ECode, String>> = Lazy::new(|| {
    [
        (ECode::UnexpectedEOF, "E1000".to_string()),
        (ECode::UnexpectedToken, "E1001".to_string()),
        (ECode::ExpectedToken, "E1002".to_string()),
        (ECode::UndefinedIdentifier, "E1003".to_string()),
        (ECode::MismatchedTypes, "E1004".to_string()),
        (ECode::MutationError, "E1005".to_string())
    ]
    .into_iter()
    .collect::<HashMap<ECode, String>>()
});

impl fmt::Display for ECode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", ERR_MAP[self])
    }
}

pub const COMBINED_SYMBOLS: &[&str] = &["==", ">=", "<=", "++", ":="];
pub const KEYWORDS: &[&str] = &["if", "else", "mut", "struct", "enum", "func"];

#[derive(Debug, Clone, PartialEq)]
pub enum ParseType {
    Determined(String),
    Inferred
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
#[allow(dead_code)]
pub enum Type {
    Int8, Int16, Int32, Int64,
    Float32, Float64,
    UInt8, UInt16, UInt32, UInt64,
    String,
    Char,
    Boolean,
    Alias(String),
    Void,
    Unit,
    Undetermined
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int8 => write!(f, "i8"),
            Self::Int16 => write!(f, "i16"),
            Self::Int32 => write!(f, "i32"),
            Self::Int64 => write!(f, "i64"),
            Self::Float32 => write!(f, "f32"),
            Self::Float64 => write!(f, "f64"),
            Self::UInt8 => write!(f, "u8"),
            Self::UInt16 => write!(f, "u16"),
            Self::UInt32 => write!(f, "u32"),
            Self::UInt64 => write!(f, "u64"),
            Self::String => write!(f, "string"),
            Self::Char => write!(f, "char"),
            Self::Boolean => write!(f, "bool"),
            Self::Alias(s) => write!(f, "{}", s),
            Self::Void => write!(f, "void"),
            Self::Unit => write!(f, "unit"),
            Self::Undetermined => write!(f, "{{undetermined}}")
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
#[allow(dead_code)]
pub enum Symbol {
    Variable { name: String, type_: Type, mutability: bool }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
#[allow(dead_code)]
pub enum CompilerSymbol {
    Variable { stack_offset: usize, value: TypedValue }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
#[allow(dead_code)]
pub struct TypedValue {
    pub value: Value,
    pub type_: Type
}
#[derive(Debug, Clone, PartialEq)]
pub struct Error {
    pub code: ECode,
    pub details: String,
    pub span: Span,
    pub src: String,
    pub path: String,
    pub note: Option<String>,
    pub help: Option<String>
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format())
    }
}

impl Error {
    fn format(&self) -> String {
        const SPREAD: usize = 2;
        
        let mut output = String::new();
        
        // Header
        output.push_str(&format!("{} {}\n", "error:".red().bold(), self.details.red()));
        
        // Location line
        let location_info = self.format_location(SPREAD);
        output.push_str(&location_info);
        output.push_str(&format!(" {:width$} {}\n",
            "",
            "│".cyan(),
            width = self.calculate_max_digits(
                        self.span.line + SPREAD
        )));
        
        // Error details
        let error_line = self.format_error_line(SPREAD);
        output.push_str(&error_line);
        output.push_str(&format!("\n {:width$} {}",
            "",
            "│".cyan(),
            width = self.calculate_max_digits(
                        self.span.line + SPREAD
        )));

        if let Some(note) = &self.note {
            output.push_str(
                &format!(
                    "\n {:width$} {} {}: {}", 
                    "", 
                    "=".cyan().bold(),
                    "note".bold(),
                    note, 
                    width = self.calculate_max_digits(
                        self.span.line + SPREAD
                    )
                )
            )
        }
        if let Some(help) = &self.help {
            output.push_str(
                &format!(
                    "\n {:width$} > {}: {}", 
                    "",
                    "hint".cyan().bold(), 
                    help,
                    width = self.calculate_max_digits(
                        self.span.line + SPREAD
                    )
                )
            )
        }
        
        output
    }

    fn format_location(&self, spread: usize) -> String {
        let line = self.span.line;
        let digits = self.calculate_max_digits(line + spread);
        
        format!(
            "{:width$}{} {}:{}:{}\n",
            "",
            "┌─".cyan().bold(),
            self.path.italic(),
            line + 1,
            self.span.column,
            width = digits + 2
        )
    }
    
    fn format_error_line(&self, spread: usize) -> String {
        let line = self.span.line;
        let lines: Vec<&str> = self.src.split('\n').collect();
        let digits = self.calculate_max_digits(line + spread);
        
        let mut result = String::new();
        
        // Previous lines
        result.push_str(&self.format_context_lines(line, spread, false, &lines, digits));
        
        // Main line
        result.push_str(&format!(
            " {:width$} {} {}\n",
            (line + 1).to_string().cyan().bold(),
            "│".cyan(),
            lines[line],
            width = digits
        ));
        result.push_str(&format!(
            " {:width$} {} {}{} {}",
            "",
            "│".cyan(),
            " ".repeat(self.span.column),
            "¯".repeat(self.span.end_pos + 1 - self.span.start_pos).red().bold(),
            self.details.red().bold(),
            width = digits
        ));
        
        // Next lines
        result.push_str(&self.format_context_lines(line, spread, true, &lines, digits));
        
        result
    }
    
    fn format_context_lines(
        &self,
        current_line: usize,
        spread: usize,
        is_after: bool,
        lines: &[&str],
        digits: usize,
    ) -> String {
        let mut context = String::new();
        
        let range = if is_after {
            (1..=spread).collect::<Vec<_>>()
        } else {
            (1..=spread).rev().collect::<Vec<_>>()
        };
        
        for i in range {
            let target_line = if is_after {
                current_line.checked_add(i)
            } else {
                current_line.checked_sub(i)
            };
            
            if let Some(line_num) = target_line {
                if line_num < lines.len() {
                    let line_content = if is_after {
                        format!("\n {:width$} {} {}", (line_num + 1).to_string().cyan().bold(), "│".cyan(), lines[line_num], width = digits)
                    } else {
                        format!(" {:width$} {} {}\n", (line_num + 1).to_string().cyan().bold(), "│".cyan(), lines[line_num], width = digits)
                    };
                    context.push_str(&line_content);
                }
            }
        }
        
        context
    }
    
    fn calculate_max_digits(&self, max_line_num: usize) -> usize {
        if max_line_num == 0 {
            1
        } else {
            max_line_num.ilog10() as usize + 1
        }
    }
}