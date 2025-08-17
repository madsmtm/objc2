//! Define two classes in the same module with the same name, and verify that
//! the linker yells at us when doing so.
use objc2::{define_class, ClassType};
use objc2::runtime::{AnyClass, NSObject};

fn class1() -> &'static AnyClass {
    define_class!(
        #[unsafe(super = NSObject)]
        struct MustBeUnique;
    );

    MustBeUnique::class()
}

fn class2() -> &'static AnyClass {
    define_class!(
        #[unsafe(super = NSObject)]
        struct MustBeUnique;
    );

    MustBeUnique::class()
}

fn main() {
    class1();
    class2();
}
