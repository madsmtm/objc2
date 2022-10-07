use super::__exported::NSArray;
use super::__exported::NSString;
use crate::Foundation::generated::NSNotification::*;
use crate::Foundation::generated::NSObject::*;
use crate::Foundation::generated::NSRunLoop::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSUndoManager;
    unsafe impl ClassType for NSUndoManager {
        type Super = NSObject;
    }
);
impl NSUndoManager {
    pub unsafe fn beginUndoGrouping(&self) {
        msg_send![self, beginUndoGrouping]
    }
    pub unsafe fn endUndoGrouping(&self) {
        msg_send![self, endUndoGrouping]
    }
    pub unsafe fn groupingLevel(&self) -> NSInteger {
        msg_send![self, groupingLevel]
    }
    pub unsafe fn disableUndoRegistration(&self) {
        msg_send![self, disableUndoRegistration]
    }
    pub unsafe fn enableUndoRegistration(&self) {
        msg_send![self, enableUndoRegistration]
    }
    pub unsafe fn isUndoRegistrationEnabled(&self) -> bool {
        msg_send![self, isUndoRegistrationEnabled]
    }
    pub unsafe fn groupsByEvent(&self) -> bool {
        msg_send![self, groupsByEvent]
    }
    pub unsafe fn setGroupsByEvent(&self, groupsByEvent: bool) {
        msg_send![self, setGroupsByEvent: groupsByEvent]
    }
    pub unsafe fn levelsOfUndo(&self) -> NSUInteger {
        msg_send![self, levelsOfUndo]
    }
    pub unsafe fn setLevelsOfUndo(&self, levelsOfUndo: NSUInteger) {
        msg_send![self, setLevelsOfUndo: levelsOfUndo]
    }
    pub unsafe fn runLoopModes(&self) -> Id<NSArray<NSRunLoopMode>, Shared> {
        msg_send_id![self, runLoopModes]
    }
    pub unsafe fn setRunLoopModes(&self, runLoopModes: &NSArray<NSRunLoopMode>) {
        msg_send![self, setRunLoopModes: runLoopModes]
    }
    pub unsafe fn undo(&self) {
        msg_send![self, undo]
    }
    pub unsafe fn redo(&self) {
        msg_send![self, redo]
    }
    pub unsafe fn undoNestedGroup(&self) {
        msg_send![self, undoNestedGroup]
    }
    pub unsafe fn canUndo(&self) -> bool {
        msg_send![self, canUndo]
    }
    pub unsafe fn canRedo(&self) -> bool {
        msg_send![self, canRedo]
    }
    pub unsafe fn isUndoing(&self) -> bool {
        msg_send![self, isUndoing]
    }
    pub unsafe fn isRedoing(&self) -> bool {
        msg_send![self, isRedoing]
    }
    pub unsafe fn removeAllActions(&self) {
        msg_send![self, removeAllActions]
    }
    pub unsafe fn removeAllActionsWithTarget(&self, target: &Object) {
        msg_send![self, removeAllActionsWithTarget: target]
    }
    pub unsafe fn registerUndoWithTarget_selector_object(
        &self,
        target: &Object,
        selector: Sel,
        anObject: Option<&Object>,
    ) {
        msg_send![
            self,
            registerUndoWithTarget: target,
            selector: selector,
            object: anObject
        ]
    }
    pub unsafe fn prepareWithInvocationTarget(&self, target: &Object) -> Id<Object, Shared> {
        msg_send_id![self, prepareWithInvocationTarget: target]
    }
    pub unsafe fn registerUndoWithTarget_handler(&self, target: &Object, undoHandler: TodoBlock) {
        msg_send![self, registerUndoWithTarget: target, handler: undoHandler]
    }
    pub unsafe fn setActionIsDiscardable(&self, discardable: bool) {
        msg_send![self, setActionIsDiscardable: discardable]
    }
    pub unsafe fn undoActionIsDiscardable(&self) -> bool {
        msg_send![self, undoActionIsDiscardable]
    }
    pub unsafe fn redoActionIsDiscardable(&self) -> bool {
        msg_send![self, redoActionIsDiscardable]
    }
    pub unsafe fn undoActionName(&self) -> Id<NSString, Shared> {
        msg_send_id![self, undoActionName]
    }
    pub unsafe fn redoActionName(&self) -> Id<NSString, Shared> {
        msg_send_id![self, redoActionName]
    }
    pub unsafe fn setActionName(&self, actionName: &NSString) {
        msg_send![self, setActionName: actionName]
    }
    pub unsafe fn undoMenuItemTitle(&self) -> Id<NSString, Shared> {
        msg_send_id![self, undoMenuItemTitle]
    }
    pub unsafe fn redoMenuItemTitle(&self) -> Id<NSString, Shared> {
        msg_send_id![self, redoMenuItemTitle]
    }
    pub unsafe fn undoMenuTitleForUndoActionName(
        &self,
        actionName: &NSString,
    ) -> Id<NSString, Shared> {
        msg_send_id![self, undoMenuTitleForUndoActionName: actionName]
    }
    pub unsafe fn redoMenuTitleForUndoActionName(
        &self,
        actionName: &NSString,
    ) -> Id<NSString, Shared> {
        msg_send_id![self, redoMenuTitleForUndoActionName: actionName]
    }
}
