use std::{fs, path::Path};

pub fn build_main() {
    println!("cargo:warning=file: {:?}", Path::new(file!()));
    let mut linker_path = fs::canonicalize(Path::new(file!())).unwrap();

    linker_path.pop();
    linker_path.pop();
    linker_path.push("linker.ld");

    let linker_path = linker_path.to_str().unwrap();

    // Tell cargo to pass the linker script to the linker..
    println!("cargo:rustc-link-arg=-T{}", linker_path);
    // ..and to re-run if it changes.
    println!("cargo:rerun-if-changed={}", linker_path);
}
