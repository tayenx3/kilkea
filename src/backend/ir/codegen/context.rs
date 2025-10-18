use crate::backend::ir::{entities::{BlockID, ValueID}, inst::BlockCall};

use super::{builders::Builder, super::entities::Function};

#[derive(Debug)]
pub struct Context {
    modules: Vec<Module>
}

#[derive(Debug, Clone)]
pub struct Module {
    name: String,
    pub(crate) functions: Vec<Function>
}

impl Context {
    pub fn new() -> Self {
        Self { modules: Vec::new() }
    }

    pub fn create_module(&mut self, name: &str) -> &mut Module {
        self.modules.push(Module {
            name: name.to_string(),
            functions: Vec::new()
        });
        self.modules.last_mut().unwrap()
    }

    pub fn modules(&self) -> Vec<Module> {
        self.modules.clone()
    }
}

impl Module {
    pub fn builder(&mut self) -> Builder {
        Builder::new(self)
    }

    pub fn display(&self) -> String {
        let mut output: String = String::new();
        for function in &self.functions {
            let mut args = String::new();
            let params = function.sig.params.clone();
            for (id, arg_type) in params.iter().enumerate() {
                args.push_str(&format!("{} %{}", arg_type, id));
                if id <= params.len() {
                    args.push_str(", ")
                }
            }
            output.push_str(&format!("func {} : @{}({}) {{\n", function.sig.return_ty, function.alias, args));
            for block in &function.blocks {
                output.push_str(&format!("{}({}):\n", block.id, block.params.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(", ")));
                for inst in &block.insts {
                    output.push_str(&format!("  {}\n", inst))
                }
            }
            output.push_str("}");
        }

        output
    }
}