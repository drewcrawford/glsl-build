use glsl_build::VertexCompiler;
use buildkit::{CompileSettingsBuilder, SourceFileStrategy, Configuration, PathType};
use std::env::temp_dir;
use std::path::PathBuf;
use std::str::FromStr;

#[test] fn simple() {
    let mut intermediate_path = temp_dir();
    intermediate_path.push("glsl_build");

    let mut settings = CompileSettingsBuilder::new();
    settings.source_strategy(SourceFileStrategy::SearchFromManifest(vec![PathBuf::from_str("tests/simple/").unwrap()]))
        .intermediate_path(PathType::Exact(intermediate_path))
        .configuration(Configuration::Debug);
    let settings = settings.finish();
    let o = buildkit::CompileSystem::<VertexCompiler>::build(&settings);
   assert_eq!(o.len(),1);
}