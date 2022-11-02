//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSDragOperation {
        NSDragOperationNone = 0,
        NSDragOperationCopy = 1,
        NSDragOperationLink = 2,
        NSDragOperationGeneric = 4,
        NSDragOperationPrivate = 8,
        NSDragOperationMove = 16,
        NSDragOperationDelete = 32,
        NSDragOperationEvery = 18446744073709551615,
        NSDragOperationAll_Obsolete = 15,
        NSDragOperationAll = NSDragOperationAll_Obsolete,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSDraggingFormation {
        NSDraggingFormationDefault = 0,
        NSDraggingFormationNone = 1,
        NSDraggingFormationPile = 2,
        NSDraggingFormationList = 3,
        NSDraggingFormationStack = 4,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSDraggingContext {
        NSDraggingContextOutsideApplication = 0,
        NSDraggingContextWithinApplication = 1,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSDraggingItemEnumerationOptions {
        NSDraggingItemEnumerationConcurrent = NSEnumerationConcurrent,
        NSDraggingItemEnumerationClearNonenumeratedImages = 1 << 16,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSSpringLoadingHighlight {
        NSSpringLoadingHighlightNone = 0,
        NSSpringLoadingHighlightStandard = 1,
        NSSpringLoadingHighlightEmphasized = 2,
    }
);

pub type NSDraggingInfo = NSObject;

pub type NSDraggingDestination = NSObject;

pub type NSDraggingSource = NSObject;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSSpringLoadingOptions {
        NSSpringLoadingDisabled = 0,
        NSSpringLoadingEnabled = 1 << 0,
        NSSpringLoadingContinuousActivation = 1 << 1,
        NSSpringLoadingNoHover = 1 << 3,
    }
);

pub type NSSpringLoadingDestination = NSObject;

extern_methods!(
    /// NSDraggingSourceDeprecated
    unsafe impl NSObject {
        #[method_id(@__retain_semantics Other namesOfPromisedFilesDroppedAtDestination:)]
        pub unsafe fn namesOfPromisedFilesDroppedAtDestination(
            &self,
            dropDestination: &NSURL,
        ) -> Option<Id<NSArray<NSString>, Shared>>;

        #[method(draggingSourceOperationMaskForLocal:)]
        pub unsafe fn draggingSourceOperationMaskForLocal(&self, flag: bool) -> NSDragOperation;

        #[method(draggedImage:beganAt:)]
        pub unsafe fn draggedImage_beganAt(&self, image: Option<&NSImage>, screenPoint: NSPoint);

        #[method(draggedImage:endedAt:operation:)]
        pub unsafe fn draggedImage_endedAt_operation(
            &self,
            image: Option<&NSImage>,
            screenPoint: NSPoint,
            operation: NSDragOperation,
        );

        #[method(draggedImage:movedTo:)]
        pub unsafe fn draggedImage_movedTo(&self, image: Option<&NSImage>, screenPoint: NSPoint);

        #[method(ignoreModifierKeysWhileDragging)]
        pub unsafe fn ignoreModifierKeysWhileDragging(&self) -> bool;

        #[method(draggedImage:endedAt:deposited:)]
        pub unsafe fn draggedImage_endedAt_deposited(
            &self,
            image: Option<&NSImage>,
            screenPoint: NSPoint,
            flag: bool,
        );
    }
);
