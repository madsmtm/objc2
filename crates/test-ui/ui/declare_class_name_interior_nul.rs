//! Ensure that creating class names containing NUL bytes fail.
use objc2::{define_class, ClassType};
use objc2::runtime::NSObject;

define_class!(
    #[name = "NulAtEnd\0"]
    #[unsafe(super = NSObject)]
    struct NulAtEnd;
);

define_class!(
    #[name = "NulIn\0Middle"]
    #[unsafe(super = NSObject)]
    struct NulInMiddle;
);

define_class!(
    #[name = "\0NulAtStart"]
    #[unsafe(super = NSObject)]
    struct NulAtStart;
);

fn main() {
    NulAtEnd::class();
    NulInMiddle::class();
    NulAtStart::class();
}
