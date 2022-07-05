use super::Encoding;

pub(crate) const fn static_int_str_len(mut n: u128) -> usize {
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

pub(crate) const fn static_int_str_array<const RES: usize>(mut n: u128) -> [u8; RES] {
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
        n /= 10;
        rev_i += 1;
    }
    rev
}

pub(crate) const fn static_encoding_str_len(encoding: Encoding<'_>) -> usize {
    use Encoding::*;

    match encoding {
        Char | Short | Int | Long | LongLong | UChar | UShort | UInt | ULong | ULongLong
        | Float | Double | LongDouble | Bool | Void | String | Object | Class | Sel | Unknown => 1,
        Block | FloatComplex | DoubleComplex | LongDoubleComplex => 2,
        BitField(b, _type) => 1 + static_int_str_len(b as u128),
        Atomic(&t) | Pointer(&t) => 1 + static_encoding_str_len(t),
        Array(len, &item) => {
            1 + static_int_str_len(len as u128) + static_encoding_str_len(item) + 1
        }
        Struct(name, items) | Union(name, items) => {
            let mut res = 1 + name.len() + 1;
            let mut i = 0;
            while i < items.len() {
                res += static_encoding_str_len(items[i]);
                i += 1;
            }
            res + 1
        }
    }
}

pub(crate) const fn static_encoding_str_array<const LEN: usize>(
    encoding: Encoding<'_>,
) -> [u8; LEN] {
    use Encoding::*;

    let mut res: [u8; LEN] = [0; LEN];

    match encoding {
        Char => res[0] = b'c',
        Short => res[0] = b's',
        Int => res[0] = b'i',
        Long => res[0] = b'l',
        LongLong => res[0] = b'q',
        UChar => res[0] = b'C',
        UShort => res[0] = b'S',
        UInt => res[0] = b'I',
        ULong => res[0] = b'L',
        ULongLong => res[0] = b'Q',
        Float => res[0] = b'f',
        Double => res[0] = b'd',
        LongDouble => res[0] = b'D',
        FloatComplex => {
            res[0] = b'j';
            res[1] = b'f';
        }
        DoubleComplex => {
            res[0] = b'j';
            res[1] = b'd';
        }
        LongDoubleComplex => {
            res[0] = b'j';
            res[1] = b'D';
        }
        Bool => res[0] = b'B',
        Void => res[0] = b'v',
        Block => {
            res[0] = b'@';
            res[1] = b'?';
        }
        String => res[0] = b'*',
        Object => res[0] = b'@',
        Class => res[0] = b'#',
        Sel => res[0] = b':',
        Unknown => res[0] = b'?',
        BitField(b, &_type) => {
            let mut res_i = 0;

            res[res_i] = b'b';
            res_i += 1;

            let mut i = 0;
            // We use 3 even though it creates an oversized array
            let arr = static_int_str_array::<3>(b as u128);
            while i < static_int_str_len(b as u128) {
                res[res_i] = arr[i];
                res_i += 1;
                i += 1;
            }
        }
        Pointer(&t) => {
            let mut res_i = 0;

            res[res_i] = b'^';
            res_i += 1;

            let mut i = 0;
            // We use LEN even though it creates an oversized array
            let arr = static_encoding_str_array::<LEN>(t);
            while i < static_encoding_str_len(t) {
                res[res_i] = arr[i];
                res_i += 1;
                i += 1;
            }
        }
        Atomic(&t) => {
            let mut res_i = 0;

            res[res_i] = b'A';
            res_i += 1;

            let mut i = 0;
            // We use LEN even though it creates an oversized array
            let arr = static_encoding_str_array::<LEN>(t);
            while i < static_encoding_str_len(t) {
                res[res_i] = arr[i];
                res_i += 1;
                i += 1;
            }
        }
        Array(len, &item) => {
            let mut res_i = 0;

            res[res_i] = b'[';
            res_i += 1;

            let mut i = 0;
            // We use 20 even though it creates an oversized array
            let arr = static_int_str_array::<20>(len as u128);
            while i < static_int_str_len(len as u128) {
                res[res_i] = arr[i];
                res_i += 1;
                i += 1;
            }

            let mut i = 0;
            // We use LEN even though it creates an oversized array
            let arr = static_encoding_str_array::<LEN>(item);
            while i < static_encoding_str_len(item) {
                res[res_i] = arr[i];
                res_i += 1;
                i += 1;
            }

            res[res_i] = b']';
        }
        Struct(name, items) | Union(name, items) => {
            let mut res_i = 0;

            match encoding {
                Struct(_, _) => res[res_i] = b'{',
                Union(_, _) => res[res_i] = b'(',
                _ => {}
            };
            res_i += 1;

            let mut name_i = 0;
            let name = name.as_bytes();
            while name_i < name.len() {
                res[res_i] = name[name_i];
                res_i += 1;
                name_i += 1;
            }

            res[res_i] = b'=';
            res_i += 1;

            let mut items_i = 0;
            while items_i < items.len() {
                // We use LEN even though it creates an oversized array
                let field_res = static_encoding_str_array::<LEN>(items[items_i]);

                let mut item_res_i = 0;
                while item_res_i < static_encoding_str_len(items[items_i]) {
                    res[res_i] = field_res[item_res_i];
                    res_i += 1;
                    item_res_i += 1;
                }
                items_i += 1;
            }

            match encoding {
                Struct(_, _) => res[res_i] = b'}',
                Union(_, _) => res[res_i] = b')',
                _ => {}
            };
        }
    };
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! const_int_str {
        ($n:expr) => {{
            const X: [u8; static_int_str_len($n as u128)] = static_int_str_array($n as u128);
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

        assert_eq!(STR_0, "0");
        assert_eq!(STR_4, "4");
        assert_eq!(STR_42, "42");
        assert_eq!(STR_100, "100");
        assert_eq!(STR_999, "999");
        assert_eq!(STR_1236018655, "1236018655");
    }

    // static encoding tests are in `encoding.rs`
}
