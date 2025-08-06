#![allow(unused_imports, non_snake_case)]
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

#[cfg(feature = "UIView")]
extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicoordinatespace?language=objc)
    pub unsafe trait UICoordinateSpace: NSObjectProtocol + MainThreadOnly {
        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(convertPoint:toCoordinateSpace:))]
        #[unsafe(method_family = none)]
        fn convertPoint_toCoordinateSpace(
            &self,
            point: CGPoint,
            coordinate_space: &ProtocolObject<dyn UICoordinateSpace>,
        ) -> CGPoint;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(convertPoint:fromCoordinateSpace:))]
        #[unsafe(method_family = none)]
        fn convertPoint_fromCoordinateSpace(
            &self,
            point: CGPoint,
            coordinate_space: &ProtocolObject<dyn UICoordinateSpace>,
        ) -> CGPoint;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(convertRect:toCoordinateSpace:))]
        #[unsafe(method_family = none)]
        fn convertRect_toCoordinateSpace(
            &self,
            rect: CGRect,
            coordinate_space: &ProtocolObject<dyn UICoordinateSpace>,
        ) -> CGRect;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(convertRect:fromCoordinateSpace:))]
        #[unsafe(method_family = none)]
        fn convertRect_fromCoordinateSpace(
            &self,
            rect: CGRect,
            coordinate_space: &ProtocolObject<dyn UICoordinateSpace>,
        ) -> CGRect;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(bounds))]
        #[unsafe(method_family = none)]
        fn bounds(&self) -> CGRect;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaxis?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg(feature = "UIGeometry")]
pub struct UIAxis(pub NSUInteger);
bitflags::bitflags! {
    impl UIAxis: NSUInteger {
        #[doc(alias = "UIAxisNeither")]
        const Neither = 0;
        #[doc(alias = "UIAxisHorizontal")]
        const Horizontal = 1<<0;
        #[doc(alias = "UIAxisVertical")]
        const Vertical = 1<<1;
        #[doc(alias = "UIAxisBoth")]
        const Both = UIAxis::Horizontal.0|UIAxis::Vertical.0;
    }
}

#[cfg(feature = "UIGeometry")]
unsafe impl Encode for UIAxis {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

#[cfg(feature = "UIGeometry")]
unsafe impl RefEncode for UIAxis {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uirectedge?language=objc)
// NS_OPTIONS
#[cfg(feature = "UIGeometry")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIRectEdge(pub NSUInteger);
bitflags::bitflags! {
    impl UIRectEdge: NSUInteger {
        #[doc(alias = "UIRectEdgeNone")]
        const None = 0;
        #[doc(alias = "UIRectEdgeTop")]
        const Top = 1<<0;
        #[doc(alias = "UIRectEdgeLeft")]
        const Left = 1<<1;
        #[doc(alias = "UIRectEdgeBottom")]
        const Bottom = 1<<2;
        #[doc(alias = "UIRectEdgeRight")]
        const Right = 1<<3;
        #[doc(alias = "UIRectEdgeAll")]
        const All = UIRectEdge::Top.0|UIRectEdge::Left.0|UIRectEdge::Bottom.0|UIRectEdge::Right.0;
    }
}

#[cfg(feature = "UIGeometry")]
unsafe impl Encode for UIRectEdge {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

#[cfg(feature = "UIGeometry")]
unsafe impl RefEncode for UIRectEdge {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
