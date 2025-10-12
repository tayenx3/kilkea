use colored::Colorize;
use cranelift::{prelude::{*, isa::{IsaBuilder, TargetIsa}}, codegen::*};
use cranelift_module::{Linkage, Module as ClifModule};
use cranelift_object::{ObjectBuilder, ObjectModule};
use target_lexicon::Triple;
use std::sync::Arc;
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};
use super::*;
use super::super::global::Type;

use crate::frontend::*;

#[allow(dead_code)]
pub struct ASTCompiler {
    target: Triple,
    isa_builder: IsaBuilder<Result<Arc<dyn TargetIsa>, CodegenError>>,
    debug: bool,
    module: Module,
    src: String,
    path: String
}

impl ASTCompiler {
    pub fn new(debug: bool, module: Module, src: String, path: String) -> Result<Self, String> {
        let target = Triple::host();
        if debug {
            println!("{}", "Constructing compiler...".cyan().bold());
            println!("{} `{}`", "Full triple:".cyan(), target.to_string());
            println!("{} `{:?}`", "Architecture:", target.architecture);
            println!("{} `{:?}`", "Vendor:", target.vendor);
            println!("{} `{:?}`", "OS:", target.operating_system);
            println!("{} `{:?}`", "Environment:", target.environment);
        }

        let isa_builder = cranelift_native::builder()
            .map_err(|e| format!("ISA failed for {}:\n{}", target, e))?;

        Ok(Self {
            target,
            isa_builder,
            debug,
            module,
            src,
            path
        })
    }

    fn create_isa(&self) -> Result<Arc<dyn TargetIsa>, String> {
        let flags = settings::Flags::new(settings::builder());

        self.isa_builder.finish(flags)
            .map_err(|e| format!("Failed to create ISA: {}", e))
    }

    pub fn compile_module(&mut self) -> Result<(), String> {
        if self.debug {
            println!("{}", "Starting compilation...".cyan().bold());
        }

        let isa = self.create_isa()?;
        if self.debug {
            println!("{} {}", "ISA created:".green(), isa.name());
        }

        let obj_builder = ObjectBuilder::new(
            isa,
            &*self.path.strip_suffix(".kl").unwrap_or("main"),
            cranelift_module::default_libcall_names(),
        ).map_err(|e| format!("Failed to create object builder: {}", e))?;

        let mut clif_module = ObjectModule::new(obj_builder);

        if self.debug {
            println!("{}", "Object module created".green());
        }

        let mut ctx = clif_module.make_context();
        
        ctx.func.signature.returns.push(AbiParam::new(I64_TYPE));
        ctx.func.name = ir::UserFuncName::testcase("main");

        let mut builder_ctx = FunctionBuilderContext::new();
        let mut builder = FunctionBuilder::new(&mut ctx.func, &mut builder_ctx);

        let entry_block = builder.create_block();
        builder.append_block_params_for_function_params(entry_block);
        builder.switch_to_block(entry_block);
        builder.seal_block(entry_block);

        let mut i = TypedValue {
            value: builder.ins().iconst(I32_TYPE, 0),
            type_: Type::Int64
        };
        for s in self.module.0.clone() {
            i = self.compile_node(s, &mut builder)?;
        }
        builder.ins().return_(&[i.value]);
        
        builder.finalize();

        // Add to module
        let func_id = clif_module.declare_function("main", Linkage::Export, &ctx.func.signature)
            .map_err(|e| format!("Failed to declare function: {}", e))?;
        
        clif_module.define_function(func_id, &mut ctx)
            .map_err(|e| format!("Failed to define function: {}", e))?;

        if self.debug {
            println!("{}\n{}", "AST Compilation finished:".cyan().bold(), ctx.func.to_string())
        }

        self.create_object_file(clif_module)?;

        Ok(())
    }

