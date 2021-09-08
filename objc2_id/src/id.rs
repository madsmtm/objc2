use core::any::Any;
use core::fmt;
use core::hash;
use core::marker::PhantomData;
use core::mem::ManuallyDrop;
use core::ops::{Deref, DerefMut};
use core::ptr::NonNull;

use objc2::rc::WeakPtr;
use objc2::runtime::Object;
use objc2::Message;

/// A type used to mark that a struct owns the object(s) it contains,
/// so it has the sole references to them.
pub enum Owned {}
/// A type used to mark that the object(s) a struct contains are shared,
/// so there may be other references to them.
pub enum Shared {}

/// A type that marks what type of ownership a struct has over the object(s)
/// it contains; specifically, either [`Owned`] or [`Shared`].
pub trait Ownership: Any {}
impl Ownership for Owned {}
impl Ownership for Shared {}

/// An pointer for Objective-C reference counted objects.
///
/// [`Id`] strongly references or "retains" the given object `T`, and
/// "releases" it again when dropped, thereby ensuring it will be deallocated
/// at the right time.
///
/// An [`Id`] can either be [`Owned`] or [`Shared`], represented with the `O`
/// type parameter.
///
/// If owned, it is guaranteed that there are no other references to the
/// object, and the [`Id`] can therefore be mutably dereferenced.
///
/// If shared, however, it can only be immutably dereferenced because there
/// may be other references to the object, since a shared [`Id`] can be cloned
/// to provide exactly that.
///
/// An owned [`Id`] can be safely "downgraded", that is, turned into to a
/// shared [`Id`] using [`Id::share`], but there is no way to do the opposite
/// safely.
///
/// `Option<Id<T, O>>` is guaranteed to have the same size as a pointer to the
/// object.
///
/// # Comparison to `std` types
///
/// `Id<T, Owned>` can be thought of as the Objective-C equivalent of [`Box`]
/// from the standard library: It is a unique pointer to some allocated
/// object, and that means you're allowed to get a mutable reference to it.
///
/// Likewise, `Id<T, Shared>` is the Objective-C equivalent of [`Arc`]: It is
/// a reference-counting pointer that, when cloned, increases the reference
/// count.
///
/// # Caveats
///
/// If the inner type implements [`Drop`], that implementation will not be
/// called, since there is no way to ensure that the Objective-C runtime will
/// do so. If you need to run some code when the object is destroyed,
/// implement the `dealloc` method instead.
///
/// # Examples
///
/// ```no_run
/// let owned: Id<T, Owned>;
/// let mut_ref: &mut T = *owned;
/// // Do something with `&mut T` here
///
/// let shared: Id<T, Shared> = owned.share();
/// let cloned: Id<T, Shared> = shared.clone();
/// // Do something with `&T` here
/// ```
#[repr(transparent)]
// TODO: Figure out if `Message` bound on `T` would be better here?
pub struct Id<T, O = Owned> {
    /// A pointer to the contained object. The pointer is always retained.
    ///
    /// It is important that this is `NonNull`, since we want to dereference
    /// it later, and be able to use the null-pointer optimization.
    ///
    /// Additionally, covariance is correct because we're either the unique
    /// owner of `T` (O = Owned), or `T` is immutable (O = Shared).
    ptr: NonNull<T>,
    /// Necessary for dropck even though we never actually run T's destructor,
    /// because it might have a `dealloc` that assumes that contained
    /// references outlive the type.
    ///
    /// See <https://doc.rust-lang.org/nightly/nomicon/phantom-data.html>
    item: PhantomData<T>,
    /// To prevent warnings about unused type parameters.
    own: PhantomData<O>,
}

