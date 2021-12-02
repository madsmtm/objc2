//! Parsing encodings from their string representation.
#![deny(unsafe_code)]

use crate::Encoding;

pub(crate) const QUALIFIERS: &[char] = &[
    'r', // const
    'n', // in
    'N', // inout
    'o', // out
    'O', // bycopy
    'R', // byref
    'V', // oneway
];

pub(crate) fn rm_enc_prefix<'a>(s: &'a str, enc: &Encoding<'_>) -> Option<&'a str> {
    use Encoding::*;
    let code = match *enc {
        Char => "c",
        Short => "s",
        Int => "i",
        Long => "l",
        LongLong => "q",
        UChar => "C",
        UShort => "S",
        UInt => "I",
        ULong => "L",
        ULongLong => "Q",
        Float => "f",
        Double => "d",
        LongDouble => "D",
        FloatComplex => "jf",
        DoubleComplex => "jd",
        LongDoubleComplex => "jD",
        Bool => "B",
        Void => "v",
        String => "*",
        Object => "@",
        Block => "@?",
        Class => "#",
        Sel => ":",
        Unknown => "?",
        BitField(b) => {
            let s = s.strip_prefix('b')?;
            return rm_int_prefix(s, b as usize);
        }
        Pointer(t) => {
            let s = s.strip_prefix('^')?;
            return rm_enc_prefix(s, t);
        }
        Array(len, item) => {
            let mut s = s;
            s = s.strip_prefix('[')?;
            s = rm_int_prefix(s, len)?;
            s = rm_enc_prefix(s, item)?;
            return s.strip_prefix(']');
        }
        Struct(name, fields) => {
            let mut s = s;
            s = s.strip_prefix('{')?;
            s = s.strip_prefix(name)?;
            s = s.strip_prefix('=')?;
            for field in fields {
                s = rm_enc_prefix(s, field)?;
            }
            return s.strip_prefix('}');
        }
        Union(name, members) => {
            let mut s = s;
            s = s.strip_prefix('(')?;
            s = s.strip_prefix(name)?;
            s = s.strip_prefix('=')?;
            for member in members {
                s = rm_enc_prefix(s, member)?;
            }
            return s.strip_prefix(')');
        }
    };

    s.strip_prefix(code)
}

fn chomp_int(s: &str) -> Option<(usize, &str)> {
    // Chomp until we hit a non-digit
    let (num, t) = match s.find(|c: char| !c.is_digit(10)) {
        Some(i) => s.split_at(i),
        None => (s, ""),
    };
    num.parse().map(|n| (n, t)).ok()
}

fn rm_int_prefix(s: &str, other: usize) -> Option<&str> {
    chomp_int(s).and_then(|(n, t)| if other == n { Some(t) } else { None })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nested() {
        let enc = Encoding::Struct(
            "A",
            &[
                Encoding::Struct("B", &[Encoding::Char, Encoding::Int]),
                Encoding::Char,
                Encoding::Int,
            ],
        );
        assert!(eq_enc("{A={B=ci}ci}", &enc));
        assert!(!eq_enc("{A={B=ci}ci", &enc));
    }

    #[test]
    fn test_bitfield() {
        assert!(eq_enc("b32", &Encoding::BitField(32)));
        assert!(!eq_enc("b", &Encoding::BitField(32)));
        assert!(!eq_enc("b-32", &Encoding::BitField(32)));
    }

    #[test]
    fn test_qualifiers() {
        assert!(eq_enc("Vv", &Encoding::Void));
        assert!(eq_enc("r*", &Encoding::String));
    }

    #[test]
    fn test_unicode() {
        let fields = &[Encoding::Char, Encoding::Int];
        assert!(eq_enc("{☃=ci}", &Encoding::Struct("☃", fields)));
    }
}
