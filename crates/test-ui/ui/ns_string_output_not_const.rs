use icrate::Foundation::NSString;
use icrate::ns_string;

fn main() {
    static STRING: &NSString = ns_string!("abc");
}
