use objc2::ffi::NSUInteger;

use crate::CoreFoundation::*;

impl CGPoint {
    /// Create a new point with the given coordinates.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use icrate::Foundation::CGPoint;
    /// assert_eq!(CGPoint::new(10.0, -2.3), CGPoint { x: 10.0, y: -2.3 });
    /// ```
    #[inline]
    #[doc(alias = "NSMakePoint")]
    #[doc(alias = "CGPointMake")]
    pub const fn new(x: CGFloat, y: CGFloat) -> Self {
        Self { x, y }
    }

    /// A point with both coordinates set to `0.0`.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use icrate::Foundation::CGPoint;
    /// assert_eq!(CGPoint::ZERO, CGPoint { x: 0.0, y: 0.0 });
    /// ```
    #[doc(alias = "NSZeroPoint")]
    #[doc(alias = "CGPointZero")]
    #[doc(alias = "ORIGIN")]
    pub const ZERO: Self = Self::new(0.0, 0.0);
}

impl CGSize {
    /// Create a new size with the given dimensions.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use icrate::Foundation::CGSize;
    /// let size = CGSize::new(10.0, 2.3);
    /// assert_eq!(size.width, 10.0);
    /// assert_eq!(size.height, 2.3);
    /// ```
    ///
    /// Negative values are allowed (though often undesired).
    ///
    /// ```
    /// use icrate::Foundation::CGSize;
    /// let size = CGSize::new(-1.0, 0.0);
    /// assert_eq!(size.width, -1.0);
    /// ```
    #[inline]
    #[doc(alias = "NSMakeSize")]
    #[doc(alias = "CGSizeMake")]
    pub const fn new(width: CGFloat, height: CGFloat) -> Self {
        // The documentation for NSSize explicitly says:
        // > If the value of width or height is negative, however, the
        // > behavior of some methods may be undefined.
        //
        // But since this type can come from FFI, we'll leave it up to the
        // user to ensure that it is used safely.
        Self { width, height }
    }

    /// Convert the size to a non-negative size.
    ///
    /// This can be used to convert the size to a safe value.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use icrate::Foundation::CGSize;
    /// assert_eq!(CGSize::new(-1.0, 1.0).abs(), CGSize::new(1.0, 1.0));
    /// ```
    #[inline]
    pub fn abs(self) -> Self {
        Self::new(self.width.abs(), self.height.abs())
    }

    /// A size that is 0.0 in both dimensions.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use icrate::Foundation::CGSize;
    /// assert_eq!(CGSize::ZERO, CGSize { width: 0.0, height: 0.0 });
    /// ```
    #[doc(alias = "NSZeroSize")]
    #[doc(alias = "CGSizeZero")]
    pub const ZERO: Self = Self::new(0.0, 0.0);
}

impl CGRect {
    /// Create a new rectangle with the given origin and dimensions.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use icrate::Foundation::{CGPoint, CGRect, CGSize};
    /// let origin = CGPoint::new(10.0, -2.3);
    /// let size = CGSize::new(5.0, 0.0);
    /// let rect = CGRect::new(origin, size);
    /// ```
    #[inline]
    #[doc(alias = "NSMakeRect")]
    #[doc(alias = "CGRectMake")]
    pub const fn new(origin: CGPoint, size: CGSize) -> Self {
        Self { origin, size }
    }

    /// A rectangle with origin (0.0, 0.0) and zero width and height.
    #[doc(alias = "NSZeroRect")]
    #[doc(alias = "CGRectZero")]
    pub const ZERO: Self = Self::new(CGPoint::ZERO, CGSize::ZERO);

    /// Returns a rectangle with a positive width and height.
    ///
    /// This is often useful
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use icrate::Foundation::{CGPoint, CGRect, CGSize};
    /// let origin = CGPoint::new(1.0, 1.0);
    /// let size = CGSize::new(-5.0, -2.0);
    /// let rect = CGRect::new(origin, size);
    /// assert_eq!(rect.standardize().size, CGSize::new(5.0, 2.0));
    /// ```
    #[inline]
    #[doc(alias = "CGRectStandardize")]
    pub fn standardize(self) -> Self {
        Self::new(self.origin, self.size.abs())
    }

