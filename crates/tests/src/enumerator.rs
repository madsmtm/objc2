use objc2_foundation::{NSEnumerator, NSObject};

// We mark `new` safe, but it actually panics.
#[test]
#[cfg_attr(not(feature = "catch-all"), ignore = "aborts the test")]
#[should_panic = "NSInvalidArgumentException"]
fn empty() {
    let enumerator = NSEnumerator::<NSObject>::new();
    // *** -[NSEnumerator nextObject]: method sent to an instance (0xcafebabe) of an abstract class.  Create a concrete instance!
    assert_eq!(enumerator.iter().count(), 0);
}
