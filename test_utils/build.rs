extern crate gcc;

use std::default::Default;

fn main() {
    let config = Default::default();
    gcc::compile_library("libblock_utils.a", &config, &["block_utils.m"]);
}
