extern crate gcc;

fn main() {
    gcc::Build::new()
        .file("src/cpucounter.c")
        .flag_if_supported("-fomit-frame-pointer")
        .opt_level(3)
        .compile("cpucounter");
}
