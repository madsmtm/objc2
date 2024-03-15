#![allow(unused_variables)]
use icrate::Foundation::ns_string;

fn main() {
    let s: &str = "abc";
    let _ = ns_string!(s);
}
