use clang::PlatformAvailability;

#[derive(Debug, Clone, PartialEq)]
pub struct Availability {
    #[allow(dead_code)]
    inner: Vec<()>,
}

impl Availability {
    pub fn parse(_availability: Vec<PlatformAvailability>) -> Self {
        Self { inner: Vec::new() }
    }
}
