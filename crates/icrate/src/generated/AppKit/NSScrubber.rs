//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

pub type NSScrubberDataSource = NSObject;

pub type NSScrubberDelegate = NSObject;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSScrubberMode {
        NSScrubberModeFixed = 0,
        NSScrubberModeFree = 1,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSScrubberAlignment {
        NSScrubberAlignmentNone = 0,
        NSScrubberAlignmentLeading = 1,
        NSScrubberAlignmentTrailing = 2,
        NSScrubberAlignmentCenter = 3,
    }
);

extern_class!(
    #[derive(Debug)]
    pub struct NSScrubberSelectionStyle;

    unsafe impl ClassType for NSScrubberSelectionStyle {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSScrubberSelectionStyle {
        #[method_id(@__retain_semantics Other outlineOverlayStyle)]
        pub unsafe fn outlineOverlayStyle() -> Id<NSScrubberSelectionStyle, Shared>;

        #[method_id(@__retain_semantics Other roundedBackgroundStyle)]
        pub unsafe fn roundedBackgroundStyle() -> Id<NSScrubberSelectionStyle, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other makeSelectionView)]
        pub unsafe fn makeSelectionView(&self) -> Option<Id<NSScrubberSelectionView, Shared>>;
    }
);

extern_class!(
    #[derive(Debug)]
    pub struct NSScrubber;

    unsafe impl ClassType for NSScrubber {
        type Super = NSView;
    }
);

extern_methods!(
    unsafe impl NSScrubber {
        #[method_id(@__retain_semantics Other dataSource)]
        pub unsafe fn dataSource(&self) -> Option<Id<NSScrubberDataSource, Shared>>;

        #[method(setDataSource:)]
        pub unsafe fn setDataSource(&self, dataSource: Option<&NSScrubberDataSource>);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSScrubberDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSScrubberDelegate>);

        #[method_id(@__retain_semantics Other scrubberLayout)]
        pub unsafe fn scrubberLayout(&self) -> Id<NSScrubberLayout, Shared>;

        #[method(setScrubberLayout:)]
        pub unsafe fn setScrubberLayout(&self, scrubberLayout: &NSScrubberLayout);

        #[method(numberOfItems)]
        pub unsafe fn numberOfItems(&self) -> NSInteger;

        #[method(highlightedIndex)]
        pub unsafe fn highlightedIndex(&self) -> NSInteger;

        #[method(selectedIndex)]
        pub unsafe fn selectedIndex(&self) -> NSInteger;

        #[method(setSelectedIndex:)]
        pub unsafe fn setSelectedIndex(&self, selectedIndex: NSInteger);

        #[method(mode)]
        pub unsafe fn mode(&self) -> NSScrubberMode;

        #[method(setMode:)]
        pub unsafe fn setMode(&self, mode: NSScrubberMode);

        #[method(itemAlignment)]
        pub unsafe fn itemAlignment(&self) -> NSScrubberAlignment;

        #[method(setItemAlignment:)]
        pub unsafe fn setItemAlignment(&self, itemAlignment: NSScrubberAlignment);

        #[method(isContinuous)]
        pub unsafe fn isContinuous(&self) -> bool;

        #[method(setContinuous:)]
        pub unsafe fn setContinuous(&self, continuous: bool);

        #[method(floatsSelectionViews)]
        pub unsafe fn floatsSelectionViews(&self) -> bool;

        #[method(setFloatsSelectionViews:)]
        pub unsafe fn setFloatsSelectionViews(&self, floatsSelectionViews: bool);

        #[method_id(@__retain_semantics Other selectionBackgroundStyle)]
        pub unsafe fn selectionBackgroundStyle(
            &self,
        ) -> Option<Id<NSScrubberSelectionStyle, Shared>>;

        #[method(setSelectionBackgroundStyle:)]
        pub unsafe fn setSelectionBackgroundStyle(
            &self,
            selectionBackgroundStyle: Option<&NSScrubberSelectionStyle>,
        );

        #[method_id(@__retain_semantics Other selectionOverlayStyle)]
        pub unsafe fn selectionOverlayStyle(&self) -> Option<Id<NSScrubberSelectionStyle, Shared>>;

        #[method(setSelectionOverlayStyle:)]
        pub unsafe fn setSelectionOverlayStyle(
            &self,
            selectionOverlayStyle: Option<&NSScrubberSelectionStyle>,
        );

        #[method(showsArrowButtons)]
        pub unsafe fn showsArrowButtons(&self) -> bool;

        #[method(setShowsArrowButtons:)]
        pub unsafe fn setShowsArrowButtons(&self, showsArrowButtons: bool);

        #[method(showsAdditionalContentIndicators)]
        pub unsafe fn showsAdditionalContentIndicators(&self) -> bool;

        #[method(setShowsAdditionalContentIndicators:)]
        pub unsafe fn setShowsAdditionalContentIndicators(
            &self,
            showsAdditionalContentIndicators: bool,
        );

        #[method_id(@__retain_semantics Other backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Option<Id<NSColor, Shared>>;

        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, backgroundColor: Option<&NSColor>);

        #[method_id(@__retain_semantics Other backgroundView)]
        pub unsafe fn backgroundView(&self) -> Option<Id<NSView, Shared>>;

        #[method(setBackgroundView:)]
        pub unsafe fn setBackgroundView(&self, backgroundView: Option<&NSView>);

        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(
            this: Option<Allocated<Self>>,
            frameRect: NSRect,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Id<Self, Shared>;

        #[method(reloadData)]
        pub unsafe fn reloadData(&self);

        #[method(performSequentialBatchUpdates:)]
        pub unsafe fn performSequentialBatchUpdates(&self, updateBlock: TodoBlock);

        #[method(insertItemsAtIndexes:)]
        pub unsafe fn insertItemsAtIndexes(&self, indexes: &NSIndexSet);

        #[method(removeItemsAtIndexes:)]
        pub unsafe fn removeItemsAtIndexes(&self, indexes: &NSIndexSet);

        #[method(reloadItemsAtIndexes:)]
        pub unsafe fn reloadItemsAtIndexes(&self, indexes: &NSIndexSet);

        #[method(moveItemAtIndex:toIndex:)]
        pub unsafe fn moveItemAtIndex_toIndex(&self, oldIndex: NSInteger, newIndex: NSInteger);

        #[method(scrollItemAtIndex:toAlignment:)]
        pub unsafe fn scrollItemAtIndex_toAlignment(
            &self,
            index: NSInteger,
            alignment: NSScrubberAlignment,
        );

        #[method_id(@__retain_semantics Other itemViewForItemAtIndex:)]
        pub unsafe fn itemViewForItemAtIndex(
            &self,
            index: NSInteger,
        ) -> Option<Id<NSScrubberItemView, Shared>>;

        #[method(registerClass:forItemIdentifier:)]
        pub unsafe fn registerClass_forItemIdentifier(
            &self,
            itemViewClass: Option<&Class>,
            itemIdentifier: &NSUserInterfaceItemIdentifier,
        );

        #[method(registerNib:forItemIdentifier:)]
        pub unsafe fn registerNib_forItemIdentifier(
            &self,
            nib: Option<&NSNib>,
            itemIdentifier: &NSUserInterfaceItemIdentifier,
        );

        #[method_id(@__retain_semantics Other makeItemWithIdentifier:owner:)]
        pub unsafe fn makeItemWithIdentifier_owner(
            &self,
            itemIdentifier: &NSUserInterfaceItemIdentifier,
            owner: Option<&Object>,
        ) -> Option<Id<NSScrubberItemView, Shared>>;
    }
);
