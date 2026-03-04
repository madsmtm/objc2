//! Convert encodings to strings at `const`-time.

use crate::{
    helper::{ContainerKind, Helper, NestingLevel},
    parse::verify_name,
};

use super::Encoding;

/// Length of the integer when formatted in base 10.
///
/// This will never return more than `20`.
const fn static_int_str_len(mut n: u64) -> usize {
    let mut i = 0;
    if n == 0 {
        return 1;
    }
    while n > 0 {
        n /= 10;
        i += 1;
    }
    i
}

/// Integer formatted in base 10 and placed in an array.
const fn static_int_str_array<const RES: usize>(mut n: u64) -> [u8; RES] {
    let mut res: [u8; RES] = [0; RES];
    let mut i = 0;
    if n == 0 {
        res[0] = b'0';
        return res;
    }
    while n > 0 {
        res[i] = b'0' + (n % 10) as u8;
        n /= 10;
        i += 1;
    }

    let mut rev: [u8; RES] = [0; RES];
    let mut rev_i = 0;
    while 0 < i {
        i -= 1;
        rev[rev_i] = res[i];
        rev_i += 1;
    }
    rev
}

/// Length of the encoding string.
pub(crate) const fn static_encoding_str_len(encoding: &Encoding, level: NestingLevel) -> usize {
    match Helper::new(encoding) {
        Helper::Primitive(primitive) => primitive.to_str().len(),
        Helper::BitField(size, None) => 1 + static_int_str_len(size as u64),
        Helper::BitField(size, Some((offset, t))) => {
            1 + static_int_str_len(*offset)
                + static_encoding_str_len(t, level.bitfield())
                + static_int_str_len(size as u64)
        }
        Helper::Indirection(kind, t) => 1 + static_encoding_str_len(t, level.indirection(kind)),
        Helper::Array(len, item) => {
            1 + static_int_str_len(len) + static_encoding_str_len(item, level.array()) + 1
        }
        Helper::Container(_, name, items) => {
            // Don't verify name here, we'll check it in `str_array` instead.
            // This reduces the amount of post-mono errors the user will get.
            let mut res = 1 + name.len();
            if let Some(level) = level.container_include_fields() {
                res += 1;
                let mut i = 0;
                while i < items.len() {
                    res += static_encoding_str_len(&items[i], level);
                    i += 1;
                }
            }
            res + 1
        }
        Helper::NoneInvalid => 0,
    }
}

