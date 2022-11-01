//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug)]
    pub struct NSPredicateEditor;

    unsafe impl ClassType for NSPredicateEditor {
        type Super = NSRuleEditor;
    }
);

extern_methods!(
    unsafe impl NSPredicateEditor {
        #[method_id(@__retain_semantics Other rowTemplates)]
        pub unsafe fn rowTemplates(&self) -> Id<NSArray<NSPredicateEditorRowTemplate>, Shared>;

        #[method(setRowTemplates:)]
        pub unsafe fn setRowTemplates(&self, rowTemplates: &NSArray<NSPredicateEditorRowTemplate>);
    }
);
