use super::__exported::NSBezierPath;
use super::__exported::NSColor;
use super::__exported::NSColorSpace;
use crate::AppKit::generated::AppKitDefines::*;
use crate::Foundation::generated::NSArray::*;
use crate::Foundation::generated::NSGeometry::*;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSGradient;
    unsafe impl ClassType for NSGradient {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSGradient {
        #[method_id(initWithStartingColor:endingColor:)]
        pub unsafe fn initWithStartingColor_endingColor(
            &self,
            startingColor: &NSColor,
            endingColor: &NSColor,
        ) -> Option<Id<Self, Shared>>;
        #[method_id(initWithColors:)]
        pub unsafe fn initWithColors(
            &self,
            colorArray: &NSArray<NSColor>,
        ) -> Option<Id<Self, Shared>>;
        #[method_id(initWithColors:atLocations:colorSpace:)]
        pub unsafe fn initWithColors_atLocations_colorSpace(
            &self,
            colorArray: &NSArray<NSColor>,
            locations: *mut CGFloat,
            colorSpace: &NSColorSpace,
        ) -> Option<Id<Self, Shared>>;
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Id<Self, Shared>;
        #[method(drawFromPoint:toPoint:options:)]
        pub unsafe fn drawFromPoint_toPoint_options(
            &self,
            startingPoint: NSPoint,
            endingPoint: NSPoint,
            options: NSGradientDrawingOptions,
        );
        #[method(drawInRect:angle:)]
        pub unsafe fn drawInRect_angle(&self, rect: NSRect, angle: CGFloat);
        #[method(drawInBezierPath:angle:)]
        pub unsafe fn drawInBezierPath_angle(&self, path: &NSBezierPath, angle: CGFloat);
        #[method(drawFromCenter:radius:toCenter:radius:options:)]
        pub unsafe fn drawFromCenter_radius_toCenter_radius_options(
            &self,
            startCenter: NSPoint,
            startRadius: CGFloat,
            endCenter: NSPoint,
            endRadius: CGFloat,
            options: NSGradientDrawingOptions,
        );
        #[method(drawInRect:relativeCenterPosition:)]
        pub unsafe fn drawInRect_relativeCenterPosition(
            &self,
            rect: NSRect,
            relativeCenterPosition: NSPoint,
        );
        #[method(drawInBezierPath:relativeCenterPosition:)]
        pub unsafe fn drawInBezierPath_relativeCenterPosition(
            &self,
            path: &NSBezierPath,
            relativeCenterPosition: NSPoint,
        );
        #[method_id(colorSpace)]
        pub unsafe fn colorSpace(&self) -> Id<NSColorSpace, Shared>;
        #[method(numberOfColorStops)]
        pub unsafe fn numberOfColorStops(&self) -> NSInteger;
        #[method(getColor:location:atIndex:)]
        pub unsafe fn getColor_location_atIndex(
            &self,
            color: Option<&mut Id<NSColor, Shared>>,
            location: *mut CGFloat,
            index: NSInteger,
        );
        #[method_id(interpolatedColorAtLocation:)]
        pub unsafe fn interpolatedColorAtLocation(&self, location: CGFloat) -> Id<NSColor, Shared>;
    }
);
