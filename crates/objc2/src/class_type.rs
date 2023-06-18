use crate::msg_send_id;
use crate::mutability::{IsAllocableAnyThread, IsRetainable, Mutability};
use crate::rc::{Allocated, Id};
use crate::runtime::AnyClass;
use crate::Message;

/// Marks types that represent specific classes.
///
/// Sometimes it is enough to generically know that a type is messageable,
/// e.g. [`rc::Id`][crate::rc::Id] works with any type that implements the
/// [`Message`] trait. But often, you have an object that you know represents
/// a specific Objective-C class - this trait allows you to communicate that,
/// as well as a few properties of the class to the rest of the type-system.
///
/// This is implemented automatically for your type by the
/// [`declare_class!`][crate::declare_class] and
/// [`extern_class!`][crate::extern_class] macros.
///
///
/// # Safety
///
/// 1. The type must represent a specific class.
/// 2. [`Self::Super`] must be a superclass of the class (or something that
///    represents any object, like [`AnyObject`][crate::runtime::AnyObject]).
/// 3. [`Self::Mutability`] must be specified correctly.
///
///    Note that very little Objective-C code follows Rust's usual ownership
///    model. If you think your type's mutability should be [`Mutable`], think
///    again, it _very_ rarely should!
///
///    If you're unsure of what to do, [`InteriorMutable`] + avoiding `&mut`
///    is usually a good starting point.
/// 4. [`Self::NAME`] must be correct.
/// 5. The class returned by [`Self::class`] must be the class that this type
///    represents.
///
/// [`Mutable`]: crate::mutability::Mutable
/// [`InteriorMutable`]: crate::mutability::InteriorMutable
///
///
/// # Examples
///
/// Use the trait to access the [`AnyClass`] of an object.
///
/// ```
/// use objc2::{ClassType, msg_send_id};
/// use objc2::rc::Id;
/// # use objc2::runtime::{NSObject as MyObject};
///
/// // Get the class of the object.
/// let cls = <MyObject as ClassType>::class();
/// // Or, since the trait is in scope.
/// let cls = MyObject::class();
///
/// // We can now access properties of the class.
/// assert_eq!(cls.name(), MyObject::NAME);
///
/// // And we can send messages to the class.
/// //
/// // SAFETY:
/// // - The class is `MyObject`, which can safely be initialized with `new`.
/// // - The return type is correctly specified.
/// let obj: Id<MyObject> = unsafe { msg_send_id![cls, new] };
/// ```
///
/// Use the trait to allocate a new instance of an object.
///
/// ```
/// use objc2::{ClassType, msg_send_id};
/// use objc2::rc::Id;
/// # use objc2::runtime::{NSObject as MyObject};
///
/// let obj = MyObject::alloc();
///
/// // Now we can call initializers on this newly allocated object.
/// //
/// // SAFETY: `MyObject` can safely be initialized with `init`.
/// let obj: Id<MyObject> = unsafe { msg_send_id![obj, init] };
/// ```
///
/// Use the [`extern_class!`][crate::extern_class] macro to implement this
/// trait for a type.
///
/// ```
/// use objc2::runtime::NSObject;
/// use objc2::{extern_class, mutability, ClassType};
///
/// extern_class!(
///     struct MyClass;
///
///     // SAFETY: The superclass and the mutability is correctly specified.
///     unsafe impl ClassType for MyClass {
///         type Super = NSObject;
///         type Mutability = mutability::InteriorMutable;
///         # // For testing purposes
///         # const NAME: &'static str = "NSObject";
///     }
/// );
///
/// let cls = MyClass::class();
/// let obj = MyClass::alloc();
/// ```
///
/// Implement the trait manually for a class with a lifetime parameter.
///
/// ```
#[doc = include_str!("../examples/class_with_lifetime.rs")]
/// ```
pub unsafe trait ClassType: Message {
    /// The superclass of this class.
    ///
    /// If you have implemented [`Deref`] for your type, it is highly
    /// recommended that this is equal to [`Deref::Target`].
    ///
    /// This may be [`AnyObject`] if the class is a root class.
    ///
    /// [`Deref`]: std::ops::Deref
    /// [`Deref::Target`]: std::ops::Deref::Target
    /// [`AnyObject`]: crate::runtime::AnyObject
    type Super: Message;

