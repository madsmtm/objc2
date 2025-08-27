use alloc::vec::Vec;
use core::ffi::c_uint;
use core::ffi::CStr;
use core::fmt;
use core::panic::{RefUnwindSafe, UnwindSafe};

use super::{AnyObject, Bool, MallocSlice, MethodDescription};
use crate::encode::{Encoding, RefEncode};
use crate::ffi;
use crate::Message;

/// An opaque type that represents a protocol in the Objective-C runtime.
///
/// See [`ProtocolObject`] for objects that implement a specific protocol.
///
/// [`ProtocolObject`]: crate::runtime::ProtocolObject
//
// The naming of this follows GNUStep; this struct does not exist in Apple's
// runtime, there `Protocol` is a type alias of `objc_object`.
#[repr(C)]
#[doc(alias = "objc_protocol")]
pub struct AnyProtocol {
    inner: AnyObject,
}

/// Use [`AnyProtocol`] instead.
#[deprecated = "renamed to `runtime::AnyProtocol`"]
pub type Protocol = AnyProtocol;

// SAFETY: AnyProtocol is immutable (and can be retrieved from AnyClass anyhow).
unsafe impl Sync for AnyProtocol {}
unsafe impl Send for AnyProtocol {}
impl UnwindSafe for AnyProtocol {}
impl RefUnwindSafe for AnyProtocol {}
// Note that Unpin is not applicable.

impl AnyProtocol {
    /// Returns the protocol definition of a specified protocol, or [`None`]
    /// if the protocol is not registered with the Objective-C runtime.
    #[inline]
    #[doc(alias = "objc_getProtocol")]
    pub fn get(name: &CStr) -> Option<&'static Self> {
        unsafe {
            let proto = ffi::objc_getProtocol(name.as_ptr());
            proto.cast::<Self>().as_ref()
        }
    }

    /// Obtains the list of registered protocol definitions.
    #[doc(alias = "objc_copyProtocolList")]
    pub fn protocols() -> MallocSlice!(&'static Self) {
        unsafe {
            let mut count: c_uint = 0;
            let protocols: *mut &Self = ffi::objc_copyProtocolList(&mut count).cast();
            MallocSlice::from_array(protocols, count as usize)
        }
    }

    /// Get a list of the protocols to which this protocol conforms.
    #[doc(alias = "protocol_copyProtocolList")]
    pub fn adopted_protocols(&self) -> MallocSlice!(&AnyProtocol) {
        unsafe {
            let mut count: c_uint = 0;
            let protocols: *mut &AnyProtocol =
                ffi::protocol_copyProtocolList(self, &mut count).cast();
            MallocSlice::from_array(protocols, count as usize)
        }
    }

    /// Checks whether this protocol conforms to the specified protocol.
    #[inline]
    #[doc(alias = "protocol_conformsToProtocol")]
    pub fn conforms_to(&self, proto: &AnyProtocol) -> bool {
        unsafe { ffi::protocol_conformsToProtocol(self, proto).as_bool() }
    }

    /// Returns the name of self.
    #[inline]
    #[doc(alias = "protocol_getName")]
    pub fn name(&self) -> &CStr {
        unsafe { CStr::from_ptr(ffi::protocol_getName(self)) }
    }

    fn method_descriptions_inner(&self, required: bool, instance: bool) -> Vec<MethodDescription> {
        let mut count: c_uint = 0;
        let descriptions = unsafe {
            ffi::protocol_copyMethodDescriptionList(
                self,
                Bool::new(required),
                Bool::new(instance),
                &mut count,
            )
        };
        if descriptions.is_null() {
            return Vec::new();
        }
        let descriptions = unsafe { MallocSlice::from_array(descriptions, count as usize) };
        descriptions
            .iter()
            .map(|desc| {
                unsafe { MethodDescription::from_raw(*desc) }.expect("invalid method description")
            })
            .collect()
    }

    #[allow(dead_code)]
    #[doc(alias = "protocol_copyMethodDescriptionList")]
    pub(crate) fn method_descriptions(&self, required: bool) -> Vec<MethodDescription> {
        self.method_descriptions_inner(required, true)
    }

    #[allow(dead_code)]
    #[doc(alias = "protocol_copyMethodDescriptionList")]
    pub(crate) fn class_method_descriptions(&self, required: bool) -> Vec<MethodDescription> {
        self.method_descriptions_inner(required, false)
    }
}

