extern crate gcc;

fn main() {
    gcc::compile_library("libblock_utils.a", &["block_utils.m"]);
}
