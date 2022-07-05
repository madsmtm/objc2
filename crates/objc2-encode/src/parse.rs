//! Parsing encodings from their string representation.
#![deny(unsafe_code)]

use core::convert::TryInto;

use crate::helper::{ContainerKind, Helper, IndirectionKind, NestingLevel, Primitive};
use crate::Encoding;

/// Check whether a struct or union name is a valid identifier
pub(crate) const fn verify_name(name: &str) -> bool {
    let bytes = name.as_bytes();

    if let b"?" = bytes {
        return true;
    }

    if bytes.len() == 0 {
        return false;
    }

    let mut i = 0;
    while i < bytes.len() {
        let byte = bytes[i];
        if !(byte.is_ascii_alphanumeric() || byte == b'_') {
            return false;
        }
        i += 1;
    }
    true
}

pub(crate) const QUALIFIERS: &[char] = &[
    'r', // const
    'n', // in
    'N', // inout
    'o', // out
    'O', // bycopy
    'R', // byref
    'V', // oneway
];

pub(crate) fn rm_enc_prefix<'a>(
    s: &'a str,
    enc: &Encoding,
    level: NestingLevel,
) -> Option<&'a str> {
    use Helper::*;
    match Helper::new(enc) {
        Primitive(primitive) => s.strip_prefix(primitive.to_str()),
        BitField(b, _type) => {
            // TODO: Use the type on GNUStep (nesting level?)
            let s = s.strip_prefix('b')?;
            rm_int_prefix(s, b as usize)
        }
        Indirection(kind, t) => {
            let s = s.strip_prefix(kind.prefix())?;
            rm_enc_prefix(s, t, level.indirection(kind))
        }
        Array(len, item) => {
            let mut s = s;
            s = s.strip_prefix('[')?;
            s = rm_int_prefix(s, len)?;
            s = rm_enc_prefix(s, item, level.array())?;
            s.strip_prefix(']')
        }
        Container(kind, name, fields) => {
            let mut s = s;
            s = s.strip_prefix(kind.start())?;
            s = s.strip_prefix(name)?;
            if level.include_container_fields() {
                s = s.strip_prefix('=')?;
                for field in fields {
                    s = rm_enc_prefix(s, field, level.container())?;
                }
            }
            s.strip_prefix(kind.end())
        }
    }
}

fn chomp_int(s: &str) -> Option<(usize, &str)> {
    // Chomp until we hit a non-digit
    let (num, t) = match s.find(|c: char| !c.is_ascii_digit()) {
        Some(i) => s.split_at(i),
        None => (s, ""),
    };
    num.parse().map(|n| (n, t)).ok()
}

fn rm_int_prefix(s: &str, other: usize) -> Option<&str> {
    chomp_int(s).and_then(|(n, t)| if other == n { Some(t) } else { None })
}

#[allow(unused)]
enum EncodingToken<'a> {
    Primitive(Primitive),
    BitFieldStart(u8),
    Indirection(IndirectionKind),
    ArrayStart(usize),
    ArrayEnd,
    ContainerStart(ContainerKind, &'a str),
    ContainerSeparator,
    ContainerEnd(ContainerKind),
}

#[allow(unused)]
fn chomp(s: &str) -> Option<(EncodingToken<'_>, &str)> {
    let (first, rest) = {
        let mut chars = s.chars();
        match chars.next() {
            Some(first) => (first, chars.as_str()),
            None => return None,
        }
    };

    let primitive = match first {
        'c' => Primitive::Char,
        's' => Primitive::Short,
        'i' => Primitive::Int,
        'l' => Primitive::Long,
        'q' => Primitive::LongLong,
        'C' => Primitive::UChar,
        'S' => Primitive::UShort,
        'I' => Primitive::UInt,
        'L' => Primitive::ULong,
        'Q' => Primitive::ULongLong,
        'f' => Primitive::Float,
        'd' => Primitive::Double,
        'D' => Primitive::LongDouble,
        'j' => {
            if let Some(rest) = rest.strip_prefix('f') {
                return Some((EncodingToken::Primitive(Primitive::FloatComplex), rest));
            }
            if let Some(rest) = rest.strip_prefix('d') {
                return Some((EncodingToken::Primitive(Primitive::DoubleComplex), rest));
            }
            if let Some(rest) = rest.strip_prefix('D') {
                return Some((EncodingToken::Primitive(Primitive::LongDoubleComplex), rest));
            }
            return None;
        }
        'B' => Primitive::Bool,
        'v' => Primitive::Void,
        '*' => Primitive::String,
        '@' => {
            // Special handling for blocks
            if let Some(rest) = rest.strip_prefix('?') {
                return Some((EncodingToken::Primitive(Primitive::Block), rest));
            }
            Primitive::Object
        }
        '#' => Primitive::Class,
        ':' => Primitive::Sel,
        '?' => Primitive::Unknown,
        'b' => {
            return chomp_int(rest)
                .map(|(b, rest)| (EncodingToken::BitFieldStart(b.try_into().unwrap()), rest));
        }
        '^' => return Some((EncodingToken::Indirection(IndirectionKind::Pointer), rest)),
        'A' => return Some((EncodingToken::Indirection(IndirectionKind::Atomic), rest)),
        '[' => {
            return chomp_int(rest).map(|(n, rest)| (EncodingToken::ArrayStart(n), rest));
        }
        ']' => return Some((EncodingToken::ArrayEnd, rest)),
        '{' => {
            return rest.split_once('=').map(|(name, rest)| {
                (
                    EncodingToken::ContainerStart(ContainerKind::Struct, name),
                    rest,
                )
            });
        }
        '}' => return Some((EncodingToken::ContainerEnd(ContainerKind::Struct), rest)),
        '(' => {
            return rest.split_once('=').map(|(name, rest)| {
                (
                    EncodingToken::ContainerStart(ContainerKind::Union, name),
                    rest,
                )
            });
        }
        ')' => return Some((EncodingToken::ContainerEnd(ContainerKind::Union), rest)),
        _ => return None,
    };
    Some((EncodingToken::Primitive(primitive), rest))
}
