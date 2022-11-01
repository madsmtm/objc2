//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;

pub type NSTextContentManagerEnumerationOptions = NSUInteger;
pub const NSTextContentManagerEnumerationOptionsNone: NSTextContentManagerEnumerationOptions = 0;
pub const NSTextContentManagerEnumerationOptionsReverse: NSTextContentManagerEnumerationOptions =
    1 << 0;

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
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSTextContentManagerDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSTextContentManagerDelegate>);

        #[method_id(@__retain_semantics Other textLayoutManagers)]
        pub unsafe fn textLayoutManagers(&self) -> Id<NSArray<NSTextLayoutManager>, Shared>;

        #[method(addTextLayoutManager:)]
        pub unsafe fn addTextLayoutManager(&self, textLayoutManager: &NSTextLayoutManager);

        #[method(removeTextLayoutManager:)]
        pub unsafe fn removeTextLayoutManager(&self, textLayoutManager: &NSTextLayoutManager);

        #[method_id(@__retain_semantics Other primaryTextLayoutManager)]
        pub unsafe fn primaryTextLayoutManager(&self) -> Option<Id<NSTextLayoutManager, Shared>>;

        #[method(setPrimaryTextLayoutManager:)]
        pub unsafe fn setPrimaryTextLayoutManager(
            &self,
            primaryTextLayoutManager: Option<&NSTextLayoutManager>,
        );

        #[method(synchronizeTextLayoutManagers:)]
        pub unsafe fn synchronizeTextLayoutManagers(&self, completionHandler: TodoBlock);

        #[method_id(@__retain_semantics Other textElementsForRange:)]
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
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSTextContentStorageDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSTextContentStorageDelegate>);

        #[method_id(@__retain_semantics Other attributedString)]
        pub unsafe fn attributedString(&self) -> Option<Id<NSAttributedString, Shared>>;

        #[method(setAttributedString:)]
        pub unsafe fn setAttributedString(&self, attributedString: Option<&NSAttributedString>);

        #[method_id(@__retain_semantics Other attributedStringForTextElement:)]
        pub unsafe fn attributedStringForTextElement(
            &self,
            textElement: &NSTextElement,
        ) -> Option<Id<NSAttributedString, Shared>>;

        #[method_id(@__retain_semantics Other textElementForAttributedString:)]
        pub unsafe fn textElementForAttributedString(
            &self,
            attributedString: &NSAttributedString,
        ) -> Option<Id<NSTextElement, Shared>>;

        #[method_id(@__retain_semantics Other locationFromLocation:withOffset:)]
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

        #[method_id(@__retain_semantics Other adjustedRangeFromRange:forEditingTextSelection:)]
        pub unsafe fn adjustedRangeFromRange_forEditingTextSelection(
            &self,
            textRange: &NSTextRange,
            forEditingTextSelection: bool,
        ) -> Option<Id<NSTextRange, Shared>>;
    }
);

extern "C" {
    pub static NSTextContentStorageUnsupportedAttributeAddedNotification:
        &'static NSNotificationName;
}
