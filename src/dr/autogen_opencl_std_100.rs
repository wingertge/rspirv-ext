// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

use rspirv::{dr, spirv};
pub trait CLOpBuilder {
    #[doc = "Appends an acos instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn acos(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an acos instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn acos_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an acosh instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn acosh(&mut self, result_type: spirv::Word, x: spirv::Word)
        -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an acosh instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn acosh_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an acospi instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn acospi(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an acospi instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn acospi_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an asin instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn asin(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an asin instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn asin_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an asinh instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn asinh(&mut self, result_type: spirv::Word, x: spirv::Word)
        -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an asinh instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn asinh_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an asinpi instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn asinpi(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an asinpi instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn asinpi_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an atan instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn atan(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an atan instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn atan_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an atan2 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn atan2(
        &mut self,
        result_type: spirv::Word,
        y: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an atan2 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn atan2_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        y: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an atanh instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn atanh(&mut self, result_type: spirv::Word, x: spirv::Word)
        -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an atanh instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn atanh_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an atanpi instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn atanpi(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an atanpi instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn atanpi_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an atan2pi instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn atan2pi(
        &mut self,
        result_type: spirv::Word,
        y: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an atan2pi instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn atan2pi_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        y: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an cbrt instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn cbrt(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an cbrt instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn cbrt_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an ceil instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn ceil(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an ceil instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn ceil_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an copysign instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn copysign(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an copysign instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn copysign_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an cos instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn cos(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an cos instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn cos_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an cosh instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn cosh(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an cosh instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn cosh_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an cospi instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn cospi(&mut self, result_type: spirv::Word, x: spirv::Word)
        -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an cospi instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn cospi_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an erfc instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn erfc(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an erfc instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn erfc_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an erf instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn erf(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an erf instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn erf_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an exp instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn exp(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an exp instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn exp_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an exp2 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn exp2(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an exp2 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn exp2_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an exp10 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn exp10(&mut self, result_type: spirv::Word, x: spirv::Word)
        -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an exp10 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn exp10_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an expm1 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn expm1(&mut self, result_type: spirv::Word, x: spirv::Word)
        -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an expm1 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn expm1_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an fabs instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn fabs(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an fabs instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn fabs_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an fdim instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn fdim(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an fdim instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn fdim_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an floor instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn floor(&mut self, result_type: spirv::Word, x: spirv::Word)
        -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an floor instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn floor_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an fma instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn fma(
        &mut self,
        result_type: spirv::Word,
        a: spirv::Word,
        b: spirv::Word,
        c: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an fma instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn fma_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        a: spirv::Word,
        b: spirv::Word,
        c: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an fmax instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn fmax(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an fmax instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn fmax_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an fmin instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn fmin(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an fmin instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn fmin_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an fmod instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn fmod(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an fmod instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn fmod_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an fract instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn fract(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        ptr: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an fract instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn fract_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        ptr: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an frexp instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn frexp(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        exp: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an frexp instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn frexp_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        exp: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an hypot instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn hypot(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an hypot instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn hypot_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an ilogb instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn ilogb(&mut self, result_type: spirv::Word, x: spirv::Word)
        -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an ilogb instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn ilogb_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an ldexp instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn ldexp(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        k: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an ldexp instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn ldexp_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        k: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an lgamma instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn lgamma(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an lgamma instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn lgamma_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an lgamma_r instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn lgamma_r(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        signp: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an lgamma_r instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn lgamma_r_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        signp: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an log instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn log(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an log instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn log_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an log2 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn log2(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an log2 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn log2_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an log10 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn log10(&mut self, result_type: spirv::Word, x: spirv::Word)
        -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an log10 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn log10_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an log1p instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn log1p(&mut self, result_type: spirv::Word, x: spirv::Word)
        -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an log1p instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn log1p_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an logb instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn logb(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an logb instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn logb_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an mad instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn mad(
        &mut self,
        result_type: spirv::Word,
        a: spirv::Word,
        b: spirv::Word,
        c: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an mad instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn mad_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        a: spirv::Word,
        b: spirv::Word,
        c: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an maxmag instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn maxmag(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an maxmag instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn maxmag_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an minmag instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn minmag(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an minmag instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn minmag_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an modf instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn modf(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        iptr: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an modf instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn modf_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        iptr: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an nan instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn nan(
        &mut self,
        result_type: spirv::Word,
        nancode: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an nan instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn nan_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        nancode: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an nextafter instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn nextafter(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an nextafter instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn nextafter_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an pow instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn pow(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an pow instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn pow_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an pown instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn pown(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an pown instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn pown_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an powr instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn powr(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an powr instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn powr_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an remainder instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn remainder(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an remainder instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn remainder_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an remquo instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn remquo(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
        quo: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an remquo instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn remquo_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
        quo: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an rint instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn rint(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an rint instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn rint_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an rootn instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn rootn(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an rootn instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn rootn_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an round instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn round(&mut self, result_type: spirv::Word, x: spirv::Word)
        -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an round instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn round_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an rsqrt instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn rsqrt(&mut self, result_type: spirv::Word, x: spirv::Word)
        -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an rsqrt instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn rsqrt_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an sin instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn sin(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an sin instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn sin_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an sincos instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn sincos(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        cosval: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an sincos instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn sincos_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        cosval: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an sinh instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn sinh(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an sinh instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn sinh_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an sinpi instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn sinpi(&mut self, result_type: spirv::Word, x: spirv::Word)
        -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an sinpi instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn sinpi_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an sqrt instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn sqrt(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an sqrt instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn sqrt_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an tan instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn tan(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an tan instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn tan_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an tanh instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn tanh(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an tanh instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn tanh_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an tanpi instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn tanpi(&mut self, result_type: spirv::Word, x: spirv::Word)
        -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an tanpi instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn tanpi_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an tgamma instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn tgamma(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an tgamma instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn tgamma_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an trunc instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn trunc(&mut self, result_type: spirv::Word, x: spirv::Word)
        -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an trunc instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn trunc_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an half_cos instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn half_cos(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an half_cos instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn half_cos_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an half_divide instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn half_divide(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an half_divide instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn half_divide_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an half_exp instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn half_exp(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an half_exp instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn half_exp_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an half_exp2 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn half_exp2(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an half_exp2 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn half_exp2_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an half_exp10 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn half_exp10(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an half_exp10 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn half_exp10_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an half_log instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn half_log(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an half_log instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn half_log_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an half_log2 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn half_log2(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an half_log2 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn half_log2_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an half_log10 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn half_log10(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an half_log10 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn half_log10_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an half_powr instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn half_powr(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an half_powr instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn half_powr_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an half_recip instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn half_recip(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an half_recip instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn half_recip_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an half_rsqrt instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn half_rsqrt(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an half_rsqrt instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn half_rsqrt_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an half_sin instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn half_sin(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an half_sin instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn half_sin_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an half_sqrt instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn half_sqrt(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an half_sqrt instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn half_sqrt_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an half_tan instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn half_tan(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an half_tan instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn half_tan_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an native_cos instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn native_cos(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an native_cos instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn native_cos_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an native_divide instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn native_divide(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an native_divide instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn native_divide_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an native_exp instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn native_exp(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an native_exp instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn native_exp_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an native_exp2 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn native_exp2(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an native_exp2 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn native_exp2_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an native_exp10 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn native_exp10(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an native_exp10 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn native_exp10_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an native_log instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn native_log(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an native_log instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn native_log_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an native_log2 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn native_log2(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an native_log2 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn native_log2_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an native_log10 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn native_log10(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an native_log10 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn native_log10_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an native_powr instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn native_powr(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an native_powr instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn native_powr_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an native_recip instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn native_recip(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an native_recip instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn native_recip_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an native_rsqrt instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn native_rsqrt(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an native_rsqrt instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn native_rsqrt_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an native_sin instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn native_sin(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an native_sin instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn native_sin_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an native_sqrt instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn native_sqrt(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an native_sqrt instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn native_sqrt_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an native_tan instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn native_tan(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an native_tan instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn native_tan_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an s_abs instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn s_abs(&mut self, result_type: spirv::Word, x: spirv::Word)
        -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an s_abs instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn s_abs_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an s_abs_diff instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn s_abs_diff(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an s_abs_diff instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn s_abs_diff_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an s_add_sat instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn s_add_sat(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an s_add_sat instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn s_add_sat_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an u_add_sat instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn u_add_sat(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an u_add_sat instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn u_add_sat_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an s_hadd instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn s_hadd(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an s_hadd instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn s_hadd_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an u_hadd instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn u_hadd(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an u_hadd instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn u_hadd_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an s_rhadd instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn s_rhadd(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an s_rhadd instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn s_rhadd_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an u_rhadd instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn u_rhadd(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an u_rhadd instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn u_rhadd_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an s_clamp instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn s_clamp(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        minval: spirv::Word,
        maxval: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an s_clamp instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn s_clamp_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        minval: spirv::Word,
        maxval: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an u_clamp instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn u_clamp(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        minval: spirv::Word,
        maxval: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an u_clamp instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn u_clamp_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        minval: spirv::Word,
        maxval: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an clz instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn clz(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an clz instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn clz_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an ctz instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn ctz(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an ctz instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn ctz_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an s_mad_hi instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn s_mad_hi(
        &mut self,
        result_type: spirv::Word,
        a: spirv::Word,
        b: spirv::Word,
        c: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an s_mad_hi instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn s_mad_hi_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        a: spirv::Word,
        b: spirv::Word,
        c: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an u_mad_sat instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn u_mad_sat(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
        z: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an u_mad_sat instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn u_mad_sat_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
        z: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an s_mad_sat instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn s_mad_sat(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
        z: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an s_mad_sat instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn s_mad_sat_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
        z: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an s_max instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn s_max(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an s_max instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn s_max_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an u_max instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn u_max(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an u_max instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn u_max_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an s_min instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn s_min(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an s_min instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn s_min_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an u_min instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn u_min(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an u_min instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn u_min_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an s_mul_hi instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn s_mul_hi(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an s_mul_hi instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn s_mul_hi_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an rotate instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn rotate(
        &mut self,
        result_type: spirv::Word,
        v: spirv::Word,
        i: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an rotate instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn rotate_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        v: spirv::Word,
        i: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an s_sub_sat instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn s_sub_sat(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an s_sub_sat instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn s_sub_sat_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an u_sub_sat instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn u_sub_sat(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an u_sub_sat instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn u_sub_sat_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an u_upsample instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn u_upsample(
        &mut self,
        result_type: spirv::Word,
        hi: spirv::Word,
        lo: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an u_upsample instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn u_upsample_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hi: spirv::Word,
        lo: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an s_upsample instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn s_upsample(
        &mut self,
        result_type: spirv::Word,
        hi: spirv::Word,
        lo: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an s_upsample instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn s_upsample_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hi: spirv::Word,
        lo: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an popcount instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn popcount(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an popcount instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn popcount_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an s_mad24 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn s_mad24(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
        z: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an s_mad24 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn s_mad24_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
        z: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an u_mad24 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn u_mad24(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
        z: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an u_mad24 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn u_mad24_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
        z: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an s_mul24 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn s_mul24(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an s_mul24 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn s_mul24_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an u_mul24 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn u_mul24(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an u_mul24 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn u_mul24_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an u_abs instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn u_abs(&mut self, result_type: spirv::Word, x: spirv::Word)
        -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an u_abs instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn u_abs_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an u_abs_diff instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn u_abs_diff(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an u_abs_diff instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn u_abs_diff_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an u_mul_hi instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn u_mul_hi(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an u_mul_hi instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn u_mul_hi_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an u_mad_hi instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn u_mad_hi(
        &mut self,
        result_type: spirv::Word,
        a: spirv::Word,
        b: spirv::Word,
        c: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an u_mad_hi instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn u_mad_hi_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        a: spirv::Word,
        b: spirv::Word,
        c: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an fclamp instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn fclamp(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        minval: spirv::Word,
        maxval: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an fclamp instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn fclamp_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        minval: spirv::Word,
        maxval: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an degrees instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn degrees(
        &mut self,
        result_type: spirv::Word,
        radians: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an degrees instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn degrees_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        radians: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an fmax_common instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn fmax_common(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an fmax_common instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn fmax_common_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an fmin_common instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn fmin_common(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an fmin_common instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn fmin_common_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an mix instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn mix(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
        a: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an mix instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn mix_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
        a: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an radians instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn radians(
        &mut self,
        result_type: spirv::Word,
        degrees: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an radians instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn radians_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        degrees: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an step instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn step(
        &mut self,
        result_type: spirv::Word,
        edge: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an step instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn step_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        edge: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an smoothstep instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn smoothstep(
        &mut self,
        result_type: spirv::Word,
        edge0: spirv::Word,
        edge1: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an smoothstep instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn smoothstep_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        edge0: spirv::Word,
        edge1: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an sign instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn sign(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an sign instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn sign_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an cross instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn cross(
        &mut self,
        result_type: spirv::Word,
        p0: spirv::Word,
        p1: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an cross instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn cross_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p0: spirv::Word,
        p1: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an distance instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn distance(
        &mut self,
        result_type: spirv::Word,
        p0: spirv::Word,
        p1: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an distance instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn distance_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p0: spirv::Word,
        p1: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an length instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn length(
        &mut self,
        result_type: spirv::Word,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an length instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn length_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an normalize instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn normalize(
        &mut self,
        result_type: spirv::Word,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an normalize instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn normalize_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an fast_distance instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn fast_distance(
        &mut self,
        result_type: spirv::Word,
        p0: spirv::Word,
        p1: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an fast_distance instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn fast_distance_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p0: spirv::Word,
        p1: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an fast_length instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn fast_length(
        &mut self,
        result_type: spirv::Word,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an fast_length instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn fast_length_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an fast_normalize instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn fast_normalize(
        &mut self,
        result_type: spirv::Word,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an fast_normalize instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn fast_normalize_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an bitselect instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn bitselect(
        &mut self,
        result_type: spirv::Word,
        a: spirv::Word,
        b: spirv::Word,
        c: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an bitselect instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn bitselect_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        a: spirv::Word,
        b: spirv::Word,
        c: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an select instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn select(
        &mut self,
        result_type: spirv::Word,
        a: spirv::Word,
        b: spirv::Word,
        c: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an select instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn select_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        a: spirv::Word,
        b: spirv::Word,
        c: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an vloadn instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn vloadn(
        &mut self,
        result_type: spirv::Word,
        offset: spirv::Word,
        p: spirv::Word,
        n: u32,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an vloadn instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn vloadn_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        offset: spirv::Word,
        p: spirv::Word,
        n: u32,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an vstoren instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn vstoren(
        &mut self,
        result_type: spirv::Word,
        data: spirv::Word,
        offset: spirv::Word,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an vstoren instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn vstoren_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        data: spirv::Word,
        offset: spirv::Word,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an vload_half instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn vload_half(
        &mut self,
        result_type: spirv::Word,
        offset: spirv::Word,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an vload_half instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn vload_half_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        offset: spirv::Word,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an vload_halfn instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn vload_halfn(
        &mut self,
        result_type: spirv::Word,
        offset: spirv::Word,
        p: spirv::Word,
        n: u32,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an vload_halfn instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn vload_halfn_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        offset: spirv::Word,
        p: spirv::Word,
        n: u32,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an vstore_half instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn vstore_half(
        &mut self,
        result_type: spirv::Word,
        data: spirv::Word,
        offset: spirv::Word,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an vstore_half instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn vstore_half_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        data: spirv::Word,
        offset: spirv::Word,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an vstore_half_r instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn vstore_half_r(
        &mut self,
        result_type: spirv::Word,
        data: spirv::Word,
        offset: spirv::Word,
        p: spirv::Word,
        mode: spirv::FPRoundingMode,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an vstore_half_r instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn vstore_half_r_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        data: spirv::Word,
        offset: spirv::Word,
        p: spirv::Word,
        mode: spirv::FPRoundingMode,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an vstore_halfn instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn vstore_halfn(
        &mut self,
        result_type: spirv::Word,
        data: spirv::Word,
        offset: spirv::Word,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an vstore_halfn instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn vstore_halfn_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        data: spirv::Word,
        offset: spirv::Word,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an vstore_halfn_r instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn vstore_halfn_r(
        &mut self,
        result_type: spirv::Word,
        data: spirv::Word,
        offset: spirv::Word,
        p: spirv::Word,
        mode: spirv::FPRoundingMode,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an vstore_halfn_r instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn vstore_halfn_r_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        data: spirv::Word,
        offset: spirv::Word,
        p: spirv::Word,
        mode: spirv::FPRoundingMode,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an vloada_halfn instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn vloada_halfn(
        &mut self,
        result_type: spirv::Word,
        offset: spirv::Word,
        p: spirv::Word,
        n: u32,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an vloada_halfn instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn vloada_halfn_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        offset: spirv::Word,
        p: spirv::Word,
        n: u32,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an vstorea_halfn instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn vstorea_halfn(
        &mut self,
        result_type: spirv::Word,
        data: spirv::Word,
        offset: spirv::Word,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an vstorea_halfn instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn vstorea_halfn_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        data: spirv::Word,
        offset: spirv::Word,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an vstorea_halfn_r instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn vstorea_halfn_r(
        &mut self,
        result_type: spirv::Word,
        data: spirv::Word,
        offset: spirv::Word,
        p: spirv::Word,
        mode: spirv::FPRoundingMode,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an vstorea_halfn_r instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn vstorea_halfn_r_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        data: spirv::Word,
        offset: spirv::Word,
        p: spirv::Word,
        mode: spirv::FPRoundingMode,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an shuffle instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn shuffle(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        shuffle_mask: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an shuffle instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn shuffle_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        shuffle_mask: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an shuffle2 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn shuffle2(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
        shuffle_mask: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an shuffle2 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn shuffle2_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
        shuffle_mask: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an printf instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn printf(
        &mut self,
        result_type: spirv::Word,
        format: spirv::Word,
        additional_arguments: impl IntoIterator<Item = spirv::Word>,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an printf instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn printf_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        format: spirv::Word,
        additional_arguments: impl IntoIterator<Item = spirv::Word>,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an prefetch instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn prefetch(
        &mut self,
        result_type: spirv::Word,
        ptr: spirv::Word,
        num_elements: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an prefetch instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn prefetch_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ptr: spirv::Word,
        num_elements: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
}
impl CLOpBuilder for rspirv::dr::Builder {
    #[allow(clippy::too_many_arguments)]
    fn acos(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error> {
        self.acos_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn acos_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::acos as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn acosh(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.acosh_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn acosh_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::acosh as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn acospi(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.acospi_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn acospi_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::acospi as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn asin(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error> {
        self.asin_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn asin_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::asin as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn asinh(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.asinh_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn asinh_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::asinh as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn asinpi(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.asinpi_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn asinpi_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::asinpi as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn atan(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error> {
        self.atan_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn atan_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::atan as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn atan2(
        &mut self,
        result_type: spirv::Word,
        y: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.atan2_id(result_type, None, y, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn atan2_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        y: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(y), dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::atan2 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn atanh(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.atanh_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn atanh_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::atanh as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn atanpi(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.atanpi_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn atanpi_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::atanpi as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn atan2pi(
        &mut self,
        result_type: spirv::Word,
        y: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.atan2pi_id(result_type, None, y, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn atan2pi_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        y: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(y), dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::atan2pi as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn cbrt(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error> {
        self.cbrt_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn cbrt_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::cbrt as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn ceil(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error> {
        self.ceil_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn ceil_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::ceil as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn copysign(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.copysign_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    fn copysign_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::copysign as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn cos(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error> {
        self.cos_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn cos_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::cos as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn cosh(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error> {
        self.cosh_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn cosh_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::cosh as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn cospi(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cospi_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn cospi_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::cospi as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn erfc(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error> {
        self.erfc_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn erfc_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::erfc as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn erf(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error> {
        self.erf_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn erf_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::erf as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn exp(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error> {
        self.exp_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn exp_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::exp as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn exp2(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error> {
        self.exp2_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn exp2_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::exp2 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn exp10(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.exp10_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn exp10_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::exp10 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn expm1(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.expm1_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn expm1_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::expm1 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn fabs(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error> {
        self.fabs_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn fabs_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::fabs as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn fdim(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.fdim_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    fn fdim_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::fdim as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn floor(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.floor_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn floor_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::floor as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn fma(
        &mut self,
        result_type: spirv::Word,
        a: spirv::Word,
        b: spirv::Word,
        c: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.fma_id(result_type, None, a, b, c)
    }
    #[allow(clippy::too_many_arguments)]
    fn fma_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        a: spirv::Word,
        b: spirv::Word,
        c: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(a),
            dr::Operand::IdRef(b),
            dr::Operand::IdRef(c),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::fma as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn fmax(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.fmax_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    fn fmax_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::fmax as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn fmin(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.fmin_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    fn fmin_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::fmin as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn fmod(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.fmod_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    fn fmod_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::fmod as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn fract(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        ptr: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.fract_id(result_type, None, x, ptr)
    }
    #[allow(clippy::too_many_arguments)]
    fn fract_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        ptr: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(ptr)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::fract as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn frexp(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        exp: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.frexp_id(result_type, None, x, exp)
    }
    #[allow(clippy::too_many_arguments)]
    fn frexp_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        exp: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(exp)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::frexp as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn hypot(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.hypot_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    fn hypot_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::hypot as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn ilogb(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.ilogb_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn ilogb_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::ilogb as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn ldexp(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        k: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.ldexp_id(result_type, None, x, k)
    }
    #[allow(clippy::too_many_arguments)]
    fn ldexp_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        k: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(k)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::ldexp as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn lgamma(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.lgamma_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn lgamma_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::lgamma as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn lgamma_r(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        signp: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.lgamma_r_id(result_type, None, x, signp)
    }
    #[allow(clippy::too_many_arguments)]
    fn lgamma_r_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        signp: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(signp)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::lgamma_r as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn log(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error> {
        self.log_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn log_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::log as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn log2(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error> {
        self.log2_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn log2_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::log2 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn log10(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.log10_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn log10_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::log10 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn log1p(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.log1p_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn log1p_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::log1p as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn logb(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error> {
        self.logb_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn logb_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::logb as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn mad(
        &mut self,
        result_type: spirv::Word,
        a: spirv::Word,
        b: spirv::Word,
        c: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.mad_id(result_type, None, a, b, c)
    }
    #[allow(clippy::too_many_arguments)]
    fn mad_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        a: spirv::Word,
        b: spirv::Word,
        c: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(a),
            dr::Operand::IdRef(b),
            dr::Operand::IdRef(c),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::mad as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn maxmag(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.maxmag_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    fn maxmag_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::maxmag as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn minmag(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.minmag_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    fn minmag_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::minmag as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn modf(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        iptr: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.modf_id(result_type, None, x, iptr)
    }
    #[allow(clippy::too_many_arguments)]
    fn modf_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        iptr: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(iptr)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::modf as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn nan(
        &mut self,
        result_type: spirv::Word,
        nancode: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.nan_id(result_type, None, nancode)
    }
    #[allow(clippy::too_many_arguments)]
    fn nan_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        nancode: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(nancode)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::nan as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn nextafter(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.nextafter_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    fn nextafter_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::nextafter as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn pow(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.pow_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    fn pow_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::pow as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn pown(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.pown_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    fn pown_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::pown as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn powr(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.powr_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    fn powr_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::powr as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn remainder(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.remainder_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    fn remainder_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::remainder as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn remquo(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
        quo: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.remquo_id(result_type, None, x, y, quo)
    }
    #[allow(clippy::too_many_arguments)]
    fn remquo_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
        quo: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(x),
            dr::Operand::IdRef(y),
            dr::Operand::IdRef(quo),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::remquo as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn rint(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error> {
        self.rint_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn rint_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::rint as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn rootn(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.rootn_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    fn rootn_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::rootn as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn round(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.round_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn round_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::round as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn rsqrt(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.rsqrt_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn rsqrt_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::rsqrt as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn sin(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error> {
        self.sin_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn sin_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::sin as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn sincos(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        cosval: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.sincos_id(result_type, None, x, cosval)
    }
    #[allow(clippy::too_many_arguments)]
    fn sincos_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        cosval: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(cosval)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::sincos as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn sinh(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error> {
        self.sinh_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn sinh_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::sinh as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn sinpi(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.sinpi_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn sinpi_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::sinpi as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn sqrt(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error> {
        self.sqrt_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn sqrt_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::sqrt as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn tan(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error> {
        self.tan_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn tan_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::tan as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn tanh(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error> {
        self.tanh_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn tanh_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::tanh as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn tanpi(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.tanpi_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn tanpi_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::tanpi as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn tgamma(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.tgamma_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn tgamma_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::tgamma as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn trunc(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.trunc_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn trunc_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::trunc as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn half_cos(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.half_cos_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn half_cos_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::half_cos as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn half_divide(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.half_divide_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    fn half_divide_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::half_divide as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn half_exp(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.half_exp_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn half_exp_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::half_exp as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn half_exp2(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.half_exp2_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn half_exp2_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::half_exp2 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn half_exp10(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.half_exp10_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn half_exp10_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::half_exp10 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn half_log(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.half_log_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn half_log_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::half_log as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn half_log2(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.half_log2_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn half_log2_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::half_log2 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn half_log10(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.half_log10_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn half_log10_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::half_log10 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn half_powr(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.half_powr_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    fn half_powr_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::half_powr as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn half_recip(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.half_recip_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn half_recip_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::half_recip as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn half_rsqrt(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.half_rsqrt_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn half_rsqrt_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::half_rsqrt as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn half_sin(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.half_sin_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn half_sin_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::half_sin as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn half_sqrt(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.half_sqrt_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn half_sqrt_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::half_sqrt as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn half_tan(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.half_tan_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn half_tan_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::half_tan as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn native_cos(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.native_cos_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn native_cos_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::native_cos as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn native_divide(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.native_divide_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    fn native_divide_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::native_divide as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn native_exp(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.native_exp_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn native_exp_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::native_exp as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn native_exp2(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.native_exp2_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn native_exp2_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::native_exp2 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn native_exp10(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.native_exp10_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn native_exp10_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::native_exp10 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn native_log(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.native_log_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn native_log_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::native_log as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn native_log2(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.native_log2_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn native_log2_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::native_log2 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn native_log10(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.native_log10_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn native_log10_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::native_log10 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn native_powr(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.native_powr_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    fn native_powr_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::native_powr as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn native_recip(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.native_recip_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn native_recip_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::native_recip as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn native_rsqrt(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.native_rsqrt_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn native_rsqrt_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::native_rsqrt as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn native_sin(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.native_sin_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn native_sin_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::native_sin as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn native_sqrt(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.native_sqrt_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn native_sqrt_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::native_sqrt as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn native_tan(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.native_tan_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn native_tan_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::native_tan as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn s_abs(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.s_abs_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn s_abs_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::s_abs as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn s_abs_diff(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.s_abs_diff_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    fn s_abs_diff_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::s_abs_diff as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn s_add_sat(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.s_add_sat_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    fn s_add_sat_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::s_add_sat as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn u_add_sat(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.u_add_sat_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    fn u_add_sat_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::u_add_sat as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn s_hadd(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.s_hadd_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    fn s_hadd_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::s_hadd as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn u_hadd(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.u_hadd_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    fn u_hadd_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::u_hadd as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn s_rhadd(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.s_rhadd_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    fn s_rhadd_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::s_rhadd as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn u_rhadd(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.u_rhadd_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    fn u_rhadd_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::u_rhadd as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn s_clamp(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        minval: spirv::Word,
        maxval: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.s_clamp_id(result_type, None, x, minval, maxval)
    }
    #[allow(clippy::too_many_arguments)]
    fn s_clamp_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        minval: spirv::Word,
        maxval: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(x),
            dr::Operand::IdRef(minval),
            dr::Operand::IdRef(maxval),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::s_clamp as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn u_clamp(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        minval: spirv::Word,
        maxval: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.u_clamp_id(result_type, None, x, minval, maxval)
    }
    #[allow(clippy::too_many_arguments)]
    fn u_clamp_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        minval: spirv::Word,
        maxval: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(x),
            dr::Operand::IdRef(minval),
            dr::Operand::IdRef(maxval),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::u_clamp as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn clz(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error> {
        self.clz_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn clz_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::clz as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn ctz(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error> {
        self.ctz_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn ctz_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::ctz as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn s_mad_hi(
        &mut self,
        result_type: spirv::Word,
        a: spirv::Word,
        b: spirv::Word,
        c: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.s_mad_hi_id(result_type, None, a, b, c)
    }
    #[allow(clippy::too_many_arguments)]
    fn s_mad_hi_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        a: spirv::Word,
        b: spirv::Word,
        c: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(a),
            dr::Operand::IdRef(b),
            dr::Operand::IdRef(c),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::s_mad_hi as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn u_mad_sat(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
        z: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.u_mad_sat_id(result_type, None, x, y, z)
    }
    #[allow(clippy::too_many_arguments)]
    fn u_mad_sat_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
        z: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(x),
            dr::Operand::IdRef(y),
            dr::Operand::IdRef(z),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::u_mad_sat as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn s_mad_sat(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
        z: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.s_mad_sat_id(result_type, None, x, y, z)
    }
    #[allow(clippy::too_many_arguments)]
    fn s_mad_sat_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
        z: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(x),
            dr::Operand::IdRef(y),
            dr::Operand::IdRef(z),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::s_mad_sat as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn s_max(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.s_max_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    fn s_max_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::s_max as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn u_max(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.u_max_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    fn u_max_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::u_max as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn s_min(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.s_min_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    fn s_min_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::s_min as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn u_min(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.u_min_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    fn u_min_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::u_min as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn s_mul_hi(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.s_mul_hi_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    fn s_mul_hi_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::s_mul_hi as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn rotate(
        &mut self,
        result_type: spirv::Word,
        v: spirv::Word,
        i: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.rotate_id(result_type, None, v, i)
    }
    #[allow(clippy::too_many_arguments)]
    fn rotate_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        v: spirv::Word,
        i: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(v), dr::Operand::IdRef(i)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::rotate as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn s_sub_sat(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.s_sub_sat_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    fn s_sub_sat_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::s_sub_sat as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn u_sub_sat(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.u_sub_sat_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    fn u_sub_sat_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::u_sub_sat as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn u_upsample(
        &mut self,
        result_type: spirv::Word,
        hi: spirv::Word,
        lo: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.u_upsample_id(result_type, None, hi, lo)
    }
    #[allow(clippy::too_many_arguments)]
    fn u_upsample_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hi: spirv::Word,
        lo: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(hi), dr::Operand::IdRef(lo)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::u_upsample as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn s_upsample(
        &mut self,
        result_type: spirv::Word,
        hi: spirv::Word,
        lo: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.s_upsample_id(result_type, None, hi, lo)
    }
    #[allow(clippy::too_many_arguments)]
    fn s_upsample_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        hi: spirv::Word,
        lo: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(hi), dr::Operand::IdRef(lo)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::s_upsample as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn popcount(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.popcount_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn popcount_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::popcount as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn s_mad24(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
        z: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.s_mad24_id(result_type, None, x, y, z)
    }
    #[allow(clippy::too_many_arguments)]
    fn s_mad24_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
        z: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(x),
            dr::Operand::IdRef(y),
            dr::Operand::IdRef(z),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::s_mad24 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn u_mad24(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
        z: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.u_mad24_id(result_type, None, x, y, z)
    }
    #[allow(clippy::too_many_arguments)]
    fn u_mad24_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
        z: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(x),
            dr::Operand::IdRef(y),
            dr::Operand::IdRef(z),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::u_mad24 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn s_mul24(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.s_mul24_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    fn s_mul24_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::s_mul24 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn u_mul24(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.u_mul24_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    fn u_mul24_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::u_mul24 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn u_abs(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.u_abs_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn u_abs_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::u_abs as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn u_abs_diff(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.u_abs_diff_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    fn u_abs_diff_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::u_abs_diff as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn u_mul_hi(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.u_mul_hi_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    fn u_mul_hi_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::u_mul_hi as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn u_mad_hi(
        &mut self,
        result_type: spirv::Word,
        a: spirv::Word,
        b: spirv::Word,
        c: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.u_mad_hi_id(result_type, None, a, b, c)
    }
    #[allow(clippy::too_many_arguments)]
    fn u_mad_hi_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        a: spirv::Word,
        b: spirv::Word,
        c: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(a),
            dr::Operand::IdRef(b),
            dr::Operand::IdRef(c),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::u_mad_hi as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn fclamp(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        minval: spirv::Word,
        maxval: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.fclamp_id(result_type, None, x, minval, maxval)
    }
    #[allow(clippy::too_many_arguments)]
    fn fclamp_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        minval: spirv::Word,
        maxval: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(x),
            dr::Operand::IdRef(minval),
            dr::Operand::IdRef(maxval),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::fclamp as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn degrees(
        &mut self,
        result_type: spirv::Word,
        radians: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.degrees_id(result_type, None, radians)
    }
    #[allow(clippy::too_many_arguments)]
    fn degrees_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        radians: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(radians)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::degrees as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn fmax_common(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.fmax_common_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    fn fmax_common_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::fmax_common as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn fmin_common(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.fmin_common_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    fn fmin_common_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::fmin_common as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn mix(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
        a: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.mix_id(result_type, None, x, y, a)
    }
    #[allow(clippy::too_many_arguments)]
    fn mix_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
        a: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(x),
            dr::Operand::IdRef(y),
            dr::Operand::IdRef(a),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::mix as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn radians(
        &mut self,
        result_type: spirv::Word,
        degrees: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.radians_id(result_type, None, degrees)
    }
    #[allow(clippy::too_many_arguments)]
    fn radians_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        degrees: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(degrees)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::radians as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn step(
        &mut self,
        result_type: spirv::Word,
        edge: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.step_id(result_type, None, edge, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn step_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        edge: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(edge), dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::step as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn smoothstep(
        &mut self,
        result_type: spirv::Word,
        edge0: spirv::Word,
        edge1: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.smoothstep_id(result_type, None, edge0, edge1, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn smoothstep_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        edge0: spirv::Word,
        edge1: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(edge0),
            dr::Operand::IdRef(edge1),
            dr::Operand::IdRef(x),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::smoothstep as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn sign(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error> {
        self.sign_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn sign_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::sign as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn cross(
        &mut self,
        result_type: spirv::Word,
        p0: spirv::Word,
        p1: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cross_id(result_type, None, p0, p1)
    }
    #[allow(clippy::too_many_arguments)]
    fn cross_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p0: spirv::Word,
        p1: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(p0), dr::Operand::IdRef(p1)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::cross as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn distance(
        &mut self,
        result_type: spirv::Word,
        p0: spirv::Word,
        p1: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.distance_id(result_type, None, p0, p1)
    }
    #[allow(clippy::too_many_arguments)]
    fn distance_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p0: spirv::Word,
        p1: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(p0), dr::Operand::IdRef(p1)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::distance as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn length(
        &mut self,
        result_type: spirv::Word,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.length_id(result_type, None, p)
    }
    #[allow(clippy::too_many_arguments)]
    fn length_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(p)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::length as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn normalize(
        &mut self,
        result_type: spirv::Word,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.normalize_id(result_type, None, p)
    }
    #[allow(clippy::too_many_arguments)]
    fn normalize_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(p)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::normalize as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn fast_distance(
        &mut self,
        result_type: spirv::Word,
        p0: spirv::Word,
        p1: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.fast_distance_id(result_type, None, p0, p1)
    }
    #[allow(clippy::too_many_arguments)]
    fn fast_distance_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p0: spirv::Word,
        p1: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(p0), dr::Operand::IdRef(p1)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::fast_distance as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn fast_length(
        &mut self,
        result_type: spirv::Word,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.fast_length_id(result_type, None, p)
    }
    #[allow(clippy::too_many_arguments)]
    fn fast_length_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(p)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::fast_length as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn fast_normalize(
        &mut self,
        result_type: spirv::Word,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.fast_normalize_id(result_type, None, p)
    }
    #[allow(clippy::too_many_arguments)]
    fn fast_normalize_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(p)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::fast_normalize as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn bitselect(
        &mut self,
        result_type: spirv::Word,
        a: spirv::Word,
        b: spirv::Word,
        c: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.bitselect_id(result_type, None, a, b, c)
    }
    #[allow(clippy::too_many_arguments)]
    fn bitselect_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        a: spirv::Word,
        b: spirv::Word,
        c: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(a),
            dr::Operand::IdRef(b),
            dr::Operand::IdRef(c),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::bitselect as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn select(
        &mut self,
        result_type: spirv::Word,
        a: spirv::Word,
        b: spirv::Word,
        c: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.select_id(result_type, None, a, b, c)
    }
    #[allow(clippy::too_many_arguments)]
    fn select_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        a: spirv::Word,
        b: spirv::Word,
        c: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(a),
            dr::Operand::IdRef(b),
            dr::Operand::IdRef(c),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::select as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn vloadn(
        &mut self,
        result_type: spirv::Word,
        offset: spirv::Word,
        p: spirv::Word,
        n: u32,
    ) -> Result<spirv::Word, dr::Error> {
        self.vloadn_id(result_type, None, offset, p, n)
    }
    #[allow(clippy::too_many_arguments)]
    fn vloadn_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        offset: spirv::Word,
        p: spirv::Word,
        n: u32,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(offset),
            dr::Operand::IdRef(p),
            dr::Operand::LiteralBit32(n),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::vloadn as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn vstoren(
        &mut self,
        result_type: spirv::Word,
        data: spirv::Word,
        offset: spirv::Word,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.vstoren_id(result_type, None, data, offset, p)
    }
    #[allow(clippy::too_many_arguments)]
    fn vstoren_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        data: spirv::Word,
        offset: spirv::Word,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(data),
            dr::Operand::IdRef(offset),
            dr::Operand::IdRef(p),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::vstoren as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn vload_half(
        &mut self,
        result_type: spirv::Word,
        offset: spirv::Word,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.vload_half_id(result_type, None, offset, p)
    }
    #[allow(clippy::too_many_arguments)]
    fn vload_half_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        offset: spirv::Word,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(offset), dr::Operand::IdRef(p)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::vload_half as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn vload_halfn(
        &mut self,
        result_type: spirv::Word,
        offset: spirv::Word,
        p: spirv::Word,
        n: u32,
    ) -> Result<spirv::Word, dr::Error> {
        self.vload_halfn_id(result_type, None, offset, p, n)
    }
    #[allow(clippy::too_many_arguments)]
    fn vload_halfn_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        offset: spirv::Word,
        p: spirv::Word,
        n: u32,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(offset),
            dr::Operand::IdRef(p),
            dr::Operand::LiteralBit32(n),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::vload_halfn as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn vstore_half(
        &mut self,
        result_type: spirv::Word,
        data: spirv::Word,
        offset: spirv::Word,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.vstore_half_id(result_type, None, data, offset, p)
    }
    #[allow(clippy::too_many_arguments)]
    fn vstore_half_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        data: spirv::Word,
        offset: spirv::Word,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(data),
            dr::Operand::IdRef(offset),
            dr::Operand::IdRef(p),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::vstore_half as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn vstore_half_r(
        &mut self,
        result_type: spirv::Word,
        data: spirv::Word,
        offset: spirv::Word,
        p: spirv::Word,
        mode: spirv::FPRoundingMode,
    ) -> Result<spirv::Word, dr::Error> {
        self.vstore_half_r_id(result_type, None, data, offset, p, mode)
    }
    #[allow(clippy::too_many_arguments)]
    fn vstore_half_r_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        data: spirv::Word,
        offset: spirv::Word,
        p: spirv::Word,
        mode: spirv::FPRoundingMode,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(data),
            dr::Operand::IdRef(offset),
            dr::Operand::IdRef(p),
            dr::Operand::FPRoundingMode(mode),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::vstore_half_r as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn vstore_halfn(
        &mut self,
        result_type: spirv::Word,
        data: spirv::Word,
        offset: spirv::Word,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.vstore_halfn_id(result_type, None, data, offset, p)
    }
    #[allow(clippy::too_many_arguments)]
    fn vstore_halfn_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        data: spirv::Word,
        offset: spirv::Word,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(data),
            dr::Operand::IdRef(offset),
            dr::Operand::IdRef(p),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::vstore_halfn as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn vstore_halfn_r(
        &mut self,
        result_type: spirv::Word,
        data: spirv::Word,
        offset: spirv::Word,
        p: spirv::Word,
        mode: spirv::FPRoundingMode,
    ) -> Result<spirv::Word, dr::Error> {
        self.vstore_halfn_r_id(result_type, None, data, offset, p, mode)
    }
    #[allow(clippy::too_many_arguments)]
    fn vstore_halfn_r_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        data: spirv::Word,
        offset: spirv::Word,
        p: spirv::Word,
        mode: spirv::FPRoundingMode,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(data),
            dr::Operand::IdRef(offset),
            dr::Operand::IdRef(p),
            dr::Operand::FPRoundingMode(mode),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::vstore_halfn_r as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn vloada_halfn(
        &mut self,
        result_type: spirv::Word,
        offset: spirv::Word,
        p: spirv::Word,
        n: u32,
    ) -> Result<spirv::Word, dr::Error> {
        self.vloada_halfn_id(result_type, None, offset, p, n)
    }
    #[allow(clippy::too_many_arguments)]
    fn vloada_halfn_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        offset: spirv::Word,
        p: spirv::Word,
        n: u32,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(offset),
            dr::Operand::IdRef(p),
            dr::Operand::LiteralBit32(n),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::vloada_halfn as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn vstorea_halfn(
        &mut self,
        result_type: spirv::Word,
        data: spirv::Word,
        offset: spirv::Word,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.vstorea_halfn_id(result_type, None, data, offset, p)
    }
    #[allow(clippy::too_many_arguments)]
    fn vstorea_halfn_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        data: spirv::Word,
        offset: spirv::Word,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(data),
            dr::Operand::IdRef(offset),
            dr::Operand::IdRef(p),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::vstorea_halfn as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn vstorea_halfn_r(
        &mut self,
        result_type: spirv::Word,
        data: spirv::Word,
        offset: spirv::Word,
        p: spirv::Word,
        mode: spirv::FPRoundingMode,
    ) -> Result<spirv::Word, dr::Error> {
        self.vstorea_halfn_r_id(result_type, None, data, offset, p, mode)
    }
    #[allow(clippy::too_many_arguments)]
    fn vstorea_halfn_r_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        data: spirv::Word,
        offset: spirv::Word,
        p: spirv::Word,
        mode: spirv::FPRoundingMode,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(data),
            dr::Operand::IdRef(offset),
            dr::Operand::IdRef(p),
            dr::Operand::FPRoundingMode(mode),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::vstorea_halfn_r as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn shuffle(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        shuffle_mask: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.shuffle_id(result_type, None, x, shuffle_mask)
    }
    #[allow(clippy::too_many_arguments)]
    fn shuffle_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        shuffle_mask: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(shuffle_mask)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::shuffle as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn shuffle2(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
        shuffle_mask: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.shuffle2_id(result_type, None, x, y, shuffle_mask)
    }
    #[allow(clippy::too_many_arguments)]
    fn shuffle2_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
        shuffle_mask: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(x),
            dr::Operand::IdRef(y),
            dr::Operand::IdRef(shuffle_mask),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::shuffle2 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn printf(
        &mut self,
        result_type: spirv::Word,
        format: spirv::Word,
        additional_arguments: impl IntoIterator<Item = spirv::Word>,
    ) -> Result<spirv::Word, dr::Error> {
        self.printf_id(result_type, None, format, additional_arguments)
    }
    #[allow(clippy::too_many_arguments)]
    fn printf_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        format: spirv::Word,
        additional_arguments: impl IntoIterator<Item = spirv::Word>,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(format)];
        args.extend(additional_arguments.into_iter().map(dr::Operand::IdRef));
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::printf as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn prefetch(
        &mut self,
        result_type: spirv::Word,
        ptr: spirv::Word,
        num_elements: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.prefetch_id(result_type, None, ptr, num_elements)
    }
    #[allow(clippy::too_many_arguments)]
    fn prefetch_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        ptr: spirv::Word,
        num_elements: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "OpenCL.std.100");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(ptr), dr::Operand::IdRef(num_elements)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::CLOp::prefetch as spirv::Word,
            args,
        )
    }
}