    /// The smallest coordinate of the rectangle.
    #[inline]
    #[doc(alias = "CGRectGetMinX")]
    #[doc(alias = "CGRectGetMinY")]
    #[doc(alias = "NSMinX")]
    #[doc(alias = "NSMinY")]
    pub fn min(self) -> CGPoint {
        self.origin
    }

    /// The center point of the rectangle.
    #[inline]
    #[doc(alias = "CGRectGetMidX")]
    #[doc(alias = "CGRectGetMidY")]
    #[doc(alias = "NSMidX")]
    #[doc(alias = "NSMidY")]
    pub fn mid(self) -> CGPoint {
        CGPoint::new(
            self.origin.x + (self.size.width * 0.5),
            self.origin.y + (self.size.height * 0.5),
        )
    }

    /// The largest coordinate of the rectangle.
    #[inline]
    #[doc(alias = "CGRectGetMaxX")]
    #[doc(alias = "CGRectGetMaxY")]
    #[doc(alias = "NSMaxX")]
    #[doc(alias = "NSMaxY")]
    pub fn max(self) -> CGPoint {
        CGPoint::new(
            self.origin.x + self.size.width,
            self.origin.y + self.size.height,
        )
    }

    /// Returns whether a rectangle has zero width or height.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use icrate::Foundation::{CGPoint, CGRect, CGSize};
    /// assert!(CGRect::ZERO.is_empty());
    /// let point = CGPoint::new(1.0, 2.0);
    /// assert!(CGRect::new(point, CGSize::ZERO).is_empty());
    /// assert!(!CGRect::new(point, CGSize::new(1.0, 1.0)).is_empty());
    /// ```
    #[inline]
    #[doc(alias = "CGRectIsEmpty")]
    pub fn is_empty(self) -> bool {
        !(self.size.width > 0.0 && self.size.height > 0.0)
        // TODO: NaN handling?
        // self.size.width <= 0.0 || self.size.height <= 0.0
    }

    // TODO: NSContainsRect / CGRectContainsRect
    // TODO: NSDivideRect / CGRectDivide
    // TODO: NSInsetRect / CGRectInset
    // TODO: NSIntegralRect / CGRectIntegral
    // TODO: NSIntersectionRect / CGRectIntersection
    // TODO: NSUnionRect / CGRectUnion
    // TODO: NSIntersectsRect / CGRectIntersectsRect
    // TODO: NSMouseInRect
    // TODO: NSMouseInRect
    // TODO: NSPointInRect / CGRectContainsPoint
    // TODO: NSOffsetRect / CGRectOffset

    // TODO: CGRectIsNull
    // TODO: CGRectIsInfinite
    // TODO: CGRectInfinite
    // TODO: CGRectNull

    // TODO: NSHeight / CGRectGetHeight (standardized)
    // TODO: NSWidth / CGRectGetWidth (standardized)
}

/// A point in a Cartesian coordinate system.
///
/// This is a convenience alias for [`CGPoint`]. For ease of use, it is
/// available on all platforms, though in practice it is only useful on macOS.
///
/// See [Apple's documentation](https://developer.apple.com/documentation/foundation/nspoint?language=objc).
pub type NSPoint = CGPoint;

/// A two-dimensional size.
///
/// This is a convenience alias for [`CGSize`]. For ease of use, it is
/// available on all platforms, though in practice it is only useful on macOS.
///
/// See [Apple's documentation](https://developer.apple.com/documentation/foundation/nssize?language=objc).
pub type NSSize = CGSize;

/// A rectangle.
///
/// This is a convenience alias for [`CGRect`]. For ease of use, it is
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
        use crate::Foundation::{NSEqualPoints, NSEqualRects, NSEqualSizes};

        // We assume that comparisons handle e.g. `x` and `y` in the same way,
        // therefore we set the coordinates / dimensions to the same.
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
