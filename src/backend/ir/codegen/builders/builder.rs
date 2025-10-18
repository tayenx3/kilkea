use crate::backend::ir::prelude::{Module, FunctionSignature};

use super::super::super::entities::Function;
use super::*;
use std::{rc::Rc, cell::RefCell};

#[derive(Debug)]
pub struct Builder<'a> {
    pub(crate) module: &'a mut Module,
    pub(crate) functions: Vec<Function>
}

impl<'a> Builder<'a> {
    pub(crate) fn new(module: &'a mut Module) -> Self {
        Self {
            module,
            functions: Vec::new(),
        }
    }

    pub fn create_function(&self, name: &str, sig: impl Into<FunctionSignature> + Clone) -> FunctionBuilder {
        FunctionBuilder {
            function: Function {
                alias: name.to_string(),
                blocks: Vec::new(),
                sig: sig.clone().into()
            },
            next_id: Rc::from(RefCell::new(sig.into().params.len())),
            next_block_id: 0usize,
        }
    }

    pub fn eat_function(&mut self, function: Function) {            
        self.functions.push(function);
    }

    pub fn build(self) {
        self.module.functions.extend(self.functions)
    }
}