use std::path::PathBuf;
use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Collect all .proto files recursively under proto/
    let proto_files: Vec<PathBuf> = WalkDir::new("proto")
        .into_iter()
        .filter_map(|entry| {
            let path = entry.ok()?.path().to_path_buf();
            if path.extension().map_or(false, |ext| ext == "proto") {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir("src/proto") // or your desired output path
        .compile(&proto_files, &["proto"])?; // "proto" is the root of includes

    Ok(())
}
