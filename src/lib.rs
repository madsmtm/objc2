use std::fmt;

pub trait Encoding { }

pub struct Int;
impl Encoding for Int { }
impl fmt::Display for Int {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&'i', formatter)
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
        self.completion.did_parse(Int);
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
}
