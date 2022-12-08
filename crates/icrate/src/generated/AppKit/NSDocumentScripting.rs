//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_methods!(
    /// NSScripting
    unsafe impl NSDocument {
        #[method_id(@__retain_semantics Other lastComponentOfFileName)]
        pub unsafe fn lastComponentOfFileName(&self) -> Id<NSString, Shared>;

        #[method(setLastComponentOfFileName:)]
        pub unsafe fn setLastComponentOfFileName(&self, lastComponentOfFileName: &NSString);

        #[method_id(@__retain_semantics Other handleSaveScriptCommand:)]
        pub unsafe fn handleSaveScriptCommand(
            &self,
            command: &NSScriptCommand,
        ) -> Option<Id<Object, Shared>>;

        #[method_id(@__retain_semantics Other handleCloseScriptCommand:)]
        pub unsafe fn handleCloseScriptCommand(
            &self,
            command: &NSCloseCommand,
        ) -> Option<Id<Object, Shared>>;

        #[method_id(@__retain_semantics Other handlePrintScriptCommand:)]
        pub unsafe fn handlePrintScriptCommand(
            &self,
            command: &NSScriptCommand,
        ) -> Option<Id<Object, Shared>>;

        #[method_id(@__retain_semantics Other objectSpecifier)]
        pub unsafe fn objectSpecifier(&self) -> Id<NSScriptObjectSpecifier, Shared>;
    }
);
