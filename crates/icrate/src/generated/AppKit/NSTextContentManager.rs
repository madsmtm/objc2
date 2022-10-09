use super::__exported::NSTextElement;
use super::__exported::NSTextLayoutManager;
use super::__exported::NSTextLocation;
use super::__exported::NSTextParagraph;
use super::__exported::NSTextRange;
use super::__exported::NSTextStorage;
use super::__exported::NSTextStorageObserving;
use crate::AppKit::generated::AppKitDefines::*;
use crate::Foundation::generated::NSArray::*;
use crate::Foundation::generated::NSNotification::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSTextElementProvider = NSObject;
extern_class!(
    #[derive(Debug)]
    pub struct NSTextContentManager;
    unsafe impl ClassType for NSTextContentManager {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSTextContentManager {
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>>;
        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSTextContentManagerDelegate, Shared>>;
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSTextContentManagerDelegate>);
        #[method_id(textLayoutManagers)]
        pub unsafe fn textLayoutManagers(&self) -> Id<NSArray<NSTextLayoutManager>, Shared>;
        #[method(addTextLayoutManager:)]
        pub unsafe fn addTextLayoutManager(&self, textLayoutManager: &NSTextLayoutManager);
        #[method(removeTextLayoutManager:)]
        pub unsafe fn removeTextLayoutManager(&self, textLayoutManager: &NSTextLayoutManager);
        #[method_id(primaryTextLayoutManager)]
        pub unsafe fn primaryTextLayoutManager(&self) -> Option<Id<NSTextLayoutManager, Shared>>;
        #[method(setPrimaryTextLayoutManager:)]
        pub unsafe fn setPrimaryTextLayoutManager(
            &self,
            primaryTextLayoutManager: Option<&NSTextLayoutManager>,
        );
        #[method(synchronizeTextLayoutManagers:)]
        pub unsafe fn synchronizeTextLayoutManagers(&self, completionHandler: TodoBlock);
        #[method_id(textElementsForRange:)]
        pub unsafe fn textElementsForRange(
            &self,
            range: &NSTextRange,
        ) -> Id<NSArray<NSTextElement>, Shared>;
        #[method(hasEditingTransaction)]
        pub unsafe fn hasEditingTransaction(&self) -> bool;
        #[method(performEditingTransactionUsingBlock:)]
        pub unsafe fn performEditingTransactionUsingBlock(&self, transaction: TodoBlock);
        #[method(recordEditActionInRange:newTextRange:)]
        pub unsafe fn recordEditActionInRange_newTextRange(
            &self,
            originalTextRange: &NSTextRange,
            newTextRange: &NSTextRange,
        );
        #[method(automaticallySynchronizesTextLayoutManagers)]
        pub unsafe fn automaticallySynchronizesTextLayoutManagers(&self) -> bool;
        #[method(setAutomaticallySynchronizesTextLayoutManagers:)]
        pub unsafe fn setAutomaticallySynchronizesTextLayoutManagers(
            &self,
            automaticallySynchronizesTextLayoutManagers: bool,
        );
        #[method(automaticallySynchronizesToBackingStore)]
        pub unsafe fn automaticallySynchronizesToBackingStore(&self) -> bool;
        #[method(setAutomaticallySynchronizesToBackingStore:)]
        pub unsafe fn setAutomaticallySynchronizesToBackingStore(
            &self,
            automaticallySynchronizesToBackingStore: bool,
        );
    }
);
pub type NSTextContentManagerDelegate = NSObject;
pub type NSTextContentStorageDelegate = NSObject;
extern_class!(
    #[derive(Debug)]
    pub struct NSTextContentStorage;
    unsafe impl ClassType for NSTextContentStorage {
        type Super = NSTextContentManager;
    }
);
extern_methods!(
    unsafe impl NSTextContentStorage {
        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSTextContentStorageDelegate, Shared>>;
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSTextContentStorageDelegate>);
        #[method_id(attributedString)]
        pub unsafe fn attributedString(&self) -> Option<Id<NSAttributedString, Shared>>;
        #[method(setAttributedString:)]
        pub unsafe fn setAttributedString(&self, attributedString: Option<&NSAttributedString>);
        #[method_id(attributedStringForTextElement:)]
        pub unsafe fn attributedStringForTextElement(
            &self,
            textElement: &NSTextElement,
        ) -> Option<Id<NSAttributedString, Shared>>;
        #[method_id(textElementForAttributedString:)]
        pub unsafe fn textElementForAttributedString(
            &self,
            attributedString: &NSAttributedString,
        ) -> Option<Id<NSTextElement, Shared>>;
        #[method_id(locationFromLocation:withOffset:)]
        pub unsafe fn locationFromLocation_withOffset(
            &self,
            location: &NSTextLocation,
            offset: NSInteger,
        ) -> Option<Id<NSTextLocation, Shared>>;
        #[method(offsetFromLocation:toLocation:)]
        pub unsafe fn offsetFromLocation_toLocation(
            &self,
            from: &NSTextLocation,
            to: &NSTextLocation,
        ) -> NSInteger;
        #[method_id(adjustedRangeFromRange:forEditingTextSelection:)]
        pub unsafe fn adjustedRangeFromRange_forEditingTextSelection(
            &self,
            textRange: &NSTextRange,
            forEditingTextSelection: bool,
        ) -> Option<Id<NSTextRange, Shared>>;
    }
);
