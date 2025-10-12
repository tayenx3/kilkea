use std::fmt;
use std::collections::HashMap;
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