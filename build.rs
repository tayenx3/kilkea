use std::env;
use std::fs;
use std::path::Path;

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn main() {
    println!("cargo:rerun-if-changed=bundles/");
    
    let out_dir = env::var("OUT_DIR").unwrap();
    let bundle_dest = Path::new(&out_dir).join("bundles");
    
    // Copy bundled tools to output directory
    if Path::new("bundles").exists() {
        if let Err(e) = copy_dir_all("bundles", &bundle_dest) {
            eprintln!("Warning: Could not copy bundles: {}", e);
        }
    }
    
    println!("cargo:rustc-env=BUNDLE_DIR={}", bundle_dest.display());
}