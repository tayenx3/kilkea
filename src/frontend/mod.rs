//! # Frontend module
//! 
//! * Contains the lexer, parser, and type checker

pub mod lexer;
pub mod parser;

#[allow(unused_imports)]
pub use {
    lexer::*, 
    parser::*, 
    super::global::*
};