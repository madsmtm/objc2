use super::AutoreleasePool;

/// A type used to mark that a struct owns the object(s) it contains,
/// so it has the sole references to them.
pub enum Owned {}

/// A type used to mark that the object(s) a struct contains are shared,
/// so there may be other references to them.
pub enum Shared {}

mod private {
    pub trait Sealed {}

    impl Sealed for super::Owned {}
    impl Sealed for super::Shared {}
}

/// A type that marks what type of ownership a struct has over the object(s)
/// it contains; specifically, either [`Owned`] or [`Shared`].
///
/// This trait is sealed and not meant to be implemented outside of the this
/// crate.
pub trait Ownership: private::Sealed + 'static {
    type Reference<'a, T: 'a>;

    unsafe fn as_ref_pool<'p, T: 'p>(
        pool: &'p AutoreleasePool,
        ptr: *mut T,
    ) -> Self::Reference<'p, T>;
}

impl Ownership for Owned {
    type Reference<'a, T: 'a> = &'a mut T;

    unsafe fn as_ref_pool<'p, T: 'p>(
        pool: &'p AutoreleasePool,
        ptr: *mut T,
    ) -> Self::Reference<'p, T> {
        unsafe { pool.ptr_as_mut(ptr) }
    }
}
impl Ownership for Shared {
    type Reference<'a, T: 'a> = &'a T;

    unsafe fn as_ref_pool<'p, T: 'p>(
        pool: &'p AutoreleasePool,
        ptr: *mut T,
    ) -> Self::Reference<'p, T> {
        unsafe { pool.ptr_as_ref(ptr) }
    }
}
