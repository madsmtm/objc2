pub use self::comparison_result::NSComparisonResult;
pub use self::copying::{NSCopying, NSMutableCopying};
pub use self::enumerator::{NSEnumerator, NSFastEnumeration, NSFastEnumerator};
pub use self::geometry::{CGFloat, CGPoint, CGRect, CGSize};
pub use self::range::NSRange;
pub use self::thread::{is_main_thread, is_multi_threaded, MainThreadMarker};

mod array;
mod attributed_string;
mod bundle;
mod comparison_result;
mod copying;
mod data;
mod dictionary;
mod enumerator;
mod error;
mod exception;
mod geometry;
mod lock;
mod mutable_array;
mod mutable_attributed_string;
mod mutable_data;
mod mutable_dictionary;
mod mutable_set;
mod mutable_string;
mod number;
mod process_info;
mod range;
mod set;
mod string;
mod thread;
mod uuid;
mod value;

#[cfg(test)]
mod tests {
    use core::panic::{RefUnwindSafe, UnwindSafe};

    use crate::Foundation::*;
    use objc2::rc::{Id, Owned, Shared};

    // We expect most Foundation types to be UnwindSafe and RefUnwindSafe,
    // since they follow Rust's usual mutability rules (&T = immutable).
    //
    // A _lot_ of Objective-C code out there would be subtly broken if e.g.
    // `NSString` wasn't exception safe!
    // As an example: -[NSArray objectAtIndex:] can throw, but it is still
    // perfectly valid to access the array after that!
    //
    // Note that e.g. `&mut NSMutableString` is still not exception safe, but
    // that is the entire idea of `UnwindSafe` (that if the object could have
    // been mutated, it is not exception safe).
    //
    // Also note that this is still just a speed bump, not actually part of
    // any unsafe contract; we really protect against it if something is not
    // exception safe, since `UnwindSafe` is a safe trait.
    fn assert_unwindsafe<T: UnwindSafe + RefUnwindSafe>() {}

    fn assert_auto_traits<T: Send + Sync + UnwindSafe + RefUnwindSafe>() {
        assert_unwindsafe::<T>();
    }

    #[test]
    fn send_sync_unwindsafe() {
        assert_auto_traits::<NSArray<NSString, Shared>>();
        assert_auto_traits::<NSArray<NSString, Owned>>();
        assert_auto_traits::<Id<NSArray<NSString, Shared>, Shared>>();
        assert_auto_traits::<Id<NSArray<NSString, Owned>, Shared>>();
        assert_auto_traits::<Id<NSArray<NSString, Shared>, Owned>>();
        assert_auto_traits::<Id<NSArray<NSString, Owned>, Owned>>();

        assert_auto_traits::<NSAttributedString>();
        assert_auto_traits::<NSComparisonResult>();
        assert_auto_traits::<NSData>();
        assert_auto_traits::<NSDictionary<NSString, NSString>>();
        assert_auto_traits::<NSSet<NSString, Shared>>();
        assert_auto_traits::<NSSet<NSString, Owned>>();
        assert_auto_traits::<Id<NSSet<NSString, Shared>, Shared>>();
        assert_auto_traits::<Id<NSSet<NSString, Owned>, Shared>>();
        assert_auto_traits::<Id<NSSet<NSString, Shared>, Owned>>();
        assert_auto_traits::<Id<NSSet<NSString, Owned>, Owned>>();
        // TODO: Figure out if Send + Sync is safe?
        // assert_auto_traits::<NSEnumerator<NSString>>();
        // assert_auto_traits::<NSFastEnumerator<NSArray<NSString, Shared>>>();
        assert_auto_traits::<NSError>();
        assert_auto_traits::<NSException>();
        assert_auto_traits::<CGFloat>();
        assert_auto_traits::<NSPoint>();
        assert_auto_traits::<NSRect>();
        assert_auto_traits::<NSSize>();
        assert_auto_traits::<NSMutableArray<NSString, Shared>>();
        assert_auto_traits::<NSMutableAttributedString>();
        assert_auto_traits::<NSMutableData>();
        assert_auto_traits::<NSMutableDictionary<NSString, NSString>>();
        assert_auto_traits::<NSMutableSet<NSString, Shared>>();
        assert_auto_traits::<NSMutableString>();
        assert_auto_traits::<NSNumber>();
        // assert_auto_traits::<NSObject>(); // Intentional
        assert_auto_traits::<NSProcessInfo>();
        assert_auto_traits::<NSRange>();
        assert_auto_traits::<NSString>();
        assert_unwindsafe::<MainThreadMarker>(); // Intentional
        assert_auto_traits::<NSThread>();
        #[cfg(not(macos_10_7))]
        assert_auto_traits::<NSUUID>();
        // assert_auto_traits::<NSValue>(); // Intentional
        assert_unwindsafe::<NSZone>(); // Intentional
    }
}
