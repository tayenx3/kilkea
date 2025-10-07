use super::kilkwell::context::*;

#[allow(dead_code)]
fn compile_test() {
    let context = Context::new();
    let module = context.create_module("main");
    let builder = module.builder();

    let i32_type = builder.r#type().i32();
    let return_value = builder.value().i32(5);

    let main = module.func()
        .with_name("main")
        .with_return_type(i32_type)
        .build();

    let three = builder.value().i32(3);
    let five = builder.value().i32(5);

    main.add(three, five, "sum");

    main.ret(return_value);

    let result = module.to_ir_string();
    println!("{}", result)
}