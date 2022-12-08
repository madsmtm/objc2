//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_methods!(
    /// Scripting
    unsafe impl NSTextStorage {
        #[method_id(@__retain_semantics Other attributeRuns)]
        pub unsafe fn attributeRuns(&self) -> Id<NSArray<NSTextStorage>, Shared>;

        #[method(setAttributeRuns:)]
        pub unsafe fn setAttributeRuns(&self, attributeRuns: &NSArray<NSTextStorage>);

        #[method_id(@__retain_semantics Other paragraphs)]
        pub unsafe fn paragraphs(&self) -> Id<NSArray<NSTextStorage>, Shared>;

        #[method(setParagraphs:)]
        pub unsafe fn setParagraphs(&self, paragraphs: &NSArray<NSTextStorage>);

        #[method_id(@__retain_semantics Other words)]
        pub unsafe fn words(&self) -> Id<NSArray<NSTextStorage>, Shared>;

        #[method(setWords:)]
        pub unsafe fn setWords(&self, words: &NSArray<NSTextStorage>);

        #[method_id(@__retain_semantics Other characters)]
        pub unsafe fn characters(&self) -> Id<NSArray<NSTextStorage>, Shared>;

        #[method(setCharacters:)]
        pub unsafe fn setCharacters(&self, characters: &NSArray<NSTextStorage>);

        #[method_id(@__retain_semantics Other font)]
        pub unsafe fn font(&self) -> Option<Id<NSFont, Shared>>;

        #[method(setFont:)]
        pub unsafe fn setFont(&self, font: Option<&NSFont>);

        #[method_id(@__retain_semantics Other foregroundColor)]
        pub unsafe fn foregroundColor(&self) -> Option<Id<NSColor, Shared>>;

        #[method(setForegroundColor:)]
        pub unsafe fn setForegroundColor(&self, foregroundColor: Option<&NSColor>);
    }
);
