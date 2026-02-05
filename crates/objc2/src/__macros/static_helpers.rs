//! Various helpers for static classes and selectors.

/// Convert an input `&[u8]` slice to an array with known length.
///
/// Useful for making it possible to place strings directly in a static.
#[doc(hidden)]
#[macro_export]
macro_rules! __statics_string_to_known_length_bytes {
    ($inp:ident) => {{
        let mut res: [$crate::__macros::u8; $inp.len()] = [0; $inp.len()];
        let mut i = 0;
        while i < $inp.len() {
            res[i] = $inp[i];
            i += 1;
        }
        res
    }};
}
