#![cfg(feature = "CGGeometry")]
#![cfg(feature = "std")]
#![cfg(target_vendor = "apple")]

use objc2_core_foundation::{CGFloat, CGPoint, CGRect, CGSize};
use objc2_core_graphics::{CGRectIsEmpty, CGRectStandardize};

#[test]
fn rectangle_helpers_match_core_graphics() {
    let values = [
        CGFloat::NEG_INFINITY,
        -CGFloat::MAX,
        -5.0,
        -CGFloat::MIN_POSITIVE,
        -0.0,
        0.0,
        CGFloat::MIN_POSITIVE,
        5.0,
        CGFloat::MAX,
        CGFloat::INFINITY,
        CGFloat::NAN,
    ];
    for x in values {
        for y in values {
            for width in values {
                for height in values {
                    let rect = CGRect::new(CGPoint::new(x, y), CGSize::new(width, height));
                    assert_eq!(rect.is_empty(), CGRectIsEmpty(rect), "{rect:?}");
                    let actual = rect.standardize();
                    let expected = CGRectStandardize(rect);
                    for (actual, expected) in [
                        (actual.origin.x, expected.origin.x),
                        (actual.origin.y, expected.origin.y),
                        (actual.size.width, expected.size.width),
                        (actual.size.height, expected.size.height),
                    ] {
                        // NaN has no equality relation, but must remain NaN.
                        assert!(
                            actual.to_bits() == expected.to_bits()
                                || (actual.is_nan() && expected.is_nan()),
                            "{rect:?}: expected {expected:?}, got {actual:?}",
                        );
                    }
                }
            }
        }
    }
}
