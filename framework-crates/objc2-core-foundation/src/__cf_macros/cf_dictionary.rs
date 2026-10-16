//! Convenience macro for creating `CFDictionary`, but in a less
//! clunky-looking way than `CFDictionary::from_slices(&keys, &values)`.

/// Creates an immutable [`CFDictionary`] containing the arguments.
///
/// Keys and values are separated by a `=>`, and entries are separated by
/// commas (with a trailing comma being allowed).
///
/// This macro translates into a call to [`from_slices`], which means that the
/// keys and values must be behind `&`, and that you might have to specify the
/// resulting dictionary type explicitly if the compiler infers the wrong one.
///
/// [`CFDictionary`]: crate::CFDictionary
/// [`from_slices`]: crate::CFDictionary::from_slices
///
///
/// # Example
///
/// Prepare a query for using in `objc2-security`'s `SecItemCopyMatching`:
///
/// ```
/// use objc2_core_foundation::{
///     cf_dictionary, CFBoolean, CFDictionary, CFRetained, CFString, CFType,
/// };
/// # #[cfg(requires_security)]
/// use objc2_security::{
///     kSecAttrAccount, kSecAttrService, kSecClass, kSecClassGenericPassword, kSecReturnData,
/// };
/// #
/// # let dummy = CFString::from_str("dummy");
/// # let kSecAttrAccount = &*dummy;
/// # let kSecAttrService = &*dummy;
/// # let kSecClass = &*dummy;
/// # let kSecClassGenericPassword = &*dummy;
/// # let kSecReturnData = &*dummy;
///
/// // Note the explicit return type. Might not always be necessary, depends
/// // on whether the compiler can infer the types involved.
/// let query: CFRetained<CFDictionary<CFString, CFType>> = cf_dictionary! {
///     unsafe { kSecClass } => unsafe { kSecClassGenericPassword },
///     unsafe { kSecAttrService } => &CFString::from_str("my_service"),
///     unsafe { kSecAttrAccount } => &CFString::from_str("user_account"),
///     unsafe { kSecReturnData } => CFBoolean::new(true),
/// };
///
/// // SecItemCopyMatching(query, ...);
/// ```
#[macro_export]
macro_rules! cf_dictionary {
    {
        $($key:expr => $value:expr),* $(,)?
    } => {
        $crate::CFDictionary::from_slices(
            &[$($key),*],
            &[$($value),*],
        )
    };
}

#[cfg(test)]
#[cfg(feature = "CFString")]
mod tests {
    use crate::{CFDictionary, CFRetained, CFString, CFType};

    #[test]
    fn empty() {
        let dict: CFRetained<CFDictionary<CFString, CFType>> = cf_dictionary! {};
        assert_eq!(dict.len(), 0);
        let dict: CFRetained<CFDictionary<CFType, CFString>> = cf_dictionary! {,};
        assert_eq!(dict.len(), 0);
    }

    #[test]
    fn duplicate_keys() {
        let foo = CFString::from_str("foo");
        let dict = cf_dictionary! {
            &*foo => &*CFString::from_str("bar1"),
            &*foo => &*CFString::from_str("bar2"),
        };
        assert_eq!(dict.len(), 1);
        let bar = CFString::from_str("bar");
        assert!(dict.get(&foo).unwrap().has_prefix(&bar));
    }
}
