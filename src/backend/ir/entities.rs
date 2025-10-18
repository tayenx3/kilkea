use std::fmt;

use super::inst::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Const {
    I8(i8), I16(i16), I32(i32), I64(i64),
    U8(u8), U16(u16), U32(u32), U64(u64),
    F32(f32), F64(f64),
    Bool(u8),
    Void,
}

impl Const {
    pub(crate) fn get_type(&self) -> String {
        match self {
            Self::I8(_) => "i8".to_string(),
            Self::I16(_) => "i16".to_string(),
            Self::I32(_) => "i32".to_string(),
            Self::I64(_) => "i64".to_string(),
            Self::U8(_) => "u8".to_string(),
            Self::U16(_) => "u16".to_string(),
            Self::U32(_) => "u32".to_string(),
            Self::U64(_) => "u64".to_string(),
            Self::F32(_) => "f32".to_string(),
            Self::F64(_) => "f64".to_string(),
            Self::Bool(_) => "bool".to_string(),
            Self::Void => "void".to_string(),
        }
    }

    pub(crate) fn get_value(&self) -> String {
        match self {
            Self::I8(i) => i.to_string(),
            Self::I16(i) => i.to_string(),
            Self::I32(i) => i.to_string(),
            Self::I64(i) => i.to_string(),
            Self::U8(i) => i.to_string(),
            Self::U16(i) => i.to_string(),
            Self::U32(i) => i.to_string(),
            Self::U64(i) => i.to_string(),
            Self::F32(i) => i.to_string(),
            Self::F64(i) => i.to_string(),
            Self::Bool(i) => i.to_string(),
            Self::Void => "VOID".to_string(),
        }
    }
}

impl fmt::Display for Const {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} : {}", self.get_type(), self.get_value())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Type {
    I8, I16, I32, I64,
    U8, U16, U32, U64,
    F32, F64,
    Bool,
    Void,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::I8 => write!(f, "i8"),
            Self::I16 => write!(f, "i16"),
            Self::I32 => write!(f, "i32"),
            Self::I64 => write!(f, "i64"),
            Self::U8 => write!(f, "u8"),
            Self::U16 => write!(f, "u16"),
            Self::U32 => write!(f, "u32"),
            Self::U64 => write!(f, "u64"),
            Self::F32 => write!(f, "f32"),
            Self::F64 => write!(f, "f64"),
            Self::Bool => write!(f, "bool"),
            Self::Void => write!(f, "void"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ValueID(pub usize, pub Type);

impl fmt::Display for ValueID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "%{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ParamID(pub usize, pub Type);

impl fmt::Display for ParamID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} %{}", self.1, self.0)
    }
}

impl From<ParamID> for ValueID {
    fn from(param: ParamID) -> Self {
        ValueID(param.0, param.1)
    }
}

impl From<ValueID> for ParamID {
    fn from(value: ValueID) -> Self {
        ParamID(value.0, value.1)
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    pub(crate) alias: String,
    pub(crate) blocks: Vec<Block>,
    pub(crate) sig: FunctionSignature
}

#[derive(Debug, Clone)]
pub struct Block {
    pub(crate) id: BlockID,
    pub(crate) insts: Vec<Inst>,
    pub(crate) params: Vec<ParamID>
}

#[derive(Debug, Clone)]
pub struct FunctionSignature {
    pub(crate) params: Vec<Type>,
    pub(crate) return_ty: Type
}

impl From<&FunctionSignature> for FunctionSignature {
    fn from(sig: &FunctionSignature) -> Self {
        sig.clone()
    }
}

impl From<&mut FunctionSignature> for FunctionSignature {
    fn from(sig: &mut FunctionSignature) -> Self {
        sig.clone()
    }
}

impl FunctionSignature {
    pub fn new() -> Self {
        Self {
            params: Vec::new(),
            return_ty: Type::Void
        }
    }

    pub fn with_params(mut self, params: Vec<Type>) -> Self {
        self.params = params;
        self
    }

    pub fn add_param(mut self, param: Type) -> Self {
        self.params.push(param);
        self
    }

    pub fn with_return_ty(mut self, return_ty: Type) -> Self {
        self.return_ty = return_ty;
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockID(pub usize);

impl fmt::Display for BlockID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "u{}", self.0)
    }
}