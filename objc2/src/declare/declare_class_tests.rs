#![deny(deprecated)]
use crate::foundation::NSObject;
use crate::{declare_class, ClassType};

// Test that adding the `deprecated` attribute does not mean that warnings
// when using the method internally are output.
declare_class!(
    struct DeclareClassDepreactedMethod {}

    unsafe impl ClassType for DeclareClassDepreactedMethod {
        type Super = NSObject;
    }

    #[deprecated]
    unsafe impl DeclareClassDepreactedMethod {
        #[method(deprecatedOnImpl)]
        fn deprecated_on_impl() {}
    }

    unsafe impl DeclareClassDepreactedMethod {
        #[deprecated]
        #[method(deprecatedOnMethod)]
        fn deprecated_on_method() {}
    }
);

#[test]
fn test_deprecated() {
    let _cls = DeclareClassDepreactedMethod::class();
}