/// Encoding string as an array.
// TODO(MSRV 1.83): Write to `&mut [u8]` instead to avoid having to allocate
// an appropriately sized array at every usage site.
pub(crate) const fn static_encoding_str_array<const LEN: usize>(
    encoding: &Encoding,
    level: NestingLevel,
) -> [u8; LEN] {
    let mut res: [u8; LEN] = [0; LEN];
    let mut res_i = 0;

    match Helper::new(encoding) {
        Helper::Primitive(primitive) => {
            let s = primitive.to_str().as_bytes();
            let mut i = 0;
            while i < s.len() {
                // Copy from `s` to `res`.
                // This is a general pattern that's used a bunch below.
                res[i] = s[i];
                i += 1;
            }
        }
        Helper::BitField(size, None) => {
            res[res_i] = b'b';
            res_i += 1;

            // We use 3 even though it creates an oversized array.
            let arr = static_int_str_array::<3>(size as u64);
            let len = static_int_str_len(size as u64);
            let mut i = 0;
            while i < len {
                res[res_i] = arr[i];
                res_i += 1;
                i += 1;
            }
        }
        Helper::BitField(size, Some((offset, t))) => {
            let level = level.bitfield();
            res[res_i] = b'b';
            res_i += 1;

            // We use 20 even though it creates an oversized array.
            let arr = static_int_str_array::<20>(*offset);
            let len = static_int_str_len(*offset);
            let mut i = 0;
            while i < len {
                res[res_i] = arr[i];
                res_i += 1;
                i += 1;
            }

            // We use LEN even though it creates an oversized array.
            // This could probably be reduced to 1, do we need to support
            // bitfields with arbitrary encodings?
            let arr = static_encoding_str_array::<LEN>(t, level);
            let len = static_encoding_str_len(t, level);
            let mut i = 0;
            while i < len {
                res[res_i] = arr[i];
                res_i += 1;
                i += 1;
            }

            // We use 3 even though it creates an oversized array.
            let arr = static_int_str_array::<3>(size as u64);
            let len = static_int_str_len(size as u64);
            let mut i = 0;
            while i < len {
                res[res_i] = arr[i];
                res_i += 1;
                i += 1;
            }
        }
        Helper::Indirection(kind, t) => {
            let level = level.indirection(kind);
            res[res_i] = kind.prefix_byte();
            res_i += 1;

            // We use LEN even though it creates an oversized array.
            let arr = static_encoding_str_array::<LEN>(t, level);
            let len = static_encoding_str_len(t, level);
            let mut i = 0;
            while i < len {
                res[res_i] = arr[i];
                res_i += 1;
                i += 1;
            }
        }
        Helper::Array(len, item) => {
            let level = level.array();

            res[res_i] = b'[';
            res_i += 1;

            // We use 20 even though it creates an oversized array.
            let arr = static_int_str_array::<20>(len);
            let len = static_int_str_len(len);
            let mut i = 0;
            while i < len {
                res[res_i] = arr[i];
                res_i += 1;
                i += 1;
            }

            // We use LEN even though it creates an oversized array.
            let arr = static_encoding_str_array::<LEN>(item, level);
            let len = static_encoding_str_len(item, level);
            let mut i = 0;
            while i < len {
                res[res_i] = arr[i];
                res_i += 1;
                i += 1;
            }

            res[res_i] = b']';
        }
        Helper::Container(kind, name, items) => {
            if !verify_name(name) {
                match kind {
                    ContainerKind::Struct => panic!("struct name was not a valid identifier"),
                    ContainerKind::Union => panic!("union name was not a valid identifier"),
                }
            }

            res[res_i] = kind.start_byte();
            res_i += 1;

            let name = name.as_bytes();
            let mut name_i = 0;
            while name_i < name.len() {
                res[res_i] = name[name_i];
                res_i += 1;
                name_i += 1;
            }

            if let Some(level) = level.container_include_fields() {
                res[res_i] = b'=';
                res_i += 1;

                let mut items_i = 0;
                while items_i < items.len() {
                    // We use LEN even though it creates an oversized array.
                    let field_res = static_encoding_str_array::<LEN>(&items[items_i], level);
                    let len = static_encoding_str_len(&items[items_i], level);
                    let mut item_res_i = 0;
                    while item_res_i < len {
                        res[res_i] = field_res[item_res_i];
                        res_i += 1;
                        item_res_i += 1;
                    }
                    items_i += 1;
                }
            }

            res[res_i] = kind.end_byte();
        }
        Helper::NoneInvalid => {}
    };
    res
}

#[cfg(test)]
mod tests {
    use std::string::ToString;

    use super::*;

    macro_rules! const_int_str {
        ($n:expr) => {{
            const X: [u8; static_int_str_len($n)] = static_int_str_array($n);
            unsafe { core::str::from_utf8_unchecked(&X) }
        }};
    }

    #[test]
    fn test_const_int_str() {
        const STR_0: &str = const_int_str!(0);
        const STR_4: &str = const_int_str!(4);
        const STR_42: &str = const_int_str!(42);
        const STR_100: &str = const_int_str!(100);
        const STR_999: &str = const_int_str!(999);
        const STR_1236018655: &str = const_int_str!(1236018655);
        const STR_MAX: &str = const_int_str!(u64::MAX);

        assert_eq!(STR_0, "0");
        assert_eq!(STR_4, "4");
        assert_eq!(STR_42, "42");
        assert_eq!(STR_100, "100");
        assert_eq!(STR_999, "999");
        assert_eq!(STR_1236018655, "1236018655");
        assert_eq!(STR_MAX, u64::MAX.to_string());
    }

    #[test]
    fn test_str_len_max() {
        assert_eq!(static_int_str_len(u8::MAX as _), 3);
        assert_eq!(static_int_str_len(u16::MAX as _), 5);
        assert_eq!(static_int_str_len(u32::MAX as _), 10);
        assert_eq!(static_int_str_len(u64::MAX), 20);
    }

    // static encoding tests are in `encoding.rs`
}
