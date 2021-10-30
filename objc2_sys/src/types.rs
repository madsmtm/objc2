//! Objective-C type aliases.

use crate::{
    objc_class, objc_ivar, objc_method, objc_object, objc_property, objc_protocol, objc_selector,
};

/// The BOOL typedef for Apple's objc4.
///
/// Don't be fooled by the backup definition in `objc.h`; __OBJC_BOOL_IS_BOOL
/// is always defined by `clang` when compiling Objective-C sources. The below
/// cfgs are determined experimentally via. cross compiling.
#[cfg(apple)]
mod inner {
    // __OBJC_BOOL_IS_BOOL
    #[cfg(any(
        // aarch64-apple-*
        target_arch = "aarch64",
        // + x86_64-apple-ios (but not x86_64-apple-ios-macabi)
        all(target_os = "ios", target_pointer_width = "64", not(target_abi_macabi)),
        // + x86_64-apple-tvos
        all(target_os = "tvos", target_pointer_width = "64"),
        // + *-apple-watchos (no Rust targets with this yet)
        target_os = "watchos",
    ))]
    // C: _Bool
    pub type BOOL = bool;

    // Inverse of the above
    #[cfg(not(any(
        target_arch = "aarch64",
        all(target_os = "ios", target_pointer_width = "64", not(target_abi_macabi)),
        all(target_os = "tvos", target_pointer_width = "64"),
        target_os = "watchos",
    )))]
    // C: (explicitly) signed char
    pub type BOOL = i8;
}

// GNUStep's and Microsoft's libobjc2
#[cfg(all(any(gnustep, winobjc), libobjc2_strict_apple_compat))]
mod inner {
    // C: (explicitly) signed char
    pub type BOOL = i8;
}

#[cfg(all(any(gnustep, winobjc), not(libobjc2_strict_apple_compat)))]
mod inner {
    // windows && !32bit-MinGW
    #[cfg(all(windows, not(all(target_pointer_width = "64", target_env = "gnu"))))]
    pub type BOOL = std::os::raw::c_int;

    // The inverse
    #[cfg(not(all(windows, not(all(target_pointer_width = "64", target_env = "gnu")))))]
    // C: unsigned char
    pub type BOOL = u8;
}

// ObjFW???
#[cfg(objfw)]
mod inner {
    pub type BOOL = todo!();
}

/// The Objective-C `BOOL` type.
///
/// The type of this varies across platforms, so to convert an it into a Rust
/// [`bool`], always compare it with [`YES`][`crate::YES`] or [`NO`][`crate::NO`].
///
/// Note that this type implements `objc2_encode::Encode` and
/// `objc2_encode::RefEncode`, but the `RefEncode` implementation is wrong
/// on some platforms! You should only use this on FFI boundaries, otherwise
/// prefer `objc2::runtime::Bool`.
pub type BOOL = inner::BOOL;

/// An immutable pointer to a selector.
///
/// Type alias provided for convenience.
pub type SEL = *const objc_selector;

/// A mutable pointer to a class.
///
/// Type alias provided for convenience.
pub type Class = *mut objc_class;

/// A mutable pointer to an object / instance.
///
/// Type alias provided for convenience.
pub type id = *mut objc_object;

/// An immutable pointer to an instance variable.
///
/// Type alias provided for convenience.
pub type Ivar = *const objc_ivar;

/// A mutable pointer to a method.
///
/// Type alias provided for convenience.
pub type Method = *mut objc_method;

/// An opaque type that represents a protocol.
///
/// This is not just a type alias of [`objc_object`], but of [`objc_protocol`]
/// instead, for better type safety. Their internal representation is the same,
/// so the functionality is just a cast away.
///
/// Type alias provided for convenience.
pub type Protocol = objc_protocol;

/// A mutable pointer to a property.
///
/// Type alias provided for convenience.
pub type objc_property_t = *mut objc_property;
