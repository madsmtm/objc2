use crate::{Encode, Encoding, RefEncode};

#[cfg(target_pointer_width = "64")]
type InnerFloat = f64;
#[cfg(not(target_pointer_width = "64"))]
type InnerFloat = f32;

/// The basic type for all floating-point values.
///
/// This is [`f32`] on 32-bit platforms and [`f64`] on 64-bit platforms.
///
/// This technically belongs to the `CoreGraphics` framework, but we define it
/// here for convenience.
///
/// See [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgfloat?language=objc).
// Defined in CoreGraphics/CGBase.h
// TODO: Use a newtype here?
pub type CGFloat = InnerFloat;

// NSGeometry types are just aliases to CGGeometry types on iOS, tvOS, watchOS
// and macOS 64bit (and hence their Objective-C encodings are different).
//
// TODO: Adjust `objc2-encode` so that this is handled there, and so that we
// can effectively just forget about it and use `NS` and `CG` types equally.
#[cfg(all(
    feature = "apple",
    not(all(target_os = "macos", target_pointer_width = "32"))
))]
mod names {
    pub(super) const POINT: &str = "CGPoint";
    pub(super) const SIZE: &str = "CGSize";
    pub(super) const RECT: &str = "CGRect";
}

#[cfg(any(
    feature = "gnustep-1-7",
    all(target_os = "macos", target_pointer_width = "32")
))]
mod names {
    pub(super) const POINT: &str = "_NSPoint";
    pub(super) const SIZE: &str = "_NSSize";
    pub(super) const RECT: &str = "_NSRect";
}

/// A point in a two-dimensional coordinate system.
///
/// This technically belongs to the `CoreGraphics` framework, but we define it
/// here for convenience.
///
/// See [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cgpoint?language=objc).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct CGPoint {
    /// The x-coordinate of the point.
    pub x: CGFloat,
    /// The y-coordinate of the point.
    pub y: CGFloat,
}

unsafe impl Encode for CGPoint {
    const ENCODING: Encoding =
        Encoding::Struct(names::POINT, &[CGFloat::ENCODING, CGFloat::ENCODING]);
}

unsafe impl RefEncode for CGPoint {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

impl CGPoint {
    /// Create a new point with the given coordinates.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use objc2::foundation::CGPoint;
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
    /// use objc2::foundation::CGPoint;
    /// assert_eq!(CGPoint::ZERO, CGPoint { x: 0.0, y: 0.0 });
    /// ```
    #[doc(alias = "NSZeroPoint")]
    #[doc(alias = "CGPointZero")]
    #[doc(alias = "ORIGIN")]
    pub const ZERO: Self = Self::new(0.0, 0.0);
}

/// A two-dimensional size.
///
/// The width and height are guaranteed to be non-negative, so methods that
/// expect that can safely accept [`CGSize`] as a parameter.
///
/// This technically belongs to the `CoreGraphics` framework, but we define it
/// here for convenience.
///
/// See [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cgsize?language=objc).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct CGSize {
    width: CGFloat,
    height: CGFloat,
}

unsafe impl Encode for CGSize {
    const ENCODING: Encoding =
        Encoding::Struct(names::SIZE, &[CGFloat::ENCODING, CGFloat::ENCODING]);
}

unsafe impl RefEncode for CGSize {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

impl CGSize {
    /// Create a new size with the given dimensions.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use objc2::foundation::CGSize;
    /// let size = CGSize::new(10.0, 2.3);
    /// assert_eq!(size.width(), 10.0);
    /// assert_eq!(size.height(), 2.3);
    /// ```
    ///
    /// ```should_panic
    /// use objc2::foundation::CGSize;
    /// let size = CGSize::new(-1.0, 0.0);
    /// ```
    #[inline]
    #[doc(alias = "NSMakeSize")]
    #[doc(alias = "CGSizeMake")]
    pub fn new(width: CGFloat, height: CGFloat) -> Self {
        // The documentation explicitly says:
        // > If the value of width or height is negative, however, the
        // > behavior of some methods may be undefined.
        //
        // Hence, we _must_ disallow negative widths and heights.

        // TODO: Prevent NaN? Prevent infinities?
        match (width < 0.0, height < 0.0) {
            (true, true) => panic!("CGSize cannot have negative width and height"),
            (true, false) => panic!("CGSize cannot have negative width"),
            (false, true) => panic!("CGSize cannot have negative height"),
            (false, false) => Self { width, height },
        }
    }

