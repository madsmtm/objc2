extern crate cc;

fn main() {
    cc::Build::new()
        .compiler("clang")
        .file("extern/block_utils.c")
        .flag("-fblocks")
        .compile("libblock_utils.a");
}