// TODO: Maybe make most of these functions "associated" functions instead?
impl<T, O> Id<T, O>
where
    T: Message,
    O: Ownership,
{
    unsafe fn new(ptr: NonNull<T>) -> Id<T, O> {
        Id {
            ptr,
            item: PhantomData,
            own: PhantomData,
        }
    }

    /// Constructs an [`Id`] from a pointer to an unretained object and
    /// retains it.
    ///
    /// # Panics
    ///
    /// Panics if the pointer is null.
    ///
    /// # Safety
    ///
    /// The pointer must be to a valid object and the caller must ensure the
    /// ownership is correct.
    #[doc(alias = "objc_retain")]
    pub unsafe fn from_ptr(ptr: *mut T) -> Id<T, O> {
        let nonnull = NonNull::new(ptr).expect("Attempted to construct an Id from a null pointer");
        let ptr: *mut T = objc2_sys::objc_retain(nonnull.as_ptr() as *mut _) as *mut _;
        debug_assert_eq!(
            ptr,
            nonnull.as_ptr(),
            "objc_retain did not return the same pointer"
        );
        Id::new(NonNull::new_unchecked(ptr))
    }

    /// Constructs an [`Id`] from a pointer to a retained object; this won't
    /// retain the pointer, so the caller must ensure the object has a +1
    /// retain count.
    ///
    /// # Panics
    ///
    /// Panics if the pointer is null.
    ///
    /// # Safety
    ///
    /// The pointer must be to a valid object and the caller must ensure the
    /// ownership is correct.
    pub unsafe fn from_retained_ptr(ptr: *mut T) -> Id<T, O> {
        Id::new(NonNull::new(ptr).expect("Attempted to construct an Id from a null pointer"))
    }
}

// TODO: Change this to be a [`From`] implementation.
impl<T> Id<T, Owned>
where
    T: Message,
{
    /// Downgrade an owned [`Id`] to a [`ShareId`], allowing it to be cloned.
    pub fn share(self) -> ShareId<T> {
        let ptr = ManuallyDrop::new(self).ptr;
        unsafe { Id::new(ptr) }
    }
}

impl<T> Clone for Id<T, Shared>
where
    T: Message,
{
    /// Makes a clone of the shared object.
    ///
    /// This increases the object's reference count.
    #[doc(alias = "objc_retain")]
    #[doc(alias = "retain")]
    #[inline]
    fn clone(&self) -> ShareId<T> {
        // SAFETY: We already know the `ptr` is valid
        unsafe { Id::from_ptr(self.ptr.as_ptr()) }
    }
}

/// `#[may_dangle]` (see [this][dropck_eyepatch]) doesn't apply here since we
/// don't run `T`'s destructor (rather, we want to discourage having `T`s with
/// a destructor); and even if we did run the destructor, it would not be safe
/// to add since we cannot verify that a `dealloc` method doesn't access
/// borrowed data.
///
/// [dropck_eyepatch]: https://doc.rust-lang.org/nightly/nomicon/dropck.html#an-escape-hatch
impl<T, O> Drop for Id<T, O> {
    /// Releases the retained object.
    ///
    /// The contained object's destructor (if it has one) is never run!
    #[doc(alias = "objc_release")]
    #[doc(alias = "release")]
    #[inline]
    fn drop(&mut self) {
        // We could technically run the destructor for `T` when `O = Owned`,
        // and when `O = Shared` with (retainCount == 1), but that would be
        // confusing and inconsistent since we cannot guarantee that it's run.

        // SAFETY: The `ptr` is guaranteed to be valid and have at least one
        // retain count
        unsafe { objc2_sys::objc_release(self.ptr.as_ptr() as *mut _) };
    }
}

/// The `Send` implementation requires `T: Sync` because `Id<T, Shared>` give
/// access to `&T`.
///
/// Additiontally, it requires `T: Send` because if `T: !Send`, you could
/// clone a `Id<T, Shared>`, send it to another thread, and drop the clone
/// last, making `dealloc` get called on the other thread, and violate
/// `T: !Send`.
unsafe impl<T: Sync + Send> Send for Id<T, Shared> {}

/// The `Sync` implementation requires `T: Sync` because `&Id<T, Shared>` give
/// access to `&T`.
///
/// Additiontally, it requires `T: Send`, because if `T: !Send`, you could
/// clone a `&Id<T, Shared>` from another thread, and drop the clone last,
/// making `dealloc` get called on the other thread, and violate `T: !Send`.
unsafe impl<T: Sync + Send> Sync for Id<T, Shared> {}

/// `Id<T, Owned>` are `Send` if `T` is `Send` because they give the same
/// access as having a T directly.
unsafe impl<T: Send> Send for Id<T, Owned> {}

/// `Id<T, Owned>` are `Sync` if `T` is `Sync` because they give the same
/// access as having a `T` directly.
unsafe impl<T: Sync> Sync for Id<T, Owned> {}

impl<T, O> Deref for Id<T, O> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T> DerefMut for Id<T, Owned> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { self.ptr.as_mut() }
    }
}

