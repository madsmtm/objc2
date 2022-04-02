use super::Encoding;

pub(crate) const fn static_int_str_len(mut n: u128) -> usize {
    let mut i = 0;
    if n == 0 {
        return 1;
    }
    while n > 0 {
        n = n / 10;
        i += 1;
    }
    i
}

pub(crate) const fn static_int_str_array<const RES: usize>(mut n: u128) -> [u8; RES] {
    let mut res: [u8; RES] = [0; RES];
    let mut i = 0;
    if n == 0 {
        res[0] = '0' as u8;
        return res;
    }
    while n > 0 {
        res[i] = '0' as u8 + (n % 10) as u8;
        n = n / 10;
        i += 1;
    }

    let mut rev: [u8; RES] = [0; RES];
    let mut rev_i = 0;
    while 0 < i {
        i -= 1;
        rev[rev_i] = res[i];
        n = n / 10;
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
        Pointer(&t) => 1 + static_encoding_str_len(t),
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
        Char => res[0] = 'c' as u8,
        Short => res[0] = 's' as u8,
        Int => res[0] = 'i' as u8,
        Long => res[0] = 'l' as u8,
        LongLong => res[0] = 'q' as u8,
        UChar => res[0] = 'C' as u8,
        UShort => res[0] = 'S' as u8,
        UInt => res[0] = 'I' as u8,
        ULong => res[0] = 'L' as u8,
        ULongLong => res[0] = 'Q' as u8,
        Float => res[0] = 'f' as u8,
        Double => res[0] = 'd' as u8,
        LongDouble => res[0] = 'D' as u8,
        FloatComplex => {
            res[0] = 'j' as u8;
            res[1] = 'f' as u8;
        }
        DoubleComplex => {
            res[0] = 'j' as u8;
            res[1] = 'd' as u8;
        }
        LongDoubleComplex => {
            res[0] = 'j' as u8;
            res[1] = 'D' as u8;
        }
        Bool => res[0] = 'B' as u8,
        Void => res[0] = 'v' as u8,
        Block => {
            res[0] = '@' as u8;
            res[1] = '?' as u8;
        }
        String => res[0] = '*' as u8,
        Object => res[0] = '@' as u8,
        Class => res[0] = '#' as u8,
        Sel => res[0] = ':' as u8,
        Unknown => res[0] = '?' as u8,
        BitField(b, &_type) => {
            let mut res_i = 0;

            res[res_i] = 'b' as u8;
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

            res[res_i] = '^' as u8;
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

            res[res_i] = '[' as u8;
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

            res[res_i] = ']' as u8;
        }
        Struct(name, items) | Union(name, items) => {
            let mut res_i = 0;

            match encoding {
                Struct(_, _) => res[res_i] = '{' as u8,
                Union(_, _) => res[res_i] = '(' as u8,
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

            res[res_i] = '=' as u8;
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
                Struct(_, _) => res[res_i] = '}' as u8,
                Union(_, _) => res[res_i] = ')' as u8,
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
            unsafe { core::mem::transmute::<&[u8], &str>(&X) }
        }};
    }

    #[test]
    fn test_const_int_str() {
        const STR_0: &'static str = const_int_str!(0);
        const STR_4: &'static str = const_int_str!(4);
        const STR_42: &'static str = const_int_str!(42);
        const STR_100: &'static str = const_int_str!(100);
        const STR_999: &'static str = const_int_str!(999);
        const STR_1236018655: &'static str = const_int_str!(1236018655);

        assert_eq!(STR_0, "0");
        assert_eq!(STR_4, "4");
        assert_eq!(STR_42, "42");
        assert_eq!(STR_100, "100");
        assert_eq!(STR_999, "999");
        assert_eq!(STR_1236018655, "1236018655");
    }

    macro_rules! const_encoding {
        ($e:expr) => {{
            const E: $crate::Encoding<'static> = $e;
            const X: [u8; static_encoding_str_len(E)] = static_encoding_str_array(E);
            unsafe { core::mem::transmute::<&'static [u8], &'static str>(&X) }
        }};
    }

    #[test]
    fn test_const_encoding() {
        const CHAR: &'static str = const_encoding!(Encoding::Char);
        assert_eq!(CHAR, "c");
        const BLOCK: &'static str = const_encoding!(Encoding::Block);
        assert_eq!(BLOCK, "@?");
        const STRUCT: &'static str =
            const_encoding!(Encoding::Struct("abc", &[Encoding::Int, Encoding::Double]));
        assert_eq!(STRUCT, "{abc=id}");
        const VARIOUS: &'static str = const_encoding!(Encoding::Struct(
            "abc",
            &[
                Encoding::Pointer(&Encoding::Array(8, &Encoding::Bool)),
                Encoding::Union("def", &[Encoding::Block]),
                Encoding::Pointer(&Encoding::Pointer(&Encoding::BitField(255, &Encoding::Int))),
                Encoding::Unknown,
            ]
        ));
        assert_eq!(VARIOUS, "{abc=^[8B](def=@?)^^b255?}");
    }
}
