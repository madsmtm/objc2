//! Test that implementing Drop for a class created with extern_class! fails.
use objc2::extern_class;
use objc2::runtime::NSObject;

extern_class!(
    #[unsafe(super(NSObject))]
    struct MyClass;
);

impl Drop for MyClass {
    fn drop(&mut self) {}
}

fn main() {}
