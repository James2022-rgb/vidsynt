use std::env;
use std::path::PathBuf;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let parent_crate_dir = PathBuf::from(&crate_dir).parent().unwrap().to_path_buf();
    let output_file = PathBuf::from(&crate_dir)
        .join("include")
        .join("vidsynt.h");

    println!("cargo:rerun-if-changed=../src/ffi");

    let mut config = cbindgen::Config::default();
    config.language = cbindgen::Language::C;
    config.include_guard = Some("VIDSYNT_H".to_string());
    config.header = Some("/**\n * vidsynt C API\n * \n * H.265/HEVC bitstream parser library\n * \n * This is an auto-generated header file from the Rust vidsynt library.\n */".to_string());
    config.include_version = true;
    config.namespace = None;
    config.sys_includes = vec!["stdint.h".to_string(), "stdbool.h".to_string()];
    config.includes = vec![];
    config.line_length = 100;
    config.tab_width = 4;
    config.documentation = true;
    config.documentation_style = cbindgen::DocumentationStyle::C99;
    config.style = cbindgen::Style::Both;
    config.cpp_compat = true;

    // Export specific enums even if not used in function signatures
    config.export.include = vec!["VidsyntHevcNaluType".to_string()];

    cbindgen::Builder::new()
        .with_crate(parent_crate_dir)
        .with_config(config)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(&output_file);

    println!("cargo:warning=Generated C header at: {:?}", output_file);
}
