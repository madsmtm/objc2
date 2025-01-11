#![cfg(feature = "CFDate")]

#[cfg(test)]
mod tests {
    use crate::{CFTimeZoneCopyDefault, CFTimeZoneCopySystem, CFTimeZoneGetName};

    #[test]
    fn cmp() {
        let system = unsafe { CFTimeZoneCopySystem().unwrap() };
        let default = unsafe { CFTimeZoneCopyDefault().unwrap() };
        assert_eq!(system, default);
        assert_eq!(
            unsafe { CFTimeZoneGetName(&system) }.unwrap(),
            unsafe { CFTimeZoneGetName(&default) }.unwrap(),
        );
    }
}
