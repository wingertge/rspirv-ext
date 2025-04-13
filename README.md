rspirv-ext
======

An *extension* to [rspirv](https://github.com/gfx-rs/rspirv/tree/master), adding builder methods for
several SPIR-V *extensions*. Uses a heavily stripped down and modified version of `rspirv-autogen`.

Documentation
-------------

In total rspirv-ext APIs contains:

* The [SPIR-V extension headers][doc-headers] (all extension structs, enums, and constants)
* A [data representation][doc-dr] of SPIR-V extension methods for the `rspirv` builder
* A [structured representation][doc-sr] of the debug extensions, automatically handling the conversion of all constant arguments to deduplicated `OpConstant` and `OpString` instructions

The Khronos SPIR-V [JSON grammar][json-grammar] is leveraged to generate parts
of the source code using [`rspirv-autogen-ext`][autogen].

Please see the links to docs.rs for detailed documentation.

Status
------

Only includes four of the extensions right now and mainly targets the `Shader.DebugInfo.100` extension,
since it's a lot more complex than most.

Directory Organization
----------------------

There are multiple crates inside this repo:

* `autogen/`: Crate to generate various Rust code snippets used in the modules
  in `src/spirv/` and `src/dr/`, from the extension's JSON grammar. If you are not
  modifying `src/spirv/` or `src/dr/`, you don't need to care about this directory.
* `src/`: The `rspirv-ext` crate.

Build
-----

```sh
git clone https://github.com/wingertge/rspirv-ext.git /path/to/rspirv-ext
```

If you just want to compile and use the `rspirv-ext` crate:

```sh
cd /path/to/rspirv-ext
cargo build
```

If you want to refresh the `rspirv-ext` crate with new code
snippets generated from SPIR-V's JSON grammar:

```sh
cd /path/to/rspirv-ext
# Clone the SPIRV-Headers repo
git submodule update --init
cargo run -p rspirv-autogen-ext
```

[autogen]: ./autogen/README.md
[doc-headers]: https://docs.rs/rspirv-ext/*/rspirv-ext/spirv/index.html
[json-grammar]: https://github.com/KhronosGroup/SPIRV-Headers/tree/master/include/spirv
[doc-dr]: https://docs.rs/rspirv-ext/*/rspirv-ext/dr/index.html
[doc-sr]: https://docs.rs/rspirv-ext/*/rspirv-ext/sr/index.html
