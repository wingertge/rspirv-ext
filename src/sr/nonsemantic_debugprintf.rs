use rspirv::{dr, spirv};

use crate::dr::autogen_nonsemantic_debugprintf::DebugPrintFOpBuilder;

pub trait DebugPrintfBuilder {
    #[doc = "Appends an DebugPrintf instruction and returns the result id, or return the existing id if the instruction was already present."]
    fn debug_printf(
        &mut self,
        format: impl Into<String>,
        args: impl IntoIterator<Item = spirv::Word>,
    ) -> Result<(), dr::Error>;
}
impl DebugPrintfBuilder for rspirv::dr::Builder {
    #[allow(clippy::too_many_arguments)]
    fn debug_printf(
        &mut self,
        format: impl Into<String>,
        id_ref: impl IntoIterator<Item = spirv::Word>,
    ) -> Result<(), dr::Error> {
        let format = self.string(format);

        DebugPrintFOpBuilder::debug_printf(self, format, id_ref).map(|_| ())
    }
}
