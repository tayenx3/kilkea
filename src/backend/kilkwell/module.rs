use llvm_sys::prelude::*;
use llvm_sys::core::*;
use std::ffi::CString;

use super::context::Context;

#[derive(Debug, Clone)]
pub struct TypeBuilder;
#[allow(dead_code)]
impl TypeBuilder {
    pub fn i32(&self) -> LLVMTypeRef { unsafe { LLVMInt32Type() } }
    pub fn i64(&self) -> LLVMTypeRef { unsafe { LLVMInt64Type() } }
}

#[derive(Debug, Clone)]
pub struct ValueBuilder;
#[allow(dead_code)]
impl ValueBuilder {
    pub fn i32(&self, i: i32) -> LLVMValueRef { 
        unsafe { LLVMConstInt(LLVMInt32Type(), i.try_into().unwrap(), 0) }
    }
    pub fn i64(&self, i: i64) -> LLVMValueRef { 
        unsafe { LLVMConstInt(LLVMInt64Type(), i.try_into().unwrap(), 0) }
    }
}

#[derive(Debug, Clone)]
pub struct BuilderBuf {
    type_builder: TypeBuilder,
    value_builder: ValueBuilder
}

#[allow(dead_code)]
impl BuilderBuf {
    pub fn r#type(&self) -> TypeBuilder {
        self.type_builder.clone()
    }

    pub fn value(&self) -> ValueBuilder {
        self.value_builder.clone()
    }
}

#[derive(Debug, Clone)]
pub struct Module<'a> {
    context: &'a Context,
    pub module: LLVMModuleRef,
}

#[allow(dead_code)]
impl<'a> Module<'a> {
    pub fn new(context: &'a Context, name: &str) -> Self {
        unsafe {
            let name_c = CString::new(name).unwrap();
            let module = LLVMModuleCreateWithNameInContext(name_c.as_ptr(), context.context);
            
            Self { context, module }
        }
    }

    pub fn to_ir_string(&self) -> String {
        unsafe {
            let ir_cstring = LLVMPrintModuleToString(self.module);
            let ir_string = std::ffi::CStr::from_ptr(ir_cstring)
                .to_str()
                .unwrap()
                .to_string();
            LLVMDisposeMessage(ir_cstring);  // Don't forget this!
            ir_string
        }
    }

    pub fn func(&self) -> FuncBuilder {
        FuncBuilder::new(self)
    }
    
    pub fn builder(&self) -> BuilderBuf {
        BuilderBuf {
            type_builder: TypeBuilder,
            value_builder: ValueBuilder
        }
    }
}

impl<'a> Drop for Module<'a> {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeModule(self.module);
        }
    }
}

#[derive(Debug, Clone)]
pub struct FuncBuilder<'a> {
    module: &'a Module<'a>,
    name: Option<String>,
    return_type: Option<LLVMTypeRef>,
    builder: LLVMBuilderRef,  // NEW: Need a builder for instructions
}

#[allow(dead_code)]
impl<'a> FuncBuilder<'a> {
    fn new(module: &'a Module) -> Self {
        unsafe {
            Self {
                module,
                name: None,
                return_type: None,
                builder: LLVMCreateBuilderInContext(module.context.context),
            }
        }
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn with_return_type(mut self, return_type: LLVMTypeRef) -> Self {
        self.return_type = Some(return_type);
        self
    }

    pub fn build(self) -> Function<'a> {  // Changed to return Function
        unsafe {
            let return_type = self.return_type.unwrap_or_else(|| {
                LLVMVoidTypeInContext(self.module.context.context)
            });
            
            let function_type = LLVMFunctionType(
                return_type,
                std::ptr::null_mut(),
                0,
                0
            );
            
            let name = self.name.unwrap_or_else(|| "anonymous".to_string());
            let name_c = CString::new(name).unwrap();
            
            let function = LLVMAddFunction(self.module.module, name_c.as_ptr(), function_type);
            
            let entry_name = CString::new("entry").unwrap();
            let entry_block = LLVMAppendBasicBlockInContext(
                self.module.context.context, 
                function, 
                entry_name.as_ptr()
            );
            LLVMPositionBuilderAtEnd(self.builder, entry_block);
            
            Function {
                function,
                builder: self.builder,
                _marker: std::marker::PhantomData,
            }
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Function<'a> {
    function: LLVMValueRef,
    builder: LLVMBuilderRef,
    _marker: std::marker::PhantomData<&'a ()>,
}

#[allow(dead_code)]
impl<'a> Function<'a> {
    pub fn ret(&self, value: LLVMValueRef) {
        unsafe {
            LLVMBuildRet(self.builder, value);
        }
    }
    
    pub fn add(&self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> LLVMValueRef {
        unsafe {
            let name_c = CString::new(name).unwrap();
            LLVMBuildAdd(self.builder, lhs, rhs, name_c.as_ptr())
        }
    }

    pub fn sub(&self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> LLVMValueRef {
        unsafe {
            let name_c = CString::new(name).unwrap();
            LLVMBuildSub(self.builder, lhs, rhs, name_c.as_ptr())
        }
    }

    pub fn mul(&self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> LLVMValueRef {
        unsafe {
            let name_c = CString::new(name).unwrap();
            LLVMBuildMul(self.builder, lhs, rhs, name_c.as_ptr())
        }
    }

    pub fn div(&self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> LLVMValueRef {
        unsafe {
            let name_c = CString::new(name).unwrap();
            LLVMBuildSDiv(self.builder, lhs, rhs, name_c.as_ptr())
        }
    }
}

impl<'a> Drop for Function<'a> {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeBuilder(self.builder);
        }
    }
}