use llvm_sys::prelude::*;
use llvm_sys::core::*;
use super::*;

#[derive(Debug, Clone)]
pub struct Context {
    pub context: LLVMContextRef,
}

#[allow(dead_code)]
impl Context {
    pub fn new() -> Self {
        unsafe {
            Self {
                context: LLVMContextCreate()
            }
        }
    }

    pub fn create_module(&self, name: &str) -> Module {
        Module::new(self, name)
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            LLVMContextDispose(self.context);
        }
    }
}