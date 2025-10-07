//! Kilkwell - Kilkea's Safe LLVM Backend
//! Fearless code generation for fearless concurrency!

pub mod context;
pub mod module;

#[allow(unused_imports)]
pub use {
    context::*,
    module::*
};