//! Helper traits for Id.

use super::Id;
use crate::mutability::IsMutable;

/// Helper trait to implement [`Default`] on [`Id`].
pub trait DefaultId {
    /// The default [`Id`] for a type.
    ///
    /// On most objects the implementation would be sending a message to the
    /// `new` selector.
    fn default_id() -> Id<Self>;
}

impl<T: ?Sized + DefaultId> Default for Id<T> {
    #[inline]
    fn default() -> Self {
        T::default_id()
    }
}

/// Helper trait to implement [`IntoIterator`] on [`Id`].
///
/// This should be implemented in exactly the same fashion as if you were
/// implementing `IntoIterator` for your type normally.
//
// Note that [`Box<T>` gets to cheat with regards moves][box-move], so
// `boxed.into_iter()` is possible, while `id.into_iter()` is not possible
// without this helper trait.
//
// [box-move]: https://doc.rust-lang.org/reference/expressions.html#moved-and-copied-types
pub trait IdIntoIterator {
    /// The type of the elements being iterated over.
    type Item;

    /// Which kind of iterator are we turning this into?
    type IntoIter: Iterator<Item = Self::Item>;

    /// Creates an iterator from an [`Id`].
    ///
    /// You would normally not call this function directly; instead, you'd
    /// call [`into_iter`](IntoIterator::into_iter) on an [`Id`].
    fn id_into_iter(this: Id<Self>) -> Self::IntoIter;
}

// Note: These `IntoIterator` implementations conflict with an `Iterator`
// implementation for `Id`.
//
// For our case however (in contrast with `Box`), that is the better tradeoff,
// which I will show with an example:
//
// ```
// let xs = Box::new(vec![]);
// for x in &xs { // Doesn't compile, `&Box` doesn't implement `IntoIterator`
//     // ...
// }
// ```
//
// Here, you're expected to write `xs.iter()` or `&**xs` instead, which is
// fairly acceptable, since usually people don't wrap things in boxes so much;
// but in Objective-C, _everything_ is wrapped in an `Id`, and hence we should
// attempt to make that common case easier:
//
// ```
// let obj = NSArray::new(); // `Id<NSArray<_>>`
// for item in &obj { // Should compile
//     // ...
// }
// ```
//
// The loss of the `Iterator` impl is a bit unfortunate, but not a big deal,
// since there is only one iterator in Objective-C anyhow, `NSEnumerator`, and
// for that we can make other abstractions instead.
impl<T: ?Sized + IdIntoIterator> IntoIterator for Id<T> {
    type Item = <T as IdIntoIterator>::Item;
    type IntoIter = <T as IdIntoIterator>::IntoIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        T::id_into_iter(self)
    }
}

impl<'a, T: ?Sized> IntoIterator for &'a Id<T>
where
    &'a T: IntoIterator,
{
    type Item = <&'a T as IntoIterator>::Item;
    type IntoIter = <&'a T as IntoIterator>::IntoIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        (&**self).into_iter()
    }
}

impl<'a, T: ?Sized + IsMutable> IntoIterator for &'a mut Id<T>
where
    &'a mut T: IntoIterator,
{
    type Item = <&'a mut T as IntoIterator>::Item;
    type IntoIter = <&'a mut T as IntoIterator>::IntoIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        (&mut **self).into_iter()
    }
}

/// Helper trait to implement [`FromIterator`] on [`Id`].
///
/// This should be implemented in exactly the same fashion as if you were
/// implementing `FromIterator` for your type normally.
pub trait IdFromIterator<T>: Sized {
    /// Creates an `Id` from an iterator.
    fn id_from_iter<I>(iter: I) -> Id<Self>
    where
        I: IntoIterator<Item = T>;
}

impl<T, U: IdFromIterator<T>> FromIterator<T> for Id<U> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        U::id_from_iter(iter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mutability::Mutable;
    use crate::runtime::NSObject;
    use crate::{declare_class, msg_send_id, ClassType, DeclaredClass};

    declare_class!(
        #[derive(PartialEq, Eq, Hash, Debug)]
        struct Collection;

        unsafe impl ClassType for Collection {
            type Super = NSObject;
            type Mutability = Mutable;
            const NAME: &'static str = "MyCustomCollection";
        }

        impl DeclaredClass for Collection {}
    );

    impl DefaultId for Collection {
        fn default_id() -> Id<Self> {
            unsafe { msg_send_id![Collection::class(), new] }
        }
    }

    struct Iter<'a> {
        _inner: &'a Collection,
    }

    impl<'a> Iterator for Iter<'a> {
        type Item = &'a NSObject;
        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    impl<'a> IntoIterator for &'a Collection {
        type Item = &'a NSObject;
        type IntoIter = Iter<'a>;

        fn into_iter(self) -> Self::IntoIter {
            Iter { _inner: self }
        }
    }

    struct IterMut<'a> {
        _inner: &'a mut Collection,
    }

    impl<'a> Iterator for IterMut<'a> {
        type Item = &'a mut NSObject;
        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    impl<'a> IntoIterator for &'a mut Collection {
        // Usually only valid if a mutable object is stored in the collection.
        type Item = &'a mut NSObject;
        type IntoIter = IterMut<'a>;

        fn into_iter(self) -> Self::IntoIter {
            IterMut { _inner: self }
        }
    }

    struct IntoIter {
        _inner: Id<Collection>,
    }

    impl Iterator for IntoIter {
        type Item = Id<NSObject>;
        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    impl IdIntoIterator for Collection {
        type Item = Id<NSObject>;
        type IntoIter = IntoIter;

        fn id_into_iter(this: Id<Self>) -> Self::IntoIter {
            IntoIter { _inner: this }
        }
    }

    impl IdFromIterator<Id<NSObject>> for Collection {
        fn id_from_iter<I: IntoIterator<Item = Id<NSObject>>>(_iter: I) -> Id<Self> {
            Collection::default_id()
        }
    }

    #[test]
    fn test_default() {
        let obj1: Id<Collection> = Default::default();
        let obj2 = Collection::default_id();
        assert_ne!(obj1, obj2);
    }

    #[test]
    fn test_into_iter() {
        let mut obj: Id<Collection> = Default::default();

        for _ in &*obj {}
        for _ in &obj {}

        for _ in &mut *obj {}
        for _ in &mut obj {}

        for _ in obj {}
    }

    #[test]
    fn test_from_iter() {
        let _: Id<Collection> = [NSObject::new()].into_iter().collect();
    }
}
