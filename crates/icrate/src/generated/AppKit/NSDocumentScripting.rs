use super::__exported::NSCloseCommand;
use super::__exported::NSScriptCommand;
use super::__exported::NSScriptObjectSpecifier;
use super::__exported::NSString;
use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSDocument::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_methods!(
    #[doc = "NSScripting"]
    unsafe impl NSDocument {
        #[method_id(lastComponentOfFileName)]
        pub unsafe fn lastComponentOfFileName(&self) -> Id<NSString, Shared>;
        #[method(setLastComponentOfFileName:)]
        pub unsafe fn setLastComponentOfFileName(&self, lastComponentOfFileName: &NSString);
        #[method_id(handleSaveScriptCommand:)]
        pub unsafe fn handleSaveScriptCommand(
            &self,
            command: &NSScriptCommand,
        ) -> Option<Id<Object, Shared>>;
        #[method_id(handleCloseScriptCommand:)]
        pub unsafe fn handleCloseScriptCommand(
            &self,
            command: &NSCloseCommand,
        ) -> Option<Id<Object, Shared>>;
        #[method_id(handlePrintScriptCommand:)]
        pub unsafe fn handlePrintScriptCommand(
            &self,
            command: &NSScriptCommand,
        ) -> Option<Id<Object, Shared>>;
        #[method_id(objectSpecifier)]
        pub unsafe fn objectSpecifier(&self) -> Id<NSScriptObjectSpecifier, Shared>;
    }
);
