/*!
Rust build.rs integration for [glslc](https://github.com/google/shaderc).

Based on [buildkit](https://github.com/drewcrawford/buildkit), the world's tiniest buildsystem,
this script lets you compile GLSL shaders to SPIR-V with cargo as part of your Rust build process.

This crate might be compared to [shaderc](https://docs.rs/shaderc/0.6.2/shaderc/). Differences include:

1.  No support for compiling shaderc from source; you must have it installed already.  On the plus side, this means
    it's fast to use.
2.  Support for [buildkit](https://github.com/drewcrawford/buildkit) features, such as skipping compiles if no files changed.
3.  Free for noncommercial and "small commercial" use.

This crate provides separate compilers for vertex ([VertexCompiler]) and fragment ([FragmentCompiler]) shaders.  Other options might be investigated in the future.

# Usage

1.  Install glslc to your path
2.  In `build.rs`, call [build_rs].  This will internally call [CompileSystem::build_rs] on all supported compiler types.
*/

use buildkit::{CompileStep, Configuration, CompileSystem};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::ffi::OsStr;
use std::fs::{OpenOptions};
use std::io::{Read, Write, Seek, SeekFrom};

///One implementation in many types
fn compile_one<'a>(path: &Path, intermediate_dir: &Path, configuration: &Configuration, dependency_path: &Path,out_extension: &OsStr, flags: impl Iterator<Item=&'a str>) -> PathBuf {
    let output_file = buildkit::suggest_intermediate_file(path, intermediate_dir.to_owned(), out_extension);
    let mut collect_flags = Vec::from_iter(flags);
    if !collect_flags.iter().any(|e| e.starts_with("--target-env")) {
        collect_flags.push("--target-env=vulkan1.3");
    }
    let mut cmd = Command::new("glslc");
    cmd.arg(path)
        .arg("-MD")
        .args(["-MF",dependency_path.to_str().unwrap()])
        .args(["-o",output_file.to_str().unwrap()])
        .args(collect_flags);

    match configuration {
        Configuration::Debug => {
            cmd.arg("-g") //generate debug info
                .arg("-O0"); //no optimization
        }
        Configuration::Release => {
            cmd.arg("-O");
        }
    }
    let output = cmd
        .output().unwrap();

    //print stdout and stderr
    let output_str = String::from_utf8(output.stdout).unwrap();
    print!("{}",output_str);
    let err_str = String::from_utf8(output.stderr).unwrap();
    for line in err_str.lines() {
        if line.contains("warning:") {
            eprintln!("cargo:warning={line}")
        }
        else {
            eprintln!("{line}");
        }
    }


    /*Need to work around https://github.com/google/shaderc/issues/1220
    The assumption we make here is nobody intended to have a file with spaces.
    This may not be, strictly speaking, true, but hopefully upstream will fix this
    and we will remove the hack.
     */
    let mut file = OpenOptions::new().read(true).write(true).open(dependency_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents = contents.replace(r#"\"#, r#"\\"#);
    file.seek(SeekFrom::Start(0)).unwrap();
    file.write_fmt(format_args!("{}",contents)).unwrap();
    file.flush().unwrap();
    if !output.status.success() {
        panic!("glsl compiler reported an error");
    }
    output_file
}

///Compiles vertex shaders
pub struct VertexCompiler;
impl CompileStep for VertexCompiler {
    const SOURCE_FILE_EXTENSION: &'static str = "vert";
    fn compile_one<'a>(path: &Path, intermediate_dir: &Path, configuration: &Configuration, dependency_path: &Path, flags: impl Iterator<Item=&'a str>) -> PathBuf {
        compile_one(path,intermediate_dir,configuration,dependency_path,&OsStr::new("vert.spirv"), flags)
    }
}

///Compiles fragment shaders
pub struct FragmentCompiler;
impl CompileStep for FragmentCompiler {
    const SOURCE_FILE_EXTENSION: &'static str = "frag";
    fn compile_one<'a>(path: &Path, intermediate_dir: &Path, configuration: &Configuration, dependency_path: &Path, flags: impl Iterator<Item=&'a str>) -> PathBuf {
        compile_one(path,intermediate_dir,configuration,dependency_path,&OsStr::new("frag.spirv"), flags)
    }
}

pub type VertexCompileSystem = CompileSystem<VertexCompiler>;
pub type FragmentCompileSystem = CompileSystem<FragmentCompiler>;

///Compiles all shader types
pub fn build_rs(exe_path: PathBuf) -> Vec<PathBuf> {
    let mut vertex_paths = CompileSystem::<VertexCompiler>::build_rs(exe_path.clone());
    vertex_paths.append(&mut CompileSystem::<FragmentCompiler>::build_rs(exe_path));
    vertex_paths
}