    /// A size that is 0.0 in both dimensions.
    #[doc(alias = "NSZeroSize")]
    #[doc(alias = "CGSizeZero")]
    pub const ZERO: Self = Self {
        width: 0.0,
        height: 0.0,
    };

    #[inline]
    pub const fn width(self) -> CGFloat {
        self.width
    }

    #[inline]
    pub const fn height(self) -> CGFloat {
        self.height
    }
}

/// The location and dimensions of a rectangle.
///
/// In the default Core Graphics coordinate space (macOS), the origin is
/// located in the lower-left corner of the rectangle and the rectangle
/// extends towards the upper-right corner.
///
/// If the context has a flipped coordinate space (iOS, tvOS, watchOS) the
/// origin is in the upper-left corner and the rectangle extends towards the
/// lower-right corner.
///
/// This technically belongs to the `CoreGraphics` framework, but we define it
/// here for convenience.
///
/// See [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cgrect?language=objc).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct CGRect {
    /// The coordinates of the rectangle’s origin.
    pub origin: CGPoint,
    /// The dimensions of the rectangle.
    pub size: CGSize,
}

unsafe impl Encode for CGRect {
    const ENCODING: Encoding =
        Encoding::Struct(names::RECT, &[CGPoint::ENCODING, CGSize::ENCODING]);
}

unsafe impl RefEncode for CGRect {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

impl CGRect {
    /// Create a new rectangle with the given origin and dimensions.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use objc2::foundation::{CGPoint, CGRect, CGSize};
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
    /// use objc2::foundation::{CGPoint, CGRect, CGSize};
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

    // TODO: CGRectStandardize
    // TODO: CGRectIsNull
    // TODO: CGRectIsInfinite
    // TODO: CGRectInfinite
    // TODO: CGRectNull

    // TODO: NSHeight / CGRectGetHeight (standardized)
    // TODO: NSWidth / CGRectGetWidth (standardized)
}

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

// TODO: struct NSEdgeInsets
// TODO: enum NSRectEdge

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic = "CGSize cannot have negative width and height"]
    fn test_cgsize_new_both_negative() {
        CGSize::new(-1.0, -1.0);
    }

    #[test]
    #[should_panic = "CGSize cannot have negative width"]
    fn test_cgsize_new_width_negative() {
        CGSize::new(-1.0, 1.0);
    }

    #[test]
    #[should_panic = "CGSize cannot have negative height"]
    fn test_cgsize_new_height_negative() {
        CGSize::new(1.0, -1.0);
    }

    #[test]
    fn test_cgsize_new() {
        CGSize::new(1.0, 1.0);
        CGSize::new(0.0, -0.0);
        CGSize::new(-0.0, 0.0);
        CGSize::new(-0.0, -0.0);
    }

    // We know the Rust implementation handles NaN, infinite, negative zero
    // and so on properly, so let's ensure that NSEqualXXX handles these as
    // well (so that we're confident that the implementations are equivalent).
    #[test]
    #[cfg(any(all(feature = "apple", target_os = "macos"), feature = "gnustep-1-7"))] // or macabi
    fn test_partial_eq() {
        use crate::runtime::Bool;

        // Note: No need to use "C-unwind"
        extern "C" {
            fn NSEqualPoints(a: NSPoint, b: NSPoint) -> Bool;
            fn NSEqualSizes(a: NSSize, b: NSSize) -> Bool;
            fn NSEqualRects(a: NSRect, b: NSRect) -> Bool;
        }

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
