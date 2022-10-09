use crate::AppKit::generated::NSFont::*;
use crate::Foundation::generated::NSAttributedString::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
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
