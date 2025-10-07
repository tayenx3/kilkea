// build.rs
fn main() {
    let llvm_path = "C:\\Users\\DELL\\scoop\\apps\\llvm\\current";
    println!("cargo:rustc-link-search=native={}\\lib", llvm_path);
    println!("cargo:rustc-link-lib=LLVM-C");
    
    // Copy required DLLs to output directory
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let target_dir = std::path::Path::new(&out_dir)
        .ancestors().nth(3).unwrap(); // Get target/debug directory
    
    let dll_files = ["LLVM-C.dll", "LLVM.dll"]; // Add other DLLs as needed
    
    for dll in &dll_files {
        let src = format!("{}\\bin\\{}", llvm_path, dll);
        let dst = target_dir.join(dll);
        
        if std::path::Path::new(&src).exists() {
            let _ = std::fs::copy(&src, dst);
        }
    }
}