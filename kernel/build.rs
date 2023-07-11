fn main() {
    let arch = build_target::target_arch().unwrap();
    let linker_path = match arch {
        build_target::Arch::X86_64 => "arch/x86_64/linker.ld",
        _ => "",
    };

    // Tell cargo to pass the linker script to the linker..
    println!("cargo:rustc-link-arg=-T{}", linker_path);
    // ..and to re-run if it changes.
    println!("cargo:rerun-if-changed={}", linker_path);
}
