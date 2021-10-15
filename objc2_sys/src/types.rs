//! Objective-C type aliases.

use crate::{
    objc_class, objc_ivar, objc_method, objc_object, objc_property, objc_protocol, objc_selector,
};

#[cfg(all(apple, not(target_arch = "aarch64")))]
// C: (explicitly) signed char
type BOOL_INNER = i8;

#[cfg(all(gnustep, not(target_arch = "aarch64")))]
// TODO: Only if STRICT_APPLE_COMPATIBILITY is NOT defined.
// TODO: (__vxworks || _WIN32) becomes BOOL = c_int.
// C: unsigned char
type BOOL_INNER = u8;

#[cfg(target_arch = "aarch64")]
// C: _Bool
type BOOL_INNER = bool;

/// The Objective-C `BOOL` type.
///
/// The type of this varies across platforms, so to convert an it into a Rust
/// [`bool`], always compare it with [`YES`][`crate::YES`] or [`NO`][`crate::NO`].
pub type BOOL = BOOL_INNER;

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
