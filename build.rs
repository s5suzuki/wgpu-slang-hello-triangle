use std::path::{Path, PathBuf};

fn compile(src: &PathBuf, dst: &Path, stage: &str, entry_function: &str) {
    let status = std::process::Command::new("slangc")
        .args([
            src.to_str().unwrap(),
            "-target",
            "spirv",
            "-profile",
            "spirv_1_6",
            "-entry",
            entry_function,
            "-stage",
            stage,
            "-o",
            dst.to_str().unwrap(),
        ])
        .status()
        .unwrap_or_else(|_| panic!("Failed to run slangc for {:?}", src));
    if !status.success() {
        panic!("Slang compilation failed for {:?}", src);
    }
}

fn main() {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());

    let shader_path = manifest_dir.join("shaders").join("shader.slang");
    println!("cargo:rerun-if-changed={}", shader_path.as_path().display());

    let out_dir = manifest_dir.join("src").join("shaders");
    std::fs::create_dir_all(&out_dir).unwrap();

    compile(&shader_path, &out_dir.join("vert.spv"), "vertex", "vs_main");
    compile(
        &shader_path,
        &out_dir.join("frag.spv"),
        "fragment",
        "fs_main",
    );
}
