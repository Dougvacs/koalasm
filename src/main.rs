mod parser;
use std::collections::HashMap;
use std::path::Path;

use inkwell::{context::Context, values::PointerValue};
use inkwell::targets::{FileType, InitializationConfig, Target, TargetMachine};

use crate::parser::{parse_add, parse_addi, parse_li};

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
    // Initialise registers
    let mut xn = HashMap::<String, PointerValue>::new();
    for i in 0..32 {
        let xi = builder
            .build_alloca(i64_type, &format!("x{}", i))
            .unwrap();
        builder.build_store(xi, i64_type.const_int(0, false))?;
        xn.insert(format!("x{}", i), xi);
    }
    let add = |rd: usize, rs1: usize, rs2: usize| {
        let xrs1_ptr = *xn.get(&format!("x{}", rs1)).unwrap();
        let xrs2_ptr = *xn.get(&format!("x{}", rs2)).unwrap();
        let lhs = builder.build_load(i64_type, xrs1_ptr, &("load".to_string()+&format!("x{}", rs1))).unwrap().into_int_value();
        let rhs = builder.build_load(i64_type, xrs2_ptr, &("load".to_string()+&format!("x{}", rs2))).unwrap().into_int_value();
        let sum = builder.build_int_add(lhs, rhs, "sum").unwrap();
        let _  = builder.build_store(*xn.get(&format!("x{}", rd)).unwrap(), sum);
    };
    let li = |rd: usize, imm: i64| {
        let imm_val = i64_type.const_int(imm as u64, true);
        builder.build_store(*xn.get(&format!("x{}", rd)).unwrap(), imm_val).unwrap();
    };
    let addi = |rd: usize, rs1: usize, imm: i64| {
        let rs1_val = builder.build_load(i64_type, *xn.get(&format!("x{}", rs1)).unwrap(), "").unwrap().into_int_value();
        let imm_val = i64_type.const_int(imm as u64, true);
        let sum = builder.build_int_add(rs1_val, imm_val, "addi").unwrap();
        builder.build_store(*xn.get(&format!("x{}", rd)).unwrap(), sum).unwrap();
    };

    parse_add("add x3, x1, x2", &add);
    parse_add("add x4, x1, x3", &add);
    parse_li("li x1, 5", &li);
    parse_addi("addi x3, x3, 10", &addi);

    builder.build_return(None)?;
    module.print_to_stderr();
    target_machine
        .write_to_file(&module, FileType::Object, Path::new("output.o"))?;
    Ok(())
}
