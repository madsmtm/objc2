use core::fmt;

use Encoding;

/// Types that can be used as callbacks while iterating over `Encodings`.
///
/// A special trait is necessary rather than a normal closure because
/// the type of each `Encoding` may be different.
pub trait EncodingsIterateCallback {
    /// This method is called for each encoding of an `Encodings`.
    /// The return value indicates whether iteration should stop;
    /// return `true` to stop the iteration before completion.
    fn call<T: ?Sized + Encoding>(&mut self, &T) -> bool;
}

/// Types that represent a collection of `Encoding`s.
pub trait Encodings {
    /// Iterates over the encodings of Self,
    /// calling the provided callback for each.
    fn each<F: EncodingsIterateCallback>(&self, &mut F);

    /// Returns whether Self is equal to the given encodings,
    fn eq_encodings<T: ?Sized + Encodings>(&self, encs: &T) -> bool;

    /// Writes each encoding of Self in order to the given writer.
    fn write_all<W: fmt::Write>(&self, writer: &mut W) -> fmt::Result {
        let mut writer = EncodingsWriter::new(writer);
        self.each(&mut writer);
        writer.result()
    }
}

trait IndexEncodings: Encodings {
    fn encoding_at_eq<T: ?Sized + Encoding>(&self, u8, &T) -> bool;
    fn len(&self) -> u8;
}

macro_rules! count_idents {
    () => (0);
    ($a:ident) => (1);
    ($a:ident, $($b:ident),+) => (1 + count_idents!($($b),*));
}

macro_rules! fmt_repeat {
    () => ("");
    ($a:ident) => ("{}");
    ($a:ident, $($b:ident),+) => (concat!("{}", fmt_repeat!($($b),*)));
}

macro_rules! encodings_impl {
    ($($i:expr => $a:ident : $t:ident),*) => (
        #[allow(unused)]
        impl<$($t: Encoding),*> Encodings for ($($t,)*) {
            fn each<X: EncodingsIterateCallback>(&self, callback: &mut X) {
                let ($(ref $a,)*) = *self;
                $(if callback.call($a) { return; })*
            }

            fn eq_encodings<X: ?Sized + Encodings>(&self, encs: &X) -> bool {
                let mut comparator = IndexEncodingsComparator::new(self);
                encs.each(&mut comparator);
                comparator.was_equal()
            }
        }

        impl<$($t: Encoding),*> IndexEncodings for ($($t,)*) {
            #[allow(unused)]
            fn encoding_at_eq<T: ?Sized + Encoding>(&self, index: u8, other: &T) -> bool {
                let ($(ref $a,)*) = *self;
                match index {
                    $($i => $a.eq_encoding(other),)*
                    _ => false,
                }
            }

            fn len(&self) -> u8 { count_idents!($($t),*) }
        }
    );
}

encodings_impl!();
encodings_impl!(0 => a: A);
encodings_impl!(0 => a: A, 1 => b: B);
encodings_impl!(0 => a: A, 1 => b: B, 2 => c: C);
encodings_impl!(0 => a: A, 1 => b: B, 2 => c: C, 3 => d: D);
encodings_impl!(0 => a: A, 1 => b: B, 2 => c: C, 3 => d: D, 4 => e: E);
encodings_impl!(0 => a: A, 1 => b: B, 2 => c: C, 3 => d: D, 4 => e: E, 5 => f: F);
encodings_impl!(0 => a: A, 1 => b: B, 2 => c: C, 3 => d: D, 4 => e: E, 5 => f: F, 6 => g: G);
encodings_impl!(0 => a: A, 1 => b: B, 2 => c: C, 3 => d: D, 4 => e: E, 5 => f: F, 6 => g: G, 7 => h: H);
encodings_impl!(0 => a: A, 1 => b: B, 2 => c: C, 3 => d: D, 4 => e: E, 5 => f: F, 6 => g: G, 7 => h: H, 8 => i: I);
encodings_impl!(0 => a: A, 1 => b: B, 2 => c: C, 3 => d: D, 4 => e: E, 5 => f: F, 6 => g: G, 7 => h: H, 8 => i: I, 9 => j: J);
encodings_impl!(0 => a: A, 1 => b: B, 2 => c: C, 3 => d: D, 4 => e: E, 5 => f: F, 6 => g: G, 7 => h: H, 8 => i: I, 9 => j: J, 10 => k: K);
encodings_impl!(0 => a: A, 1 => b: B, 2 => c: C, 3 => d: D, 4 => e: E, 5 => f: F, 6 => g: G, 7 => h: H, 8 => i: I, 9 => j: J, 10 => k: K, 11 => l: L);

impl<T> Encodings for [T] where T: Encoding {
    fn each<F: EncodingsIterateCallback>(&self, callback: &mut F) {
        for enc in self {
            if callback.call(enc) { break; }
        }
    }

    fn eq_encodings<E: ?Sized + Encodings>(&self, encs: &E) -> bool {
        let mut comparator = IndexEncodingsComparator::new(self);
        encs.each(&mut comparator);
        comparator.was_equal()
    }
}

impl<T> IndexEncodings for [T] where T: Encoding {
    fn encoding_at_eq<E: ?Sized + Encoding>(&self, index: u8, other: &E) -> bool {
        self.get(index as usize).map_or(false, |e| e.eq_encoding(other))
    }

    fn len(&self) -> u8 {
        // For some reason in no_std self.len() is ambiguous
        // between SliceExt and ExactSizeIterator
        fn get_len<T>(s: &[T]) -> usize { s.len() }

        get_len(self) as u8
    }
}

struct IndexEncodingsComparator<'a, T>
        where T: 'a + ?Sized + IndexEncodings {
    encs: &'a T,
    index: u8,
    all_equal: bool,
}

impl<'a, T> IndexEncodingsComparator<'a, T>
        where T: 'a + ?Sized + IndexEncodings {
    fn new(encs: &T) -> IndexEncodingsComparator<T> {
        IndexEncodingsComparator {
            encs: encs,
            index: 0,
            all_equal: true,
        }
    }

    fn was_equal(self) -> bool {
        self.all_equal && self.index == self.encs.len()
    }
}

impl<'a, T> EncodingsIterateCallback for IndexEncodingsComparator<'a, T>
        where T: 'a + ?Sized + IndexEncodings {
    fn call<E: ?Sized + Encoding>(&mut self, encoding: &E) -> bool {
        let index = self.index;
        if index >= self.encs.len() {
            // stop iteration
            return true;
        }

        self.index += 1;
        if !self.encs.encoding_at_eq(index, encoding) {
            self.all_equal = false;
            // stop iteration
            true
        } else {
            // don't stop iteration
            false
        }
    }
}

struct EncodingsWriter<'a, W> where W: 'a + fmt::Write {
    writer: &'a mut W,
    result: fmt::Result,
}

impl<'a, W> EncodingsWriter<'a, W> where W: 'a + fmt::Write {
    fn new(writer: &mut W) -> EncodingsWriter<W> {
        EncodingsWriter {
            writer: writer,
            result: Ok(()),
        }
    }

    fn result(self) -> fmt::Result {
        self.result
    }
}

impl<'a, W> EncodingsIterateCallback for EncodingsWriter<'a, W>
        where W: 'a + fmt::Write {
    fn call<E: ?Sized + Encoding>(&mut self, encoding: &E) -> bool {
        self.result = write!(self.writer, "{}", encoding);
        // stop iteration if we hit an error
        self.result.is_err()
    }
}
