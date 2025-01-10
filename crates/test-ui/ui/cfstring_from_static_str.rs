//! Test that a `'static` str used in `CFString` can't later be modified.
use objc2_core_foundation::CFString;

fn main() {
    // Create a `&'static mut str`.
    let s = String::from("xyz").leak();

    // Construct a `CFString` from it.
    let cf = CFString::from_static_str(s);
    assert_eq!(cf.to_string(), "xyz");

    // Modify the string.
    s.make_ascii_uppercase();

    // This would be invalid, since CFString is expected to be immutable.
    assert_eq!(cf.to_string(), "XYZ");
}
