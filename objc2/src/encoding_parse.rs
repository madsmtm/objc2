//! Tools for parsing Objective-C encodings.
//!
//! TODO: Move these to `objc2-encode` when more stable.
use core::fmt;
use core::num::ParseIntError;
use core::str;
use std::error::Error;

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

    fn extract_encoding(&mut self) -> Result<(&'a str, Option<isize>), EncodingParseError<'a>> {
        // TODO: See objrs' approach:
        // https://gitlab.com/objrs/objrs/-/blob/b4f6598696b3fa622e6fddce7aff281770b0a8c2/src/test.rs
        // https://github.com/gnustep/libobjc2/blob/v2.1/encoding2.c
        // https://github.com/apple-oss-distributions/objc4/blob/objc4-841.13/runtime/objc-typeencoding.mm

        let mut data = Data::new(self.s);

        // Qualifiers
        // These can only appear at the start of a specific type's encoding
        // Note: We know this can't fail, since it is checked in `Iterator`
        if let b'r' | b'n' | b'N' | b'o' | b'O' | b'R' | b'V' = data.peek().unwrap() {
            // Skip qualifier
            data.advance();
        }

        data.extract_encoding_part()?;

        let (enc, rest) = data.split_final();

        let (stack_layout, rest) = StackLayout::extract(rest);
        self.s = rest;

        let stack_layout = stack_layout
            .to_int()
            .map_err(|_| EncodingParseError::InvalidStackLayoutInteger)?;
        Ok((enc, stack_layout))
    }
}

impl<'a> Iterator for MethodTypesEncodingIter<'a> {
    type Item = Result<(&'a str, Option<isize>), EncodingParseError<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.s.is_empty() {
            return None;
        }
        Some(self.extract_encoding())
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
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

    fn to_int(&self) -> Result<Option<isize>, ParseIntError> {
        if self.s.is_empty() {
            return Ok(None);
        }
        // TODO: Unsure of the type here!
        self.s.parse().map(Some)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub(crate) struct Data<'a> {
    // Always "behind"/"at" the current character
    split_point: usize,
    b: &'a [u8],
}

impl fmt::Display for Data<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "byte-index {} in {}",
            self.split_point,
            str::from_utf8(self.b).unwrap()
        )
    }
}

