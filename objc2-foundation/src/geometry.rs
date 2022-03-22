use objc2::{Encode, Encoding, RefEncode};

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
/// See [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgfloat?language=objc)
/// and the related [`core_graphics_types::base::CGFloat`](https://docs.rs/core-graphics-types/0.1.1/core_graphics_types/base/type.CGFloat.html).
// Defined in CoreGraphics/CGBase.h
// TODO: Use a newtype here?
pub type CGFloat = InnerFloat;

// NSGeometry types are just aliases to CGGeometry types on iOS, tvOS, watchOS
// and macOS 64bit (and hence their Objective-C encodings are different).
#[cfg(all(apple, not(all(target_os = "macos", target_pointer_width = "32"))))]
mod names {
    pub(super) const POINT: &str = "_CGPoint";
    pub(super) const SIZE: &str = "_CGSize";
    pub(super) const RECT: &str = "_CGRect";
}

#[cfg(any(gnustep, all(target_os = "macos", target_pointer_width = "32")))]
mod names {
    pub(super) const POINT: &str = "_NSPoint";
    pub(super) const SIZE: &str = "_NSSize";
    pub(super) const RECT: &str = "_NSRect";
}

/// A point in a Cartesian coordinate system.
///
/// For ease of use, this is available on all platforms, though in practice it
/// is only useful on macOS.
///
/// See [Apple's documentation](https://developer.apple.com/documentation/foundation/nspoint?language=objc).
/// and the related [`core_graphics_types::geometry::CGPoint`](https://docs.rs/core-graphics-types/0.1.1/core_graphics_types/geometry/struct.CGPoint.html).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct NSPoint {
    /// The x-coordinate of the point.
    pub x: CGFloat,
    /// The y-coordinate of the point.
    pub y: CGFloat,
}

unsafe impl Encode for NSPoint {
    const ENCODING: Encoding<'static> =
        Encoding::Struct(names::POINT, &[CGFloat::ENCODING, CGFloat::ENCODING]);
}

unsafe impl RefEncode for NSPoint {
    const ENCODING_REF: Encoding<'static> = Encoding::Pointer(&Self::ENCODING);
}

impl NSPoint {
    /// Create a new point with the given coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// use objc2_foundation::NSPoint;
    /// assert_eq!(NSPoint::new(10.0, -2.3), NSPoint { x: 10.0, y: -2.3 });
    /// ```
    #[inline]
    #[doc(alias = "NSMakePoint")]
    pub const fn new(x: CGFloat, y: CGFloat) -> Self {
        // TODO: Prevent NaN?
        Self { x, y }
    }

    /// A point with both coordinates set to `0.0`.
    #[doc(alias = "NSZeroPoint")]
    #[doc(alias = "ORIGIN")]
    pub const ZERO: Self = Self::new(0.0, 0.0);

    // #[inline]
    // pub const fn x_f64(self) -> f64 {
    //     self.x.into()
    // }

    // #[inline]
    // pub const fn y_f64(self) -> f64 {
    //     self.y.into()
    // }
}

/// A two-dimensional size.
///
/// The width and height are guaranteed to be non-negative, so methods that
/// expect that can safely accept [`NSSize`] as a parameter.
///
/// For ease of use, this is available on all platforms, though in practice it
/// is only useful on macOS.
///
/// See [Apple's documentation](https://developer.apple.com/documentation/foundation/nssize?language=objc).
/// and the related [`core_graphics_types::geometry::CGSize`](https://docs.rs/core-graphics-types/0.1.1/core_graphics_types/geometry/struct.CGSize.html).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct NSSize {
    width: CGFloat,
    height: CGFloat,
}

unsafe impl Encode for NSSize {
    const ENCODING: Encoding<'static> =
        Encoding::Struct(names::SIZE, &[CGFloat::ENCODING, CGFloat::ENCODING]);
}

unsafe impl RefEncode for NSSize {
    const ENCODING_REF: Encoding<'static> = Encoding::Pointer(&Self::ENCODING);
}

impl NSSize {
    /// Create a new size with the given dimensions.
    ///
    /// # Examples
    ///
    /// ```
    /// use objc2_foundation::NSSize;
    /// let size = NSSize::new(10.0, 2.3);
    /// assert_eq!(size.width(), 10.0);
    /// assert_eq!(size.height(), 2.3);
    /// ```
    ///
    /// ```should_panic
    /// use objc2_foundation::NSSize;
    /// let size = NSSize::new(-1.0, 0.0);
    /// ```
    #[inline]
    #[doc(alias = "NSMakeSize")]
    pub fn new(width: CGFloat, height: CGFloat) -> Self {
        // The documentation explicitly says:
        // > If the value of width or height is negative, however, the
        // > behavior of some methods may be undefined.
        //
        // Hence, we _must_ disallow negative widths and heights.

        // TODO: Prevent NaN? Prevent infinities?
        match (width < 0.0, height < 0.0) {
            (true, true) => panic!("NSSize cannot have negative width and height"),
            (true, false) => panic!("NSSize cannot have negative width"),
            (false, true) => panic!("NSSize cannot have negative height"),
            (false, false) => Self { width, height },
        }
    }

