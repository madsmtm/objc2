//! Helpers to construct encoding strings for methods at `const`-time.
use objc2_encode::Encoding;

pub const fn method_encoding_str_len(ret: &Encoding, args: &[Encoding]) -> usize {
    let mut len = 0;
    len += ret.str_len();
    len += 1; // Encoding::Object
    len += 1; // Encoding::Sel
    let mut i = 0;
    while i < args.len() {
        len += args[i].str_len();
        i += 1;
    }
    len
}

pub const fn method_encoding_str_array<const LEN: usize>(
    ret: &Encoding,
    args: &[Encoding],
) -> [u8; LEN] {
    let mut res: [u8; LEN] = [0; LEN];
    let mut res_i = 0;

    let mut i = 0;
    // We use LEN even though it creates an oversized array.
    let arr = ret.str_array::<LEN>();
    while i < ret.str_len() {
        res[res_i] = arr[i];
        i += 1;
        res_i += 1;
    }

    // Encoding::Object
    res[res_i] = b'@';
    res_i += 1;

    // Encoding::Sel
    res[res_i] = b':';
    res_i += 1;

    let mut i = 0;
    while i < args.len() {
        let mut j = 0;
        // We use LEN even though it creates an oversized array.
        let arr = args[i].str_array::<LEN>();
        while j < args[i].str_len() {
            res[res_i] = arr[j];
            j += 1;
            res_i += 1;
        }
        i += 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encode::{Encode, EncodeReturn};
    use crate::runtime::{method_type_encoding, Bool, NSObject};

    #[track_caller]
    fn check(ret: &Encoding, args: &[Encoding]) {
        // Compare against reference impl
        let expected = method_type_encoding(ret, args).into_string().unwrap();

        let len = method_encoding_str_len(ret, args);
        assert_eq!(expected.len(), len, "incorrect length");
        assert!(len < 100);

        let arr = method_encoding_str_array::<100>(ret, args);
        let (s, rest) = arr.split_at(len);
        let s = str::from_utf8(s).unwrap();
        assert_eq!(expected, s, "incorrect output: {arr:?}");

        assert!(rest.iter().all(|x| *x == 0), "rest must be zero: {arr:?}");
    }

    #[test]
    fn various() {
        check(&<()>::ENCODING_RETURN, &[]);
        check(
            &<*const NSObject>::ENCODING_RETURN,
            &[<&NSObject>::ENCODING],
        );
        check(
            &<&&&&&&i32>::ENCODING_RETURN,
            &[<&&&&&&NSObject>::ENCODING, <&&&&&&Bool>::ENCODING],
        );
    }
}
