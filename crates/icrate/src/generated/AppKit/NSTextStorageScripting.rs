#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_methods!(
    #[doc = "Scripting"]
    unsafe impl NSTextStorage {
        #[method_id(attributeRuns)]
        pub unsafe fn attributeRuns(&self) -> Id<NSArray<NSTextStorage>, Shared>;
        #[method(setAttributeRuns:)]
        pub unsafe fn setAttributeRuns(&self, attributeRuns: &NSArray<NSTextStorage>);
        #[method_id(paragraphs)]
        pub unsafe fn paragraphs(&self) -> Id<NSArray<NSTextStorage>, Shared>;
        #[method(setParagraphs:)]
        pub unsafe fn setParagraphs(&self, paragraphs: &NSArray<NSTextStorage>);
        #[method_id(words)]
        pub unsafe fn words(&self) -> Id<NSArray<NSTextStorage>, Shared>;
        #[method(setWords:)]
        pub unsafe fn setWords(&self, words: &NSArray<NSTextStorage>);
        #[method_id(characters)]
        pub unsafe fn characters(&self) -> Id<NSArray<NSTextStorage>, Shared>;
        #[method(setCharacters:)]
        pub unsafe fn setCharacters(&self, characters: &NSArray<NSTextStorage>);
        #[method_id(font)]
        pub unsafe fn font(&self) -> Option<Id<NSFont, Shared>>;
        #[method(setFont:)]
        pub unsafe fn setFont(&self, font: Option<&NSFont>);
        #[method_id(foregroundColor)]
        pub unsafe fn foregroundColor(&self) -> Option<Id<NSColor, Shared>>;
        #[method(setForegroundColor:)]
        pub unsafe fn setForegroundColor(&self, foregroundColor: Option<&NSColor>);
    }
);
