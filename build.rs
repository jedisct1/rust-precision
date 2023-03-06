#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
use rustc_version::{version_meta, Channel};

#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
fn asm_detect() {
    let using_nightly = version_meta().unwrap().channel == Channel::Nightly;
    let asm_capable_target = cfg!(any(
        target_arch = "x86",
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "riscv64",
    ));
    if using_nightly && asm_capable_target {
        println!("cargo:rustc-cfg=asm");
    } else {
        cc::Build::new()
            .file("src/cpucounter.c")
            .flag_if_supported("-fomit-frame-pointer")
            .opt_level(3)
            .compile("cpucounter");
    }
}

fn main() {
    #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    asm_detect();
}
