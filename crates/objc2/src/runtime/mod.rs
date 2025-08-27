//! # Direct runtime bindings.
//!
//! This module contains safe(r) bindings to common parts of the Objective-C
//! runtime. See the [`ffi`][crate::ffi] module for details on the raw
//! bindings.
//!
//!
//! # Example
//!
//! Using features of the runtime to query information about `NSObject`.
//!
//! ```
#![doc = include_str!("../../examples/introspection.rs")]
//! ```
#![allow(clippy::missing_panics_doc)]

/// We do not want to expose `MallocSlice` to end users, because in the
/// future, we want to be able to change it to `Box<[T], MallocAllocator>`.
///
/// So instead we use an unnameable type.
macro_rules! MallocSlice {
    ($t:ty) => {
        impl std::ops::Deref<Target = [$t]> + AsRef<[$t]> + std::fmt::Debug
    };
}

/// Same as `MallocSlice!`.
macro_rules! MallocCStr {
    () => {
        impl std::ops::Deref<Target = CStr> + AsRef<CStr> + std::fmt::Debug
    };
}

/// Implement PartialEq, Eq and Hash using pointer semantics; there's not
/// really a better way to do it for this type
macro_rules! standard_pointer_impls {
    ($name:ident) => {
        impl PartialEq for $name {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                ptr::eq(self, other)
            }
        }
        impl Eq for $name {}
        impl hash::Hash for $name {
            #[inline]
            fn hash<H: hash::Hasher>(&self, state: &mut H) {
                let ptr: *const Self = self;
                ptr.hash(state)
            }
        }
    };
}

// Note: While this is not public, it is still a breaking change to remove,
// since `objc2-foundation` relies on it.
#[doc(hidden)]
pub mod __nsstring;

mod anyclass;
mod anyobject;
mod anyprotocol;
mod bool;
mod define;
mod ivar;
mod malloc;
mod message_receiver;
mod method;
mod method_encoding_iter;
mod method_implementation;
mod nsobject;
mod nsproxy;
mod nszone;
mod protocol_object;
mod retain_release_fast;
mod sel;
#[cfg(test)]
pub(crate) mod test_utils;
mod verify;

// Note: While this is not public, it is still a breaking change to remove,
// since `objc2-foundation` relies on it.
#[doc(hidden)]
pub use self::nsproxy::NSProxy as __NSProxy;

pub use self::anyclass::AnyClass;
#[allow(deprecated)]
pub use self::anyclass::Class;
pub use self::anyobject::AnyObject;
#[allow(deprecated)]
pub use self::anyobject::Object;
pub use self::anyprotocol::AnyProtocol;
#[allow(deprecated)]
pub use self::anyprotocol::Protocol;
pub use self::bool::Bool;
pub use self::define::{ClassBuilder, ProtocolBuilder};
pub use self::ivar::Ivar;
pub use self::message_receiver::MessageReceiver;
pub use self::method::{Imp, Method};
pub use self::method_implementation::MethodImplementation;
pub use self::nsobject::{NSObject, NSObjectProtocol};
pub use self::nszone::NSZone;
pub use self::protocol_object::{ImplementedBy, ProtocolObject};
pub use self::sel::Sel;
pub use self::verify::VerificationError;

#[allow(deprecated)]
pub use crate::ffi::{BOOL, NO, YES};

pub(crate) use self::malloc::{MallocCStr, MallocSlice};
pub(crate) use self::method::MethodDescription;
pub(crate) use self::method_encoding_iter::{EncodingParseError, MethodEncodingIter};
pub(crate) use self::retain_release_fast::{objc_release_fast, objc_retain_fast};
pub(crate) use self::verify::{verify_method_signature, Inner};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{sel, Encode};
    use alloc::format;
    use alloc::string::ToString;
    use core::mem::size_of;

    #[test]
    fn test_encode() {
        fn assert_enc<T: Encode>(expected: &str) {
            assert_eq!(&T::ENCODING.to_string(), expected);
        }
        assert_enc::<&AnyObject>("@");
        assert_enc::<*mut AnyObject>("@");
        assert_enc::<&AnyClass>("#");
        assert_enc::<Sel>(":");
        assert_enc::<Option<Sel>>(":");
        assert_enc::<Imp>("^?");
        assert_enc::<Option<Imp>>("^?");
        assert_enc::<&AnyProtocol>("@");
    }

    #[test]
    fn test_send_sync() {
        fn assert_send_sync<T: Send + Sync + ?Sized>() {}
        assert_send_sync::<Bool>();
        assert_send_sync::<AnyClass>();
        assert_send_sync::<Ivar>();
        assert_send_sync::<Method>();
        assert_send_sync::<AnyProtocol>();
        assert_send_sync::<Sel>();
    }

    #[test]
    fn test_debug_display() {
        let sel = sel!(abc:);
        assert_eq!(format!("{sel}"), "abc:");
        assert_eq!(format!("{sel:?}"), "Sel(\"abc:\")");
        let cls = test_utils::custom_class();
        assert_eq!(format!("{cls}"), "CustomObject");
        assert_eq!(
            format!("{cls:?}"),
            "AnyClass { name: \"CustomObject\", .. }"
        );
        let protocol = test_utils::custom_protocol();
        assert_eq!(format!("{protocol}"), "CustomProtocol");
        assert_eq!(
            format!("{protocol:?}"),
            "AnyProtocol { name: \"CustomProtocol\", .. }"
        );

        let object = test_utils::custom_object();
        assert_eq!(
            format!("{:?}", &*object),
            format!("CustomObject(<CustomObject: {:p}>)", &*object)
        );
    }

    #[test]
    fn test_sizes() {
        assert_eq!(size_of::<Sel>(), size_of::<*const ()>());
        assert_eq!(size_of::<Sel>(), size_of::<Option<Sel>>());

        // These must be zero-sized until we get extern types, otherwise the
        // optimizer may invalidly assume something about their layout.
        assert_eq!(size_of::<AnyClass>(), 0);
        assert_eq!(size_of::<AnyObject>(), 0);
        assert_eq!(size_of::<AnyProtocol>(), 0);
        assert_eq!(size_of::<Ivar>(), 0);
        assert_eq!(size_of::<Method>(), 0);
    }
}
