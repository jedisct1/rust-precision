extern crate gcc;

fn main() {
    gcc::Build::new()
        .file("src/cpucounter.c")
        .flag("-fomit-frame-pointer")
        .opt_level(2)
        .compile("cpucounter");
}
