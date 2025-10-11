//! # Frontend module
//! 
//! * Contains the lexer, parser, and type checker

pub mod lexer;
pub mod parser;
pub mod plugins;
pub mod typechecker;
pub mod typeregistry;

#[allow(unused_imports)]
pub use {
    lexer::*, 
    parser::*, 
    plugins::*, 
    typechecker::*,
    typeregistry::*,
    super::global::*
};