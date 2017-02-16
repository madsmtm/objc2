use core::fmt;

use Encoding;

pub trait Encodings {
    fn eq<C: EncodingsComparator>(&self, C) -> bool;
    fn write_all<W: fmt::Write>(&self, &mut W) -> fmt::Result;
}

pub trait IndexEncodings: Encodings {
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
        impl<$($t: Encoding),*> Encodings for ($($t,)*) {
            #[allow(unused)]
            fn eq<X: EncodingsComparator>(&self, mut fields: X) -> bool {
                let ($(ref $a,)*) = *self;
                $(fields.eq_next($a) &&)* fields.is_finished()
            }

            fn write_all<W: fmt::Write>(&self, formatter: &mut W) -> fmt::Result {
                let ($(ref $a,)*) = *self;
                write!(formatter, fmt_repeat!($($t),*), $($a),*)
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
    fn eq<C: EncodingsComparator>(&self, mut comparator: C) -> bool {
        for enc in self {
            if !comparator.eq_next(enc) {
                return false;
            }
        }
        comparator.is_finished()
    }

    fn write_all<W: fmt::Write>(&self, formatter: &mut W) -> fmt::Result {
        for enc in self {
            write!(formatter, "{}", enc)?;
        }
        Ok(())
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

pub trait EncodingsComparator {
    fn eq_next<T: ?Sized + Encoding>(&mut self, &T) -> bool;
    fn is_finished(&self) -> bool;
}

pub struct IndexEncodingsComparator<'a, T>
        where T: 'a + ?Sized + IndexEncodings {
    encs: &'a T,
    index: u8,
}

impl<'a, T> IndexEncodingsComparator<'a, T>
        where T: 'a + ?Sized + IndexEncodings {
    pub fn new(encs: &'a T) -> IndexEncodingsComparator<'a, T> {
        IndexEncodingsComparator { encs: encs, index: 0 }
    }
}

impl<'a, T> EncodingsComparator for IndexEncodingsComparator<'a, T>
        where T: 'a + ?Sized + IndexEncodings {
    fn eq_next<E: ?Sized + Encoding>(&mut self, other: &E) -> bool {
        let index = self.index;
        if index < self.encs.len() {
            self.index += 1;
            self.encs.encoding_at_eq(index, other)
        } else {
            false
        }
    }

    fn is_finished(&self) -> bool {
        self.index >= self.encs.len()
    }
}
