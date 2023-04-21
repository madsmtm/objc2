#![cfg(feature = "Foundation_NSMutableData")]
#[cfg(feature = "block")]
use alloc::vec::Vec;
use core::ops::{Index, IndexMut, Range};
use core::slice::{self, SliceIndex};
use std::io;

use objc2::rc::DefaultId;

use crate::common::*;
use crate::Foundation::{NSMutableData, NSRange};

extern_methods!(
    /// Creation methods
    unsafe impl NSMutableData {
        #[method_id(new)]
        pub fn new() -> Id<Self>;

        pub fn with_bytes(bytes: &[u8]) -> Id<Self> {
            let bytes_ptr = bytes.as_ptr() as *mut c_void;
            unsafe { Self::initWithBytes_length(Self::alloc(), bytes_ptr, bytes.len()) }
        }

        #[cfg(feature = "block")]
        pub fn from_vec(bytes: Vec<u8>) -> Id<Self> {
            unsafe { Id::cast(super::data::with_vec(Self::class(), bytes)) }
        }
    }

    /// Mutation methods
    unsafe impl NSMutableData {
        #[doc(alias = "mutableBytes")]
        pub fn bytes_mut(&mut self) -> &mut [u8] {
            if let Some(ptr) = self.mutableBytes() {
                let ptr: *mut u8 = ptr.as_ptr().cast();
                unsafe { slice::from_raw_parts_mut(ptr, self.len()) }
            } else {
                // The bytes pointer may be null for length zero on GNUStep
                &mut []
            }
        }

        #[doc(alias = "appendBytes:length:")]
        pub fn extend_from_slice(&mut self, bytes: &[u8]) {
            let bytes_ptr: NonNull<c_void> =
                NonNull::new(bytes.as_ptr() as *mut u8).unwrap().cast();
            unsafe { self.appendBytes_length(bytes_ptr, bytes.len()) }
        }

        pub fn push(&mut self, byte: u8) {
            self.extend_from_slice(&[byte])
        }

        #[doc(alias = "replaceBytesInRange:withBytes:length:")]
        pub fn replace_range(&mut self, range: Range<usize>, bytes: &[u8]) {
            let range = NSRange::from(range);
            // No need to verify the length of the range here,
            // `replaceBytesInRange:` just zero-fills if out of bounds.
            let ptr = bytes.as_ptr() as *mut c_void;
            unsafe { self.replaceBytesInRange_withBytes_length(range, ptr, bytes.len()) }
        }

        pub fn set_bytes(&mut self, bytes: &[u8]) {
            let len = self.len();
            self.replace_range(0..len, bytes);
        }
    }
);

impl AsRef<[u8]> for NSMutableData {
    fn as_ref(&self) -> &[u8] {
        self.bytes()
    }
}

impl AsMut<[u8]> for NSMutableData {
    fn as_mut(&mut self) -> &mut [u8] {
        self.bytes_mut()
    }
}

impl<I: SliceIndex<[u8]>> Index<I> for NSMutableData {
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        Index::index(self.bytes(), index)
    }
}

impl<I: SliceIndex<[u8]>> IndexMut<I> for NSMutableData {
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        IndexMut::index_mut(self.bytes_mut(), index)
    }
}

// impl FromIterator<u8> for Id<NSMutableData> {
//     fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
//         let iter = iter.into_iter();
//         let (lower, _) = iter.size_hint();
//         let data = Self::with_capacity(lower);
//         for item in iter {
//             data.push(item);
//         }
//         data
//     }
// }

impl Extend<u8> for NSMutableData {
    /// You should use [`extend_from_slice`] whenever possible, it is more
    /// performant.
    ///
    /// [`extend_from_slice`]: Self::extend_from_slice
    fn extend<T: IntoIterator<Item = u8>>(&mut self, iter: T) {
        let iterator = iter.into_iter();
        iterator.for_each(move |item| self.push(item));
    }
}

// Vec also has this impl
impl<'a> Extend<&'a u8> for NSMutableData {
    fn extend<T: IntoIterator<Item = &'a u8>>(&mut self, iter: T) {
        let iterator = iter.into_iter();
        iterator.for_each(move |item| self.push(*item));
    }
}

impl io::Write for NSMutableData {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.extend_from_slice(buf);
        Ok(())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl DefaultId for NSMutableData {
    #[inline]
    fn default_id() -> Id<Self> {
        Self::new()
    }
}

impl<'a> IntoIterator for &'a NSMutableData {
    type Item = &'a u8;
    type IntoIter = core::slice::Iter<'a, u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.bytes().iter()
    }
}

impl<'a> IntoIterator for &'a mut NSMutableData {
    type Item = &'a mut u8;
    type IntoIter = core::slice::IterMut<'a, u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.bytes_mut().iter_mut()
    }
}
