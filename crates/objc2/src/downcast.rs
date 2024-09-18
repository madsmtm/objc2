use crate::ClassType;

/// [`DowncastTarget`] is an unsafe marker trait that can be implemented on types that also
/// implement [`ClassType`]. This is a promise that the self-type is a valid "downcast target"
/// (What this exactly means is described in the Safety section).
///
/// # Safety
///
/// Ideally, every type that implements `ClassType` can also be a valid downcast target,
/// however this would be unsound when used with generics, because we can only trivially decide
/// whether the "base container" is an instance of some class type,
/// but anything related to the generic arguments is unknown.
///
/// In this case:
///
/// ```ignore
/// let obj: Id<NSObject> = Id::into_super(NSString::new());
/// // This works and is safe.
/// let obj: &NSString = obj.downcast::<NSString>().unwrap();
/// ```
///
/// `NSString` has a [`DowncastTarget`] implementation, which looks like:
///
/// ```ignore
/// // This is safe.
/// unsafe impl Downcastable for NSString { }
/// ```
///
/// However with generics:
///
/// ```ignore
/// let obj: Id<NSArray<NSString>> = NSArray::new();
/// // This is invalid and doesn't type check.
/// let obj = obj.downcast::<NSArray<NSData>>();
/// ```
///
/// The example above doesn't work because [`DowncastTarget`] isn't implemented for
/// `for<T> NSArray<T>`. Doing so would be unsound because downcasting can only trivially
/// determine whether the base class (in this case `NSArray`) matches the receiver class type.
///
pub unsafe trait DowncastTarget: ClassType {}
