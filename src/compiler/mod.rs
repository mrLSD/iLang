//! # Сompiler
//!
//! Native compilation and builders

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
pub fn builder(src: String) -> Result<(), String> {
    let context = Context::create();
    let memory_buffer = MemoryBuffer::create_from_memory_range(src.as_bytes(), "amin");
    let module = context
        .create_module_from_ir(memory_buffer)
        .map_err(|v| v.to_string())?;

    let target_machine = get_native_target_machine()?;
    apply_target_to_module(&target_machine, &module);
    Ok(())
}
