use icrate::Foundation::NSString;
use objc2::ns_string;

fn main() {
    static STRING: &NSString = ns_string!("abc");
}
