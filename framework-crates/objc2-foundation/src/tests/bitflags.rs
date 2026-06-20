#![cfg(feature = "NSObjCRuntime")]
use crate::{NSEnumerationOptions, NSUInteger};

#[test]
fn externally_defined_flags() {
    assert_eq!(NSEnumerationOptions::all().0, NSUInteger::MAX);
}
