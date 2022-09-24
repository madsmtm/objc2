//! Parsing encodings from their string representation.
#![deny(unsafe_code)]

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

#[derive(Debug, PartialEq, Eq, Hash)]
pub(crate) enum ErrorKind {
    UnexpectedEnd,
    Unknown(u8),
    UnknownAfterComplex(u8),
    ExpectedInteger,
    IntegerTooLarge,
    WrongEndContainer(ContainerKind),
    InvalidIdentifier(ContainerKind),
}

type Result<T, E = ErrorKind> = core::result::Result<T, E>;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub(crate) struct Parser<'a> {
    data: &'a str,
    // Always "behind"/"at" the current character
    split_point: usize,
}

impl<'a> Parser<'a> {
    pub(crate) fn new(data: &'a str) -> Self {
        Self {
            split_point: 0,
            data,
        }
    }

    fn peek(&self) -> Result<u8> {
        self.try_peek().ok_or(ErrorKind::UnexpectedEnd)
    }

    fn try_peek(&self) -> Option<u8> {
        self.data.as_bytes().get(self.split_point).copied()
    }

    fn advance(&mut self) {
        self.split_point += 1;
    }

    fn consume_while(&mut self, mut condition: impl FnMut(u8) -> bool) {
        while let Some(b) = self.try_peek() {
            if condition(b) {
                self.advance();
            } else {
                break;
            }
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.try_peek().is_none()
    }
}

impl Parser<'_> {
    /// Strip leading qualifiers, if any.
    pub(crate) fn strip_leading_qualifiers(&mut self) {
        const QUALIFIERS: &[u8] = &[
            b'r', // const
            b'n', // in
            b'N', // inout
            b'o', // out
            b'O', // bycopy
            b'R', // byref
            b'V', // oneway
                  // b'!', // GCINVISIBLE
        ];

        self.consume_while(|b| QUALIFIERS.contains(&b));
    }

    /// Chomp until we hit a non-digit.
    ///
    /// + and - prefixes are not supported.
    fn chomp_digits(&mut self) -> Result<&str> {
        let old_split_point = self.split_point;

        // Parse first digit (which must be present).
        if !self.peek()?.is_ascii_digit() {
            return Err(ErrorKind::ExpectedInteger);
        }

        // Parse the rest, stopping if we hit a non-digit.
        self.consume_while(|b| b.is_ascii_digit());

        Ok(&self.data[old_split_point..self.split_point])
    }

    fn parse_usize(&mut self) -> Result<usize> {
        self.chomp_digits()?
            .parse()
            .map_err(|_| ErrorKind::IntegerTooLarge)
    }

    fn parse_u8(&mut self) -> Result<u8> {
        self.chomp_digits()?
            .parse()
            .map_err(|_| ErrorKind::IntegerTooLarge)
    }
}

/// Check if the data matches an expected value.
///
/// The errors here aren't currently used, so they're hackily set up.
impl Parser<'_> {
    fn expect_byte(&mut self, byte: u8) -> Option<()> {
        if self.try_peek()? == byte {
            self.advance();
            Some(())
        } else {
            None
        }
    }

    fn expect_str(&mut self, s: &str) -> Option<()> {
        for b in s.as_bytes() {
            self.expect_byte(*b)?;
        }
        Some(())
    }

    fn expect_usize(&mut self, int: usize) -> Option<()> {
        if self.parse_usize().ok()? == int {
            Some(())
        } else {
            None
        }
    }

    pub(crate) fn expect_encoding(&mut self, enc: &Encoding, level: NestingLevel) -> Option<()> {
        let helper = Helper::new(enc, level);
        match helper {
            Helper::Primitive(primitive) => self.expect_str(primitive.to_str()),
            Helper::BitField(b, _type, _level) => {
                // TODO: Use the type on GNUStep (nesting level?)
                self.expect_byte(b'b')?;
                self.expect_usize(b as usize)
            }
            Helper::Indirection(kind, t, level) => {
                self.expect_byte(kind.prefix_byte())?;
                self.expect_encoding(t, level)
            }
            Helper::Array(len, item, level) => {
                self.expect_byte(b'[')?;
                self.expect_usize(len)?;
                self.expect_encoding(item, level)?;
                self.expect_byte(b']')
            }
            Helper::Container(kind, name, items, level) => {
                self.expect_byte(kind.start_byte())?;
                self.expect_str(name)?;
                if let Some(items) = items {
                    self.expect_byte(b'=')?;
                    for item in items {
                        self.expect_encoding(item, level)?;
                    }
                }
                self.expect_byte(kind.end_byte())
            }
        }
    }
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

