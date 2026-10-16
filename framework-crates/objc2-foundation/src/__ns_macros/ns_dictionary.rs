//! Convenience macro for creating `NSDictionary`, but in a less
//! clunky-looking way than `NSDictionary::from_slices(&keys, &values)`.

/// Creates an immutable [`NSDictionary`] containing the arguments.
///
/// Keys and values are separated by a `=>`, and entries are separated by
/// commas (with a trailing comma being allowed).
///
/// This macro translates into a call to [`from_slices`], which means that the
/// keys and values must be behind `&`, and that you might have to specify the
/// resulting dictionary type explicitly if the compiler infers the wrong one.
///
/// Unlike [`ns_string!`], this macro creates a new dictionary every time it's
/// called (but on the other hand works with arbitrary runtime expressions).
///
/// [`NSDictionary`]: crate::NSDictionary
/// [`from_slices`]: crate::NSDictionary::from_slices
/// [`ns_string!`]: crate::ns_string
///
///
/// # Examples
///
/// Creating a dictionary containing strings.
///
/// ```
/// use objc2_foundation::{ns_dictionary, ns_string};
///
/// let error_domains = ns_dictionary! {
///     ns_string!("cocoa") => unsafe { objc2_foundation::NSCocoaErrorDomain },
///     ns_string!("posix") => unsafe { objc2_foundation::NSPOSIXErrorDomain },
///     ns_string!("osstatus") => unsafe { objc2_foundation::NSOSStatusErrorDomain },
///     ns_string!("mach") => unsafe { objc2_foundation::NSMachErrorDomain },
/// };
///
/// let error_domains_rev = ns_dictionary! {
///     unsafe { objc2_foundation::NSCocoaErrorDomain } => ns_string!("cocoa"),
///     unsafe { objc2_foundation::NSPOSIXErrorDomain } => ns_string!("posix"),
///     unsafe { objc2_foundation::NSOSStatusErrorDomain } => ns_string!("osstatus"),
///     unsafe { objc2_foundation::NSMachErrorDomain } => ns_string!("mach"),
/// };
/// ```
///
/// Using various different value types by converting them all to `AnyObject`:
///
/// ```
/// use objc2::rc::Retained;
/// use objc2::runtime::AnyObject;
/// use objc2_foundation::{
///     ns_dictionary, ns_string, NSArray, NSDictionary, NSString, NSNumber,
/// };
///
/// let plist: Retained<NSDictionary<NSString, AnyObject>> = ns_dictionary! {
///     // Explicitly dereference to AnyObject.
///     ns_string!("a string") => &**ns_string!("foo"),
///
///     // Implicitly dereference to AnyObject.
///     ns_string!("another string") => ns_string!("bar"),
///
///     // Also implicitly dereference to AnyObject, still requires `&` though.
///     ns_string!("a bool") => &NSNumber::new_bool(true),
///     ns_string!("an integer") => &NSNumber::new_i32(42),
///
///     // Nested dictionaries and arrays are possible.
///     ns_string!("an array") => &NSArray::from_slice(&[
///         ns_string!("elem1"),
///         ns_string!("elem2"),
///     ]),
///     ns_string!("a dict") => &ns_dictionary! {
///         ns_string!("key") => ns_string!("value"),
///     },
/// };
/// ```
#[macro_export]
macro_rules! ns_dictionary {
    {
        $($key:expr => $value:expr),+ $(,)?
    } => {
        $crate::NSDictionary::from_slices(
            &[$($key),*],
            &[$($value),*],
        )
    };
    {
        // Empty dictionary.
        $(,)?
    } => {
        // This requires a separate branch, otherwise the compiler can't
        // figure out the key type.
        $crate::NSDictionary::new()
    };
}

#[cfg(test)]
#[cfg(feature = "NSString")]
#[cfg(feature = "NSObject")]
mod tests {
    use objc2::rc::Retained;

    use crate::{ns_string, NSDictionary, NSObject, NSString};

    #[test]
    fn empty() {
        let dict: Retained<NSDictionary<NSString, NSObject>> = ns_dictionary! {};
        assert_eq!(dict.len(), 0);
        let dict: Retained<NSDictionary<NSObject, NSString>> = ns_dictionary! {,};
        assert_eq!(dict.len(), 0);
    }

    #[test]
    fn duplicate_keys() {
        let foo = ns_string!("foo");
        let dict = ns_dictionary! {
            foo => ns_string!("bar1"),
            foo => ns_string!("bar2"),
        };
        assert_eq!(dict.len(), 1);
        assert!(dict.objectForKey(foo).unwrap().hasPrefix(ns_string!("bar")));
    }
}
