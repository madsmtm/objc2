//! Tools for parsing Objective-C encodings.
//!
//! TODO: Move these to `objc2-encode` when more stable.
use core::fmt;
use core::num::ParseIntError;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct StackLayout<'a> {
    s: &'a str,
}

impl<'a> StackLayout<'a> {
    fn new(s: &'a str) -> Self {
        Self { s }
    }

    fn extract(types: &'a str) -> (Self, &'a str) {
        let rest = types.trim_start_matches(|c: char| c.is_ascii_digit() || c == '-' || c == '+');
        let stack_layout = &types[..types.len() - rest.len()];
        (Self::new(stack_layout), rest)
    }

    fn to_int(&self) -> Result<Option<i64>, ParseIntError> {
        if self.s.is_empty() {
            return Ok(None);
        }
        // TODO: Unsure of the type here!
        self.s.parse().map(Some)
    }
}

/// The string is approximately equivalent to:
///
/// ```no_run
/// # use objc2::{sel, class};
/// use objc2::runtime::Method;
/// let method: Method;
/// # let sel = sel!(ordinalityOfUnit:inUnit:forDate:);
/// # let method = class!(NSCalendar).instance_method(sel).unwrap();
/// let mut types = method.return_type().to_owned();
/// for i in 0..method.arguments_count() {
///    types.push_str(&method.argument_type(i).unwrap())
/// }
/// let iter = MethodTypesEncodingIter::new(types);
/// ```
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct MethodTypesEncodingIter<'a> {
    s: &'a str,
}

impl<'a> MethodTypesEncodingIter<'a> {
    pub(crate) fn new(s: &'a str) -> Self {
        Self { s }
    }
}

impl<'a> Iterator for MethodTypesEncodingIter<'a> {
    type Item = Result<(&'a str, StackLayout<'a>), EncodingParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.s.is_empty() {
            return None;
        }
        Some(extract_encoding(self.s).map(|(enc, rest)| {
            let (stack_layout, rest) = StackLayout::extract(rest);
            self.s = rest;
            (enc, stack_layout)
        }))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum EncodingParseError {
    UnexpectedEnd,
    Unknown,
    UnknownAfterComplex,
}

impl fmt::Display for EncodingParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Data<'a> {
    // Always "behind"/"at" the current character
    split_point: usize,
    b: &[u8],
}

impl<'a> Data<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            split_point: 0,
            b: s.as_bytes(),
        }
    }

    fn peek(&self) -> Result<u8, EncodingParseError> {
        self.b
            .get(self.split_point)
            .ok_or(EncodingParseError::UnexpectedEnd)
    }

    fn next(&mut self) -> u8 {
        let res = self
            .b
            .get(self.split_point)
            .ok_or(EncodingParseError::UnexpectedEnd);
        self.advance();
        res
    }

    fn advance(&mut self) {
        self.split_point += 1;
    }
}

fn extract_encoding_part(
    data: &mut Data<'a>,
    enclosing: Option<char>,
) -> Result<(), EncodingParseError> {
    match data.peek()? {
        // Primitive
        b'c' | b's' | b'i' | b'l' | b'q' | b'C' | b'S' | b'I' | b'L' | b'Q' | b'f' | b'd'
        | b'D' | b'B' | b'v' | b'*' | b'#' | b':' | b'?' => {
            data.advance();
            Ok(())
        }
        // Primitive 128bit (TODO: Properly support)
        b't' | b'T' => {
            data.advance();
            Ok(())
        }
        // Complex
        b'j' => {
            data.advance();
            match data.peek()? {
                'f' | 'd' | 'D' => {
                    data.advance();
                    Ok(())
                }
                _ => Err(EncodingParseError::UnknownAfterComplex),
            }
        }
        // Object / Block
        b'@' => {
            data.advance();
            match data.peek()? {
                '?' => {
                    // Block
                    data.advance();
                    Ok(())
                }
                _ => {
                    // Object
                    Ok(())
                }
            }
        }
        // Indirection types (atomic + pointer)
        b'A' | b'^' => {
            data.advance();
            // Parse inner item
            return extract_encoding_part(data, enclosing);
        }
        b'b' => {
            // TODO: Parse the type on GNUStep
            todo!()
        }
        // Container (struct + union)
        b'{' | b'(' => {
            res_i += 1;
            todo!()
        }
        b'(' => {}
        // Array
        b'[' => {
            res_i += 1;
            todo!()
        }
        _ => Err(EncodingParseError::Unknown),
    }
}

fn extract_encoding(s: &str) -> Result<(&str, &str), EncodingParseError> {
    // TODO: See objrs' approach:
    // https://gitlab.com/objrs/objrs/-/blob/b4f6598696b3fa622e6fddce7aff281770b0a8c2/src/test.rs
    // https://github.com/gnustep/libobjc2/blob/v2.1/encoding2.c
    // https://github.com/apple-oss-distributions/objc4/blob/objc4-841.13/runtime/objc-typeencoding.mm

    let mut data = Data::new(s);

    // Qualifiers
    // These can only appear at the start of a specific type's encoding
    if let b'r' | b'n' | b'N' | b'o' | b'O' | b'R' | b'V' = data.peek()? {
        // Skip qualifier
        data.advance();
    }

    extract_encoding_part(&mut data, None)?;

    Ok((
        &s[..res_i],
        core::str::from_utf8(&s.as_bytes()[res_i..]).unwrap(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec::Vec;

    fn assert_stack_layout(types: &str, expected: (&str, &str)) {
        let (sl, rest) = StackLayout::extract(types);
        assert_eq!(sl.s, expected.0);
        assert_eq!(rest, expected.1);
    }

    #[test]
    fn stack_layout_extract() {
        assert_stack_layout("", ("", ""));
        assert_stack_layout("abc", ("", "abc"));
        assert_stack_layout("abc12abc", ("", "abc12abc"));
        assert_stack_layout("0", ("0", ""));
        assert_stack_layout("1abc", ("1", "abc"));
        assert_stack_layout("42def24", ("42", "def24"));
        assert_stack_layout("12345678909876543210xyz", ("12345678909876543210", "xyz"));

        assert_stack_layout("-1a", ("-1", "a"));
        assert_stack_layout("-1a", ("-1", "a"));

        // GNU runtime's register parameter hint??
        assert_stack_layout("+1a", ("+1", "a"));
    }

    fn assert_encoding_extract(s: &str, expected: &[(&str, Option<i64>)]) {
        let actual: Vec<_> = MethodTypesEncodingIter::new(s)
            .map(|res| res.map(|(enc, sl)| (enc, sl.to_int().unwrap())))
            .collect::<Result<_, _>>()
            .unwrap();
        assert_eq!(&actual, expected);
    }

    #[test]
    fn extract_encoding_bitfield() {
        assert_encoding_extract(
            "@48@0:8Ad16r^*24{bitfield=b64b1}32i48",
            &[
                ("@", Some(48)),
                ("@", Some(0)),
                (":", Some(8)),
                ("Ad", Some(16)),
                ("r^*", Some(24)),
                ("{bitfield=b64b1}", Some(32)),
                ("i", Some(48)),
            ],
        );
    }

    #[test]
    fn extract_encoding_complex() {
        assert_encoding_extract(
            "jf16@0:8",
            &[("jf", Some(16)), ("@", Some(0)), (":", Some(8))],
        );
        assert_encoding_extract("jf@:", &[("jf", None), ("@", None), (":", None)]);
    }
}