impl Parser<'_> {
    fn parse_container_name(&mut self, kind: ContainerKind) -> Result<&str> {
        let old_split_point = self.split_point;

        // Parse name until hits `=`
        loop {
            let b = self
                .try_peek()
                .ok_or_else(|| ErrorKind::WrongEndContainer(kind))?;
            if b == b'=' || b == kind.end_byte() {
                break;
            } else {
                self.advance();
            }
        }

        let s = &self.data[old_split_point..self.split_point];

        if !verify_name(s) {
            return Err(ErrorKind::InvalidIdentifier(kind));
        }

        Ok(s)
    }

    #[allow(dead_code)]
    fn parse_token(&mut self) -> Result<Option<EncodingToken<'_>>> {
        Ok(if let Some(b) = self.try_peek() {
            self.advance();
            Some(self.parse_token_inner(b)?)
        } else {
            None
        })
    }

    fn parse_token_inner(&mut self, b: u8) -> Result<EncodingToken<'_>> {
        let prim = EncodingToken::Primitive;

        Ok(match b {
            b'c' => prim(Primitive::Char),
            b's' => prim(Primitive::Short),
            b'i' => prim(Primitive::Int),
            b'l' => prim(Primitive::Long),
            b'q' => prim(Primitive::LongLong),
            b'C' => prim(Primitive::UChar),
            b'S' => prim(Primitive::UShort),
            b'I' => prim(Primitive::UInt),
            b'L' => prim(Primitive::ULong),
            b'Q' => prim(Primitive::ULongLong),
            b'f' => prim(Primitive::Float),
            b'd' => prim(Primitive::Double),
            b'D' => prim(Primitive::LongDouble),
            b'j' => prim({
                let res = match self.peek()? {
                    b'f' => Primitive::FloatComplex,
                    b'd' => Primitive::DoubleComplex,
                    b'D' => Primitive::LongDoubleComplex,
                    b => return Err(ErrorKind::UnknownAfterComplex(b)),
                };
                self.advance();
                res
            }),
            b'B' => prim(Primitive::Bool),
            b'v' => prim(Primitive::Void),
            b'*' => prim(Primitive::String),
            b'@' => prim(match self.try_peek() {
                // Special handling for blocks
                Some(b'?') => {
                    self.advance();
                    Primitive::Block
                }
                _ => Primitive::Object,
            }),
            b'#' => prim(Primitive::Class),
            b':' => prim(Primitive::Sel),
            b'?' => prim(Primitive::Unknown),
            b'b' => EncodingToken::BitFieldStart(self.parse_u8()?),
            b'^' => EncodingToken::Indirection(IndirectionKind::Pointer),
            b'A' => EncodingToken::Indirection(IndirectionKind::Atomic),
            b'[' => EncodingToken::ArrayStart(self.parse_usize()?),
            b']' => EncodingToken::ArrayEnd,
            b'{' => {
                let kind = ContainerKind::Struct;
                let name = self.parse_container_name(kind)?;
                EncodingToken::ContainerStart(kind, name)
            }
            b'(' => {
                let kind = ContainerKind::Union;
                let name = self.parse_container_name(kind)?;
                EncodingToken::ContainerStart(kind, name)
            }
            b'=' => EncodingToken::ContainerSeparator,
            b'}' => EncodingToken::ContainerEnd(ContainerKind::Struct),
            b')' => EncodingToken::ContainerEnd(ContainerKind::Union),
            b => return Err(ErrorKind::Unknown(b)),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_container_name() {
        const KIND: ContainerKind = ContainerKind::Struct;

        fn assert_name(enc: &str, expected: Result<&str>) {
            let mut parser = Parser::new(enc);
            assert_eq!(parser.parse_container_name(KIND), expected);
        }

        assert_name("abc=def", Ok("abc"));
        assert_name("_=.a'", Ok("_"));
        assert_name("abc}def", Ok("abc"));
        assert_name("=def", Err(ErrorKind::InvalidIdentifier(KIND)));
        assert_name(".=def", Err(ErrorKind::InvalidIdentifier(KIND)));
        assert_name("", Err(ErrorKind::WrongEndContainer(KIND)));
        assert_name("abc", Err(ErrorKind::WrongEndContainer(KIND)));
        assert_name("abc)def", Err(ErrorKind::WrongEndContainer(KIND)));
    }
}