    /// A size that is 0.0 in both dimensions.
    #[doc(alias = "NSZeroSize")]
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
/// For ease of use, this is available on all platforms, though in practice it
/// is only useful on macOS.
///
/// See [Apple's documentation](https://developer.apple.com/documentation/foundation/nsrect?language=objc).
/// and the related [`core_graphics_types::geometry::CGRect`](https://docs.rs/core-graphics-types/0.1.1/core_graphics_types/geometry/struct.CGRect.html).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct NSRect {
    /// The coordinates of the rectangle’s origin.
    pub origin: NSPoint,
    /// The dimensions of the rectangle.
    pub size: NSSize,
}

unsafe impl Encode for NSRect {
    const ENCODING: Encoding<'static> =
        Encoding::Struct(names::RECT, &[NSPoint::ENCODING, NSSize::ENCODING]);
}

unsafe impl RefEncode for NSRect {
    const ENCODING_REF: Encoding<'static> = Encoding::Pointer(&Self::ENCODING);
}

impl NSRect {
    /// Create a new rectangle with the given origin and dimensions.
    ///
    /// # Examples
    ///
    /// ```
    /// use objc2_foundation::{NSPoint, NSRect, NSSize};
    /// let origin = NSPoint::new(10.0, -2.3);
    /// let size = NSSize::new(5.0, 0.0);
    /// let rect = NSRect::new(origin, size);
    /// ```
    #[inline]
    #[doc(alias = "NSMakeRect")]
    pub const fn new(origin: NSPoint, size: NSSize) -> Self {
        Self { origin, size }
    }

    /// A rectangle with origin (0.0, 0.0) and zero width and height.
    #[doc(alias = "NSZeroRect")]
    pub const ZERO: Self = Self::new(NSPoint::ZERO, NSSize::ZERO);

    /// The smallest coordinate of the rectangle.
    #[inline]
    pub fn min(self) -> NSPoint {
        self.origin
    }

    /// The center point of the rectangle.
    #[inline]
    pub fn mid(self) -> NSPoint {
        NSPoint::new(
            self.origin.x + (self.size.width * 0.5),
            self.origin.y + (self.size.height * 0.5),
        )
    }

    /// The largest coordinate of the rectangle.
    #[inline]
    pub fn max(self) -> NSPoint {
        NSPoint::new(
            self.origin.x + self.size.width,
            self.origin.y + self.size.height,
        )
    }

    /// Returns whether the area of the rectangle is zero.
    #[inline]
    pub fn is_empty(self) -> bool {
        !(self.size.width > 0.0 && self.size.height > 0.0)
        // TODO: NaN handling?
        // self.size.width <= 0.0 || self.size.height <= 0.0
    }

    // TODO: NSContainsRect
    // TODO: NSDivideRect
    // TODO: NSInsetRect
    // TODO: NSIntegralRect
    // TODO: NSIntersectionRect
    // TODO: NSUnionRect
    // TODO: NSIntersectsRect
    // TODO: NSMouseInRect
    // TODO: NSMouseInRect
    // TODO: NSPointInRect
    // TODO: NSOffsetRect
}

// TODO: struct NSEdgeInsets
// TODO: enum NSRectEdge

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic = "NSSize cannot have negative width and height"]
    fn test_nssize_new_both_negative() {
        NSSize::new(-1.0, -1.0);
    }

    #[test]
    #[should_panic = "NSSize cannot have negative width"]
    fn test_nssize_new_width_negative() {
        NSSize::new(-1.0, 1.0);
    }

    #[test]
    #[should_panic = "NSSize cannot have negative height"]
    fn test_nssize_new_height_negative() {
        NSSize::new(1.0, -1.0);
    }

    #[test]
    fn test_nssize_new() {
        NSSize::new(1.0, 1.0);
        NSSize::new(0.0, -0.0);
        NSSize::new(-0.0, 0.0);
        NSSize::new(-0.0, -0.0);
    }

    // We know the Rust implementation handles NaN, infinite, negative zero
    // and so on properly, so let's ensure that NSEqualXXX handles these as
    // well (so that we're confident that the implementations are equivalent).
    #[test]
    #[cfg(any(all(apple, target_os = "macos"), gnustep))] // or macabi
    fn test_partial_eq() {
        use objc2::runtime::Bool;

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
