use icrate::ns_string;
use icrate::Foundation::NSString;

fn main() {
    static STRING: &NSString = ns_string!("abc");
}
