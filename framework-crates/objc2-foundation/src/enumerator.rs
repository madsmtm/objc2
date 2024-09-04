//! Utilities for the `NSEnumerator` class.
use objc2::rc::Retained;
use objc2::Message;

use crate::{iter, NSEnumerator};

// TODO: Measure whether iterating through `nextObject` or fast enumeration is
// fastest.
// impl<T: Message> Iterator for NSEnumerator<T> {
//     type Item = Retained<T>;
//
//     #[inline]
//     fn next(&mut self) -> Option<Retained<T>> {
//         self.nextObject()
//     }
// }

impl<T: Message> NSEnumerator<T> {
    /// Iterate over the enumerator's elements.
    #[inline]
    pub fn iter(&self) -> Iter<'_, T> {
        Iter(iter::Iter::new(self))
    }

    /// Iterate over the enumerator without retaining the elements.
    ///
    /// Consider using the [`iter`](Self::iter) method instead, unless you're
    /// seeing performance issues from the retaining.
    ///
    /// # Safety
    ///
    /// The enumerator and the underlying collection must not be mutated while
    /// the iterator is alive.
    #[inline]
    pub unsafe fn iter_unchecked(&self) -> IterUnchecked<'_, T> {
        IterUnchecked(iter::IterUnchecked::new(self))
    }
}

unsafe impl<T: Message> iter::FastEnumerationHelper for NSEnumerator<T> {
    type Item = T;

    #[inline]
    fn maybe_len(&self) -> Option<usize> {
        None
    }
}

/// An iterator over the items in an enumerator.
#[derive(Debug)]
pub struct Iter<'a, T: Message>(iter::Iter<'a, NSEnumerator<T>>);

__impl_iter! {
    impl<'a, T: Message> Iterator<Item = Retained<T>> for Iter<'a, T> { ... }
}

/// An iterator over unretained items in an enumerator.
///
/// # Safety
///
/// The enumerator and the underlying collection must not be mutated while
/// this is alive.
#[derive(Debug)]
pub struct IterUnchecked<'a, T: Message + 'a>(iter::IterUnchecked<'a, NSEnumerator<T>>);

__impl_iter! {
    impl<'a, T: Message> Iterator<Item = &'a T> for IterUnchecked<'a, T> { ... }
}

/// A consuming iterator over the items in an enumerator.
#[derive(Debug)]
pub struct IntoIter<T: Message>(iter::IntoIter<NSEnumerator<T>>);

__impl_iter! {
    impl<T: Message> Iterator<Item = Retained<T>> for IntoIter<T> { ... }
}

__impl_into_iter! {
    impl<T: Message> IntoIterator for &NSEnumerator<T> {
        type IntoIter = Iter<'_, T>;
    }

    impl<T: Message> IntoIterator for Retained<NSEnumerator<T>> {
        #[uses(new)]
        type IntoIter = IntoIter<T>;
    }
}

// TODO: Does fast enumeration modify the enumeration while iterating?