    /// Whether the type is mutable or immutable.
    ///
    /// See the [`mutability`][crate::mutability] module for further details
    /// about class mutability.
    type Mutability: Mutability;

    /// The name of the Objective-C class that this type represents.
    ///
    /// `T::NAME` is the `const` version of `T::class().name()`.
    const NAME: &'static str;

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
    fn class() -> &'static AnyClass;

    /// Get an immutable reference to the superclass.
    // Note: It'd be safe to provide a default impl using transmute here if
    // we wanted to!
    fn as_super(&self) -> &Self::Super;

    /// Get a mutable reference to the superclass.
    // Note: No `Self: IsMutable` bound required here, since there is no way
    // to get `&mut self` in the first place.
    //
    // Or at least, if we have `&mut NSMutableString`, we're allowed to get
    // `&mut NSString`, and from that it will also make sense to allow getting
    // `&mut NSObject`.
    fn as_super_mut(&mut self) -> &mut Self::Super;

    /// Increment the reference count of the receiver.
    ///
    /// This extends the duration in which the receiver is alive by detaching
    /// it from the lifetime information carried by the reference.
    ///
    /// This is similar to using [`Clone` on `Id<Self>`][clone-id], with the
    /// addition that it can be used on a plain reference. Note however that
    /// this is not possible to use on certain types like `NSString`, since
    /// if you only hold `&NSString`, that may have come from
    /// `&mut NSMutableString`, in which case it would be unsound to erase the
    /// lifetime information carried by the reference.
    ///
    /// In cases like that, you should rather use `NSCopying::copy` (since
    /// that gives you a `NSString` whether the string was originally a
    /// `NSString` or a `NSMutableString`).
    ///
    /// [clone-id]: crate::rc::Id#impl-Clone-for-Id<T>
    //
    // Note: We could have placed this on `mutability::IsRetainable`, but
    // `ClassType` is more often already in scope, allowing easier access to
    // `obj.retain()`.
    #[inline]
    #[doc(alias = "objc_retain")]
    fn retain(&self) -> Id<Self>
    where
        Self: IsRetainable,
        Self: Sized, // Temporary
    {
        let ptr: *const Self = self;
        let ptr: *mut Self = ptr as _;
        // SAFETY:
        // - The object is known to not be mutable (or have a mutable
        //   subclass) due to the `IsRetainable` bound.
        // - The pointer is valid since it came from `&self`.
        // - The lifetime of the pointer itself is extended, but any lifetime
        //   that the object may carry is still kept within the type itself.
        let obj = unsafe { Id::retain(ptr) };
        // SAFETY: The pointer came from `&self`, which is always non-null
        // (and objc_retain always returns the same value).
        unsafe { obj.unwrap_unchecked() }
    }

    /// Allocate a new instance of the class.
    ///
    /// The return value can be used directly inside [`msg_send_id!`] to
    /// initialize the object.
    ///
    /// For classes that are only usable on the main thread, you can use
    /// `MainThreadMarker::alloc` instead.
    ///
    /// [`msg_send_id!`]: crate::msg_send_id
    //
    // Note: We could have placed this on `mutability::IsAllocableAnyThread`,
    // but `ClassType` is more often already in scope, allowing easier access
    // to `T::alloc()`.
    #[inline]
    fn alloc() -> Option<Allocated<Self>>
    where
        Self: IsAllocableAnyThread,
    {
        // SAFETY:
        // - It is always safe to (attempt to) allocate an object.
        // - The object is of the correct type, since we've used the class
        //   from `Self::class`.
        // - The object is safe to `dealloc` on the current thread (due to the
        //   `IsAllocableAnyThread` bound which guarantees it is not
        //   `MainThreadOnly`).
        //
        // See also `MainThreadMarker::alloc`.
        unsafe { msg_send_id![Self::class(), alloc] }
    }

    // TODO: `fn alloc_on_main(mtm: MainThreadMarker)`
    // TODO: `fn mtm(&self) -> MainThreadMarker where T::Mutability: MainThreadOnly`
}
