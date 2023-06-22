use crate::runtime::AnyProtocol;

/// Marks types that represent specific protocols.
///
/// This is the protocol equivalent of [`ClassType`].
///
/// This is implemented automatically by the [`extern_protocol!`] macro for
/// `dyn T`, where `T` is the protocol.
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
/// Use the trait to access the [`AnyProtocol`] of different objects.
///
/// ```
/// use objc2::ProtocolType;
/// use objc2::runtime::NSObject;
/// // Get a protocol object representing `NSObject`
/// let protocol = NSObject::protocol().expect("NSObject to have a protocol");
/// assert_eq!(NSObject::NAME, protocol.name());
/// ```
///
/// Use the [`extern_protocol!`] macro to implement this trait for a type.
///
/// ```no_run
/// use objc2::{extern_protocol, ProtocolType};
///
/// extern_protocol!(
///     unsafe trait MyProtocol {}
///     unsafe impl ProtocolType for dyn MyProtocol {}
/// );
///
/// let protocol = <dyn MyProtocol>::protocol();
/// ```
pub unsafe trait ProtocolType {
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
    fn protocol() -> Option<&'static AnyProtocol> {
        AnyProtocol::get(Self::NAME)
    }

    #[doc(hidden)]
    const __INNER: ();
}
