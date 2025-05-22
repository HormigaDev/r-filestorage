use std::{fs, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/generated");

    // 🔥 Crear carpeta si no existe
    fs::create_dir_all(&out_dir)?;

    tonic_build::configure()
        .out_dir(&out_dir)
        .compile(&["proto/storage.proto"], &["proto"])?;

    println!("cargo:rerun-if-changed=proto/storage.proto");
    println!("cargo:rerun-if-changed=proto");

    Ok(())
}
