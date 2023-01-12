#![cfg(feature = "Foundation_NSUUID")]
use icrate::Foundation::NSUUID;

#[test]
fn test_new() {
    let uuid1 = NSUUID::UUID();
    let uuid2 = NSUUID::UUID();
    assert_ne!(uuid1, uuid2, "Statistically very unlikely");
}

#[test]
fn test_bytes() {
    let uuid = NSUUID::from_bytes([10; 16]);
    assert_eq!(uuid.as_bytes(), [10; 16]);
}

#[test]
#[cfg(feature = "Foundation_NSString")]
fn display_debug() {
    let uuid = NSUUID::from_bytes([10; 16]);
    let expected = "0A0A0A0A-0A0A-0A0A-0A0A-0A0A0A0A0A0A";
    assert_eq!(format!("{uuid}"), expected);
    assert_eq!(format!("{uuid:?}"), expected);
}

// #[test]
// fn test_compare() {
//     let uuid1 = NSUUID::from_bytes([10; 16]);
//     let uuid2 = NSUUID::from_bytes([9; 16]);
//     assert!(uuid1 > uuid2);
// }

#[cfg(feature = "uuid")]
#[test]
fn test_convert_roundtrip() {
    let nsuuid1 = NSUUID::UUID();
    let uuid = nsuuid1.as_uuid();
    let nsuuid2 = NSUUID::from_uuid(uuid);
    assert_eq!(nsuuid1, nsuuid2);
}
