Rust build.rs integration for [glslc](https://github.com/google/shaderc).

Based on [buildkit](https://github.com/drewcrawford/buildkit), the world's tiniest buildsystem,
this script lets you compile GLSL shaders to SPIR-V with cargo as part of your Rust build process.

This crate might be compared to [shaderc](https://docs.rs/shaderc/0.6.2/shaderc/). Differences include:

1.  No support for compiling shaderc from source; you must have it installed already.  On the plus side, this means
    it's fast to use.
2.  Support for [buildkit](https://github.com/drewcrawford/buildkit) features, such as skipping compiles if no files changed.
3.  Free for noncommercial and "small commercial" use.

This crate provides separate compilers for vertex (`VertexCompiler`) and fragment (`FragmentCompiler`) shaders.  Other options might be investigated in the future.

# Usage

1.  Install glslc to your path
2.  In `build.rs`, call `build_rs`.  This will internally call `CompileSystem::build_rs` on all supported compiler types.