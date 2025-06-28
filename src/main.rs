use std::collections::HashMap;
use std::path::Path;

use inkwell::{context::Context, values::PointerValue};
use inkwell::targets::{FileType, InitializationConfig, Target, TargetMachine};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Target::initialize_all(&InitializationConfig::default());
    let target_triple = TargetMachine::get_default_triple();
    let target_machine = Target::from_triple(&target_triple)?.create_target_machine(
        &target_triple, 
        "x86-64", 
        "", 
        inkwell::OptimizationLevel::Default, 
        inkwell::targets::RelocMode::Default, 
        inkwell::targets::CodeModel::Default).unwrap();

    let context = Context::create();
    let module = context.create_module("rv64gc");
    let builder = context.create_builder();

    // Create function: i64 fn()
    let i64_type = context.i64_type();
    let fn_type = i64_type.fn_type(&[], false);
    let function = module.add_function("main", fn_type, None);
    let entry = context.append_basic_block(function, "entry");
    builder.position_at_end(entry);

    let mut xn = HashMap::<String, PointerValue>::new();
    for i in 0..32 {
        let xi = builder
            .build_alloca(i64_type, &format!("x{}", i))
            .unwrap();
        builder.build_store(xi, i64_type.const_int(0, false))?;
        xn.insert(format!("x{}", i), xi);
    }

    // Load values back and add them
    let loaded1 = builder.build_load(i64_type, *xn.get("x1").unwrap(), "load_x1").unwrap().into_int_value();
    let loaded2 = builder.build_load(i64_type, *xn.get("x2").unwrap(), "load_x2").unwrap().into_int_value();
    let sum = builder.build_int_add(loaded1, loaded2, "sum").unwrap();

    builder.build_return(Some(&sum))?;

    module.print_to_stderr();
    target_machine
        .write_to_file(&module, FileType::Object, Path::new("output.o"))?;
    Ok(())
}
