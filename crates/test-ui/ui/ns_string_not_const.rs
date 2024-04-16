#![allow(unused_variables)]
use objc2_foundation::ns_string;

fn main() {
    let s: &str = "abc";
    let _ = ns_string!(s);
}
