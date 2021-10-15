//! Helper traits for Id.

use core::convert::Infallible;

use super::{Id, Owned, Ownership};
use crate::Message;

/// Helper trait for functionality on slices containing [`Id`]s.
pub trait SliceId {
    /// The type of the items in the slice.
    type Item;

    /// Convert a slice of [`Id`]s into a slice of references.
    fn as_slice_ref(&self) -> &[&Self::Item];

    /// Convert a mutable slice of [`Id`]s into a mutable slice of references.
    fn as_slice_mut(&mut self) -> &mut [&Self::Item];
}

/// Helper trait for functionality on slices containing owned [`Id`]s.
pub trait SliceIdMut: SliceId {
    /// Convert a mutable slice of mutable [`Id`]s into a mutable slice of
    /// mutable references.
    fn as_mut_slice_mut(&mut self) -> &mut [&mut Self::Item];
}

impl<T: Message, O: Ownership> SliceId for [Id<T, O>] {
    type Item = T;

    fn as_slice_ref(&self) -> &[&T] {
        let ptr = self as *const Self as *const [&T];
        // SAFETY: Id<T, O> and &T have the same memory layout. Further safety
        // follows from `Deref` impl.
        unsafe { &*ptr }
    }

    fn as_slice_mut(&mut self) -> &mut [&T] {
        let ptr = self as *mut Self as *mut [&T];
        // SAFETY: Id<T, O> and &T have the same memory layout. Further safety
        // follows from `Deref` impl.
        unsafe { &mut *ptr }
    }
}

impl<T: Message> SliceIdMut for [Id<T, Owned>] {
    fn as_mut_slice_mut(&mut self) -> &mut [&mut T] {
        let ptr = self as *mut Self as *mut [&mut T];
        // SAFETY: Id<T, O> and &mut T have the same memory layout, and the
        // `Id` is `Owned` so we're allowed to hand out mutable references.
        // Further safety follows from `DerefMut` impl.
        unsafe { &mut *ptr }
    }
}

/// Helper trait to implement [`Default`] on types whoose default value is an
/// [`Id`].
// TODO: Remove `Sized` bound.
// TODO: Maybe make this `unsafe` and provide a default implementation?
pub trait DefaultId: Sized {
    /// Indicates whether the default value is mutable or immutable.
    type Ownership: Ownership;

    /// The default [`Id`] for a type.
    ///
    /// On most objects the implementation would just be sending a message to
    /// the `new` selector.
    fn default_id() -> Id<Self, Self::Ownership>;
}

impl<T: DefaultId> Default for Id<T, T::Ownership> {
    fn default() -> Self {
        T::default_id()
    }
}

pub trait FromId<T, O: Ownership> {
    fn from_id(obj: Id<T, O>) -> Self;
}

pub trait TryFromId<T, O: Ownership>: Sized {
    type Error;
    fn try_from_id(obj: Id<T, O>) -> Result<Self, Self::Error>;
}

impl<T, O: Ownership, U: FromId<T, O>> TryFromId<T, O> for U {
    type Error = Infallible;

    fn try_from_id(obj: Id<T, O>) -> Result<Self, Self::Error> {
        Ok(FromId::from_id(obj))
    }
}

/// TODO.
///
/// This is similar to [`Into`] in that it is implemented automatically for
/// all [`Id`] that implement [`FromId`]; but you will have to implement this
/// yourself in many more cases!
pub trait IntoId<T, O: Ownership> {
    fn into_id(self) -> Id<T, O>;
}

impl<T, O: Ownership, U> IntoId<U, O> for Id<T, O>
where
    Id<U, O>: FromId<T, O>,
{
    fn into_id(self) -> Id<U, O> {
        FromId::from_id(self)
    }
}

pub trait TryIntoId<T, O: Ownership> {
    type Error;
    fn try_into_id(self) -> Result<Id<T, O>, Self::Error>;
}

impl<T, O: Ownership, U: IntoId<T, O>> TryIntoId<T, O> for U {
    type Error = Infallible;

    fn try_into_id(self) -> Result<Id<T, O>, Self::Error> {
        Ok(IntoId::into_id(self))
    }
}
