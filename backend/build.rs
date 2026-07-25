use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);

    // Collect all .proto files
    let proto_files: Vec<String> = {
        let mut files = Vec::new();
        let entries = std::fs::read_dir("proto/aeroxe/v1")?;
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "proto") {
                files.push(path.display().to_string());
            }
        }
        files
    };

    tonic_build::configure()
        .build_server(false) // We're using Axum, not tonic gRPC
        .build_client(true)
        .out_dir(&out_dir)
        .compile(&proto_files, &["proto/"])?;

    // Make generated code available via include!
    println!("cargo:rerun-if-changed=proto/**/*.proto");

    Ok(())
}
