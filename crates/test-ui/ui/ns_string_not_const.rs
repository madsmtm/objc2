use objc2::ns_string;

fn main() {
    let s: &str = "abc";
    let _ = ns_string!(s);
}
