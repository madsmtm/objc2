//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};

pub const NSShowControlGlyphs: i32 = (1 << 0);
pub const NSShowInvisibleGlyphs: i32 = (1 << 1);
pub const NSWantsBidiLevels: i32 = (1 << 2);

pub type NSGlyphStorage = NSObject;

extern_class!(
    #[derive(Debug)]
    pub struct NSGlyphGenerator;

    unsafe impl ClassType for NSGlyphGenerator {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSGlyphGenerator {
        #[method(generateGlyphsForGlyphStorage:desiredNumberOfCharacters:glyphIndex:characterIndex:)]
        pub unsafe fn generateGlyphsForGlyphStorage_desiredNumberOfCharacters_glyphIndex_characterIndex(
            &self,
            glyphStorage: &NSGlyphStorage,
            nChars: NSUInteger,
            glyphIndex: *mut NSUInteger,
            charIndex: *mut NSUInteger,
        );

        #[method_id(sharedGlyphGenerator)]
        pub unsafe fn sharedGlyphGenerator() -> Id<NSGlyphGenerator, Shared>;
    }
);
