//! # Kese IR Generation
//! 
//! * Contains essential IR codegen components

#![allow(unused)]

pub mod inst;
pub mod entities;
pub mod codegen;
pub mod optimization;

pub mod prelude {
    pub use super::{
        codegen::{
            builders::*, 
            context::*
        }, 
        entities::FunctionSignature, 
        types, 
        inst::{
            CmpPred, BlockCall
        },
        optimization::*
    };
}

pub mod types {
    use super::entities::Type;

    pub const I8: Type = Type::I8;
    pub const I16: Type = Type::I16;
    pub const I32: Type = Type::I32;
    pub const I64: Type = Type::I64;
    pub const U8: Type = Type::U8;
    pub const U16: Type = Type::U16;
    pub const U32: Type = Type::U32;
    pub const U64: Type = Type::U64;
    pub const F32: Type = Type::F32;
    pub const F64: Type = Type::F64;
    pub const BOOL: Type = Type::Bool;
    pub const VOID: Type = Type::Void;
}