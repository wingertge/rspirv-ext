// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

use rspirv::{dr, spirv};
pub trait GLOpBuilder {
    #[doc = "Appends an Round instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn round(&mut self, result_type: spirv::Word, x: spirv::Word)
        -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Round instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn round_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an RoundEven instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn round_even(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an RoundEven instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn round_even_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Trunc instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn trunc(&mut self, result_type: spirv::Word, x: spirv::Word)
        -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Trunc instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn trunc_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an FAbs instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn f_abs(&mut self, result_type: spirv::Word, x: spirv::Word)
        -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an FAbs instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn f_abs_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an SAbs instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn s_abs(&mut self, result_type: spirv::Word, x: spirv::Word)
        -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an SAbs instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn s_abs_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an FSign instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn f_sign(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an FSign instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn f_sign_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an SSign instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn s_sign(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an SSign instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn s_sign_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Floor instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn floor(&mut self, result_type: spirv::Word, x: spirv::Word)
        -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Floor instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn floor_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Ceil instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn ceil(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Ceil instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn ceil_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Fract instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn fract(&mut self, result_type: spirv::Word, x: spirv::Word)
        -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Fract instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn fract_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Radians instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn radians(
        &mut self,
        result_type: spirv::Word,
        degrees: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Radians instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn radians_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        degrees: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Degrees instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn degrees(
        &mut self,
        result_type: spirv::Word,
        radians: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Degrees instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn degrees_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        radians: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Sin instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn sin(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Sin instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn sin_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Cos instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn cos(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Cos instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn cos_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Tan instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn tan(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Tan instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn tan_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Asin instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn asin(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Asin instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn asin_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Acos instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn acos(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Acos instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn acos_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Atan instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn atan(
        &mut self,
        result_type: spirv::Word,
        y_over_x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Atan instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn atan_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        y_over_x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Sinh instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn sinh(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Sinh instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn sinh_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Cosh instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn cosh(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Cosh instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn cosh_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Tanh instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn tanh(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Tanh instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn tanh_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Asinh instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn asinh(&mut self, result_type: spirv::Word, x: spirv::Word)
        -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Asinh instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn asinh_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Acosh instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn acosh(&mut self, result_type: spirv::Word, x: spirv::Word)
        -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Acosh instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn acosh_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Atanh instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn atanh(&mut self, result_type: spirv::Word, x: spirv::Word)
        -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Atanh instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn atanh_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Atan2 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn atan2(
        &mut self,
        result_type: spirv::Word,
        y: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Atan2 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn atan2_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        y: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Pow instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn pow(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Pow instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn pow_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Exp instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn exp(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Exp instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn exp_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Log instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn log(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Log instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn log_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Exp2 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn exp2(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Exp2 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn exp2_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Log2 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn log2(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Log2 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn log2_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Sqrt instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn sqrt(&mut self, result_type: spirv::Word, x: spirv::Word) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Sqrt instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn sqrt_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an InverseSqrt instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn inverse_sqrt(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an InverseSqrt instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn inverse_sqrt_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Determinant instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn determinant(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Determinant instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn determinant_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an MatrixInverse instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn matrix_inverse(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an MatrixInverse instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn matrix_inverse_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Modf instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn modf(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        i: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Modf instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn modf_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        i: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an ModfStruct instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn modf_struct(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an ModfStruct instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn modf_struct_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an FMin instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn f_min(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an FMin instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn f_min_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an UMin instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn u_min(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an UMin instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn u_min_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an SMin instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn s_min(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an SMin instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn s_min_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an FMax instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn f_max(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an FMax instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn f_max_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an UMax instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn u_max(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an UMax instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn u_max_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an SMax instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn s_max(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an SMax instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn s_max_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an FClamp instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn f_clamp(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        min_val: spirv::Word,
        max_val: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an FClamp instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn f_clamp_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        min_val: spirv::Word,
        max_val: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an UClamp instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn u_clamp(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        min_val: spirv::Word,
        max_val: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an UClamp instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn u_clamp_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        min_val: spirv::Word,
        max_val: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an SClamp instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn s_clamp(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        min_val: spirv::Word,
        max_val: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an SClamp instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn s_clamp_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        min_val: spirv::Word,
        max_val: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an FMix instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn f_mix(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
        a: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an FMix instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn f_mix_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
        a: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an IMix instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn i_mix(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
        a: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an IMix instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn i_mix_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
        a: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Step instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn step(
        &mut self,
        result_type: spirv::Word,
        edge: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Step instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn step_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        edge: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an SmoothStep instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn smooth_step(
        &mut self,
        result_type: spirv::Word,
        edge0: spirv::Word,
        edge1: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an SmoothStep instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn smooth_step_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        edge0: spirv::Word,
        edge1: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Fma instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn fma(
        &mut self,
        result_type: spirv::Word,
        a: spirv::Word,
        b: spirv::Word,
        c: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Fma instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn fma_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        a: spirv::Word,
        b: spirv::Word,
        c: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Frexp instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn frexp(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        exp: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Frexp instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn frexp_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        exp: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an FrexpStruct instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn frexp_struct(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an FrexpStruct instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn frexp_struct_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Ldexp instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn ldexp(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        exp: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Ldexp instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn ldexp_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        exp: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an PackSnorm4x8 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn pack_snorm4x8(
        &mut self,
        result_type: spirv::Word,
        v: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an PackSnorm4x8 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn pack_snorm4x8_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        v: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an PackUnorm4x8 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn pack_unorm4x8(
        &mut self,
        result_type: spirv::Word,
        v: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an PackUnorm4x8 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn pack_unorm4x8_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        v: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an PackSnorm2x16 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn pack_snorm2x16(
        &mut self,
        result_type: spirv::Word,
        v: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an PackSnorm2x16 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn pack_snorm2x16_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        v: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an PackUnorm2x16 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn pack_unorm2x16(
        &mut self,
        result_type: spirv::Word,
        v: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an PackUnorm2x16 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn pack_unorm2x16_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        v: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an PackHalf2x16 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn pack_half2x16(
        &mut self,
        result_type: spirv::Word,
        v: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an PackHalf2x16 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn pack_half2x16_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        v: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an PackDouble2x32 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn pack_double2x32(
        &mut self,
        result_type: spirv::Word,
        v: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an PackDouble2x32 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn pack_double2x32_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        v: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an UnpackSnorm2x16 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn unpack_snorm2x16(
        &mut self,
        result_type: spirv::Word,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an UnpackSnorm2x16 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn unpack_snorm2x16_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an UnpackUnorm2x16 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn unpack_unorm2x16(
        &mut self,
        result_type: spirv::Word,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an UnpackUnorm2x16 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn unpack_unorm2x16_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an UnpackHalf2x16 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn unpack_half2x16(
        &mut self,
        result_type: spirv::Word,
        v: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an UnpackHalf2x16 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn unpack_half2x16_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        v: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an UnpackSnorm4x8 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn unpack_snorm4x8(
        &mut self,
        result_type: spirv::Word,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an UnpackSnorm4x8 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn unpack_snorm4x8_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an UnpackUnorm4x8 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn unpack_unorm4x8(
        &mut self,
        result_type: spirv::Word,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an UnpackUnorm4x8 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn unpack_unorm4x8_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an UnpackDouble2x32 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn unpack_double2x32(
        &mut self,
        result_type: spirv::Word,
        v: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an UnpackDouble2x32 instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn unpack_double2x32_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        v: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Length instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn length(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Length instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn length_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Distance instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn distance(
        &mut self,
        result_type: spirv::Word,
        p0: spirv::Word,
        p1: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Distance instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn distance_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p0: spirv::Word,
        p1: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Cross instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn cross(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Cross instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn cross_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Normalize instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn normalize(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Normalize instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn normalize_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an FaceForward instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn face_forward(
        &mut self,
        result_type: spirv::Word,
        n: spirv::Word,
        i: spirv::Word,
        nref: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an FaceForward instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn face_forward_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        n: spirv::Word,
        i: spirv::Word,
        nref: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Reflect instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn reflect(
        &mut self,
        result_type: spirv::Word,
        i: spirv::Word,
        n: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Reflect instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn reflect_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        i: spirv::Word,
        n: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Refract instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn refract(
        &mut self,
        result_type: spirv::Word,
        i: spirv::Word,
        n: spirv::Word,
        eta: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an Refract instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn refract_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        i: spirv::Word,
        n: spirv::Word,
        eta: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an FindILsb instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn find_i_lsb(
        &mut self,
        result_type: spirv::Word,
        value: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an FindILsb instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn find_i_lsb_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        value: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an FindSMsb instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn find_s_msb(
        &mut self,
        result_type: spirv::Word,
        value: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an FindSMsb instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn find_s_msb_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        value: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an FindUMsb instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn find_u_msb(
        &mut self,
        result_type: spirv::Word,
        value: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an FindUMsb instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn find_u_msb_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        value: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an InterpolateAtCentroid instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn interpolate_at_centroid(
        &mut self,
        result_type: spirv::Word,
        interpolant: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an InterpolateAtCentroid instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn interpolate_at_centroid_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        interpolant: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an InterpolateAtSample instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn interpolate_at_sample(
        &mut self,
        result_type: spirv::Word,
        interpolant: spirv::Word,
        sample: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an InterpolateAtSample instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn interpolate_at_sample_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        interpolant: spirv::Word,
        sample: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an InterpolateAtOffset instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn interpolate_at_offset(
        &mut self,
        result_type: spirv::Word,
        interpolant: spirv::Word,
        offset: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an InterpolateAtOffset instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn interpolate_at_offset_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        interpolant: spirv::Word,
        offset: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an NMin instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn n_min(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an NMin instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn n_min_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an NMax instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn n_max(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an NMax instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn n_max_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an NClamp instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn n_clamp(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        min_val: spirv::Word,
        max_val: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
    #[doc = "Appends an NClamp instruction and returns the result id, or return the existing id if the instruction was already present."]
    #[allow(clippy::too_many_arguments)]
    fn n_clamp_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        min_val: spirv::Word,
        max_val: spirv::Word,
    ) -> Result<spirv::Word, dr::Error>;
}
impl GLOpBuilder for rspirv::dr::Builder {
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
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Round as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn round_even(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.round_even_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn round_even_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::RoundEven as spirv::Word,
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
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Trunc as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn f_abs(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.f_abs_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn f_abs_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::FAbs as spirv::Word,
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
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::SAbs as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn f_sign(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.f_sign_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn f_sign_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::FSign as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn s_sign(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.s_sign_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn s_sign_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::SSign as spirv::Word,
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
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Floor as spirv::Word,
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
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Ceil as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn fract(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.fract_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn fract_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Fract as spirv::Word,
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
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(degrees)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Radians as spirv::Word,
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
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(radians)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Degrees as spirv::Word,
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
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Sin as spirv::Word,
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
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Cos as spirv::Word,
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
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Tan as spirv::Word,
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
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Asin as spirv::Word,
            args,
        )
    }
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
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Acos as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn atan(
        &mut self,
        result_type: spirv::Word,
        y_over_x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.atan_id(result_type, None, y_over_x)
    }
    #[allow(clippy::too_many_arguments)]
    fn atan_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        y_over_x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(y_over_x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Atan as spirv::Word,
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
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Sinh as spirv::Word,
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
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Cosh as spirv::Word,
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
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Tanh as spirv::Word,
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
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Asinh as spirv::Word,
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
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Acosh as spirv::Word,
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
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Atanh as spirv::Word,
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
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(y), dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Atan2 as spirv::Word,
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
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Pow as spirv::Word,
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
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Exp as spirv::Word,
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
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Log as spirv::Word,
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
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Exp2 as spirv::Word,
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
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Log2 as spirv::Word,
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
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Sqrt as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn inverse_sqrt(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.inverse_sqrt_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn inverse_sqrt_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::InverseSqrt as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn determinant(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.determinant_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn determinant_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Determinant as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn matrix_inverse(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.matrix_inverse_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn matrix_inverse_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::MatrixInverse as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn modf(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        i: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.modf_id(result_type, None, x, i)
    }
    #[allow(clippy::too_many_arguments)]
    fn modf_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        i: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(i)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Modf as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn modf_struct(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.modf_struct_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn modf_struct_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::ModfStruct as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn f_min(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.f_min_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    fn f_min_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::FMin as spirv::Word,
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
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::UMin as spirv::Word,
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
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::SMin as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn f_max(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.f_max_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    fn f_max_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::FMax as spirv::Word,
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
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::UMax as spirv::Word,
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
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::SMax as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn f_clamp(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        min_val: spirv::Word,
        max_val: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.f_clamp_id(result_type, None, x, min_val, max_val)
    }
    #[allow(clippy::too_many_arguments)]
    fn f_clamp_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        min_val: spirv::Word,
        max_val: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(x),
            dr::Operand::IdRef(min_val),
            dr::Operand::IdRef(max_val),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::FClamp as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn u_clamp(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        min_val: spirv::Word,
        max_val: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.u_clamp_id(result_type, None, x, min_val, max_val)
    }
    #[allow(clippy::too_many_arguments)]
    fn u_clamp_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        min_val: spirv::Word,
        max_val: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(x),
            dr::Operand::IdRef(min_val),
            dr::Operand::IdRef(max_val),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::UClamp as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn s_clamp(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        min_val: spirv::Word,
        max_val: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.s_clamp_id(result_type, None, x, min_val, max_val)
    }
    #[allow(clippy::too_many_arguments)]
    fn s_clamp_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        min_val: spirv::Word,
        max_val: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(x),
            dr::Operand::IdRef(min_val),
            dr::Operand::IdRef(max_val),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::SClamp as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn f_mix(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
        a: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.f_mix_id(result_type, None, x, y, a)
    }
    #[allow(clippy::too_many_arguments)]
    fn f_mix_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
        a: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
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
            crate::spirv::GLOp::FMix as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn i_mix(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
        a: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.i_mix_id(result_type, None, x, y, a)
    }
    #[allow(clippy::too_many_arguments)]
    fn i_mix_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
        a: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
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
            crate::spirv::GLOp::IMix as spirv::Word,
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
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(edge), dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Step as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn smooth_step(
        &mut self,
        result_type: spirv::Word,
        edge0: spirv::Word,
        edge1: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.smooth_step_id(result_type, None, edge0, edge1, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn smooth_step_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        edge0: spirv::Word,
        edge1: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
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
            crate::spirv::GLOp::SmoothStep as spirv::Word,
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
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
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
            crate::spirv::GLOp::Fma as spirv::Word,
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
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(exp)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Frexp as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn frexp_struct(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.frexp_struct_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn frexp_struct_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::FrexpStruct as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn ldexp(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        exp: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.ldexp_id(result_type, None, x, exp)
    }
    #[allow(clippy::too_many_arguments)]
    fn ldexp_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        exp: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(exp)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Ldexp as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn pack_snorm4x8(
        &mut self,
        result_type: spirv::Word,
        v: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.pack_snorm4x8_id(result_type, None, v)
    }
    #[allow(clippy::too_many_arguments)]
    fn pack_snorm4x8_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        v: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(v)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::PackSnorm4x8 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn pack_unorm4x8(
        &mut self,
        result_type: spirv::Word,
        v: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.pack_unorm4x8_id(result_type, None, v)
    }
    #[allow(clippy::too_many_arguments)]
    fn pack_unorm4x8_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        v: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(v)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::PackUnorm4x8 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn pack_snorm2x16(
        &mut self,
        result_type: spirv::Word,
        v: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.pack_snorm2x16_id(result_type, None, v)
    }
    #[allow(clippy::too_many_arguments)]
    fn pack_snorm2x16_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        v: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(v)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::PackSnorm2x16 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn pack_unorm2x16(
        &mut self,
        result_type: spirv::Word,
        v: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.pack_unorm2x16_id(result_type, None, v)
    }
    #[allow(clippy::too_many_arguments)]
    fn pack_unorm2x16_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        v: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(v)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::PackUnorm2x16 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn pack_half2x16(
        &mut self,
        result_type: spirv::Word,
        v: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.pack_half2x16_id(result_type, None, v)
    }
    #[allow(clippy::too_many_arguments)]
    fn pack_half2x16_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        v: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(v)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::PackHalf2x16 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn pack_double2x32(
        &mut self,
        result_type: spirv::Word,
        v: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.pack_double2x32_id(result_type, None, v)
    }
    #[allow(clippy::too_many_arguments)]
    fn pack_double2x32_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        v: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(v)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::PackDouble2x32 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn unpack_snorm2x16(
        &mut self,
        result_type: spirv::Word,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.unpack_snorm2x16_id(result_type, None, p)
    }
    #[allow(clippy::too_many_arguments)]
    fn unpack_snorm2x16_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(p)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::UnpackSnorm2x16 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn unpack_unorm2x16(
        &mut self,
        result_type: spirv::Word,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.unpack_unorm2x16_id(result_type, None, p)
    }
    #[allow(clippy::too_many_arguments)]
    fn unpack_unorm2x16_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(p)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::UnpackUnorm2x16 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn unpack_half2x16(
        &mut self,
        result_type: spirv::Word,
        v: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.unpack_half2x16_id(result_type, None, v)
    }
    #[allow(clippy::too_many_arguments)]
    fn unpack_half2x16_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        v: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(v)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::UnpackHalf2x16 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn unpack_snorm4x8(
        &mut self,
        result_type: spirv::Word,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.unpack_snorm4x8_id(result_type, None, p)
    }
    #[allow(clippy::too_many_arguments)]
    fn unpack_snorm4x8_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(p)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::UnpackSnorm4x8 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn unpack_unorm4x8(
        &mut self,
        result_type: spirv::Word,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.unpack_unorm4x8_id(result_type, None, p)
    }
    #[allow(clippy::too_many_arguments)]
    fn unpack_unorm4x8_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        p: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(p)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::UnpackUnorm4x8 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn unpack_double2x32(
        &mut self,
        result_type: spirv::Word,
        v: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.unpack_double2x32_id(result_type, None, v)
    }
    #[allow(clippy::too_many_arguments)]
    fn unpack_double2x32_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        v: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(v)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::UnpackDouble2x32 as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn length(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.length_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn length_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Length as spirv::Word,
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
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(p0), dr::Operand::IdRef(p1)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Distance as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn cross(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.cross_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    fn cross_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Cross as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn normalize(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.normalize_id(result_type, None, x)
    }
    #[allow(clippy::too_many_arguments)]
    fn normalize_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Normalize as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn face_forward(
        &mut self,
        result_type: spirv::Word,
        n: spirv::Word,
        i: spirv::Word,
        nref: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.face_forward_id(result_type, None, n, i, nref)
    }
    #[allow(clippy::too_many_arguments)]
    fn face_forward_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        n: spirv::Word,
        i: spirv::Word,
        nref: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(n),
            dr::Operand::IdRef(i),
            dr::Operand::IdRef(nref),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::FaceForward as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn reflect(
        &mut self,
        result_type: spirv::Word,
        i: spirv::Word,
        n: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.reflect_id(result_type, None, i, n)
    }
    #[allow(clippy::too_many_arguments)]
    fn reflect_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        i: spirv::Word,
        n: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(i), dr::Operand::IdRef(n)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Reflect as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn refract(
        &mut self,
        result_type: spirv::Word,
        i: spirv::Word,
        n: spirv::Word,
        eta: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.refract_id(result_type, None, i, n, eta)
    }
    #[allow(clippy::too_many_arguments)]
    fn refract_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        i: spirv::Word,
        n: spirv::Word,
        eta: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(i),
            dr::Operand::IdRef(n),
            dr::Operand::IdRef(eta),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::Refract as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn find_i_lsb(
        &mut self,
        result_type: spirv::Word,
        value: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.find_i_lsb_id(result_type, None, value)
    }
    #[allow(clippy::too_many_arguments)]
    fn find_i_lsb_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        value: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(value)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::FindILsb as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn find_s_msb(
        &mut self,
        result_type: spirv::Word,
        value: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.find_s_msb_id(result_type, None, value)
    }
    #[allow(clippy::too_many_arguments)]
    fn find_s_msb_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        value: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(value)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::FindSMsb as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn find_u_msb(
        &mut self,
        result_type: spirv::Word,
        value: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.find_u_msb_id(result_type, None, value)
    }
    #[allow(clippy::too_many_arguments)]
    fn find_u_msb_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        value: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(value)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::FindUMsb as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn interpolate_at_centroid(
        &mut self,
        result_type: spirv::Word,
        interpolant: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.interpolate_at_centroid_id(result_type, None, interpolant)
    }
    #[allow(clippy::too_many_arguments)]
    fn interpolate_at_centroid_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        interpolant: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(interpolant)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::InterpolateAtCentroid as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn interpolate_at_sample(
        &mut self,
        result_type: spirv::Word,
        interpolant: spirv::Word,
        sample: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.interpolate_at_sample_id(result_type, None, interpolant, sample)
    }
    #[allow(clippy::too_many_arguments)]
    fn interpolate_at_sample_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        interpolant: spirv::Word,
        sample: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(interpolant), dr::Operand::IdRef(sample)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::InterpolateAtSample as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn interpolate_at_offset(
        &mut self,
        result_type: spirv::Word,
        interpolant: spirv::Word,
        offset: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.interpolate_at_offset_id(result_type, None, interpolant, offset)
    }
    #[allow(clippy::too_many_arguments)]
    fn interpolate_at_offset_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        interpolant: spirv::Word,
        offset: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(interpolant), dr::Operand::IdRef(offset)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::InterpolateAtOffset as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn n_min(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.n_min_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    fn n_min_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::NMin as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn n_max(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.n_max_id(result_type, None, x, y)
    }
    #[allow(clippy::too_many_arguments)]
    fn n_max_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        y: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![dr::Operand::IdRef(x), dr::Operand::IdRef(y)];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::NMax as spirv::Word,
            args,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn n_clamp(
        &mut self,
        result_type: spirv::Word,
        x: spirv::Word,
        min_val: spirv::Word,
        max_val: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        self.n_clamp_id(result_type, None, x, min_val, max_val)
    }
    #[allow(clippy::too_many_arguments)]
    fn n_clamp_id(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        x: spirv::Word,
        min_val: spirv::Word,
        max_val: spirv::Word,
    ) -> Result<spirv::Word, dr::Error> {
        let extension_set = super::ext_inst_import(self, "GLSL.std.450");
        #[allow(unused_mut)]
        let mut args = vec![
            dr::Operand::IdRef(x),
            dr::Operand::IdRef(min_val),
            dr::Operand::IdRef(max_val),
        ];
        self.ext_inst(
            result_type,
            result_id,
            extension_set,
            crate::spirv::GLOp::NClamp as spirv::Word,
            args,
        )
    }
}
