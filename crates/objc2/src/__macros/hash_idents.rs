#[cfg(any(feature = "unstable-static-sel", feature = "unstable-static-class"))]
#[doc(hidden)]
pub use objc2_proc_macros::__hash_idents as proc_macro_hash_idents;

#[cfg(any(feature = "unstable-static-sel", feature = "unstable-static-class"))]
#[doc(hidden)]
#[macro_export]
macro_rules! __hash_idents {
    ($($t:tt)*) => {
        $crate::__macros::proc_macro_hash_idents!($($t)*)
    };
}

/// No-op, used to make our other macros a bit easier to read.
#[cfg(not(any(feature = "unstable-static-sel", feature = "unstable-static-class")))]
#[doc(hidden)]
#[macro_export]
macro_rules! __hash_idents {
    ($($t:tt)*) => {
        ()
    };
}

#[cfg(test)]
#[cfg(any(feature = "unstable-static-sel", feature = "unstable-static-class"))]
mod tests {
    use crate::__hash_idents;

    #[test]
    fn hash_idents_different() {
        assert_ne!(__hash_idents!(abc), __hash_idents!(def));
    }

    #[test]
    fn hash_idents_same_no_equal() {
        assert_ne!(__hash_idents!(abc), __hash_idents!(abc));
        assert_ne!(__hash_idents!(abc def ghi), __hash_idents!(abc def ghi));
    }

    #[test]
    fn hash_idents_exact_same_ident() {
        macro_rules! x {
            ($x:ident) => {
                (__hash_idents!($x), __hash_idents!($x))
            };
        }
        let (ident1, ident2) = x!(abc);
        // This is a limitation of `__hash_idents`, ideally we'd like these
        // to be different!
        assert_eq!(ident1, ident2);
    }
}
