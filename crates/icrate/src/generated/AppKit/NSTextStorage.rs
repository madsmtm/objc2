#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSTextStorage;
    unsafe impl ClassType for NSTextStorage {
        type Super = NSMutableAttributedString;
    }
);
extern_methods!(
    unsafe impl NSTextStorage {
        #[method_id(layoutManagers)]
        pub unsafe fn layoutManagers(&self) -> Id<NSArray<NSLayoutManager>, Shared>;
        #[method(addLayoutManager:)]
        pub unsafe fn addLayoutManager(&self, aLayoutManager: &NSLayoutManager);
        #[method(removeLayoutManager:)]
        pub unsafe fn removeLayoutManager(&self, aLayoutManager: &NSLayoutManager);
        #[method(editedMask)]
        pub unsafe fn editedMask(&self) -> NSTextStorageEditActions;
        #[method(editedRange)]
        pub unsafe fn editedRange(&self) -> NSRange;
        #[method(changeInLength)]
        pub unsafe fn changeInLength(&self) -> NSInteger;
        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSTextStorageDelegate, Shared>>;
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSTextStorageDelegate>);
        #[method(edited:range:changeInLength:)]
        pub unsafe fn edited_range_changeInLength(
            &self,
            editedMask: NSTextStorageEditActions,
            editedRange: NSRange,
            delta: NSInteger,
        );
        #[method(processEditing)]
        pub unsafe fn processEditing(&self);
        #[method(fixesAttributesLazily)]
        pub unsafe fn fixesAttributesLazily(&self) -> bool;
        #[method(invalidateAttributesInRange:)]
        pub unsafe fn invalidateAttributesInRange(&self, range: NSRange);
        #[method(ensureAttributesAreFixedInRange:)]
        pub unsafe fn ensureAttributesAreFixedInRange(&self, range: NSRange);
        #[method_id(textStorageObserver)]
        pub unsafe fn textStorageObserver(&self) -> Option<Id<NSTextStorageObserving, Shared>>;
        #[method(setTextStorageObserver:)]
        pub unsafe fn setTextStorageObserver(
            &self,
            textStorageObserver: Option<&NSTextStorageObserving>,
        );
    }
);
pub type NSTextStorageDelegate = NSObject;
pub type NSTextStorageObserving = NSObject;
pub type NSTextStorageEditedOptions = NSUInteger;
extern_methods!(
    #[doc = "NSDeprecatedTextStorageDelegateInterface"]
    unsafe impl NSObject {
        #[method(textStorageWillProcessEditing:)]
        pub unsafe fn textStorageWillProcessEditing(&self, notification: &NSNotification);
        #[method(textStorageDidProcessEditing:)]
        pub unsafe fn textStorageDidProcessEditing(&self, notification: &NSNotification);
    }
);
