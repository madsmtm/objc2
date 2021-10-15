//! Trivial forwarding impls on `Id`.
//!
//! Kept here to keep `id.rs` free from this boilerplate.
//!
//! `#[inline]` is used where the standard library `Box` uses it.

#![forbid(unsafe_code)]

use alloc::borrow;
use core::cmp::Ordering;
use core::fmt;
use core::hash;
use core::iter::FusedIterator;
use std::error::Error;

use super::{Id, Owned, Ownership};

impl<T: PartialEq, O: Ownership> PartialEq for Id<T, O> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        (**self).eq(&**other)
    }

    #[inline]
    fn ne(&self, other: &Self) -> bool {
        (**self).ne(&**other)
    }
}

impl<T: Eq, O: Ownership> Eq for Id<T, O> {}

impl<T: PartialOrd, O: Ownership> PartialOrd for Id<T, O> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (**self).partial_cmp(&**other)
    }
    #[inline]
    fn lt(&self, other: &Self) -> bool {
        (**self).lt(&**other)
    }
    #[inline]
    fn le(&self, other: &Self) -> bool {
        (**self).le(&**other)
    }
    #[inline]
    fn ge(&self, other: &Self) -> bool {
        (**self).ge(&**other)
    }
    #[inline]
    fn gt(&self, other: &Self) -> bool {
        (**self).gt(&**other)
    }
}

impl<T: Ord, O: Ownership> Ord for Id<T, O> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        (**self).cmp(&**other)
    }
}

impl<T: hash::Hash, O: Ownership> hash::Hash for Id<T, O> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        (**self).hash(state)
    }
}

// TODO: impl Hasher?

impl<T: fmt::Display, O: Ownership> fmt::Display for Id<T, O> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (**self).fmt(f)
    }
}

impl<T: fmt::Debug, O: Ownership> fmt::Debug for Id<T, O> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (**self).fmt(f)
    }
}

impl<I: Iterator> Iterator for Id<I, Owned> {
    type Item = I::Item;
    fn next(&mut self) -> Option<I::Item> {
        (**self).next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (**self).size_hint()
    }
    fn nth(&mut self, n: usize) -> Option<I::Item> {
        (**self).nth(n)
    }
}

impl<I: DoubleEndedIterator> DoubleEndedIterator for Id<I, Owned> {
    fn next_back(&mut self) -> Option<I::Item> {
        (**self).next_back()
    }
    fn nth_back(&mut self, n: usize) -> Option<I::Item> {
        (**self).nth_back(n)
    }
}

impl<I: ExactSizeIterator> ExactSizeIterator for Id<I, Owned> {
    fn len(&self) -> usize {
        (**self).len()
    }
}

impl<I: FusedIterator> FusedIterator for Id<I, Owned> {}

impl<T, O: Ownership> borrow::Borrow<T> for Id<T, O> {
    fn borrow(&self) -> &T {
        &**self
    }
}

impl<T> borrow::BorrowMut<T> for Id<T, Owned> {
    fn borrow_mut(&mut self) -> &mut T {
        &mut **self
    }
}

impl<T, O: Ownership> AsRef<T> for Id<T, O> {
    fn as_ref(&self) -> &T {
        &**self
    }
}

impl<T> AsMut<T> for Id<T, Owned> {
    fn as_mut(&mut self) -> &mut T {
        &mut **self
    }
}

impl<T: Error, O: Ownership> Error for Id<T, O> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        (**self).source()
    }
}

// TODO: impl io traits Seek, Read, BufRead and Write

// TODO:
// impl<F: Future + Unpin> Future for Id<F, Owned> {
//     type Output = F::Output;
//     fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
//         F::poll(Pin::new(&mut *self), cx)
//     }
// }

// TODO: impl Fn traits, CoerceUnsized, Stream and so on when stabilized
