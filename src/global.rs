use core::fmt;
use std::collections::HashMap;
use once_cell::unsync::Lazy;
pub fn prec(op: &String) -> Option<(i32, i32)> {
    match &**op {
        "+" | "-" => Some((20, 21)),
        "*" | "/" => Some((30, 31)),
        "==" | ">" | "<"
        | ">=" | "<=" | "!=" => Some((10, 11)),
        "++" => Some((40, 41)),
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
}

pub const ERR_MAP: Lazy<HashMap<ECode, String>> = Lazy::new(|| {
    [
        (ECode::UnexpectedEOF, "E1000".to_string()),
        (ECode::UnexpectedToken, "E1001".to_string()),
        (ECode::ExpectedToken, "E1002".to_string()),
        (ECode::UndefinedIdentifier, "E1003".to_string()),
        (ECode::MismatchedTypes, "E1004".to_string())
    ]
    .into_iter()
    .collect::<HashMap<ECode, String>>()
});

impl fmt::Display for ECode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", ERR_MAP[self])
    }
}

pub const COMBINED_SYMBOLS: &[&str] = &["==", ">=", "<=", "++"];
pub const KEYWORDS: &[&str] = &["let", "if", "else"];

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub struct Scope {
    symbols: HashMap<String, CompileValue>
}

impl Scope {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new()
        }
    }

    pub fn find(&self, s: &String) -> Option<CompileValue> {
        self.symbols.get(s).cloned()
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum CompileValue {
    Integer,
    Float,
    String,
    Boolean,
    Unit
}

impl CompileValue {
    pub fn to_type(&self) -> String {
        match self {
            Self::Integer => "integer".to_string(),
            Self::Float => "float".to_string(),
            Self::String => "string".to_string(),
            Self::Boolean => "boolean".to_string(),
            Self::Unit => "()".to_string()
        }
    }
}