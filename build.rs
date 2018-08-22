extern crate gcc;
extern crate rustc_version;

use rustc_version::{version_meta, Channel};

fn main() {
    let using_nightly = version_meta().unwrap().channel == Channel::Nightly;
    let asm_capable_target = cfg!(not(any(
        all(target_os = "nacl", target_arch = "le32"),
        target_arch = "asmjs",
        target_arch = "wasm32"
    )));
    if using_nightly && asm_capable_target {
        println!("cargo:rustc-cfg=asm");
    } else {
        gcc::Build::new()
            .file("src/cpucounter.c")
            .flag_if_supported("-fomit-frame-pointer")
            .opt_level(3)
            .compile("cpucounter");
    }
}
