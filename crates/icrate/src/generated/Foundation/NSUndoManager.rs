use super::__exported::NSArray;
use super::__exported::NSString;
use crate::Foundation::generated::NSNotification::*;
use crate::Foundation::generated::NSObject::*;
use crate::Foundation::generated::NSRunLoop::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSUndoManager;
    unsafe impl ClassType for NSUndoManager {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSUndoManager {
        #[method(beginUndoGrouping)]
        pub unsafe fn beginUndoGrouping(&self);
        #[method(endUndoGrouping)]
        pub unsafe fn endUndoGrouping(&self);
        #[method(groupingLevel)]
        pub unsafe fn groupingLevel(&self) -> NSInteger;
        #[method(disableUndoRegistration)]
        pub unsafe fn disableUndoRegistration(&self);
        #[method(enableUndoRegistration)]
        pub unsafe fn enableUndoRegistration(&self);
        #[method(isUndoRegistrationEnabled)]
        pub unsafe fn isUndoRegistrationEnabled(&self) -> bool;
        #[method(groupsByEvent)]
        pub unsafe fn groupsByEvent(&self) -> bool;
        #[method(setGroupsByEvent:)]
        pub unsafe fn setGroupsByEvent(&self, groupsByEvent: bool);
        #[method(levelsOfUndo)]
        pub unsafe fn levelsOfUndo(&self) -> NSUInteger;
        #[method(setLevelsOfUndo:)]
        pub unsafe fn setLevelsOfUndo(&self, levelsOfUndo: NSUInteger);
        #[method_id(runLoopModes)]
        pub unsafe fn runLoopModes(&self) -> Id<NSArray<NSRunLoopMode>, Shared>;
        #[method(setRunLoopModes:)]
        pub unsafe fn setRunLoopModes(&self, runLoopModes: &NSArray<NSRunLoopMode>);
        #[method(undo)]
        pub unsafe fn undo(&self);
        #[method(redo)]
        pub unsafe fn redo(&self);
        #[method(undoNestedGroup)]
        pub unsafe fn undoNestedGroup(&self);
        #[method(canUndo)]
        pub unsafe fn canUndo(&self) -> bool;
        #[method(canRedo)]
        pub unsafe fn canRedo(&self) -> bool;
        #[method(isUndoing)]
        pub unsafe fn isUndoing(&self) -> bool;
        #[method(isRedoing)]
        pub unsafe fn isRedoing(&self) -> bool;
        #[method(removeAllActions)]
        pub unsafe fn removeAllActions(&self);
        #[method(removeAllActionsWithTarget:)]
        pub unsafe fn removeAllActionsWithTarget(&self, target: &Object);
        #[method(registerUndoWithTarget:selector:object:)]
        pub unsafe fn registerUndoWithTarget_selector_object(
            &self,
            target: &Object,
            selector: Sel,
            anObject: Option<&Object>,
        );
        #[method_id(prepareWithInvocationTarget:)]
        pub unsafe fn prepareWithInvocationTarget(&self, target: &Object) -> Id<Object, Shared>;
        #[method(registerUndoWithTarget:handler:)]
        pub unsafe fn registerUndoWithTarget_handler(
            &self,
            target: &Object,
            undoHandler: TodoBlock,
        );
        #[method(setActionIsDiscardable:)]
        pub unsafe fn setActionIsDiscardable(&self, discardable: bool);
        #[method(undoActionIsDiscardable)]
        pub unsafe fn undoActionIsDiscardable(&self) -> bool;
        #[method(redoActionIsDiscardable)]
        pub unsafe fn redoActionIsDiscardable(&self) -> bool;
        #[method_id(undoActionName)]
        pub unsafe fn undoActionName(&self) -> Id<NSString, Shared>;
        #[method_id(redoActionName)]
        pub unsafe fn redoActionName(&self) -> Id<NSString, Shared>;
        #[method(setActionName:)]
        pub unsafe fn setActionName(&self, actionName: &NSString);
        #[method_id(undoMenuItemTitle)]
        pub unsafe fn undoMenuItemTitle(&self) -> Id<NSString, Shared>;
        #[method_id(redoMenuItemTitle)]
        pub unsafe fn redoMenuItemTitle(&self) -> Id<NSString, Shared>;
        #[method_id(undoMenuTitleForUndoActionName:)]
        pub unsafe fn undoMenuTitleForUndoActionName(
            &self,
            actionName: &NSString,
        ) -> Id<NSString, Shared>;
        #[method_id(redoMenuTitleForUndoActionName:)]
        pub unsafe fn redoMenuTitleForUndoActionName(
            &self,
            actionName: &NSString,
        ) -> Id<NSString, Shared>;
    }
);
