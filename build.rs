extern crate gcc;

fn main() {
    gcc::Build::new().file("src/cpucounter.c").compile("cpucounter");
}
