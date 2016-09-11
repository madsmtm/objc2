use std::fmt;

pub trait Encoding: fmt::Display { }

pub struct Int;
impl Encoding for Int { }
impl fmt::Display for Int {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&'i', formatter)
    }
}

pub struct Pointer<T>(T);
impl<T: Encoding> Encoding for Pointer<T> { }
impl<T: fmt::Display> fmt::Display for Pointer<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "^{}", self.0)
    }
}

trait ParserCompletion {
    fn did_parse<E: 'static + Encoding>(self, encoding: E);
}

struct Parser<C> {
    completion: C,
}

impl<C: ParserCompletion> Parser<C> {
    fn parse(self, input: &str) {
        let mut chars = input.chars();
        let c = match chars.next() {
            Some(c) => c,
            None => return,
        };

        match c {
            'i' => self.completion.did_parse(Int),
            '^' => self.parse_pointer(chars.as_str()),
            _ => (),
        }
    }

    fn parse_pointer(self, input: &str) {
        struct PointerCompletion<C>(C);

        impl<C: ParserCompletion> ParserCompletion for PointerCompletion<C> {
            fn did_parse<E: 'static + Encoding>(self, encoding: E) {
                self.0.did_parse(Pointer(encoding));
            }
        }

        let completion = PointerCompletion(self.completion);
        let parser = Parser { completion: completion };
        parser.parse(input);
    }
}

struct BoxedEncodingCompletion<F> {
    f: F,
}

impl<F> ParserCompletion for BoxedEncodingCompletion<F>
        where F: FnOnce(Box<Encoding>) {
    fn did_parse<E: 'static + Encoding>(self, encoding: E) {
        (self.f)(Box::new(encoding))
    }
}

pub fn parse(input: &str) -> Box<Encoding> {
    let mut enc = None;
    {
        let enc_ref = &mut enc;
        let callback = |e| {
            *enc_ref = Some(e);
        };
        let completion = BoxedEncodingCompletion { f: callback };
        let parser = Parser { completion: completion };
        parser.parse(input);
    }
    enc.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_display() {
        assert_eq!(Int.to_string(), "i");
    }

    #[test]
    fn test_pointer_display() {
        assert_eq!(Pointer(Int).to_string(), "^i");
    }

    #[test]
    fn test_parse_int() {
        assert_eq!(parse("i").to_string(), "i");
    }
}
