#![cfg(feature = "CFDate")]

#[cfg(test)]
mod tests {
    use crate::CFTimeZone;

    #[test]
    fn cmp() {
        let system = CFTimeZone::system();
        let default = CFTimeZone::default();
        assert_eq!(system, default);
        assert_eq!(system.name(), default.name());
    }
}
