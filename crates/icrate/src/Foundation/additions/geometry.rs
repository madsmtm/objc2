use crate::CoreFoundation::{CGPoint, CGRect, CGSize};
use objc2::ffi::NSUInteger;

/// A point in a Cartesian coordinate system.
///
/// This is just a convenience alias for [`CGPoint`]. For ease of use, it is
/// available on all platforms, though in practice it is only useful on macOS.
///
/// See [Apple's documentation](https://developer.apple.com/documentation/foundation/nspoint?language=objc).
pub type NSPoint = CGPoint;

/// A two-dimensional size.
///
/// This is just a convenience alias for [`CGSize`]. For ease of use, it is
/// available on all platforms, though in practice it is only useful on macOS.
///
/// See [Apple's documentation](https://developer.apple.com/documentation/foundation/nssize?language=objc).
pub type NSSize = CGSize;

/// A rectangle.
///
/// This is just a convenience alias for [`CGRect`]. For ease of use, it is
/// available on all platforms, though in practice it is only useful on macOS.
///
/// See [Apple's documentation](https://developer.apple.com/documentation/foundation/nsrect?language=objc).
pub type NSRect = CGRect;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSRectEdge {
        NSRectEdgeMinX = 0,
        NSRectEdgeMinY = 1,
        NSRectEdgeMaxX = 2,
        NSRectEdgeMaxY = 3,
        NSMinXEdge = NSRectEdgeMinX,
        NSMinYEdge = NSRectEdgeMinY,
        NSMaxXEdge = NSRectEdgeMaxX,
        NSMaxYEdge = NSRectEdgeMaxY,
    }
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cgsize_new() {
        CGSize::new(1.0, 1.0);
        CGSize::new(0.0, -0.0);
        CGSize::new(-0.0, 0.0);
        CGSize::new(-0.0, -0.0);
        CGSize::new(-1.0, -1.0);
        CGSize::new(-1.0, 1.0);
        CGSize::new(1.0, -1.0);
    }

    // We know the Rust implementation handles NaN, infinite, negative zero
    // and so on properly, so let's ensure that NSEqualXXX handles these as
    // well (so that we're confident that the implementations are equivalent).
    #[test]
    #[cfg(any(all(feature = "apple", target_os = "macos"), feature = "gnustep-1-7"))] // or macabi
    fn test_partial_eq() {
        use crate::CoreFoundation::CGFloat;
        use crate::Foundation::{NSEqualPoints, NSEqualRects, NSEqualSizes};

        // We assume that comparisons handle e.g. `x` and `y` in the same way,
        // therefore we just set the coordinates / dimensions to the same.
        let cases: &[(CGFloat, CGFloat)] = &[
            (0.0, 0.0),
            (-0.0, -0.0),
            (0.0, -0.0),
            (1.0, 1.0 + CGFloat::EPSILON),
            (0.0, CGFloat::MIN_POSITIVE),
            (0.0, CGFloat::EPSILON),
            (1.0, 1.0),
            (1.0, -1.0),
            // Infinity
            (CGFloat::INFINITY, CGFloat::INFINITY),
            (CGFloat::INFINITY, CGFloat::NEG_INFINITY),
            (CGFloat::NEG_INFINITY, CGFloat::NEG_INFINITY),
            // NaN
            (CGFloat::NAN, 0.0),
            (CGFloat::NAN, 1.0),
            (CGFloat::NAN, CGFloat::NAN),
            (CGFloat::NAN, -CGFloat::NAN),
            (-CGFloat::NAN, -CGFloat::NAN),
            (CGFloat::NAN, CGFloat::INFINITY),
        ];

        for case in cases {
            let point_a = NSPoint::new(case.0, case.1);
            let point_b = NSPoint::new(case.0, case.1);
            let actual = unsafe { NSEqualPoints(point_a, point_b).as_bool() };
            assert_eq!(point_a == point_b, actual);

            if case.0 >= 0.0 && case.1 >= 0.0 {
                let size_a = NSSize::new(case.0, case.1);
                let size_b = NSSize::new(case.0, case.1);
                let actual = unsafe { NSEqualSizes(size_a, size_b).as_bool() };
                assert_eq!(size_a == size_b, actual);

                let rect_a = NSRect::new(point_a, size_a);
                let rect_b = NSRect::new(point_b, size_b);
                let actual = unsafe { NSEqualRects(rect_a, rect_b).as_bool() };
                assert_eq!(rect_a == rect_b, actual);
            }
        }
    }
}
