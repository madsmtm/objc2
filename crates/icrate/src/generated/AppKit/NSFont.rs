use super::__exported::NSAffineTransform;
use super::__exported::NSFontDescriptor;
use super::__exported::NSGraphicsContext;
use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSCell::*;
use crate::AppKit::generated::NSFontDescriptor::*;
use crate::Foundation::generated::NSObject::*;
use crate::Foundation::generated::NSString::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSFont;
    unsafe impl ClassType for NSFont {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSFont {
        #[method_id(fontWithName:size:)]
        pub unsafe fn fontWithName_size(
            fontName: &NSString,
            fontSize: CGFloat,
        ) -> Option<Id<NSFont, Shared>>;
        #[method_id(fontWithName:matrix:)]
        pub unsafe fn fontWithName_matrix(
            fontName: &NSString,
            fontMatrix: NonNull<CGFloat>,
        ) -> Option<Id<NSFont, Shared>>;
        #[method_id(fontWithDescriptor:size:)]
        pub unsafe fn fontWithDescriptor_size(
            fontDescriptor: &NSFontDescriptor,
            fontSize: CGFloat,
        ) -> Option<Id<NSFont, Shared>>;
        #[method_id(fontWithDescriptor:textTransform:)]
        pub unsafe fn fontWithDescriptor_textTransform(
            fontDescriptor: &NSFontDescriptor,
            textTransform: Option<&NSAffineTransform>,
        ) -> Option<Id<NSFont, Shared>>;
        #[method_id(userFontOfSize:)]
        pub unsafe fn userFontOfSize(fontSize: CGFloat) -> Option<Id<NSFont, Shared>>;
        #[method_id(userFixedPitchFontOfSize:)]
        pub unsafe fn userFixedPitchFontOfSize(fontSize: CGFloat) -> Option<Id<NSFont, Shared>>;
        #[method(setUserFont:)]
        pub unsafe fn setUserFont(font: Option<&NSFont>);
        #[method(setUserFixedPitchFont:)]
        pub unsafe fn setUserFixedPitchFont(font: Option<&NSFont>);
        #[method_id(systemFontOfSize:)]
        pub unsafe fn systemFontOfSize(fontSize: CGFloat) -> Id<NSFont, Shared>;
        #[method_id(boldSystemFontOfSize:)]
        pub unsafe fn boldSystemFontOfSize(fontSize: CGFloat) -> Id<NSFont, Shared>;
        #[method_id(labelFontOfSize:)]
        pub unsafe fn labelFontOfSize(fontSize: CGFloat) -> Id<NSFont, Shared>;
        #[method_id(titleBarFontOfSize:)]
        pub unsafe fn titleBarFontOfSize(fontSize: CGFloat) -> Id<NSFont, Shared>;
        #[method_id(menuFontOfSize:)]
        pub unsafe fn menuFontOfSize(fontSize: CGFloat) -> Id<NSFont, Shared>;
        #[method_id(menuBarFontOfSize:)]
        pub unsafe fn menuBarFontOfSize(fontSize: CGFloat) -> Id<NSFont, Shared>;
        #[method_id(messageFontOfSize:)]
        pub unsafe fn messageFontOfSize(fontSize: CGFloat) -> Id<NSFont, Shared>;
        #[method_id(paletteFontOfSize:)]
        pub unsafe fn paletteFontOfSize(fontSize: CGFloat) -> Id<NSFont, Shared>;
        #[method_id(toolTipsFontOfSize:)]
        pub unsafe fn toolTipsFontOfSize(fontSize: CGFloat) -> Id<NSFont, Shared>;
        #[method_id(controlContentFontOfSize:)]
        pub unsafe fn controlContentFontOfSize(fontSize: CGFloat) -> Id<NSFont, Shared>;
        #[method_id(systemFontOfSize:weight:)]
        pub unsafe fn systemFontOfSize_weight(
            fontSize: CGFloat,
            weight: NSFontWeight,
        ) -> Id<NSFont, Shared>;
        #[method_id(monospacedDigitSystemFontOfSize:weight:)]
        pub unsafe fn monospacedDigitSystemFontOfSize_weight(
            fontSize: CGFloat,
            weight: NSFontWeight,
        ) -> Id<NSFont, Shared>;
        #[method_id(monospacedSystemFontOfSize:weight:)]
        pub unsafe fn monospacedSystemFontOfSize_weight(
            fontSize: CGFloat,
            weight: NSFontWeight,
        ) -> Id<NSFont, Shared>;
        #[method_id(fontWithSize:)]
        pub unsafe fn fontWithSize(&self, fontSize: CGFloat) -> Id<NSFont, Shared>;
        #[method(systemFontSize)]
        pub unsafe fn systemFontSize() -> CGFloat;
        #[method(smallSystemFontSize)]
        pub unsafe fn smallSystemFontSize() -> CGFloat;
        #[method(labelFontSize)]
        pub unsafe fn labelFontSize() -> CGFloat;
        #[method(systemFontSizeForControlSize:)]
        pub unsafe fn systemFontSizeForControlSize(controlSize: NSControlSize) -> CGFloat;
        #[method_id(fontName)]
        pub unsafe fn fontName(&self) -> Id<NSString, Shared>;
        #[method(pointSize)]
        pub unsafe fn pointSize(&self) -> CGFloat;
        #[method(matrix)]
        pub unsafe fn matrix(&self) -> NonNull<CGFloat>;
        #[method_id(familyName)]
        pub unsafe fn familyName(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(displayName)]
        pub unsafe fn displayName(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(fontDescriptor)]
        pub unsafe fn fontDescriptor(&self) -> Id<NSFontDescriptor, Shared>;
        #[method_id(textTransform)]
        pub unsafe fn textTransform(&self) -> Id<NSAffineTransform, Shared>;
        #[method(numberOfGlyphs)]
        pub unsafe fn numberOfGlyphs(&self) -> NSUInteger;
        #[method(mostCompatibleStringEncoding)]
        pub unsafe fn mostCompatibleStringEncoding(&self) -> NSStringEncoding;
        #[method_id(coveredCharacterSet)]
        pub unsafe fn coveredCharacterSet(&self) -> Id<NSCharacterSet, Shared>;
        #[method(boundingRectForFont)]
        pub unsafe fn boundingRectForFont(&self) -> NSRect;
        #[method(maximumAdvancement)]
        pub unsafe fn maximumAdvancement(&self) -> NSSize;
        #[method(ascender)]
        pub unsafe fn ascender(&self) -> CGFloat;
        #[method(descender)]
        pub unsafe fn descender(&self) -> CGFloat;
        #[method(leading)]
        pub unsafe fn leading(&self) -> CGFloat;
        #[method(underlinePosition)]
        pub unsafe fn underlinePosition(&self) -> CGFloat;
        #[method(underlineThickness)]
        pub unsafe fn underlineThickness(&self) -> CGFloat;
        #[method(italicAngle)]
        pub unsafe fn italicAngle(&self) -> CGFloat;
        #[method(capHeight)]
        pub unsafe fn capHeight(&self) -> CGFloat;
        #[method(xHeight)]
        pub unsafe fn xHeight(&self) -> CGFloat;
        #[method(isFixedPitch)]
        pub unsafe fn isFixedPitch(&self) -> bool;
        #[method(boundingRectForCGGlyph:)]
        pub unsafe fn boundingRectForCGGlyph(&self, glyph: CGGlyph) -> NSRect;
        #[method(advancementForCGGlyph:)]
        pub unsafe fn advancementForCGGlyph(&self, glyph: CGGlyph) -> NSSize;
        #[method(getBoundingRects:forCGGlyphs:count:)]
        pub unsafe fn getBoundingRects_forCGGlyphs_count(
            &self,
            bounds: NSRectArray,
            glyphs: NonNull<CGGlyph>,
            glyphCount: NSUInteger,
        );
        #[method(getAdvancements:forCGGlyphs:count:)]
        pub unsafe fn getAdvancements_forCGGlyphs_count(
            &self,
            advancements: NSSizeArray,
            glyphs: NonNull<CGGlyph>,
            glyphCount: NSUInteger,
        );
        #[method(set)]
        pub unsafe fn set(&self);
        #[method(setInContext:)]
        pub unsafe fn setInContext(&self, graphicsContext: &NSGraphicsContext);
        #[method_id(verticalFont)]
        pub unsafe fn verticalFont(&self) -> Id<NSFont, Shared>;
        #[method(isVertical)]
        pub unsafe fn isVertical(&self) -> bool;
    }
);
extern_methods!(
    #[doc = "NSFont_Deprecated"]
    unsafe impl NSFont {
        #[method(glyphWithName:)]
        pub unsafe fn glyphWithName(&self, name: &NSString) -> NSGlyph;
        #[method(boundingRectForGlyph:)]
        pub unsafe fn boundingRectForGlyph(&self, glyph: NSGlyph) -> NSRect;
        #[method(advancementForGlyph:)]
        pub unsafe fn advancementForGlyph(&self, glyph: NSGlyph) -> NSSize;
        #[method(getBoundingRects:forGlyphs:count:)]
        pub unsafe fn getBoundingRects_forGlyphs_count(
            &self,
            bounds: NSRectArray,
            glyphs: NonNull<NSGlyph>,
            glyphCount: NSUInteger,
        );
        #[method(getAdvancements:forGlyphs:count:)]
        pub unsafe fn getAdvancements_forGlyphs_count(
            &self,
            advancements: NSSizeArray,
            glyphs: NonNull<NSGlyph>,
            glyphCount: NSUInteger,
        );
        #[method(getAdvancements:forPackedGlyphs:length:)]
        pub unsafe fn getAdvancements_forPackedGlyphs_length(
            &self,
            advancements: NSSizeArray,
            packedGlyphs: NonNull<c_void>,
            length: NSUInteger,
        );
        #[method_id(printerFont)]
        pub unsafe fn printerFont(&self) -> Id<NSFont, Shared>;
        #[method_id(screenFont)]
        pub unsafe fn screenFont(&self) -> Id<NSFont, Shared>;
        #[method_id(screenFontWithRenderingMode:)]
        pub unsafe fn screenFontWithRenderingMode(
            &self,
            renderingMode: NSFontRenderingMode,
        ) -> Id<NSFont, Shared>;
        #[method(renderingMode)]
        pub unsafe fn renderingMode(&self) -> NSFontRenderingMode;
    }
);
extern_methods!(
    #[doc = "NSFont_TextStyles"]
    unsafe impl NSFont {
        #[method_id(preferredFontForTextStyle:options:)]
        pub unsafe fn preferredFontForTextStyle_options(
            style: &NSFontTextStyle,
            options: &NSDictionary<NSFontTextStyleOptionKey, Object>,
        ) -> Id<NSFont, Shared>;
    }
);
