//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};

pub type NSWindowRestoration = NSObject;

extern_methods!(
    /// NSWindowRestoration
    unsafe impl NSDocumentController {}
);

extern_methods!(
    /// NSWindowRestoration
    unsafe impl NSApplication {
        #[method(restoreWindowWithIdentifier:state:completionHandler:)]
        pub unsafe fn restoreWindowWithIdentifier_state_completionHandler(
            &self,
            identifier: &NSUserInterfaceItemIdentifier,
            state: &NSCoder,
            completionHandler: TodoBlock,
        ) -> bool;
    }
);

extern_methods!(
    /// NSUserInterfaceRestoration
    unsafe impl NSWindow {
        #[method(isRestorable)]
        pub unsafe fn isRestorable(&self) -> bool;

        #[method(setRestorable:)]
        pub unsafe fn setRestorable(&self, restorable: bool);

        #[method_id(restorationClass)]
        pub unsafe fn restorationClass(&self) -> Option<Id<TodoProtocols, Shared>>;

        #[method(setRestorationClass:)]
        pub unsafe fn setRestorationClass(&self, restorationClass: Option<&TodoProtocols>);

        #[method(disableSnapshotRestoration)]
        pub unsafe fn disableSnapshotRestoration(&self);

        #[method(enableSnapshotRestoration)]
        pub unsafe fn enableSnapshotRestoration(&self);
    }
);

extern_methods!(
    /// NSRestorableState
    unsafe impl NSResponder {
        #[method(encodeRestorableStateWithCoder:)]
        pub unsafe fn encodeRestorableStateWithCoder(&self, coder: &NSCoder);

        #[method(encodeRestorableStateWithCoder:backgroundQueue:)]
        pub unsafe fn encodeRestorableStateWithCoder_backgroundQueue(
            &self,
            coder: &NSCoder,
            queue: &NSOperationQueue,
        );

        #[method(restoreStateWithCoder:)]
        pub unsafe fn restoreStateWithCoder(&self, coder: &NSCoder);

        #[method(invalidateRestorableState)]
        pub unsafe fn invalidateRestorableState(&self);

        #[method_id(restorableStateKeyPaths)]
        pub unsafe fn restorableStateKeyPaths() -> Id<NSArray<NSString>, Shared>;

        #[method_id(allowedClassesForRestorableStateKeyPath:)]
        pub unsafe fn allowedClassesForRestorableStateKeyPath(
            keyPath: &NSString,
        ) -> Id<NSArray<TodoClass>, Shared>;
    }
);

extern_methods!(
    /// NSRestorableStateExtension
    unsafe impl NSApplication {
        #[method(extendStateRestoration)]
        pub unsafe fn extendStateRestoration(&self);

        #[method(completeStateRestoration)]
        pub unsafe fn completeStateRestoration(&self);
    }
);

extern_methods!(
    /// NSRestorableState
    unsafe impl NSDocument {
        #[method(restoreDocumentWindowWithIdentifier:state:completionHandler:)]
        pub unsafe fn restoreDocumentWindowWithIdentifier_state_completionHandler(
            &self,
            identifier: &NSUserInterfaceItemIdentifier,
            state: &NSCoder,
            completionHandler: TodoBlock,
        );

        #[method(encodeRestorableStateWithCoder:)]
        pub unsafe fn encodeRestorableStateWithCoder(&self, coder: &NSCoder);

        #[method(encodeRestorableStateWithCoder:backgroundQueue:)]
        pub unsafe fn encodeRestorableStateWithCoder_backgroundQueue(
            &self,
            coder: &NSCoder,
            queue: &NSOperationQueue,
        );

        #[method(restoreStateWithCoder:)]
        pub unsafe fn restoreStateWithCoder(&self, coder: &NSCoder);

        #[method(invalidateRestorableState)]
        pub unsafe fn invalidateRestorableState(&self);

        #[method_id(restorableStateKeyPaths)]
        pub unsafe fn restorableStateKeyPaths() -> Id<NSArray<NSString>, Shared>;

        #[method_id(allowedClassesForRestorableStateKeyPath:)]
        pub unsafe fn allowedClassesForRestorableStateKeyPath(
            keyPath: &NSString,
        ) -> Id<NSArray<TodoClass>, Shared>;
    }
);
