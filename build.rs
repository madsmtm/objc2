extern crate gcc;

fn main() {
    gcc::compile_library("libexception.a", &["extern/exception.m"]);
}
