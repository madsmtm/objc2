#![cfg(feature = "CFDate")]

#[cfg(test)]
mod tests {
    use crate::{CFTimeZoneCopyDefault, CFTimeZoneCopySystem, CFTimeZoneGetName};

    #[test]
    fn cmp() {
        let system = CFTimeZoneCopySystem().unwrap();
        let default = CFTimeZoneCopyDefault().unwrap();
        assert_eq!(system, default);
        assert_eq!(
            CFTimeZoneGetName(&system).unwrap(),
            CFTimeZoneGetName(&default).unwrap(),
        );
    }
}
