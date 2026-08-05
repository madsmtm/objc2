//! Test assembly output of `define_class!`.
#![deny(unsafe_op_in_unsafe_fn)]
// Limit to Apple targets only, since we don't particularly care about GNUStep code-size for now.
#![cfg(target_vendor = "apple")]
// Limit to 64-bit since we don't do anything special on other targets, and the assembly files are _huge_.
#![cfg(target_pointer_width = "64")]

use objc2::define_class;
use objc2::rc::Retained;
use objc2::runtime::NSObject;

// Use `extern "C"` to keep code size a bit smaller
unsafe extern "C" {
    safe fn external(param: Option<&mut Option<Retained<NSObject>>>);
}

#[inline]
fn external_none(param: Option<&mut Retained<NSObject>>) {
    external(unsafe {
        core::mem::transmute::<
            Option<&mut Retained<NSObject>>,
            Option<&mut Option<Retained<NSObject>>>,
        >(param)
    });
}

define_class!(
    #[unsafe(super(NSObject))]
    #[name = "OutParam"]
    pub struct OutParam;

    /// For rustfmt
    impl OutParam {
        #[unsafe(method(nonnull_nonnull:))]
        fn nonnull_nonnull(param: &mut Retained<NSObject>) {
            external_none(Some(param));
        }

        #[unsafe(method(null_nonnull:))]
        fn null_nonnull(param: Option<&mut Retained<NSObject>>) {
            external_none(param);
        }

        #[unsafe(method(nonnull_null:))]
        fn nonnull_null(param: &mut Option<Retained<NSObject>>) {
            external(Some(param));
        }

        #[unsafe(method(null_null:))]
        fn null_null(param: Option<&mut Option<Retained<NSObject>>>) {
            external(param);
        }

        #[unsafe(method(two:nonnull_nonnull:))]
        fn two_nonnull_nonnull(param1: &mut Retained<NSObject>, param2: &mut Retained<NSObject>) {
            external_none(Some(param1));
            external_none(Some(param2));
        }
    }
);
