//! Test various ivars that use array-like types.
use core::str::FromStr;
use std::boxed::Box;
use std::vec::Vec;

use objc2::encode::EncodingBox;
use objc2::ClassType;
use objc2_foundation::{NSDecimalNumber, NSThread};

/// Defined in the header as:
/// ```objc
/// @interface NSDecimalNumber : NSNumber {
/// @private
///     signed   int _exponent:8;
///     unsigned int _length:4;
///     unsigned int _isNegative:1;
///     unsigned int _isCompact:1;
///     unsigned int _reserved:1;
///     unsigned int _hasExternalRefCount:1;
///     unsigned int _refs:16;
///     unsigned short _mantissa[];
/// }
/// ```
#[test]
#[cfg_attr(feature = "gnustep-1-7", ignore = "bitfields have types")]
fn nsdecimal_ivar_encoding() {
    let cls = NSDecimalNumber::class();
    let expected = |last| {
        [
            ("_exponent", EncodingBox::BitField(8, None)),
            ("_length", EncodingBox::BitField(4, None)),
            ("_isNegative", EncodingBox::BitField(1, None)),
            ("_isCompact", EncodingBox::BitField(1, None)),
            ("_reserved", EncodingBox::BitField(1, None)),
            ("_hasExternalRefCount", EncodingBox::BitField(1, None)),
            ("_refs", EncodingBox::BitField(16, None)),
            ("_mantissa", last),
        ]
    };

    // Incomplete arrays -> pointer here on newer Foundation versions, but
    // -> zero-sized array on older versions.
    let expected1 = expected(EncodingBox::Pointer(Box::new(EncodingBox::UShort)));
    let expected2 = expected(EncodingBox::Array(0, Box::new(EncodingBox::UShort)));

    let actual: Vec<_> = (*cls.instance_variables())
        .iter()
        .map(|ivar| {
            (
                ivar.name().to_str().unwrap(),
                EncodingBox::from_str(ivar.type_encoding().to_str().unwrap()).unwrap(),
            )
        })
        .collect();
    assert!(expected1 == *actual || expected2 == *actual, "{actual:#?}");
}

/// Defined in the header as:
/// ```objc
/// @interface NSThread : NSObject  {
/// @private
///     id _private;
///     uint8_t _bytes[44];
/// }
/// ```
#[test]
fn nsthread_ivar_encoding() {
    let cls = NSThread::class();
    let expected = [
        ("_private", EncodingBox::Object),
        // Arrays do not decay here.
        (
            "_bytes",
            EncodingBox::Array(44, Box::new(EncodingBox::UChar)),
        ),
    ];
    let actual: Vec<_> = (*cls.instance_variables())
        .iter()
        .map(|ivar| {
            (
                ivar.name().to_str().unwrap(),
                EncodingBox::from_str(ivar.type_encoding().to_str().unwrap()).unwrap(),
            )
        })
        .collect();
    assert_eq!(expected, *actual);
}
