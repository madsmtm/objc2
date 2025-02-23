#![cfg(feature = "CSSearchableItemAttributeSet")]
use std::assert_eq;

use objc2::{rc::Retained, AllocAnyThread, ClassType};
use objc2_core_spotlight::CSLocalizedString;
use objc2_foundation::{ns_string, NSCopying, NSDictionary, NSMutableCopying};

// Test that NSCopying/NSMutableCopying returns NSString/NSMutableString for CSLocalizedString.
#[test]
fn copying() {
    let strings = NSDictionary::from_slices(&[ns_string!("en")], &[ns_string!("foo")]);

    unsafe {
        let strings = Retained::cast_unchecked::<NSDictionary>(strings);
        let obj = CSLocalizedString::initWithLocalizedStrings(CSLocalizedString::alloc(), &strings);
        assert_eq!(&**obj, ns_string!("foo"));

        let obj_copy = obj.copy();
        assert_eq!(obj_copy.class(), CSLocalizedString::class());
        assert_eq!(&**obj_copy, ns_string!("foo"));

        let obj_mutable_copy = obj.mutableCopy();
        assert_ne!(obj_mutable_copy.class(), CSLocalizedString::class());
        obj_mutable_copy.appendString(ns_string!("bar"));
        assert_eq!(&**obj_mutable_copy, ns_string!("foobar"));
    }
}
