use crate::structs::{self, Instruction, OperandKind};
use crate::utils::*;

use heck::SnakeCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

/// Returns true if the given operand kind can potentially have additional
/// parameters.
#[inline(always)]
pub fn has_additional_params(grammar: &structs::OperandKind) -> bool {
    grammar.enumerants.iter().any(|e| !e.parameters.is_empty())
}

/// Returns true if the given operand can potentially have additional
/// parameters.
pub fn operand_has_additional_params(
    operand: &structs::Operand,
    kinds: &[structs::OperandKind],
) -> bool {
    kinds
        .iter()
        .find(|kind| kind.kind == operand.kind)
        .is_some_and(has_additional_params)
}

fn get_param_or_arg_list(
    params: &[structs::Operand],
    keep_result_id: bool,
    kinds: &[structs::OperandKind],
    is_params: bool,
) -> Vec<TokenStream> {
    let mut list: Vec<_> = params
        .iter()
        .enumerate()
        .filter_map(|(param_index, param)| {
            let name = get_param_name(params, param_index);
            let kind = get_enum_underlying_type(&param.kind, true);
            if param.kind == "IdResult" {
                if keep_result_id {
                    if is_params {
                        Some(quote! { result_id: Option<spirv::Word> })
                    } else {
                        Some(quote! { result_id })
                    }
                } else {
                    None
                }
            } else if is_params {
                Some(match param.quantifier {
                    structs::Quantifier::One => quote! { #name: #kind },
                    structs::Quantifier::ZeroOrOne => quote! { #name: Option<#kind> },
                    structs::Quantifier::ZeroOrMore => {
                        quote! { #name: impl IntoIterator<Item = #kind> }
                    }
                })
            } else {
                Some(quote! { #name })
            }
        })
        .collect();
    // The last operand may require additional parameters.
    if let Some(o) = params.last() {
        if operand_has_additional_params(o, kinds) {
            if is_params {
                list.push(quote! { additional_params: impl IntoIterator<Item = dr::Operand> });
            } else {
                list.push(quote! { additional_params });
            }
        }
    }
    list
}

/// Returns the parameter list excluding result id.
fn get_param_list(
    params: &[structs::Operand],
    keep_result_id: bool,
    kinds: &[structs::OperandKind],
) -> Vec<TokenStream> {
    get_param_or_arg_list(params, keep_result_id, kinds, true)
}

fn get_arg_list(
    params: &[structs::Operand],
    keep_result_id: bool,
    kinds: &[structs::OperandKind],
) -> Vec<TokenStream> {
    get_param_or_arg_list(params, keep_result_id, kinds, false)
}

/// Returns the initializer list for all the parameters required to appear
/// once and only once.
fn get_init_list(params: &[structs::Operand]) -> Vec<TokenStream> {
    params
        .iter()
        .enumerate()
        .filter_map(|(param_index, param)| {
            if param.quantifier == structs::Quantifier::One {
                if param.kind == "IdResult" || param.kind == "IdResultType" {
                    // These two operands are not stored in the operands field.
                    None
                } else {
                    let name = get_param_name(params, param_index);
                    let kind = get_dr_operand_kind(&param.kind);
                    let value = if kind == "LiteralString" {
                        quote! { #name.into() }
                    } else {
                        quote! { #name }
                    };
                    Some(quote! { dr::Operand::#kind(#value) })
                }
            } else {
                None
            }
        })
        .collect()
}

fn get_push_extras(
    params: &[structs::Operand],
    kinds: &[structs::OperandKind],
    container: TokenStream,
) -> Vec<TokenStream> {
    let mut list: Vec<_> = params
        .iter()
        .enumerate()
        .filter_map(|(param_index, param)| {
            let name = get_param_name(params, param_index);
            match param.quantifier {
                structs::Quantifier::One => None,
                structs::Quantifier::ZeroOrOne => {
                    let kind = get_dr_operand_kind(&param.kind);
                    let value = if kind == "LiteralString" {
                        quote! { v.into() }
                    } else {
                        quote! { v }
                    };
                    Some(quote! {
                        if let Some(v) = #name {
                            #container.push(dr::Operand::#kind(#value));
                        }
                    })
                }
                structs::Quantifier::ZeroOrMore => {
                    if param.kind == "PairLiteralIntegerIdRef" {
                        Some(quote! {
                            for v in #name {
                                #container.push(v.0);
                                #container.push(dr::Operand::IdRef(v.1));
                            }
                        })
                    } else if param.kind == "PairIdRefLiteralInteger" {
                        Some(quote! {
                            for v in #name {
                                #container.push(dr::Operand::IdRef(v.0));
                                #container.push(dr::Operand::LiteralBit32(v.1));
                            }
                        })
                    } else if param.kind == "PairIdRefIdRef" {
                        Some(quote! {
                            for v in #name {
                                #container.push(dr::Operand::IdRef(v.0));
                                #container.push(dr::Operand::IdRef(v.1));
                            }
                        })
                    } else {
                        let kind = get_dr_operand_kind(&param.kind);
                        Some(quote! {
                            #container.extend(#name.into_iter().map(dr::Operand::#kind));
                        })
                    }
                }
            }
        })
        .collect();
    // The last operand may require additional parameters.
    if let Some(o) = params.last() {
        if operand_has_additional_params(o, kinds) {
            list.push(quote! {
                #container.extend(additional_params);
            });
        }
    }
    list
}

/// Returns the generated build methods for SPIR-V extension instructions by walking the given
/// SPIR-V instructions `grammar`.
pub fn gen_dr_builder_ext(
    ext_name: &str,
    op_name: &str,
    kinds: &[OperandKind],
    instructions: &[Instruction],
    with_result_type: bool,
) -> TokenStream {
    let op_name = as_ident(op_name);

    // Generate build methods for all types.
    let elements = instructions.iter().map(|inst| {
        let param_list = get_param_list(&inst.operands, false, kinds);
        let arg_list = get_arg_list(&inst.operands, false, kinds);
        // Initializer list for constructing the operands parameter
        // for Instruction.
        let init_list = get_init_list(&inst.operands);
        let extras = get_push_extras(&inst.operands,
                                     kinds,
                                     quote![args]);
        let opcode = as_ident(&inst.opname);
        let name = as_ident(&inst.opname.to_snake_case());

        let name_id = format_ident!("{}_id", name);

        if with_result_type {
            quote! {
                #[allow(clippy::too_many_arguments)]
                fn #name(&mut self, result_type: spirv::Word, #(#param_list),*) -> Result<spirv::Word, dr::Error> {
                    self.#name_id(result_type, None, #(#arg_list),*)
                }

                #[allow(clippy::too_many_arguments)]
                fn #name_id(&mut self, result_type: spirv::Word, result_id: Option<spirv::Word>, #(#param_list),*) -> Result<spirv::Word, dr::Error> {
                    let extension_set = super::ext_inst_import(self, #ext_name);

                    #[allow(unused_mut)]
                    let mut args = vec![#(#init_list),*];
                    #(#extras)*

                    self.ext_inst(result_type, result_id, extension_set, crate::spirv::#op_name::#opcode as spirv::Word, args)
                }
            }
        } else {
            quote! {
                #[allow(clippy::too_many_arguments)]
                fn #name(&mut self, #(#param_list),*) -> Result<spirv::Word, dr::Error> {
                    self.#name_id(None, #(#arg_list),*)
                }

                #[allow(clippy::too_many_arguments)]
                fn #name_id(&mut self, result_id: Option<spirv::Word>, #(#param_list),*) -> Result<spirv::Word, dr::Error> {
                    let extension_set = super::ext_inst_import(self, #ext_name);

                    let result_type = self.type_void();

                    #[allow(unused_mut)]
                    let mut args = vec![#(#init_list),*];
                    #(#extras)*

                    self.ext_inst(result_type, result_id, extension_set, crate::spirv::#op_name::#opcode as spirv::Word, args)
                }
            }
        }
    });

    // Generate build methods for all types.
    let element_defs = instructions.iter().map(|inst| {
        let param_list = get_param_list(&inst.operands, false, kinds);
        let opcode = as_ident(&inst.opname);
        let name = as_ident(&inst.opname.to_snake_case());

        let comment = format!("Appends an {} instruction and returns the result id, or return the existing id if the instruction was already present.", opcode);
        let name_id = format_ident!("{}_id", name);

        let result_type = with_result_type.then(|| quote![result_type: spirv::Word,]);

        quote! {
            #[doc = #comment]
            #[allow(clippy::too_many_arguments)]
            fn #name(&mut self, #result_type #(#param_list),*) -> Result<spirv::Word, dr::Error>;

            #[doc = #comment]
            #[allow(clippy::too_many_arguments)]
            fn #name_id(&mut self, #result_type result_id: Option<spirv::Word>, #(#param_list),*) -> Result<spirv::Word, dr::Error>;
        }
    });

    let trait_name = format_ident!("{op_name}Builder");

    quote! {
        use rspirv::{spirv, dr};

        pub trait #trait_name {
            #(#element_defs)*
        }

        impl #trait_name for rspirv::dr::Builder {
            #(#elements)*
        }
    }
}

/// Debug instructions that are local, as opposed to being appended to the types
const LOCAL_DEBUG_OPS: &[&str] = &[
    "DebugScope",
    "DebugNoScope",
    "DebugDeclare",
    "DebugValue",
    "DebugLine",
    "DebugNoLine",
    "DebugFunctionDefinition",
];

/// Returns the generated build methods for SPIR-V extension instructions by walking the given
/// SPIR-V instructions `grammar`.
pub fn gen_dr_builder_debug(
    ext_name: &str,
    op_name: &str,
    kinds: &[OperandKind],
    instructions: &[Instruction],
) -> TokenStream {
    let op_name = as_ident(op_name);

    let local_elements = instructions
        .iter()
        .filter(|inst| LOCAL_DEBUG_OPS.contains(&inst.opname.as_str()))
        .map(|inst| {
            let param_list = get_param_list(&inst.operands, false, kinds);
            let arg_list = get_arg_list(&inst.operands, false, kinds);
            // Initializer list for constructing the operands parameter
            // for Instruction.
            let init_list = get_init_list(&inst.operands);
            let extras = get_push_extras(&inst.operands,
                                         kinds,
                                         quote![args]);
            let opcode = as_ident(&inst.opname);
            let name = as_ident(&inst.opname.to_snake_case());
    
            let name_id = format_ident!("{}_id", name);
    
            quote! {
                #[allow(clippy::too_many_arguments)]
                fn #name(&mut self, #(#param_list),*) -> Result<spirv::Word, dr::Error> {
                    self.#name_id(None, #(#arg_list),*)
                }

                #[allow(clippy::too_many_arguments)]
                fn #name_id(&mut self, result_id: Option<spirv::Word>, #(#param_list),*) -> Result<spirv::Word, dr::Error> {
                    let extension_set = super::ext_inst_import(self, #ext_name);

                    let result_type = self.type_void();

                    #[allow(unused_mut)]
                    let mut args = vec![#(#init_list),*];
                    #(#extras)*

                    self.ext_inst(result_type, result_id, extension_set, crate::spirv::#op_name::#opcode as spirv::Word, args)
                }
            }
        });


    // Generate build methods for all types.
    let elements = instructions.iter().filter(|inst| {
        !LOCAL_DEBUG_OPS.contains(&inst.opname.as_str())
    }).map(|inst| {
        let param_list = get_param_list(&inst.operands, false, kinds);
        let arg_list = get_arg_list(&inst.operands, false, kinds);
        // Initializer list for constructing the operands parameter
        // for Instruction.
        let init_list = get_init_list(&inst.operands);
        let extras = get_push_extras(&inst.operands,
                                     kinds,
                                     quote![inst.operands]);
        let opcode = as_ident(&inst.opname);
        let name = as_ident(&inst.opname.to_snake_case());

        let name_id = format_ident!("{}_id", name);

        quote! {
            #[allow(clippy::too_many_arguments)]
            fn #name(&mut self, #(#param_list),*) -> spirv::Word {
                self.#name_id(None, #(#arg_list),*)
            }

            #[allow(clippy::too_many_arguments)]
            fn #name_id(&mut self, result_id: Option<spirv::Word>, #(#param_list),*) -> spirv::Word {
                let extension_set = super::ext_inst_import(self, #ext_name);
                let result_type = self.type_void();

                let mut inst = dr::Instruction::new(spirv::Op::ExtInst, Some(result_type), result_id, vec![
                    dr::Operand::IdRef(extension_set),
                    dr::Operand::LiteralExtInstInteger(crate::spirv::#op_name::#opcode as spirv::Word),
                    #(#init_list),*
                ]);

                #(#extras)*

                if let Some(id) = inst.result_id {
                    self.module_mut().types_global_values.push(inst);
                    id
                } else if let Some(id) = self.dedup_insert_type(&inst) {
                    id
                } else {
                    let id = self.id();
                    inst.result_id = Some(id);
                    self.module_mut().types_global_values.push(inst);
                    id
                }
            }
        }
    });

    // Generate build methods for all types.
    let element_defs = instructions.iter().map(|inst| {
        let param_list = get_param_list(&inst.operands, false, kinds);
        let opcode = as_ident(&inst.opname);
        let name = as_ident(&inst.opname.to_snake_case());

        let comment = format!("Appends an {} instruction and returns the result id, or return the existing id if the instruction was already present.", opcode);
        let name_id = format_ident!("{}_id", name);

        let result_ty = if LOCAL_DEBUG_OPS.contains(&inst.opname.as_str()) {
            quote![Result<spirv::Word, dr::Error>]
        } else {
            quote![spirv::Word]
        };

        quote! {
            #[doc = #comment]
            #[allow(clippy::too_many_arguments)]
            fn #name(&mut self, #(#param_list),*) -> #result_ty;

            #[doc = #comment]
            #[allow(clippy::too_many_arguments)]
            fn #name_id(&mut self, result_id: Option<spirv::Word>, #(#param_list),*) -> #result_ty;
        }
    });

    let trait_name = format_ident!("{op_name}Builder");

    quote! {
        use rspirv::{spirv, dr};

        pub trait #trait_name {
            #(#element_defs)*
        }

        impl #trait_name for rspirv::dr::Builder {
            #(#elements)*
            #(#local_elements)*
        }
    }
}
