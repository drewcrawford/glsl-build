use std::env::temp_dir;
use std::path::PathBuf;
use std::str::FromStr;
use buildkit::{CompileSettingsBuilder, Configuration, PathType, SourceFileStrategy};
use glsl_build::VertexCompiler;

#[test] fn warnings() {
    let mut intermediate_path = temp_dir();
    intermediate_path.push("glsl_build");

    let mut settings = CompileSettingsBuilder::new();
    settings.source_strategy(SourceFileStrategy::SearchFromManifest(vec![PathBuf::from_str("tests/warnings").unwrap()]))
        .intermediate_path(PathType::Exact(intermediate_path))
        .configuration(Configuration::Debug);
    let settings = settings.finish();
    let o = buildkit::CompileSystem::<VertexCompiler>::build(&settings);
    assert_eq!(o.len(),1);
}