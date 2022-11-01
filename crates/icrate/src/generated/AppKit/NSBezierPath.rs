//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;

pub type NSLineCapStyle = NSUInteger;
pub const NSLineCapStyleButt: NSLineCapStyle = 0;
pub const NSLineCapStyleRound: NSLineCapStyle = 1;
pub const NSLineCapStyleSquare: NSLineCapStyle = 2;

pub type NSLineJoinStyle = NSUInteger;
pub const NSLineJoinStyleMiter: NSLineJoinStyle = 0;
pub const NSLineJoinStyleRound: NSLineJoinStyle = 1;
pub const NSLineJoinStyleBevel: NSLineJoinStyle = 2;

pub type NSWindingRule = NSUInteger;
pub const NSWindingRuleNonZero: NSWindingRule = 0;
pub const NSWindingRuleEvenOdd: NSWindingRule = 1;

pub type NSBezierPathElement = NSUInteger;
pub const NSBezierPathElementMoveTo: NSBezierPathElement = 0;
pub const NSBezierPathElementLineTo: NSBezierPathElement = 1;
pub const NSBezierPathElementCurveTo: NSBezierPathElement = 2;
pub const NSBezierPathElementClosePath: NSBezierPathElement = 3;

extern_class!(
    #[derive(Debug)]
    pub struct NSBezierPath;

    unsafe impl ClassType for NSBezierPath {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSBezierPath {
        #[method_id(@__retain_semantics Other bezierPath)]
        pub unsafe fn bezierPath() -> Id<NSBezierPath, Shared>;

        #[method_id(@__retain_semantics Other bezierPathWithRect:)]
        pub unsafe fn bezierPathWithRect(rect: NSRect) -> Id<NSBezierPath, Shared>;

        #[method_id(@__retain_semantics Other bezierPathWithOvalInRect:)]
        pub unsafe fn bezierPathWithOvalInRect(rect: NSRect) -> Id<NSBezierPath, Shared>;

        #[method_id(@__retain_semantics Other bezierPathWithRoundedRect:xRadius:yRadius:)]
        pub unsafe fn bezierPathWithRoundedRect_xRadius_yRadius(
            rect: NSRect,
            xRadius: CGFloat,
            yRadius: CGFloat,
        ) -> Id<NSBezierPath, Shared>;

        #[method(fillRect:)]
        pub unsafe fn fillRect(rect: NSRect);

        #[method(strokeRect:)]
        pub unsafe fn strokeRect(rect: NSRect);

        #[method(clipRect:)]
        pub unsafe fn clipRect(rect: NSRect);

        #[method(strokeLineFromPoint:toPoint:)]
        pub unsafe fn strokeLineFromPoint_toPoint(point1: NSPoint, point2: NSPoint);

        #[method(drawPackedGlyphs:atPoint:)]
        pub unsafe fn drawPackedGlyphs_atPoint(packedGlyphs: NonNull<c_char>, point: NSPoint);

        #[method(defaultMiterLimit)]
        pub unsafe fn defaultMiterLimit() -> CGFloat;

        #[method(setDefaultMiterLimit:)]
        pub unsafe fn setDefaultMiterLimit(defaultMiterLimit: CGFloat);

        #[method(defaultFlatness)]
        pub unsafe fn defaultFlatness() -> CGFloat;

        #[method(setDefaultFlatness:)]
        pub unsafe fn setDefaultFlatness(defaultFlatness: CGFloat);

        #[method(defaultWindingRule)]
        pub unsafe fn defaultWindingRule() -> NSWindingRule;

        #[method(setDefaultWindingRule:)]
        pub unsafe fn setDefaultWindingRule(defaultWindingRule: NSWindingRule);

        #[method(defaultLineCapStyle)]
        pub unsafe fn defaultLineCapStyle() -> NSLineCapStyle;

        #[method(setDefaultLineCapStyle:)]
        pub unsafe fn setDefaultLineCapStyle(defaultLineCapStyle: NSLineCapStyle);

        #[method(defaultLineJoinStyle)]
        pub unsafe fn defaultLineJoinStyle() -> NSLineJoinStyle;

        #[method(setDefaultLineJoinStyle:)]
        pub unsafe fn setDefaultLineJoinStyle(defaultLineJoinStyle: NSLineJoinStyle);

        #[method(defaultLineWidth)]
        pub unsafe fn defaultLineWidth() -> CGFloat;

        #[method(setDefaultLineWidth:)]
        pub unsafe fn setDefaultLineWidth(defaultLineWidth: CGFloat);

        #[method(moveToPoint:)]
        pub unsafe fn moveToPoint(&self, point: NSPoint);

        #[method(lineToPoint:)]
        pub unsafe fn lineToPoint(&self, point: NSPoint);

        #[method(curveToPoint:controlPoint1:controlPoint2:)]
        pub unsafe fn curveToPoint_controlPoint1_controlPoint2(
            &self,
            endPoint: NSPoint,
            controlPoint1: NSPoint,
            controlPoint2: NSPoint,
        );

        #[method(closePath)]
        pub unsafe fn closePath(&self);

        #[method(removeAllPoints)]
        pub unsafe fn removeAllPoints(&self);

        #[method(relativeMoveToPoint:)]
        pub unsafe fn relativeMoveToPoint(&self, point: NSPoint);

        #[method(relativeLineToPoint:)]
        pub unsafe fn relativeLineToPoint(&self, point: NSPoint);

        #[method(relativeCurveToPoint:controlPoint1:controlPoint2:)]
        pub unsafe fn relativeCurveToPoint_controlPoint1_controlPoint2(
            &self,
            endPoint: NSPoint,
            controlPoint1: NSPoint,
            controlPoint2: NSPoint,
        );

        #[method(lineWidth)]
        pub unsafe fn lineWidth(&self) -> CGFloat;

        #[method(setLineWidth:)]
        pub unsafe fn setLineWidth(&self, lineWidth: CGFloat);

        #[method(lineCapStyle)]
        pub unsafe fn lineCapStyle(&self) -> NSLineCapStyle;

        #[method(setLineCapStyle:)]
        pub unsafe fn setLineCapStyle(&self, lineCapStyle: NSLineCapStyle);

        #[method(lineJoinStyle)]
        pub unsafe fn lineJoinStyle(&self) -> NSLineJoinStyle;

        #[method(setLineJoinStyle:)]
        pub unsafe fn setLineJoinStyle(&self, lineJoinStyle: NSLineJoinStyle);

        #[method(windingRule)]
        pub unsafe fn windingRule(&self) -> NSWindingRule;

        #[method(setWindingRule:)]
        pub unsafe fn setWindingRule(&self, windingRule: NSWindingRule);

        #[method(miterLimit)]
        pub unsafe fn miterLimit(&self) -> CGFloat;

        #[method(setMiterLimit:)]
        pub unsafe fn setMiterLimit(&self, miterLimit: CGFloat);

        #[method(flatness)]
        pub unsafe fn flatness(&self) -> CGFloat;

        #[method(setFlatness:)]
        pub unsafe fn setFlatness(&self, flatness: CGFloat);

        #[method(getLineDash:count:phase:)]
        pub unsafe fn getLineDash_count_phase(
            &self,
            pattern: *mut CGFloat,
            count: *mut NSInteger,
            phase: *mut CGFloat,
        );

        #[method(setLineDash:count:phase:)]
        pub unsafe fn setLineDash_count_phase(
            &self,
            pattern: *mut CGFloat,
            count: NSInteger,
            phase: CGFloat,
        );

        #[method(stroke)]
        pub unsafe fn stroke(&self);

        #[method(fill)]
        pub unsafe fn fill(&self);

        #[method(addClip)]
        pub unsafe fn addClip(&self);

        #[method(setClip)]
        pub unsafe fn setClip(&self);

        #[method_id(@__retain_semantics Other bezierPathByFlatteningPath)]
        pub unsafe fn bezierPathByFlatteningPath(&self) -> Id<NSBezierPath, Shared>;

        #[method_id(@__retain_semantics Other bezierPathByReversingPath)]
        pub unsafe fn bezierPathByReversingPath(&self) -> Id<NSBezierPath, Shared>;

        #[method(transformUsingAffineTransform:)]
        pub unsafe fn transformUsingAffineTransform(&self, transform: &NSAffineTransform);

        #[method(isEmpty)]
        pub unsafe fn isEmpty(&self) -> bool;

        #[method(currentPoint)]
        pub unsafe fn currentPoint(&self) -> NSPoint;

        #[method(controlPointBounds)]
        pub unsafe fn controlPointBounds(&self) -> NSRect;

        #[method(bounds)]
        pub unsafe fn bounds(&self) -> NSRect;

        #[method(elementCount)]
        pub unsafe fn elementCount(&self) -> NSInteger;

        #[method(elementAtIndex:associatedPoints:)]
        pub unsafe fn elementAtIndex_associatedPoints(
            &self,
            index: NSInteger,
            points: NSPointArray,
        ) -> NSBezierPathElement;

        #[method(elementAtIndex:)]
        pub unsafe fn elementAtIndex(&self, index: NSInteger) -> NSBezierPathElement;

        #[method(setAssociatedPoints:atIndex:)]
        pub unsafe fn setAssociatedPoints_atIndex(&self, points: NSPointArray, index: NSInteger);

        #[method(appendBezierPath:)]
        pub unsafe fn appendBezierPath(&self, path: &NSBezierPath);

        #[method(appendBezierPathWithRect:)]
        pub unsafe fn appendBezierPathWithRect(&self, rect: NSRect);

        #[method(appendBezierPathWithPoints:count:)]
        pub unsafe fn appendBezierPathWithPoints_count(
            &self,
            points: NSPointArray,
            count: NSInteger,
        );

        #[method(appendBezierPathWithOvalInRect:)]
        pub unsafe fn appendBezierPathWithOvalInRect(&self, rect: NSRect);

        #[method(appendBezierPathWithArcWithCenter:radius:startAngle:endAngle:clockwise:)]
        pub unsafe fn appendBezierPathWithArcWithCenter_radius_startAngle_endAngle_clockwise(
            &self,
            center: NSPoint,
            radius: CGFloat,
            startAngle: CGFloat,
            endAngle: CGFloat,
            clockwise: bool,
        );

        #[method(appendBezierPathWithArcWithCenter:radius:startAngle:endAngle:)]
        pub unsafe fn appendBezierPathWithArcWithCenter_radius_startAngle_endAngle(
            &self,
            center: NSPoint,
            radius: CGFloat,
            startAngle: CGFloat,
            endAngle: CGFloat,
        );

        #[method(appendBezierPathWithArcFromPoint:toPoint:radius:)]
        pub unsafe fn appendBezierPathWithArcFromPoint_toPoint_radius(
            &self,
            point1: NSPoint,
            point2: NSPoint,
            radius: CGFloat,
        );

        #[method(appendBezierPathWithCGGlyph:inFont:)]
        pub unsafe fn appendBezierPathWithCGGlyph_inFont(&self, glyph: CGGlyph, font: &NSFont);

        #[method(appendBezierPathWithCGGlyphs:count:inFont:)]
        pub unsafe fn appendBezierPathWithCGGlyphs_count_inFont(
            &self,
            glyphs: NonNull<CGGlyph>,
            count: NSInteger,
            font: &NSFont,
        );

        #[method(appendBezierPathWithRoundedRect:xRadius:yRadius:)]
        pub unsafe fn appendBezierPathWithRoundedRect_xRadius_yRadius(
            &self,
            rect: NSRect,
            xRadius: CGFloat,
            yRadius: CGFloat,
        );

        #[method(containsPoint:)]
        pub unsafe fn containsPoint(&self, point: NSPoint) -> bool;
    }
);

extern_methods!(
    /// NSBezierPathDeprecated
    unsafe impl NSBezierPath {
        #[method(cachesBezierPath)]
        pub unsafe fn cachesBezierPath(&self) -> bool;

        #[method(setCachesBezierPath:)]
        pub unsafe fn setCachesBezierPath(&self, flag: bool);

        #[method(appendBezierPathWithGlyph:inFont:)]
        pub unsafe fn appendBezierPathWithGlyph_inFont(&self, glyph: NSGlyph, font: &NSFont);

        #[method(appendBezierPathWithGlyphs:count:inFont:)]
        pub unsafe fn appendBezierPathWithGlyphs_count_inFont(
            &self,
            glyphs: NonNull<NSGlyph>,
            count: NSInteger,
            font: &NSFont,
        );

        #[method(appendBezierPathWithPackedGlyphs:)]
        pub unsafe fn appendBezierPathWithPackedGlyphs(&self, packedGlyphs: NonNull<c_char>);
    }
);

pub static NSButtLineCapStyle: NSLineCapStyle = NSLineCapStyleButt;

pub static NSRoundLineCapStyle: NSLineCapStyle = NSLineCapStyleRound;

pub static NSSquareLineCapStyle: NSLineCapStyle = NSLineCapStyleSquare;

pub static NSMiterLineJoinStyle: NSLineJoinStyle = NSLineJoinStyleMiter;

pub static NSRoundLineJoinStyle: NSLineJoinStyle = NSLineJoinStyleRound;

pub static NSBevelLineJoinStyle: NSLineJoinStyle = NSLineJoinStyleBevel;

pub static NSNonZeroWindingRule: NSWindingRule = NSWindingRuleNonZero;

pub static NSEvenOddWindingRule: NSWindingRule = NSWindingRuleEvenOdd;

pub static NSMoveToBezierPathElement: NSBezierPathElement = NSBezierPathElementMoveTo;

pub static NSLineToBezierPathElement: NSBezierPathElement = NSBezierPathElementLineTo;

pub static NSCurveToBezierPathElement: NSBezierPathElement = NSBezierPathElementCurveTo;

pub static NSClosePathBezierPathElement: NSBezierPathElement = NSBezierPathElementClosePath;
