use core::ptr::NonNull;

use crate::runtime::Protocol;
use crate::Message;

/// Marks types that represent specific protocols.
///
/// TODO.
///
///
/// # Safety
///
/// This is meant to be a sealed trait, and should not be implemented outside
/// of the `extern_protocol!` macro.
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

/// TODO
///
/// [conforming-to-protocol]: https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/ProgrammingWithObjectiveC/WorkingwithProtocols/WorkingwithProtocols.html#//apple_ref/doc/uid/TP40011210-CH11-SW3
///
/// # Safety
///
/// TODO
pub unsafe trait ConformsTo<P: ProtocolType> {
    /// Get an immutable reference to a type representing the protocol.
    fn as_protocol(&self) -> &P {
        let ptr: NonNull<Self> = NonNull::from(self);
        let ptr: NonNull<P> = ptr.cast();
        // SAFETY: TODO
        unsafe { ptr.as_ref() }
    }

    /// Get a mutable reference to a type representing the protocol.
    fn as_protocol_mut(&mut self) -> &mut P {
        let ptr: NonNull<Self> = NonNull::from(self);
        let mut ptr: NonNull<P> = ptr.cast();
        // SAFETY: TODO
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
