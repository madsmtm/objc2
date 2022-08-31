//! Trivial forwarding impls on `Id`.
//!
//! Kept here to keep `id.rs` free from this boilerplate.
//!
//! `#[inline]` is used where the standard library `Box` uses it.

#![forbid(unsafe_code)]

use alloc::borrow;
use alloc::string::String;
use alloc::vec::Vec;
use core::cmp::Ordering;
use core::fmt;
use core::future::Future;
use core::hash;
use core::iter::FusedIterator;
use core::ops::{Deref, DerefMut};
use core::pin::Pin;
use core::task::{Context, Poll};
use std::error::Error;
use std::io;

use super::{Id, Owned, Ownership};
use crate::Thin;

impl<T: PartialEq + ?Sized + Thin, O: Ownership> PartialEq for Id<T, O> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        (**self).eq(&**other)
    }

    #[inline]
    #[allow(clippy::partialeq_ne_impl)]
    fn ne(&self, other: &Self) -> bool {
        (**self).ne(&**other)
    }
}

impl<T: Eq + ?Sized + Thin, O: Ownership> Eq for Id<T, O> {}

impl<T: PartialOrd + ?Sized + Thin, O: Ownership> PartialOrd for Id<T, O> {
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

impl<T: Ord + ?Sized + Thin, O: Ownership> Ord for Id<T, O> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        (**self).cmp(&**other)
    }
}

impl<T: hash::Hash + ?Sized + Thin, O: Ownership> hash::Hash for Id<T, O> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        (**self).hash(state)
    }
}

impl<T: hash::Hasher + ?Sized + Thin> hash::Hasher for Id<T, Owned> {
    fn finish(&self) -> u64 {
        (**self).finish()
    }
    fn write(&mut self, bytes: &[u8]) {
        (**self).write(bytes)
    }
    fn write_u8(&mut self, i: u8) {
        (**self).write_u8(i)
    }
    fn write_u16(&mut self, i: u16) {
        (**self).write_u16(i)
    }
    fn write_u32(&mut self, i: u32) {
        (**self).write_u32(i)
    }
    fn write_u64(&mut self, i: u64) {
        (**self).write_u64(i)
    }
    fn write_u128(&mut self, i: u128) {
        (**self).write_u128(i)
    }
    fn write_usize(&mut self, i: usize) {
        (**self).write_usize(i)
    }
    fn write_i8(&mut self, i: i8) {
        (**self).write_i8(i)
    }
    fn write_i16(&mut self, i: i16) {
        (**self).write_i16(i)
    }
    fn write_i32(&mut self, i: i32) {
        (**self).write_i32(i)
    }
    fn write_i64(&mut self, i: i64) {
        (**self).write_i64(i)
    }
    fn write_i128(&mut self, i: i128) {
        (**self).write_i128(i)
    }
    fn write_isize(&mut self, i: isize) {
        (**self).write_isize(i)
    }
}

impl<T: fmt::Display + ?Sized + Thin, O: Ownership> fmt::Display for Id<T, O> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (**self).fmt(f)
    }
}

impl<T: fmt::Debug + ?Sized + Thin, O: Ownership> fmt::Debug for Id<T, O> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (**self).fmt(f)
    }
}

impl<I: Iterator + ?Sized + Thin> Iterator for Id<I, Owned> {
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

impl<I: DoubleEndedIterator + ?Sized + Thin> DoubleEndedIterator for Id<I, Owned> {
    fn next_back(&mut self) -> Option<I::Item> {
        (**self).next_back()
    }
    fn nth_back(&mut self, n: usize) -> Option<I::Item> {
        (**self).nth_back(n)
    }
}

impl<I: ExactSizeIterator + ?Sized + Thin> ExactSizeIterator for Id<I, Owned> {
    fn len(&self) -> usize {
        (**self).len()
    }
}

impl<I: FusedIterator + ?Sized + Thin> FusedIterator for Id<I, Owned> {}

// TODO: Consider this impl
// impl<'a, T, O: Ownership> IntoIterator for &'a Id<T, O>
// where
//     &'a T: IntoIterator,
// {
//     type Item = <&'a T as IntoIterator>::Item;
//     type IntoIter = <&'a T as IntoIterator>::IntoIter;
//
//     fn into_iter(self) -> Self::IntoIter {
//         (**self).into_iter()
//     }
// }

impl<T: ?Sized + Thin, O: Ownership> borrow::Borrow<T> for Id<T, O> {
    fn borrow(&self) -> &T {
        Deref::deref(self)
    }
}

impl<T: ?Sized + Thin> borrow::BorrowMut<T> for Id<T, Owned> {
    fn borrow_mut(&mut self) -> &mut T {
        DerefMut::deref_mut(self)
    }
}

impl<T: ?Sized + Thin, O: Ownership> AsRef<T> for Id<T, O> {
    fn as_ref(&self) -> &T {
        Deref::deref(self)
    }
}

impl<T: ?Sized + Thin> AsMut<T> for Id<T, Owned> {
    fn as_mut(&mut self) -> &mut T {
        DerefMut::deref_mut(self)
    }
}

impl<T: Error + ?Sized + Thin, O: Ownership> Error for Id<T, O> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        (**self).source()
    }
}

impl<T: io::Read + ?Sized + Thin> io::Read for Id<T, Owned> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        (**self).read(buf)
    }

    #[inline]
    fn read_vectored(&mut self, bufs: &mut [io::IoSliceMut<'_>]) -> io::Result<usize> {
        (**self).read_vectored(bufs)
    }

    #[inline]
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        (**self).read_to_end(buf)
    }

    #[inline]
    fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize> {
        (**self).read_to_string(buf)
    }

    #[inline]
    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        (**self).read_exact(buf)
    }
}

impl<T: io::Write + ?Sized + Thin> io::Write for Id<T, Owned> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        (**self).write(buf)
    }

    #[inline]
    fn write_vectored(&mut self, bufs: &[io::IoSlice<'_>]) -> io::Result<usize> {
        (**self).write_vectored(bufs)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        (**self).flush()
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        (**self).write_all(buf)
    }

    #[inline]
    fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) -> io::Result<()> {
        (**self).write_fmt(fmt)
    }
}

impl<T: io::Seek + ?Sized + Thin> io::Seek for Id<T, Owned> {
    #[inline]
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        (**self).seek(pos)
    }

    #[inline]
    fn stream_position(&mut self) -> io::Result<u64> {
        (**self).stream_position()
    }
}

impl<T: io::BufRead + ?Sized + Thin> io::BufRead for Id<T, Owned> {
    #[inline]
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        (**self).fill_buf()
    }

    #[inline]
    fn consume(&mut self, amt: usize) {
        (**self).consume(amt)
    }

    #[inline]
    fn read_until(&mut self, byte: u8, buf: &mut Vec<u8>) -> io::Result<usize> {
        (**self).read_until(byte, buf)
    }

    #[inline]
    fn read_line(&mut self, buf: &mut String) -> io::Result<usize> {
        (**self).read_line(buf)
    }
}

impl<T: Future + Unpin + ?Sized + Thin> Future for Id<T, Owned> {
    type Output = T::Output;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        T::poll(Pin::new(&mut *self), cx)
    }
}

// TODO: impl Fn traits, CoerceUnsized, Stream and so on when stabilized
