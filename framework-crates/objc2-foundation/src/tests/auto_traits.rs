#![cfg(feature = "all")] // TODO: More precise
use core::panic::{RefUnwindSafe, UnwindSafe};

use static_assertions::assert_not_impl_any;

use crate::*;
use objc2::declare_class;
use objc2::rc::Retained;
use objc2::runtime::AnyObject;

// We expect most Foundation types to be UnwindSafe and RefUnwindSafe,
// since they follow Rust's usual mutability rules (&T = immutable).
//
// A _lot_ of Objective-C code out there would be subtly broken if e.g.
// `NSString` wasn't exception safe! As an example: `-[NSArray objectAtIndex:]`
// can throw, but it is still perfectly valid to access the array after that!
//
// I'm pretty sure that even mutable classes like `NSMutableString` is still
// exception safe, since preconditions are checked before the mutation is
// executed, and more "internal" errors like Out-Of-Memory either crash the
// application, or return / throw an error value.
//
// Also note that this is still only a speed bump, not actually part of any
// unsafe contract; we can't really protect against it if something is not
// exception safe, since `UnwindSafe` is a safe trait.
fn assert_unwindsafe<T: UnwindSafe + RefUnwindSafe>() {}

fn assert_auto_traits<T: Send + Sync + UnwindSafe + RefUnwindSafe>() {
    assert_unwindsafe::<T>();
}

declare_class!(
    #[unsafe(super(NSObject))]
    #[name = "SendSyncObject"]
    struct SendSyncObject;
);

#[test]
fn test_generic_auto_traits() {
    // assert_unwindsafe::<NSArray<NSProcessInfo>>();
    // assert_unwindsafe::<Retained<NSArray<NSProcessInfo>>>();
    // assert_unwindsafe::<NSMutableArray<NSProcessInfo>>();
    // assert_unwindsafe::<Retained<NSMutableArray<NSProcessInfo>>>();
    // assert_unwindsafe::<NSDictionary<NSProcessInfo, NSProcessInfo>>();
    // assert_unwindsafe::<Retained<NSDictionary<NSProcessInfo, NSProcessInfo>>>();

    // TODO: Unpin?
    assert_not_impl_any!(NSArray<AnyObject>: Unpin);
    assert_not_impl_any!(NSMutableArray<AnyObject>: Unpin);
    assert_not_impl_any!(NSDictionary<AnyObject, AnyObject>: Unpin);

    // Collections are not Send + Sync, since they are interior mutable, i.e.
    // mutable from `&self`.
    assert_not_impl_any!(NSArray<SendSyncObject>: Send, Sync);
    assert_not_impl_any!(NSMutableArray<SendSyncObject>: Send, Sync);
    assert_not_impl_any!(NSDictionary<SendSyncObject, SendSyncObject>: Send, Sync);

    // TODO: Make these UnwindSafe?
    assert_not_impl_any!(NSDictionary<NSProcessInfo, NSProcessInfo>: UnwindSafe, RefUnwindSafe);
    assert_not_impl_any!(NSSet<NSProcessInfo>: UnwindSafe, RefUnwindSafe);
    assert_not_impl_any!(Retained<NSSet<NSProcessInfo>>: UnwindSafe, RefUnwindSafe);
    assert_not_impl_any!(NSMutableArray<NSProcessInfo>: UnwindSafe, RefUnwindSafe);
    assert_not_impl_any!(NSMutableDictionary<NSProcessInfo, NSProcessInfo>: UnwindSafe, RefUnwindSafe);
    assert_not_impl_any!(NSMutableSet<NSProcessInfo>: UnwindSafe, RefUnwindSafe);
}

#[test]
fn send_sync_unwindsafe() {
    assert_unwindsafe::<NSAttributedString>();
    assert_auto_traits::<NSComparisonResult>();
    assert_unwindsafe::<NSData>();
    // TODO: Figure out if Send + Sync is safe?
    // assert_auto_traits::<NSEnumerator2<NSProcessInfo>>();
    // assert_auto_traits::<NSFastEnumerator2<NSArray<NSProcessInfo>>>();
    assert_auto_traits::<NSError>();
    assert_auto_traits::<NSException>();
    assert_auto_traits::<CGFloat>();
    assert_auto_traits::<NSPoint>();
    assert_auto_traits::<NSRect>();
    assert_auto_traits::<NSSize>();
    assert_unwindsafe::<NSMutableAttributedString>();
    assert_unwindsafe::<NSMutableData>();
    assert_unwindsafe::<NSMutableString>();
    assert_auto_traits::<NSNumber>();
    // assert_auto_traits::<NSObject>(); // Intentional
    assert_auto_traits::<NSProcessInfo>();
    assert_auto_traits::<NSRange>();
    assert_unwindsafe::<NSString>();
    assert_unwindsafe::<MainThreadMarker>(); // Intentional
    assert_auto_traits::<NSThread>();
    assert_auto_traits::<NSUUID>();
    // assert_auto_traits::<NSValue>(); // Intentional
    assert_unwindsafe::<NSZone>(); // Intentional
}
