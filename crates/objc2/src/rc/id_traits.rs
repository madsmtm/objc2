//! Helper traits for Id.

use super::Id;
use crate::mutability::IsAllocableAnyThread;

/// Helper trait to implement [`Default`] on [`Id`].
// TODO: Maybe make this `unsafe` and provide a default implementation?
pub trait DefaultId: IsAllocableAnyThread {
    /// The default [`Id`] for a type.
    ///
    /// On most objects the implementation would just be sending a message to
    /// the `new` selector.
    fn default_id() -> Id<Self>;
}

impl<T: ?Sized + DefaultId> Default for Id<T> {
    #[inline]
    fn default() -> Self {
        T::default_id()
    }
}
