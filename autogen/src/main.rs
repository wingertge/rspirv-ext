#![recursion_limit = "128"]

mod dr;
mod header;
mod structs;
mod utils;

use std::{
    env, fs,
    io::Write,
    path::{Path, PathBuf},
    process,
};
use utils::write_autogen_comment;

fn write(path: &Path, contents: impl ToString) {
    let mut f = fs::File::create(path).unwrap_or_else(|_| panic!("cannot open file: {:?}", path));
    write_autogen_comment(&mut f);
    write!(f, "{}", contents.to_string()).unwrap()
}

fn write_formatted(path: &Path, contents: impl ToString) {
    write(path, contents);
    match process::Command::new("rustfmt").arg(path).status() {
        Ok(status) if !status.success() => {
            println!("cargo:warning=failed to rustfmt {:?}", path);
        }
        Ok(_) => {}
        Err(_) => {
            println!("cargo:warning=failed to execute rustfmt");
        }
    };
}

fn main() {
    // Path to the SPIR-V core grammar file.
    let env_var = env::var("CARGO_MANIFEST_DIR").unwrap();
    let autogen_src_dir = PathBuf::from(&env_var);

    if !autogen_src_dir.join("external/SPIRV-Headers").exists() {
        panic!("SPIRV-Headers missing - please checkout using git submodule");
    }

    let extended_instruction_sets = [
        ("GLSL.std.450", "GLOp", true, "https://www.khronos.org/registry/spir-v/specs/unified1/GLSL.std.450.html"),
        ("OpenCL.std.100", "CLOp", true, "https://www.khronos.org/registry/spir-v/specs/unified1/OpenCL.ExtendedInstructionSet.100.html"),
        ("NonSemantic.DebugPrintF", "DebugPrintFOp", false, "https://github.com/KhronosGroup/Vulkan-ValidationLayers/blob/master/docs/debug_printf.md"),
        
    ];
    let extended_instruction_sets = extended_instruction_sets.map(|(ext, op, with_result, url)| {
        let grammar: structs::ExtInstSetGrammar = serde_json::from_str(
            &std::fs::read_to_string(autogen_src_dir.join(format!(
                "external/SPIRV-Headers/include/spirv/unified1/extinst.{}.grammar.json",
                ext.to_lowercase()
            )))
            .unwrap(),
        )
        .unwrap();
        (ext, op, url, with_result, grammar)
    });

    // Extended instruction sets
    for (ext, op_name, url, with_result, grammar) in extended_instruction_sets {
        let header = header::gen_spirv_ext_header(op_name, url, &grammar);
        write_formatted(
            &autogen_src_dir.join(format!(
                "../src/spirv/autogen_{}.rs",
                ext.replace(".", "_").to_lowercase()
            )),
            header,
        );

        let builder = dr::gen_dr_builder_ext(
            ext,
            op_name,
            &grammar.operand_kinds,
            &grammar.instructions,
            with_result,
        );
        write_formatted(
            &autogen_src_dir.join(format!(
                "../src/dr/autogen_{}.rs",
                ext.replace(".", "_").to_lowercase()
            )),
            builder,
        );
    }

    let (ext, op_name, url) = ("NonSemantic.Shader.DebugInfo.100", "DebugInfoOp", "https://github.khronos.org/SPIRV-Registry/nonsemantic/NonSemantic.Shader.DebugInfo.100.html");

    let grammar: structs::ExtInstSetGrammar = serde_json::from_str(
        &std::fs::read_to_string(autogen_src_dir.join(format!(
            "external/SPIRV-Headers/include/spirv/unified1/extinst.{}.grammar.json",
            ext.to_lowercase()
        )))
        .unwrap(),
    )
    .unwrap();
    let header = header::gen_spirv_ext_header(op_name, url, &grammar);
    write_formatted(
        &autogen_src_dir.join(format!(
            "../src/spirv/autogen_{}.rs",
            ext.replace(".", "_").to_lowercase()
        )),
        header,
    );

    let builder = dr::gen_dr_builder_debug(
        ext,
        op_name,
        &grammar.operand_kinds,
        &grammar.instructions,
    );
    write_formatted(
        &autogen_src_dir.join(format!(
            "../src/dr/autogen_{}.rs",
            ext.replace(".", "_").to_lowercase()
        )),
        builder,
    );
}
