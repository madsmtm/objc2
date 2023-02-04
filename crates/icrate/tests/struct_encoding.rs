#[test]
#[cfg(feature = "Foundation_NSProcessInfo")]
#[cfg(not(feature = "gnustep-1-7"))]
fn test_operating_system_version() {
    let info = icrate::Foundation::NSProcessInfo::processInfo();
    let _version = info.operatingSystemVersion();
}

#[test]
#[cfg(feature = "Metal")]
fn test_packed_float() {
    use objc2::encode::Encode;
    assert_eq!(
        icrate::Metal::MTLPackedFloat4x3::ENCODING.to_string(),
        "{_MTLPackedFloat4x3=[4{_MTLPackedFloat3=(?={?=fff}[3f])}]}",
    );
}
