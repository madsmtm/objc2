use clang::PlatformAvailability;

#[derive(Debug, Clone)]
pub struct Availability {
    inner: Vec<PlatformAvailability>,
}

impl Availability {
    pub fn parse(availability: Vec<PlatformAvailability>) -> Self {
        Self {
            inner: availability,
        }
    }
}
