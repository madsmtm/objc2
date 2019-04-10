//! Parsing encodings from their string representation.

use crate::Encoding;

const QUALIFIERS: &'static [char] = &[
    'r', // const
    'n', // in
    'N', // inout
    'o', // out
    'O', // bycopy
    'R', // byref
    'V', // oneway
];

fn rm_enc_prefix<'a>(s: &'a str, enc: &Encoding) -> Result<&'a str, ()> {
    use Encoding::*;
    let code = match *enc {
        Char      => "c",
        Short     => "s",
        Int       => "i",
        Long      => "l",
        LongLong  => "q",
        UChar     => "C",
        UShort    => "S",
        UInt      => "I",
        ULong     => "L",
        ULongLong => "Q",
        Float     => "f",
        Double    => "d",
        Bool      => "B",
        Void      => "v",
        String    => "*",
        Object    => "@",
        Block     => "@?",
        Class     => "#",
        Sel       => ":",
        Unknown   => "?",
        BitField(b) => {
            let s = rm_prefix(s, "b")?;
            return rm_int_prefix(s, b);
        }
        Pointer(t) => {
            let s = rm_prefix(s, "^")?;
            return rm_enc_prefix(s, t);
        }
        Array(len, item) => {
            let mut s = s;
            s = rm_prefix(s, "[")?;
            s = rm_int_prefix(s, len)?;
            s = rm_enc_prefix(s, item)?;
            return rm_prefix(s, "]");
        }
        Struct(name, fields) => {
            let mut s = s;
            s = rm_prefix(s, "{")?;
            s = rm_prefix(s, name)?;
            s = rm_prefix(s, "=")?;
            for field in fields {
                s = rm_enc_prefix(s, field)?;
            }
            return rm_prefix(s, "}");
        }
        Union(name, members) => {
            let mut s = s;
            s = rm_prefix(s, "(")?;
            s = rm_prefix(s, name)?;
            s = rm_prefix(s, "=")?;
            for member in members {
                s = rm_enc_prefix(s, member)?;
            }
            return rm_prefix(s, ")");
        }
    };

    rm_prefix(s, code)
}

fn rm_int_prefix(s: &str, other: u32) -> Result<&str, ()> {
    let (num, t) = match s.find(|c: char| !c.is_digit(10)) {
        Some(i) => s.split_at(i),
        None => (s, ""),
    };
    num.parse()
        .map_err(|_| ())
        .and_then(|n| if other == n { Ok(t) } else { Err(()) })
}

fn rm_prefix<'a>(s: &'a str, other: &str) -> Result<&'a str, ()> {
    if s.starts_with(other) {
        Ok(&s[other.len()..])
    } else {
        Err(())
    }
}

pub fn eq_enc(s: &str, enc: &Encoding) -> bool {
    // strip qualifiers
    let s = s.trim_start_matches(QUALIFIERS);

    // if the given encoding can be successfully removed from the start
    // and an empty string remains, they were equal!
    rm_enc_prefix(s, enc).map(str::is_empty).unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nested() {
        let enc = Encoding::Struct("A", &[
            Encoding::Struct("B", &[
                Encoding::Char,
                Encoding::Int,
            ]),
            Encoding::Char,
            Encoding::Int,
        ]);
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
