use objc2_foundation::{ns_string, NSString};

fn main() {
    static STRING: &NSString = ns_string!("abc");
}
