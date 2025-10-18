//! # Kese's IR's Optimization Module
//! 
//! ## Introduces:
//! * Constant Folding
//! 

use std::hash::Hash;
use crate::backend::ir::prelude::*;

pub mod constantfolder;

pub use constantfolder::*;

pub trait OptimizationPass {
    fn name(&self) -> String;
    fn apply(&mut self, module: &mut Module);
}

impl std::fmt::Debug for dyn OptimizationPass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OptimizationPass({})", self.name())
    }
}

impl PartialEq for dyn OptimizationPass {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}

impl Hash for dyn OptimizationPass {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name().hash(state)
    }
}

impl Eq for dyn OptimizationPass {}

pub struct Optimizer<'a>  {
    module: &'a mut Module,
    optimizations: Vec<Box<dyn OptimizationPass>>
}

impl<'a> Optimizer<'a> {
    pub fn new(module: &'a mut Module) -> Self {
        Self {
            module,
            optimizations: Vec::new()
        }
    }

    pub fn with_constant_folder(mut self) -> Self {
        self.add_pass(Box::new(ConstantFolder::new()) as Box<dyn OptimizationPass>);
        self
    }
    
    pub fn add_pass(&mut self, pass: Box<dyn OptimizationPass>) -> &mut Self {
        if let None = self.optimizations.iter().find(|p| p.name() == "ConstantFolder") {
            self.optimizations.push(pass);
        }
        self
    }

    pub fn passes(&self) -> impl Iterator<Item = &dyn OptimizationPass> {
        self.optimizations.iter().map(|pass| pass.as_ref())
    }
    
    pub fn run(&mut self) {
        for pass in self.optimizations.iter_mut() {
            pass.apply(self.module)
        }
    }
}