impl PartialEq for AnyProtocol {
    /// Check whether the protocols are equal, or conform to each other.
    #[inline]
    #[doc(alias = "protocol_isEqual")]
    fn eq(&self, other: &Self) -> bool {
        unsafe { ffi::protocol_isEqual(self, other).as_bool() }
    }
}

impl Eq for AnyProtocol {}

// Don't implement `Hash` for protocol, it is unclear how that would work

unsafe impl RefEncode for AnyProtocol {
    // Protocols are objects internally.
    const ENCODING_REF: Encoding = Encoding::Object;
}

/// Note that protocols are objects, though sending messages to them is
/// officially deprecated.
//
// SAFETY: Protocols are NSObjects internally (and somewhat publicly, see e.g.
// `objc/Protocol.h`), and are returned as `Retained` in various places in
// Foundation. But that's considered deprecated, so we don't implement
// ClassType for them (even though the "Protocol" class exists).
unsafe impl Message for AnyProtocol {}

impl fmt::Debug for AnyProtocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AnyProtocol")
            .field("name", &self.name())
            .finish_non_exhaustive()
    }
}

impl fmt::Display for AnyProtocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Protocols are usually UTF-8, so it's probably fine to do a lossy
        // conversion here.
        fmt::Display::fmt(&self.name().to_string_lossy(), f)
    }
}

impl AsRef<Self> for AnyProtocol {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsRef<AnyObject> for AnyProtocol {
    fn as_ref(&self) -> &AnyObject {
        &self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::{test_utils, NSObject, NSObjectProtocol};
    use crate::{sel, ProtocolType};

    #[test]
    fn test_protocol() {
        let proto = test_utils::custom_protocol();
        assert_eq!(proto.name().to_str().unwrap(), "CustomProtocol");
        let class = test_utils::custom_class();
        assert!(class.conforms_to(proto));

        // The selectors are broken somehow on GNUStep < 2.0
        if cfg!(any(not(feature = "gnustep-1-7"), feature = "gnustep-2-0")) {
            let desc = MethodDescription {
                sel: sel!(setBar:),
                types: CStr::from_bytes_with_nul(b"v@:i\0").unwrap(),
            };
            assert_eq!(&proto.method_descriptions(true), &[desc]);
            let desc = MethodDescription {
                sel: sel!(getName),
                types: CStr::from_bytes_with_nul(b"*@:\0").unwrap(),
            };
            assert_eq!(&proto.method_descriptions(false), &[desc]);
            let desc = MethodDescription {
                sel: sel!(addNumber:toNumber:),
                types: CStr::from_bytes_with_nul(b"i@:ii\0").unwrap(),
            };
            assert_eq!(&proto.class_method_descriptions(true), &[desc]);
        }
        assert_eq!(&proto.class_method_descriptions(false), &[]);

        assert!(class.adopted_protocols().contains(&proto));
    }

    #[test]
    fn test_subprotocols() {
        let sub_proto = test_utils::custom_subprotocol();
        let super_proto = test_utils::custom_protocol();
        assert!(sub_proto.conforms_to(super_proto));
        assert_eq!(sub_proto.adopted_protocols()[0], super_proto);
    }

    #[test]
    fn test_protocols() {
        // Ensure that a protocol has been registered on linux
        let _ = test_utils::custom_protocol();

        assert!(AnyProtocol::protocols().len() > 0);
    }

    #[test]
    fn protocol_is_object() {
        let protocol = <dyn NSObjectProtocol>::protocol().unwrap();
        let retained = protocol.retain();
        assert_eq!(&*retained, protocol);

        // Protocols don't implement isKindOfClass: on GNUStep.
        if cfg!(feature = "gnustep-1-7") {
            return;
        }

        // In the old runtime, NSObjectProtocol are not NSObject subclasses.
        if cfg!(all(target_os = "macos", target_arch = "x86")) {
            let _ = retained.downcast::<NSObject>().unwrap_err();
        } else {
            // But elsewhere they are.
            let obj = retained.downcast::<NSObject>().unwrap();
            // Test that we can call NSObject methods on protocols.
            assert_eq!(obj, obj);
            let _ = obj.retainCount();
        }
    }
}
