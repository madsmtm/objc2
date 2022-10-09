use super::__exported::NSMutableArray;
use super::__exported::NSMutableDictionary;
use super::__exported::NSMutableSet;
use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSKeyValueBinding::*;
use crate::CoreFoundation::generated::CoreFoundation::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSController;
    unsafe impl ClassType for NSController {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSController {
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>>;
        #[method(objectDidBeginEditing:)]
        pub unsafe fn objectDidBeginEditing(&self, editor: &NSEditor);
        #[method(objectDidEndEditing:)]
        pub unsafe fn objectDidEndEditing(&self, editor: &NSEditor);
        #[method(discardEditing)]
        pub unsafe fn discardEditing(&self);
        #[method(commitEditing)]
        pub unsafe fn commitEditing(&self) -> bool;
        #[method(commitEditingWithDelegate:didCommitSelector:contextInfo:)]
        pub unsafe fn commitEditingWithDelegate_didCommitSelector_contextInfo(
            &self,
            delegate: Option<&Object>,
            didCommitSelector: Option<Sel>,
            contextInfo: *mut c_void,
        );
        #[method(isEditing)]
        pub unsafe fn isEditing(&self) -> bool;
    }
);
