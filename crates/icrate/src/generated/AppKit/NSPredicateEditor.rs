use super::__exported::NSArray;
use super::__exported::NSPredicateEditorRowTemplate;
use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSRuleEditor::*;
use crate::Foundation::generated::NSArray::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSPredicateEditor;
    unsafe impl ClassType for NSPredicateEditor {
        type Super = NSRuleEditor;
    }
);
extern_methods!(
    unsafe impl NSPredicateEditor {
        #[method_id(rowTemplates)]
        pub unsafe fn rowTemplates(&self) -> Id<NSArray<NSPredicateEditorRowTemplate>, Shared>;
        #[method(setRowTemplates:)]
        pub unsafe fn setRowTemplates(&self, rowTemplates: &NSArray<NSPredicateEditorRowTemplate>);
    }
);
