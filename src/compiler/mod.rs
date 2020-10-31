//! # Сompiler
//!
//! Native compilation and builders

use inkwell::targets::FileType;
use inkwell::{
    context::Context,
    memory_buffer::MemoryBuffer,
    module::Module,
    targets::{
        CodeModel,
        InitializationConfig,
        RelocMode,
        Target,
        TargetMachine,
    },
    OptimizationLevel,
};
use std::path::Path;
use std::process::Command;

fn apply_target_to_module(target_machine: &TargetMachine, module: &Module) {
    module.set_triple(&target_machine.get_triple());
    module.set_data_layout(&target_machine.get_target_data().get_data_layout());
}
// TODO: Set configuration options for target mathin optimization
fn get_native_target_machine() -> Result<TargetMachine, String> {
    Target::initialize_native(&InitializationConfig::default())?;
    let target_triple = TargetMachine::get_default_triple();
    let target = Target::from_triple(&target_triple).map_err(|v| v.to_string())?;
    target
        .create_target_machine(
            &target_triple,
            &TargetMachine::get_host_cpu_name().to_string(),
            &TargetMachine::get_host_cpu_features().to_string(),
            OptimizationLevel::Aggressive,
            RelocMode::PIC,
            CodeModel::Medium,
        )
        .ok_or_else(|| String::from("Failed to create target machine"))
}

/// Build executable code
pub fn builder(app_name: String, src: String) -> Result<(), String> {
    let build_dir = "build";
    let context = Context::create();
    let memory_buffer = MemoryBuffer::create_from_memory_range(src.as_bytes(), "amin");
    let module = context
        .create_module_from_ir(memory_buffer)
        .map_err(|v| v.to_string())?;

    let target_machine = get_native_target_machine()?;
    apply_target_to_module(&target_machine, &module);

    if !Path::new("build").is_dir() {
        std::fs::create_dir("build").expect("Can't create `build` directory");
    };

    let obj_file_name = format!("{}/{}.o", build_dir, app_name);
    let a_file_name = format!("{}/lib{}.a", build_dir, app_name);
    let app_file_name = format!("{}/{}", build_dir, app_name);
    let obj_file = Path::new(&obj_file_name);

    target_machine
        .write_to_file(&module, FileType::Object, obj_file)
        .map_err(|v| v.to_string())?;

    Command::new("ar")
        .args(&["crs", &obj_file_name, &a_file_name])
        .spawn()
        .map_err(|_| "Failed to run `ar` command".to_string())?
        .wait()
        .map_err(|_| "Failed to process `ar` command".to_string())?;

    Command::new("ld")
        .args(&[
            "-o",
            &app_file_name,
            "-dynamic-linker",
            "/lib64/ld-linux-x86-64.so.2",
            "/usr/lib/x86_64-linux-gnu/crt1.o",
            "/usr/lib/x86_64-linux-gnu/crti.o",
            "/usr/lib/x86_64-linux-gnu/crtn.o",
            "-lc",
            &a_file_name,
        ])
        .spawn()
        .map_err(|_| "Failed to run `ld` command".to_string())?
        .wait()
        .map_err(|_| "Failed to process `ld` command".to_string())?;
    Ok(std::fs::remove_file(obj_file).or::<String>(Ok(()))?)
}
