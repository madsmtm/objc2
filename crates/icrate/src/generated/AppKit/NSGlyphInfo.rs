use crate::AppKit::generated::NSFont::*;
use crate::Foundation::generated::NSString::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSGlyphInfo;
    unsafe impl ClassType for NSGlyphInfo {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSGlyphInfo {
        #[method_id(glyphInfoWithCGGlyph:forFont:baseString:)]
        pub unsafe fn glyphInfoWithCGGlyph_forFont_baseString(
            glyph: CGGlyph,
            font: &NSFont,
            string: &NSString,
        ) -> Option<Id<NSGlyphInfo, Shared>>;
        #[method(glyphID)]
        pub unsafe fn glyphID(&self) -> CGGlyph;
        #[method_id(baseString)]
        pub unsafe fn baseString(&self) -> Id<NSString, Shared>;
    }
);
extern_methods!(
    #[doc = "NSGlyphInfo_Deprecated"]
    unsafe impl NSGlyphInfo {
        #[method_id(glyphInfoWithGlyphName:forFont:baseString:)]
        pub unsafe fn glyphInfoWithGlyphName_forFont_baseString(
            glyphName: &NSString,
            font: &NSFont,
            string: &NSString,
        ) -> Option<Id<NSGlyphInfo, Shared>>;
        #[method_id(glyphInfoWithGlyph:forFont:baseString:)]
        pub unsafe fn glyphInfoWithGlyph_forFont_baseString(
            glyph: NSGlyph,
            font: &NSFont,
            string: &NSString,
        ) -> Option<Id<NSGlyphInfo, Shared>>;
        #[method_id(glyphInfoWithCharacterIdentifier:collection:baseString:)]
        pub unsafe fn glyphInfoWithCharacterIdentifier_collection_baseString(
            cid: NSUInteger,
            characterCollection: NSCharacterCollection,
            string: &NSString,
        ) -> Option<Id<NSGlyphInfo, Shared>>;
        #[method_id(glyphName)]
        pub unsafe fn glyphName(&self) -> Option<Id<NSString, Shared>>;
        #[method(characterIdentifier)]
        pub unsafe fn characterIdentifier(&self) -> NSUInteger;
        #[method(characterCollection)]
        pub unsafe fn characterCollection(&self) -> NSCharacterCollection;
    }
);