impl<T, O> PartialEq for Id<T, O>
where
    T: PartialEq,
{
    fn eq(&self, other: &Id<T, O>) -> bool {
        self.deref() == other.deref()
    }
}

impl<T, O> Eq for Id<T, O> where T: Eq {}

impl<T, O> hash::Hash for Id<T, O>
where
    T: hash::Hash,
{
    fn hash<H>(&self, state: &mut H)
    where
        H: hash::Hasher,
    {
        self.deref().hash(state)
    }
}

impl<T, O> fmt::Debug for Id<T, O>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.deref().fmt(f)
    }
}

impl<T, O> fmt::Pointer for Id<T, O> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self.ptr.as_ptr(), f)
    }
}

/// A convenient alias for a shared [`Id`].
pub type ShareId<T> = Id<T, Shared>;

/// A pointer type for a weak reference to an Objective-C reference counted
/// object.
pub struct WeakId<T> {
    ptr: WeakPtr,
    item: PhantomData<T>,
}

impl<T> WeakId<T>
where
    T: Message,
{
    /// Construct a new [`WeakId`] referencing the given [`ShareId`].
    pub fn new(obj: &ShareId<T>) -> WeakId<T> {
        // SAFETY: The pointer is valid
        let ptr = unsafe { WeakPtr::new(obj.ptr.as_ptr() as *mut Object) };
        WeakId {
            ptr,
            item: PhantomData,
        }
    }

    /// Load a [`ShareId`] from the [`WeakId`] if the object still exists.
    ///
    /// Returns [`None`] if the object has been deallocated.
    pub fn load(&self) -> Option<ShareId<T>> {
        let obj = self.ptr.load();
        let ptr = *ManuallyDrop::new(obj).deref().deref() as *mut T;
        NonNull::new(ptr).map(|ptr| unsafe { Id::new(ptr) })
    }
}

/// This implementation follows the same reasoning as `Id<T, Shared>`.
unsafe impl<T: Sync + Send> Sync for WeakId<T> {}

/// This implementation follows the same reasoning as `Id<T, Shared>`.
unsafe impl<T: Sync + Send> Send for WeakId<T> {}

#[cfg(test)]
mod tests {
    use core::mem::size_of;

    use super::{Id, Owned, ShareId, Shared, WeakId};
    use objc2::runtime::Object;
    use objc2::{class, msg_send};

    #[cfg(not(target_vendor = "apple"))]
    #[test]
    fn ensure_linkage() {
        unsafe { crate::get_class_to_force_linkage() };
    }

    fn retain_count(obj: &Object) -> usize {
        unsafe { msg_send![obj, retainCount] }
    }

    pub struct TestType {
        _data: [u8; 0], // TODO: `UnsafeCell`?
    }

    #[test]
    fn test_size_of() {
        assert_eq!(size_of::<Id<TestType, Owned>>(), size_of::<&TestType>());
        assert_eq!(size_of::<Id<TestType, Shared>>(), size_of::<&TestType>());
        assert_eq!(
            size_of::<Option<Id<TestType, Owned>>>(),
            size_of::<&TestType>()
        );
        assert_eq!(
            size_of::<Option<Id<TestType, Shared>>>(),
            size_of::<&TestType>()
        );
    }

    #[test]
    fn test_clone() {
        let cls = class!(NSObject);
        let obj: Id<Object> = unsafe {
            let obj: *mut Object = msg_send![cls, alloc];
            let obj: *mut Object = msg_send![obj, init];
            Id::from_retained_ptr(obj)
        };
        assert!(retain_count(&obj) == 1);

        let obj = obj.share();
        assert!(retain_count(&obj) == 1);

        let cloned = obj.clone();
        assert!(retain_count(&cloned) == 2);
        assert!(retain_count(&obj) == 2);

        drop(obj);
        assert!(retain_count(&cloned) == 1);
    }

    #[test]
    fn test_weak() {
        let cls = class!(NSObject);
        let obj: ShareId<Object> = unsafe {
            let obj: *mut Object = msg_send![cls, alloc];
            let obj: *mut Object = msg_send![obj, init];
            Id::from_retained_ptr(obj)
        };

        let weak = WeakId::new(&obj);
        let strong = weak.load().unwrap();
        let strong_ptr: *const Object = &*strong;
        let obj_ptr: *const Object = &*obj;
        assert!(strong_ptr == obj_ptr);
        drop(strong);

        drop(obj);
        assert!(weak.load().is_none());
    }
}
