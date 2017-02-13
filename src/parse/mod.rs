mod encoding;
mod ptr_encoding;
mod struct_encoding;

use encoding::Primitive;

pub use self::encoding::StrEncoding;
pub use self::ptr_encoding::StrPointerEncoding;
pub use self::struct_encoding::StrStructEncoding;

pub fn chomp(s: &str) -> (Option<&str>, &str) {
    let head_len = chomp_ptr(s)
        .or_else(|| chomp_struct(s))
        .or_else(|| {
            if let (Some(_), t) = chomp_primitive(s) {
                Some(s.len() - t.len())
            } else {
                None
            }
        });

    if let Some(head_len) = head_len {
        let (h, t) = s.split_at(head_len);
        (Some(h), t)
    } else {
        (None, s)
    }
}

fn chomp_ptr(s: &str) -> Option<usize> {
    if s.starts_with("^") {
        let (h, _) = chomp(&s[1..]);
        h.map(|h| h.len() + 1)
    } else {
        None
    }
}

fn chomp_struct(s: &str) -> Option<usize> {
    if !s.starts_with("{") {
        return None;
    }

    let mut depth = 1;
    for (i, b) in s.bytes().enumerate().skip(1) {
        if b == b'{' {
            depth += 1;
        } else if b == b'}' {
            depth -= 1;
        }

        if depth == 0 {
            return Some(i + 1);
        }
    }

    None
}

fn chomp_primitive(s: &str) -> (Option<Primitive>, &str) {
    if s.is_empty() {
        return (None, s);
    }

    let (h, t) = s.split_at(1);
    match h {
        "c" => (Some(Primitive::Char), t),
        "i" => (Some(Primitive::Int), t),
        _ => (None, s),
    }
}

enum ParseResult {
    Primitive(Primitive),
    Pointer,
    Struct,
    Error,
}

fn parse(s: &str) -> ParseResult {
    if s.starts_with('{') && s.ends_with('}') {
        ParseResult::Struct
    } else if s.starts_with('^') {
        ParseResult::Pointer
    } else {
        let (h, t) = chomp_primitive(s);
        if !t.is_empty() {
            ParseResult::Error
        } else if let Some(p) = h {
            ParseResult::Primitive(p)
        } else {
            ParseResult::Error
        }
    }
}

fn parse_struct(s: &str) -> Option<(&str, &str)> {
    if let Some(sep_pos) = s.find('=') {
        let name = &s[1..sep_pos];
        let fields = &s[sep_pos + 1..s.len() - 1];
        Some((name, fields))
    } else {
        None
    }
}

fn is_valid(s: &str) -> bool {
    match parse(s) {
        ParseResult::Primitive(_) => true,
        ParseResult::Pointer => {
            let pointee = &s[1..];
            is_valid(pointee)
        },
        ParseResult::Struct => {
            let mut fields = match parse_struct(s) {
                Some((_, fields)) => fields,
                _ => return false,
            };
            while !fields.is_empty() {
                let (h, t) = chomp(fields);
                if h.map_or(false, is_valid) {
                    return false;
                }
                fields = t;
            }
            true
        }
        ParseResult::Error => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chomp() {
        let (h, t) = chomp("{A={B=ci^{C=c}}ci}c^i{C=c}");
        assert_eq!(h, Some("{A={B=ci^{C=c}}ci}"));

        let (h, t) = chomp(t);
        assert_eq!(h, Some("c"));

        let (h, t) = chomp(t);
        assert_eq!(h, Some("^i"));

        let (h, t) = chomp(t);
        assert_eq!(h, Some("{C=c}"));

        let (h, _) = chomp(t);
        assert_eq!(h, None);
    }
}