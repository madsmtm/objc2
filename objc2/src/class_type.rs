use crate::runtime::Class;
use crate::Message;

/// Marks types that represent specific classes.
///
/// Usually it is enough to generically know that a type is messageable, e.g.
/// [`rc::Id`][crate::rc::Id] works with any type that implements the
/// [`Message`] trait. But often, you have an object that you know represents
/// a specific Objective-C class - this trait allows you to communicate that
/// to the rest of the type-system.
///
/// This is implemented automatically by the
/// [`declare_class!`][crate::declare_class] and
/// [`extern_class!`][crate::extern_class] macros.
///
///
/// # Safety
///
/// TODO
///
///
/// # Examples
///
/// Use the trait to access the [`Class`] of different objects.
///
/// ```
/// # #[cfg(feature = "gnustep-1-7")]
/// # unsafe { objc2::__gnustep_hack::get_class_to_force_linkage() };
/// use objc2::ClassType;
/// use objc2::foundation::NSObject;
/// // Get a class object representing `NSObject`
/// let cls = <NSObject as ClassType>::class(); // Or just `NSObject::class()`
/// ```
///
/// Use the [`extern_class!`][crate::extern_class] macro to implement this
/// trait for a type.
///
/// ```ignore
/// use objc2::{extern_class, ClassType};
///
/// extern_class! {
///     unsafe struct MyClass: NSObject;
/// }
///
/// let cls = MyClass::class();
/// ```
// TODO: Figure out if we should use `Deref`, or an associated type.
pub unsafe trait ClassType: Message {
    /// Get a reference to the Objective-C class that this type represents.
    ///
    /// May register the class with the runtime if it wasn't already.
    ///
    ///
    /// # Panics
    ///
    /// This may panic if something went wrong with getting or declaring the
    /// class, e.g. if the program is not properly linked to the framework
    /// that defines the class.
    fn class() -> &'static Class;
}
