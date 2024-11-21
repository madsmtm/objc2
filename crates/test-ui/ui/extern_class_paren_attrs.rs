//! Test using attributes with parentheses in extern_class!
use objc2::extern_class;
use objc2::runtime::NSObject;

extern_class!(
    #[unsafe(super(NSObject))] // Should succeed
    // But the rest should fail
    #[thread_kind(i32)]
    #[name(i32)]
    #[ivars(i32)]
    struct Parentheses;
);

fn main() {}
