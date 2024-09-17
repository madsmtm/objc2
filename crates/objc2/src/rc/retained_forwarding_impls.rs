//! Trivial forwarding impls on `Retained`.
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
use core::pin::Pin;
use core::task::{Context, Poll};
use std::error::Error;
use std::io;

use super::Retained;

#[allow(clippy::unconditional_recursion)]
impl<T: PartialEq + ?Sized> PartialEq for Retained<T> {
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

impl<T: Eq + ?Sized> Eq for Retained<T> {}

impl<T: PartialOrd + ?Sized> PartialOrd for Retained<T> {
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

impl<T: Ord + ?Sized> Ord for Retained<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        (**self).cmp(&**other)
    }
}

impl<T: hash::Hash + ?Sized> hash::Hash for Retained<T> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        (**self).hash(state);
    }
}

impl<T: ?Sized> hash::Hasher for Retained<T>
where
    for<'a> &'a T: hash::Hasher,
{
    fn finish(&self) -> u64 {
        (&**self).finish()
    }
    fn write(&mut self, bytes: &[u8]) {
        (&**self).write(bytes);
    }
    fn write_u8(&mut self, i: u8) {
        (&**self).write_u8(i);
    }
    fn write_u16(&mut self, i: u16) {
        (&**self).write_u16(i);
    }
    fn write_u32(&mut self, i: u32) {
        (&**self).write_u32(i);
    }
    fn write_u64(&mut self, i: u64) {
        (&**self).write_u64(i);
    }
    fn write_u128(&mut self, i: u128) {
        (&**self).write_u128(i);
    }
    fn write_usize(&mut self, i: usize) {
        (&**self).write_usize(i);
    }
    fn write_i8(&mut self, i: i8) {
        (&**self).write_i8(i);
    }
    fn write_i16(&mut self, i: i16) {
        (&**self).write_i16(i);
    }
    fn write_i32(&mut self, i: i32) {
        (&**self).write_i32(i);
    }
    fn write_i64(&mut self, i: i64) {
        (&**self).write_i64(i);
    }
    fn write_i128(&mut self, i: i128) {
        (&**self).write_i128(i);
    }
    fn write_isize(&mut self, i: isize) {
        (&**self).write_isize(i);
    }
}

impl<'a, T: ?Sized> hash::Hasher for &'a Retained<T>
where
    &'a T: hash::Hasher,
{
    fn finish(&self) -> u64 {
        (&***self).finish()
    }
    fn write(&mut self, bytes: &[u8]) {
        (&***self).write(bytes);
    }
    fn write_u8(&mut self, i: u8) {
        (&***self).write_u8(i);
    }
    fn write_u16(&mut self, i: u16) {
        (&***self).write_u16(i);
    }
    fn write_u32(&mut self, i: u32) {
        (&***self).write_u32(i);
    }
    fn write_u64(&mut self, i: u64) {
        (&***self).write_u64(i);
    }
    fn write_u128(&mut self, i: u128) {
        (&***self).write_u128(i);
    }
    fn write_usize(&mut self, i: usize) {
        (&***self).write_usize(i);
    }
    fn write_i8(&mut self, i: i8) {
        (&***self).write_i8(i);
    }
    fn write_i16(&mut self, i: i16) {
        (&***self).write_i16(i);
    }
    fn write_i32(&mut self, i: i32) {
        (&***self).write_i32(i);
    }
    fn write_i64(&mut self, i: i64) {
        (&***self).write_i64(i);
    }
    fn write_i128(&mut self, i: i128) {
        (&***self).write_i128(i);
    }
    fn write_isize(&mut self, i: isize) {
        (&***self).write_isize(i);
    }
}

impl<T: fmt::Display + ?Sized> fmt::Display for Retained<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (**self).fmt(f)
    }
}

impl<T: fmt::Debug + ?Sized> fmt::Debug for Retained<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (**self).fmt(f)
    }
}

impl<T: ?Sized> borrow::Borrow<T> for Retained<T> {
    fn borrow(&self) -> &T {
        // Auto-derefs
        self
    }
}

impl<T: ?Sized> AsRef<T> for Retained<T> {
    fn as_ref(&self) -> &T {
        // Auto-derefs
        self
    }
}

impl<T: Error + ?Sized> Error for Retained<T> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        (**self).source()
    }
}

impl<T: ?Sized> io::Read for Retained<T>
where
    for<'a> &'a T: io::Read,
{
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        (&**self).read(buf)
    }

    #[inline]
    fn read_vectored(&mut self, bufs: &mut [io::IoSliceMut<'_>]) -> io::Result<usize> {
        (&**self).read_vectored(bufs)
    }

    #[inline]
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        (&**self).read_to_end(buf)
    }

    #[inline]
    fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize> {
        (&**self).read_to_string(buf)
    }

    #[inline]
    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        (&**self).read_exact(buf)
    }
}

impl<'a, T: ?Sized> io::Read for &'a Retained<T>
where
    &'a T: io::Read,
{
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        (&***self).read(buf)
    }

    #[inline]
    fn read_vectored(&mut self, bufs: &mut [io::IoSliceMut<'_>]) -> io::Result<usize> {
        (&***self).read_vectored(bufs)
    }

    #[inline]
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        (&***self).read_to_end(buf)
    }

    #[inline]
    fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize> {
        (&***self).read_to_string(buf)
    }

    #[inline]
    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        (&***self).read_exact(buf)
    }
}

impl<T: ?Sized> io::Write for Retained<T>
where
    for<'a> &'a T: io::Write,
{
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        (&**self).write(buf)
    }

    #[inline]
    fn write_vectored(&mut self, bufs: &[io::IoSlice<'_>]) -> io::Result<usize> {
        (&**self).write_vectored(bufs)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        (&**self).flush()
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        (&**self).write_all(buf)
    }

    #[inline]
    fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) -> io::Result<()> {
        (&**self).write_fmt(fmt)
    }
}

impl<'a, T: ?Sized> io::Write for &'a Retained<T>
where
    &'a T: io::Write,
{
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        (&***self).write(buf)
    }

    #[inline]
    fn write_vectored(&mut self, bufs: &[io::IoSlice<'_>]) -> io::Result<usize> {
        (&***self).write_vectored(bufs)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        (&***self).flush()
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        (&***self).write_all(buf)
    }

    #[inline]
    fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) -> io::Result<()> {
        (&***self).write_fmt(fmt)
    }
}

impl<T: ?Sized> io::Seek for Retained<T>
where
    for<'a> &'a T: io::Seek,
{
    #[inline]
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        (&**self).seek(pos)
    }

    #[inline]
    fn stream_position(&mut self) -> io::Result<u64> {
        (&**self).stream_position()
    }
}

impl<'a, T: ?Sized> io::Seek for &'a Retained<T>
where
    &'a T: io::Seek,
{
    #[inline]
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        (&***self).seek(pos)
    }

    #[inline]
    fn stream_position(&mut self) -> io::Result<u64> {
        (&***self).stream_position()
    }
}

impl<'a, T: ?Sized> Future for &'a Retained<T>
where
    &'a T: Future,
{
    type Output = <&'a T as Future>::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        <&T>::poll(Pin::new(&mut &***self), cx)
    }
}

// TODO: impl Fn traits, CoerceUnsized, Stream and so on when stabilized
