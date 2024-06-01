#![no_main]
use std::str::FromStr;

use libfuzzer_sys::fuzz_target;
use objc2::encode::{Encoding, EncodingBox};

fuzz_target!(|s: &str| {
    // Limit string length to < 1024 so that we don't hit stack overflows
    if s.len() > 1024 {
        return;
    }
    // Check that parsing encodings doesn't panic
    if let Ok(enc) = EncodingBox::from_str(s) {
        // Check a "negative" case of `equivalent_to_box`
        if enc != EncodingBox::Char {
            assert_ne!(EncodingBox::Char, enc, "not equal to char");
            assert!(
                !Encoding::Char.equivalent_to_box(&enc),
                "not equivalent to char"
            );
        }

        let s2 = enc.to_string();
        // Note: s and s2 may not be equal!

        // Test roundtrip
        let enc2 = EncodingBox::from_str(&s2).expect("parsing valid encoding string");
        let s3 = enc2.to_string();
        assert_eq!(s2, s3);
    }
});
