//! Check that Apple's APIs are internally bounds checked.
//!
//! This is required for us to be able to mark them as safe.

#![cfg(target_vendor = "apple")]
#![cfg(feature = "exception")] // For `should_panic` to work

use core::{panic::AssertUnwindSafe, ptr::NonNull};
use std::string::ToString;

use objc2::AnyThread;
use objc2_foundation::{
    NSArray, NSAttributedString, NSDictionary, NSException, NSIndexSet, NSMethodSignature,
    NSMutableArray, NSNotFound, NSObject, NSOrderedSet, NSPointerArray, NSRange, NSString,
};

/// Check that the given error message was thrown by the closure.
#[track_caller]
fn assert_throws<R: std::fmt::Debug>(message: &str, f: impl FnOnce() -> R) {
    let f = AssertUnwindSafe(f);
    if cfg!(feature = "catch-all") {
        let err = std::panic::catch_unwind(f).unwrap_err();

        if let Some(s) = err.downcast_ref::<&str>() {
            assert!(s.contains(message), "{s:?} did not contain {message:?}");
        } else if let Some(s) = err.downcast_ref::<std::string::String>() {
            assert!(s.contains(message), "{s:?} did not contain {message:?}");
        } else {
            panic!("Caught panic with unknown type");
        }
    } else {
        let exc = objc2::exception::catch(f).unwrap_err().unwrap();
        let exc = exc.downcast_ref::<NSException>().unwrap();
        let reason = exc.reason().unwrap().to_string();
        assert!(
            reason.contains(message),
            "{}: {reason:?} did not contain {message:?}",
            exc.name(),
        );
    }
}

#[test]
fn array() {
    let arr = NSArray::<NSObject>::new();
    assert_throws("index 0 beyond bounds", || arr.objectAtIndex(0));
    assert_throws("index 0 beyond bounds", || arr.objectAtIndexedSubscript(0));

    let arr = NSArray::from_retained_slice(&[NSObject::new(), NSObject::new()]);
    assert_throws("index 100 beyond bounds [0 .. 1]", || {
        arr.objectAtIndex(100)
    });

    let arr = NSMutableArray::<NSObject>::new();
    assert_throws("index 100 beyond bounds", || {
        arr.insertObject_atIndex(&NSObject::new(), 100)
    });
    assert_throws("range {0, 1} extends beyond bounds", || {
        arr.removeObjectAtIndex(0)
    });
    assert_throws("index 100 beyond bounds", || {
        arr.replaceObjectAtIndex_withObject(100, &NSObject::new())
    });
}

#[test]
fn attributed_string() {
    let arr = unsafe {
        NSAttributedString::initWithString_attributes(
            NSAttributedString::alloc(),
            &NSString::from_str("foo"),
            Some(&NSDictionary::new()),
        )
    };
    let mut range = NSRange::new(42, 42);

    assert_throws("Out of bounds", || unsafe {
        arr.attributesAtIndex_effectiveRange(100, &mut range)
    });

    assert_eq!(range, NSRange::new(42, 42));
}

#[test]
fn index_set() {
    let set = NSIndexSet::new();
    assert_eq!(set.firstIndex(), NSNotFound as usize);
}

#[test]
fn method_signature() {
    let sig = c"c@:@";
    let sig = NonNull::new(sig.as_ptr().cast_mut()).unwrap();
    let sig = unsafe { NSMethodSignature::signatureWithObjCTypes(sig) }.unwrap();
    assert_throws("index (100) out of bounds [0, 2]", || {
        sig.getArgumentTypeAtIndex(100)
    });
}

#[test]
fn ordered_set() {
    let set = NSOrderedSet::<NSObject>::new();
    assert_throws("index 0 beyond bounds for empty ordered set", || {
        set.objectAtIndex(0)
    });
}

#[test]
fn pointer_array() {
    let arr = NSPointerArray::new();
    assert_throws(
        "attempt to access pointer at index 0 beyond bounds 0",
        || arr.pointerAtIndex(0),
    );
}

#[test]
fn string() {
    let s = NSString::new();
    assert_throws("Range or index out of bounds", || s.characterAtIndex(0));
}
