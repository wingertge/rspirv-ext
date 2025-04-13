use rspirv::{
    dr::{Builder, Operand},
    spirv::Word,
};

pub mod autogen_glsl_std_450;
pub mod autogen_nonsemantic_debugprintf;
pub mod autogen_nonsemantic_shader_debuginfo_100;
pub mod autogen_opencl_std_100;

/// Appends an OpExtInstImport instruction and returns the result id.
pub(crate) fn ext_inst_import(b: &mut Builder, extended_inst_set: impl Into<String>) -> Word {
    let extended_inst_set: String = extended_inst_set.into();
    let operands = vec![Operand::LiteralString(extended_inst_set.clone())];
    let existing = b
        .module_ref()
        .ext_inst_imports
        .iter()
        .find(|inst| inst.operands == operands);
    match existing {
        Some(inst) => inst.result_id.unwrap(),
        None => b.ext_inst_import(extended_inst_set),
    }
}
