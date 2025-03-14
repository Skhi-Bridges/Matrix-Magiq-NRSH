// Matrix-Magiq NRSH Build Script
// Ensures all required resources are properly bundled during compilation

use std::env;
use std::path::Path;

fn main() {
    // Tell Cargo to re-run this script if any of these files change
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=Cargo.lock");
    
    // Include the runtime WASM binary
    if Path::new("runtime/wasm/target/wasm32-unknown-unknown/release/nourish_runtime.wasm").exists() {
        println!("cargo:rustc-env=WASM_BINARY=1");
    } else {
        println!("cargo:rustc-env=WASM_BINARY=0");
    }
    
    // Setup for integration of error correction modules
    let out_dir = env::var("OUT_DIR").unwrap();
    println!("cargo:rustc-env=OUT_DIR={}", out_dir);
    
    // Indicate we're building for the Nourish Chain
    println!("cargo:rustc-env=CHAIN_TYPE=nrsh");
    
    // Set the version information
    println!("cargo:rustc-env=SUBSTRATE_CLI_IMPL_VERSION=1.0.0");
}
