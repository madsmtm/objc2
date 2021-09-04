//! Objective-C type aliases.

use crate::{
    objc_class, objc_ivar, objc_method, objc_object, objc_property, objc_protocol, objc_selector,
};

/// The Objective-C `BOOL` type.
///
/// To convert an Objective-C `BOOL` into a Rust [`bool`], compare it with
/// [`NO`][`super::NO`].
#[cfg(all(target_vendor = "apple", not(target_arch = "aarch64")))]
pub type BOOL = i8;
#[cfg(all(not(target_vendor = "apple"), not(target_arch = "aarch64")))]
pub type BOOL = u8;
#[cfg(target_arch = "aarch64")]
pub type BOOL = bool;

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
/// Type alias provided for convenience.
pub type Protocol = objc_protocol;

/// A mutable pointer to a property.
///
/// Type alias provided for convenience.
pub type objc_property_t = *mut objc_property;
