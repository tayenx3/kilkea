//! # Contains the tests for Kese's IR generation
//! 
//! * Mostly optimization tests and functionality tests
//! 

#![allow(unused)]

#[cfg(test)]
mod tests {
    use super::super::ir::prelude::*;

    // -- Non Optimized Tests --

    /// ## Add function
    /// 
    /// * Adds 5 + 6
    #[test]
    fn add() {
        let mut context = Context::new();
        let module = context.create_module("main");
        let mut builder = module.builder();

        let sig = FunctionSignature::new()
            .with_return_ty(types::I32);

        let mut function_builder = builder.create_function("add", sig);
        let block = function_builder.create_block();

        let five = block.ins().i32const(5);
        let six = block.ins().i32const(6);
        let result = block.ins().iadd(five, six);

        block.ins().ret(result);

        function_builder.eat_block(block);
        let function = function_builder.build();

        builder.eat_function(function);

        builder.build();
        eprintln!("{}", module.display());
    }

    /// ## Control flow function
    /// 
    /// * Returns 10 if true, 5 if false
    #[test]
    fn control_flow() {
        let mut context = Context::new();
        let module = context.create_module("control_flow");
        let mut builder = module.builder();

        let sig = FunctionSignature::new()
            .with_return_ty(types::I32);

        let mut function_builder = builder.create_function("control_flow", sig);
        
        let entry_block = function_builder.create_block();

        let true_block = function_builder.create_block();
        let false_block = function_builder.create_block();

        let return_block = function_builder
            .create_block()
            .with_param(types::I32);

        let first_condition = entry_block.ins().bool_(true); // Automatically changed into 1
        entry_block.ins().br(
            first_condition, 
            true_block.call(&[]),
            false_block.call(&[]),
        );

        let first_path = true_block.ins().i32const(10);
        true_block.ins().jmp(return_block.call(&[first_path]));

        let second_path = false_block.ins().i32const(5);
        false_block.ins().jmp(return_block.call(&[second_path]));

        return_block.ins().ret(return_block.get_param(0));
        
        function_builder.eat_block(entry_block);
        function_builder.eat_block(true_block);
        function_builder.eat_block(false_block);
        function_builder.eat_block(return_block);

        builder.eat_function(function_builder.build());

        builder.build();
        eprintln!("{}", module.display());
    }

    // -- Optimization Tests --
    /// ## Constant Folding Test
    /// 
    /// * Evaluate `(5 * 6 - 3 * 3) * 2` and fold into:
    /// ```plaintext
    /// %0 = 42
    /// ret %0
    /// ```
    #[test]
    fn constant_folding() {
        let mut context = Context::new();
        let module = context.create_module("const_fold");
        {
            let mut builder = module.builder();

            let sig = FunctionSignature::new();

            let mut function_builder = builder.create_function("const_folding", sig);
            
            let entry_block = function_builder.create_block();

            let five = entry_block.ins().i32const(5);
            let six = entry_block.ins().i32const(6);
            let r1 = entry_block.ins().imul(five, six);

            let three = entry_block.ins().i32const(3);
            let r2 = entry_block.ins().imul(three, three);

            let r3 = entry_block.ins().isub(r1, r2);

            let two = entry_block.ins().i32const(2);
            let final_result = entry_block.ins().imul(r3, two);

            entry_block.ins().ret(final_result);
            
            function_builder.eat_block(entry_block);

            builder.eat_function(function_builder.build());

            builder.build();
            eprintln!("Unoptimized:\n{}", module.display());
        }

        let mut optimizer = Optimizer::new(module)
            .with_constant_folder();

        optimizer.run();

        eprintln!("Optimized:\n{}", module.display());
    }
}