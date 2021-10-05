use super::{Id, Owned, Ownership};
use crate::Message;

/// Helper trait for functionality on slices containing [`Id`]s.
pub trait IdSlice {
    /// The type of the items in the slice.
    type Item;

    /// Convert a slice of [`Id`]s into a slice of references.
    fn as_slice_ref(&self) -> &[&Self::Item];

    /// Convert a mutable slice of [`Id`]s into a mutable slice of references.
    fn as_slice_mut_ref(&mut self) -> &mut [&Self::Item];
}

/// Helper trait for functionality on slices containing owned [`Id`]s.
pub trait IdSliceMut {
    /// The type of the items in the slice.
    type Item;

    /// Convert a mutable slice of mutable [`Id`]s into a mutable slice of
    /// mutable references.
    fn as_mut_slice_mut_ref(&mut self) -> &mut [&mut Self::Item];
}

impl<T: Message, O: Ownership> IdSlice for [Id<T, O>] {
    type Item = T;

    fn as_slice_ref(&self) -> &[&T] {
        let ptr = self as *const Self as *const [&T];
        // SAFETY: Id<T, O> and &T have the same memory layout. Further safety
        // follows from `Deref` impl.
        unsafe { &*ptr }
    }

    fn as_slice_mut_ref(&mut self) -> &mut [&T] {
        let ptr = self as *mut Self as *mut [&T];
        // SAFETY: Id<T, O> and &T have the same memory layout. Further safety
        // follows from `Deref` impl.
        unsafe { &mut *ptr }
    }
}

impl<T: Message> IdSliceMut for [Id<T, Owned>] {
    type Item = T;

    fn as_mut_slice_mut_ref(&mut self) -> &mut [&mut T] {
        let ptr = self as *mut Self as *mut [&mut T];
        // SAFETY: Id<T, O> and &mut T have the same memory layout, and the
        // `Id` is `Owned` so we're allowed to hand out mutable references.
        // Further safety follows from `DerefMut` impl.
        unsafe { &mut *ptr }
    }
}
