//! Test that the syntax with fields doesn't work.
//!
//! If it does in the future, we have to make sure that the class is still a
//! ZST.
use objc2::extern_class;

extern_class!(
    #[unsafe(super(objc2::runtime::NSObject))]
    pub struct NSNumber {
        var: u32,
    }
);

fn main() {}
