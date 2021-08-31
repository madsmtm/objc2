use objc_encode::{Encode, Encoding};

/// We don't know the size of NSString, so we can only hold pointers to it.
///
/// TODO: Use [`extern type`][rfc-1861] when that gets stabilized.
///
/// [rfc-1861]: https://rust-lang.github.io/rfcs/1861-extern-types.html
#[repr(C)]
struct NSString {
    _priv: [u8; 0],
}

/// Implement `Encode` for references.
///
/// This also implements for `*mut NSString` and `Option<&mut NSString>`.
unsafe impl<'a> Encode for &'a NSString {
    const ENCODING: Encoding<'static> = Encoding::Object;
}

/// Implement `Encode` for mutable references.
///
/// This also implements for `*mut NSString` and `Option<&mut NSString>`.
unsafe impl<'a> Encode for &'a mut NSString {
    const ENCODING: Encoding<'static> = Encoding::Object;
}

fn main() {
    println!("{}", <*const NSString>::ENCODING);
    println!("{}", <*mut NSString>::ENCODING);
    println!("{}", <&NSString>::ENCODING);
    println!("{}", <&mut NSString>::ENCODING);
    println!("{}", Option::<&NSString>::ENCODING);
    println!("{}", Option::<&mut NSString>::ENCODING);
}
