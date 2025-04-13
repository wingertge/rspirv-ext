// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

use rspirv::{dr, spirv};
pub trait DebugPrintFOpBuilder {
    #[doc = "Appends an DebugPrintf instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn debug_printf(
        &mut self,
        format: spirv::Word,
        id_ref: impl IntoIterator<Item = spirv::Word>,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an DebugPrintf instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn debug_printf_id(
        &mut self,
        result_id: Option<spirv::Word>,
        format: spirv::Word,
        id_ref: impl IntoIterator<Item = spirv::Word>,
    ) -> Result<spirv::Word, dr::Error>;
}
impl DebugPrintFOpBuilder for rspirv::dr::Builder {
    #[allow(clippy::too_many_arguments)]
    fn debug_printf(
        &mut self,
        format: spirv::Word,
        id_ref: impl IntoIterator<Item = spirv::Word>,
    ) -> Result<spirv::Word, dr::Error> {
        self.debug_printf_id(None, format, id_ref)
    }
    #[allow(clippy::too_many_arguments)]
    fn debug_printf_id(
        &mut self,
        result_id: Option<spirv::Word>,
        format: spirv::Word,
        id_ref: impl IntoIterator<Item = spirv::Word>,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "NonSemantic.DebugPrintF");
        let result_type = self.type_void();
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(format)];
        args.extend(id_ref.into_iter().map(dr::Operand::IdRef));
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::DebugPrintFOp::DebugPrintf as spirv::Word,
            args,
        )
    }
}
