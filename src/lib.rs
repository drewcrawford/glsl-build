/*!
BuildKit implementation for glslc compiler
*/

use buildkit::{CompileStep, Configuration, CompileSystem};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::ffi::OsStr;
use std::fs::{OpenOptions};
use std::io::{Read, Write, Seek, SeekFrom};

///One implementation in many types
fn compile_one(path: &Path, intermediate_dir: &Path, configuration: &Configuration, dependency_path: &Path) -> PathBuf {
    let output_file = buildkit::suggest_intermediate_file(path, intermediate_dir.to_owned(), OsStr::new("spirv"));
    let mut cmd = Command::new("glslc");
    cmd.arg(path)
        .arg("-MD")
        .args(["-MF",dependency_path.to_str().unwrap()])
        .args(["-o",output_file.to_str().unwrap()])
        .arg("--target-env=vulkan1.2");

    match configuration {
        Configuration::Debug => {
            cmd.arg("-g") //generate debug info
                .arg("-O0"); //no optimization
        }
        Configuration::Release => {
            cmd.arg("-O");
        }
    }
    let status = cmd
        .spawn().unwrap().wait().unwrap();

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
    if !status.success() {
        panic!("glsl compiler reported an error");
    }
    output_file
}

///Compiles vertex shaders
pub struct VertexCompiler;
impl CompileStep for VertexCompiler {
    const SOURCE_FILE_EXTENSION: &'static str = "vert";
    fn compile_one(path: &Path, intermediate_dir: &Path, configuration: &Configuration, dependency_path: &Path) -> PathBuf {
        compile_one(path,intermediate_dir,configuration,dependency_path)
    }
}

///Compiles fragment shaders
pub struct FragmentCompiler;
impl CompileStep for FragmentCompiler {
    const SOURCE_FILE_EXTENSION: &'static str = "frag";
    fn compile_one(path: &Path, intermediate_dir: &Path, configuration: &Configuration, dependency_path: &Path) -> PathBuf {
        compile_one(path,intermediate_dir,configuration,dependency_path)
    }
}

///Compiles all shader types
pub fn build_rs(exe_path: PathBuf) -> Vec<PathBuf> {
    CompileSystem::<VertexCompiler>::build_rs(exe_path)
}