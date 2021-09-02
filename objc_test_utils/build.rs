extern crate gcc;

fn main() {
    gcc::Config::new()
        .compiler("clang")
        .file("block_utils.c")
        .flag("-fblocks")
        .compile("libblock_utils.a");
}