impl<'a> Data<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            split_point: 0,
            b: s.as_bytes(),
        }
    }

    fn peek(&self) -> Result<u8, EncodingParseError<'a>> {
        self.try_peek()
            .ok_or(EncodingParseError::UnexpectedEnd(self.clone()))
    }

    fn try_peek(&self) -> Option<u8> {
        self.b.get(self.split_point).cloned()
    }

    fn advance(&mut self) {
        self.split_point += 1;
    }

    fn strip_int(&mut self) -> Result<(), EncodingParseError<'a>> {
        // + and - are not supported

        if !(self.peek()? as char).is_ascii_digit() {
            return Err(EncodingParseError::ExpectedInteger(self.clone()));
        }

        while let Some(b) = self.try_peek() {
            if (b as char).is_ascii_digit() {
                self.advance();
            }
            break;
        }

        Ok(())
    }

    fn extract_encoding_part(&mut self) -> Result<(), EncodingParseError<'a>> {
        match self.peek()? {
            // Primitive
            b'c' | b's' | b'i' | b'l' | b'q' | b'C' | b'S' | b'I' | b'L' | b'Q' | b'f' | b'd'
            | b'D' | b'B' | b'v' | b'*' | b'#' | b':' | b'?' => {
                self.advance();
                Ok(())
            }
            // Primitive 128bit (TODO: Properly support)
            b't' | b'T' => {
                self.advance();
                Ok(())
            }
            // Complex
            b'j' => {
                self.advance();
                match self.peek()? {
                    b'f' | b'd' | b'D' => {
                        self.advance();
                        Ok(())
                    }
                    b => Err(EncodingParseError::UnknownAfterComplex(self.clone(), b)),
                }
            }
            // Object / Block
            b'@' => {
                self.advance();
                match self.peek()? {
                    b'?' => {
                        // Block
                        self.advance();
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
                self.advance();
                // Parse inner item
                self.extract_encoding_part()
            }
            // Bitfield (can only appear inside struct/union)
            b'b' => {
                // TODO: Parse the type on GNUStep
                self.advance();
                self.strip_int()?;
                Ok(())
            }
            // Array
            b'[' => {
                self.advance();
                self.strip_int()?;
                self.extract_encoding_part()?;
                match self.peek()? {
                    b']' => {
                        self.advance();
                        Ok(())
                    }
                    _ => Err(EncodingParseError::WrongEndArray(self.clone())),
                }
            }
            // Container (struct + union)
            b @ (b'{' | b'(') => {
                self.advance();

                let end = match b {
                    b'{' => b'}',
                    b'(' => b')',
                    _ => unreachable!(),
                };

                // Parse struct name until hits `=`
                loop {
                    if let Some(b) = self.try_peek() {
                        if b == b'=' {
                            self.advance();
                            break;
                        } else if b == end {
                            // Premature end, struct has no content
                            break;
                        } else {
                            // Part of struct name
                            self.advance();
                        }
                    } else {
                        return Err(EncodingParseError::WrongEndContainer(self.clone()));
                    }
                }

                // Parse items (if any) until hits end
                loop {
                    if let Some(b) = self.try_peek() {
                        if b == end {
                            self.advance();
                            break;
                        } else {
                            // Wasn't the end, so try to extract one more encoding
                            self.extract_encoding_part()?;
                        }
                    } else {
                        return Err(EncodingParseError::WrongEndContainer(self.clone()));
                    }
                }

                Ok(())
            }
            b => Err(EncodingParseError::Unknown(self.clone(), b)),
        }
    }

    fn split_final(self) -> (&'a str, &'a str) {
        str::from_utf8(self.b).unwrap().split_at(self.split_point)
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub(crate) enum EncodingParseError<'a> {
    UnexpectedEnd(Data<'a>),
    Unknown(Data<'a>, u8),
    UnknownAfterComplex(Data<'a>, u8),
    ExpectedInteger(Data<'a>),
    WrongEndArray(Data<'a>),
    WrongEndContainer(Data<'a>),
    InvalidStackLayoutInteger,
}

impl fmt::Display for EncodingParseError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed parsing encoding: ")?;
        match self {
            Self::UnexpectedEnd(data) => write!(f, "unexpected end at {}", data)?,
            Self::Unknown(data, b) => {
                write!(f, "unknown encoding character {} at {}", *b as char, data)?
            }
            Self::UnknownAfterComplex(data, b) => write!(
                f,
                "unknown encoding character {} after complex at {}",
                *b as char, data
            )?,
            Self::ExpectedInteger(data) => write!(f, "Expected integer at {}", data)?,
            Self::WrongEndArray(data) => write!(f, "Expected array to be closed at {}", data)?,
            Self::WrongEndContainer(data) => {
                write!(f, "Expected union/struct to be closed at {}", data)?
            }
            Self::InvalidStackLayoutInteger => write!(f, "Invalid integer for stack layout")?,
        }
        write!(f, ". This is likely a bug, please report it!")
    }
}

impl Error for EncodingParseError<'_> {}

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

    fn assert_encoding_extract(s: &str, expected: &[(&str, Option<isize>)]) {
        let actual: Vec<_> = MethodTypesEncodingIter::new(s)
            .collect::<Result<_, _>>()
            .unwrap();
        assert_eq!(&actual, expected);
    }

    #[test]
    fn parse_bitfield() {
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
    fn parse_complex() {
        assert_encoding_extract(
            "jf16@0:8",
            &[("jf", Some(16)), ("@", Some(0)), (":", Some(8))],
        );
        assert_encoding_extract("jf@:", &[("jf", None), ("@", None), (":", None)]);
    }
}
