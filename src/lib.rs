use std::fmt;

pub trait Encoding { }

pub struct Int;

impl Encoding for Int { }
impl fmt::Debug for Int {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&'i', formatter)
    }
}
