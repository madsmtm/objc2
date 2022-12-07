use core::ptr::NonNull;

use crate::runtime::Protocol;
use crate::Message;

/// Marks types that represent specific protocols.
///
/// This is the protocol equivalent of [`ClassType`].
///
/// This is implemented automatically by the [`extern_protocol!`] macro.
///
/// [`ClassType`]: crate::ClassType
/// [`extern_protocol!`]: crate::extern_protocol
///
///
/// # Safety
///
/// This is meant to be a sealed trait, and should not be implemented outside
/// of the [`extern_protocol!`] macro.
///
///
/// # Examples
///
/// Use the trait to access the [`Protocol`] of different objects.
///
/// ```
/// # #[cfg(feature = "gnustep-1-7")]
/// # unsafe { objc2::__gnustep_hack::get_class_to_force_linkage() };
/// use objc2::ProtocolType;
/// use objc2::runtime::NSObject;
/// // Get a protocol object representing `NSObject`
/// let protocol = NSObject::protocol();
/// ```
///
/// Use the [`extern_protocol!`] macro to implement this trait for a type.
///
/// ```no_run
/// use objc2::{extern_protocol, ProtocolType};
///
/// extern_protocol!(
///     struct MyProtocol;
///
///     unsafe impl ProtocolType for MyProtocol {
///         #[method(aMethod)]
///         fn a_method(&self);
///     }
/// );
///
/// let protocol = MyProtocol::protocol();
/// ```
pub unsafe trait ProtocolType: Message {
    /// The name of the Objective-C protocol that this type represents.
    const NAME: &'static str;

    /// Get a reference to the Objective-C protocol object that this type
    /// represents.
    ///
    /// May register the protocol with the runtime if it wasn't already.
    ///
    /// Note that some protocols [are not registered with the runtime][p-obj],
    /// depending on various factors. In those cases, this function may return
    /// `None`.
    ///
    /// [p-obj]: https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/ObjectiveC/Chapters/ocProtocols.html#//apple_ref/doc/uid/TP30001163-CH15-TPXREF149
    ///
    ///
    /// # Panics
    ///
    /// This may panic if something went wrong with getting or declaring the
    /// protocol, e.g. if the program is not properly linked to the framework
    /// that defines the protocol.
    fn protocol() -> Option<&'static Protocol> {
        Protocol::get(Self::NAME)
    }

    #[doc(hidden)]
    const __INNER: ();
}

/// Marks that a type conforms to a specific protocol.
///
/// This can be implemented both for types representing classes and types
/// representing protocols.
///
/// See [Apple's documentation on conforming to protocols][conform] for more
/// information.
///
/// [conform]: https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/ProgrammingWithObjectiveC/WorkingwithProtocols/WorkingwithProtocols.html#//apple_ref/doc/uid/TP40011210-CH11-SW3
///
///
/// # Safety
///
/// The object must actually conform to the specified protocol.
pub unsafe trait ConformsTo<P: ProtocolType>: Message {
    /// Get an immutable reference to a type representing the protocol.
    fn as_protocol(&self) -> &P {
        let ptr: NonNull<Self> = NonNull::from(self);
        let ptr: NonNull<P> = ptr.cast();
        // SAFETY: Implementer ensures that the object conforms to the
        // protocol; so converting the reference here is safe.
        unsafe { ptr.as_ref() }
    }

    /// Get a mutable reference to a type representing the protocol.
    fn as_protocol_mut(&mut self) -> &mut P {
        let ptr: NonNull<Self> = NonNull::from(self);
        let mut ptr: NonNull<P> = ptr.cast();
        // SAFETY: Same as `as_protocol`.
        //
        // Since the reference came from a mutable reference to start with,
        // returning a mutable reference here is safe (the lifetime of the
        // returned reference is bound to the input).
        unsafe { ptr.as_mut() }
    }
}

// SAFETY: Trivial
unsafe impl<P: ProtocolType> ConformsTo<P> for P {
    fn as_protocol(&self) -> &P {
        self
    }

    fn as_protocol_mut(&mut self) -> &mut P {
        self
    }
}
