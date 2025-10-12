use std::env;
use std::path::Path;









fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR")?;
    let cargo_manifest_path = Path::new(&cargo_manifest_dir);

    println!("cargo:rerun-if-changed=build.rs"); 
    eprintln!("DEBUG: build.rs started.");



    Ok(())
}