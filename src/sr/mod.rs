use rspirv::{
    dr::{self, Builder},
    spirv::{self, Word},
};

pub mod nonsemantic_debugprintf;
pub mod nonsemantic_shader_debuginfo_100;

pub(crate) fn const_u32(b: &mut Builder, value: u32) -> Word {
    let ty = b.type_int(32, 0);

    let inst = dr::Instruction::new(
        spirv::Op::Constant,
        Some(ty),
        None,
        vec![dr::Operand::LiteralBit32(value)],
    );

    match b.dedup_insert_type(&inst) {
        Some(id) => id,
        None => b.constant_bit32(ty, value),
    }
}

// Add deduplicated string op
pub(crate) fn debug_string(b: &mut Builder, s: impl Into<String>) -> Word {
    let mut new_inst = dr::Instruction::new(
        spirv::Op::String,
        None,
        None,
        vec![dr::Operand::LiteralString(s.into())],
    );
    let existing = b
        .module_ref()
        .debug_string_source
        .iter()
        .find(|inst| inst.is_type_identical(&new_inst));
    match existing {
        Some(inst) => inst.result_id.unwrap(),
        None => {
            let id = b.id();
            new_inst.result_id = Some(id);
            b.module_mut().debug_string_source.push(new_inst);
            id
        }
    }
}
