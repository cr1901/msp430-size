use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

fn main() {
    // Copy `memory.x` to OUT_DIR.
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let memory_x_path: PathBuf = [
        &*env::var("CARGO_MANIFEST_DIR").unwrap(),
        "..",
        &[
            "memory-",
            &*env::var("CARGO_CFG_TARGET_ARCH").unwrap(),
            ".x",
        ]
        .join(""),
    ]
    .iter()
    .collect();

    let mut inp_memory_x = Vec::new();
    File::open(memory_x_path)
        .unwrap()
        .read_to_end(&mut inp_memory_x)
        .unwrap();

    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(&inp_memory_x)
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    // Rebuild when `memory.x` changes.
    println!("cargo:rerun-if-changed=memory.x");

    // Set link script.
    if env::var_os("CARGO_CFG_TARGET_ARCH").unwrap() != "msp430" {
        println!("cargo:rustc-link-arg=-Tlink.x");
    }
}