    fn compile_node(&mut self, node: Node, builder: &mut FunctionBuilder) -> Result<TypedValue, String> {
        if self.debug {
            println!("{} {:#?}", "Compiling node:".cyan(), node)
        }
        match node.ast_repr {
            ASTNode::IntLit(i) => Ok(TypedValue {
                value: builder.ins().iconst(I64_TYPE, i),
                type_: Type::Int64
            }),
            ASTNode::FloatLit(i) => Ok(TypedValue {
                value: builder.ins().f64const(i),
                type_: Type::Float64
            }),
            ASTNode::Bool(b) => Ok(TypedValue {
                value: builder.ins().iconst(I8_TYPE, if b { 1 } else { 0 }),
                type_: Type::Boolean
            }),
            ASTNode::StringLit(_) => todo!("String literals"),
            ASTNode::BinOp {
                op, lhs, rhs
            } => {
                let left = self.compile_node(*lhs, builder)?;
                let right = self.compile_node(*rhs, builder)?;
                match &*op.0 {
                    "+" => match (left.type_, right.type_) {
                        (Type::Int8, Type::Int8) => Ok(TypedValue {
                            value: builder.ins().iadd(left.value, right.value),
                            type_: Type::Int8
                        }),
                        (Type::Int16, Type::Int16) => Ok(TypedValue {
                            value: builder.ins().iadd(left.value, right.value),
                            type_: Type::Int16
                        }),
                        (Type::Int32, Type::Int32) => Ok(TypedValue {
                            value: builder.ins().iadd(left.value, right.value),
                            type_: Type::Int32
                        }),
                        (Type::Int64, Type::Int64) => Ok(TypedValue {
                            value: builder.ins().iadd(left.value, right.value),
                            type_: Type::Int64
                        }),
                        (Type::Float32, Type::Float32) => Ok(TypedValue {
                            value: builder.ins().fadd(left.value, right.value),
                            type_: Type::Float32
                        }),
                        (Type::Float64, Type::Float64) => Ok(TypedValue {
                            value: builder.ins().fadd(left.value, right.value),
                            type_: Type::Float64
                        }),
                        (Type::UInt8, Type::UInt8) => Ok(TypedValue {
                            value: builder.ins().iadd(left.value, right.value),
                            type_: Type::UInt8
                        }),
                        (Type::UInt16, Type::UInt16) => Ok(TypedValue {
                            value: builder.ins().iadd(left.value, right.value),
                            type_: Type::UInt16
                        }),
                        (Type::UInt32, Type::UInt32) => Ok(TypedValue {
                            value: builder.ins().iadd(left.value, right.value),
                            type_: Type::UInt32
                        }),
                        (Type::UInt64, Type::UInt64) => Ok(TypedValue {
                            value: builder.ins().iadd(left.value, right.value),
                            type_: Type::UInt64
                        }),
                        _ => unreachable!()
                    },
                    "-" => match (left.type_, right.type_) {
                        (Type::Int8, Type::Int8) => Ok(TypedValue {
                            value: builder.ins().isub(left.value, right.value),
                            type_: Type::Int8
                        }),
                        (Type::Int16, Type::Int16) => Ok(TypedValue {
                            value: builder.ins().isub(left.value, right.value),
                            type_: Type::Int16
                        }),
                        (Type::Int32, Type::Int32) => Ok(TypedValue {
                            value: builder.ins().isub(left.value, right.value),
                            type_: Type::Int32
                        }),
                        (Type::Int64, Type::Int64) => Ok(TypedValue {
                            value: builder.ins().isub(left.value, right.value),
                            type_: Type::Int64
                        }),
                        (Type::Float32, Type::Float32) => Ok(TypedValue {
                            value: builder.ins().fsub(left.value, right.value),
                            type_: Type::Float32
                        }),
                        (Type::Float64, Type::Float64) => Ok(TypedValue {
                            value: builder.ins().fsub(left.value, right.value),
                            type_: Type::Float64
                        }),
                        (Type::UInt8, Type::UInt8) => Ok(TypedValue {
                            value: builder.ins().isub(left.value, right.value),
                            type_: Type::UInt8
                        }),
                        (Type::UInt16, Type::UInt16) => Ok(TypedValue {
                            value: builder.ins().isub(left.value, right.value),
                            type_: Type::UInt16
                        }),
                        (Type::UInt32, Type::UInt32) => Ok(TypedValue {
                            value: builder.ins().isub(left.value, right.value),
                            type_: Type::UInt32
                        }),
                        (Type::UInt64, Type::UInt64) => Ok(TypedValue {
                            value: builder.ins().isub(left.value, right.value),
                            type_: Type::UInt64
                        }),
                        _ => unreachable!()
                    },
                    "*" => match (left.type_, right.type_) {
                        (Type::Int8, Type::Int8) => Ok(TypedValue {
                            value: builder.ins().imul(left.value, right.value),
                            type_: Type::Int8
                        }),
                        (Type::Int16, Type::Int16) => Ok(TypedValue {
                            value: builder.ins().imul(left.value, right.value),
                            type_: Type::Int16
                        }),
                        (Type::Int32, Type::Int32) => Ok(TypedValue {
                            value: builder.ins().imul(left.value, right.value),
                            type_: Type::Int32
                        }),
                        (Type::Int64, Type::Int64) => Ok(TypedValue {
                            value: builder.ins().imul(left.value, right.value),
                            type_: Type::Int64
                        }),
                        (Type::Float32, Type::Float32) => Ok(TypedValue {
                            value: builder.ins().fmul(left.value, right.value),
                            type_: Type::Float32
                        }),
                        (Type::Float64, Type::Float64) => Ok(TypedValue {
                            value: builder.ins().fmul(left.value, right.value),
                            type_: Type::Float64
                        }),
                        (Type::UInt8, Type::UInt8) => Ok(TypedValue {
                            value: builder.ins().imul(left.value, right.value),
                            type_: Type::UInt8
                        }),
                        (Type::UInt16, Type::UInt16) => Ok(TypedValue {
                            value: builder.ins().imul(left.value, right.value),
                            type_: Type::UInt16
                        }),
                        (Type::UInt32, Type::UInt32) => Ok(TypedValue {
                            value: builder.ins().imul(left.value, right.value),
                            type_: Type::UInt32
                        }),
                        (Type::UInt64, Type::UInt64) => Ok(TypedValue {
                            value: builder.ins().imul(left.value, right.value),
                            type_: Type::UInt64
                        }),
                        _ => unreachable!()
                    },
                    "/" => match (left.type_, right.type_) {
                        (Type::Int8, Type::Int8) => Ok(TypedValue {
                            value: builder.ins().sdiv(left.value, right.value),
                            type_: Type::Int8
                        }),
                        (Type::Int16, Type::Int16) => Ok(TypedValue {
                            value: builder.ins().sdiv(left.value, right.value),
                            type_: Type::Int16
                        }),
                        (Type::Int32, Type::Int32) => Ok(TypedValue {
                            value: builder.ins().sdiv(left.value, right.value),
                            type_: Type::Int32
                        }),
                        (Type::Int64, Type::Int64) => Ok(TypedValue {
                            value: builder.ins().sdiv(left.value, right.value),
                            type_: Type::Int64
                        }),
                        (Type::Float32, Type::Float32) => Ok(TypedValue {
                            value: builder.ins().fdiv(left.value, right.value),
                            type_: Type::Float32
                        }),
                        (Type::Float64, Type::Float64) => Ok(TypedValue {
                            value: builder.ins().fdiv(left.value, right.value),
                            type_: Type::Float64
                        }),
                        (Type::UInt8, Type::UInt8) => Ok(TypedValue {
                            value: builder.ins().sdiv(left.value, right.value),
                            type_: Type::UInt8
                        }),
                        (Type::UInt16, Type::UInt16) => Ok(TypedValue {
                            value: builder.ins().sdiv(left.value, right.value),
                            type_: Type::UInt16
                        }),
                        (Type::UInt32, Type::UInt32) => Ok(TypedValue {
                            value: builder.ins().sdiv(left.value, right.value),
                            type_: Type::UInt32
                        }),
                        (Type::UInt64, Type::UInt64) => Ok(TypedValue {
                            value: builder.ins().sdiv(left.value, right.value),
                            type_: Type::UInt64
                        }),
                        _ => unreachable!()
                    },
                    _ => todo!("{}", op.0)
                }
            },
            _ => todo!("{:#?}", node.ast_repr)
        }
    }

    fn create_object_file(&self, module: ObjectModule) -> Result<(), String> {
        if self.debug {
            println!("{}", "Generating object file...".cyan());
        }

        let object_product = module.finish();

        let object_data = object_product.emit()
            .map_err(|e| e.to_string())?;

        std::fs::write("output.o", &object_data)
            .map_err(|e| format!("Failed to write object file: {}", e))?;

        if self.debug {
            println!("{}", "Object file written: output.o".green());
            println!("{} {}", "Object size:".green(), object_data.len());
        }

        Ok(())
    }
}

pub struct TypedValue {
    value: Value,
    type_: Type
}