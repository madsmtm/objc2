//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;

__inner_extern_class!(
    #[derive(Debug)]
    pub struct NSTableViewDiffableDataSource<
        SectionIdentifierType: Message = Object,
        ItemIdentifierType: Message = Object,
    > {
        _inner0: PhantomData<*mut SectionIdentifierType>,
        _inner1: PhantomData<*mut ItemIdentifierType>,
    }

    unsafe impl<SectionIdentifierType: Message, ItemIdentifierType: Message> ClassType
        for NSTableViewDiffableDataSource<SectionIdentifierType, ItemIdentifierType>
    {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl<SectionIdentifierType: Message, ItemIdentifierType: Message>
        NSTableViewDiffableDataSource<SectionIdentifierType, ItemIdentifierType>
    {
        #[method_id(initWithTableView:cellProvider:)]
        pub unsafe fn initWithTableView_cellProvider(
            this: Option<Allocated<Self>>,
            tableView: &NSTableView,
            cellProvider: NSTableViewDiffableDataSourceCellProvider,
        ) -> Id<Self, Shared>;

        #[method_id(init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(new)]
        pub unsafe fn new() -> Id<Self, Shared>;

        #[method_id(snapshot)]
        pub unsafe fn snapshot(
            &self,
        ) -> Id<NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>, Shared>;

        #[method(applySnapshot:animatingDifferences:)]
        pub unsafe fn applySnapshot_animatingDifferences(
            &self,
            snapshot: &NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>,
            animatingDifferences: bool,
        );

        #[method(applySnapshot:animatingDifferences:completion:)]
        pub unsafe fn applySnapshot_animatingDifferences_completion(
            &self,
            snapshot: &NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>,
            animatingDifferences: bool,
            completion: TodoBlock,
        );

        #[method_id(itemIdentifierForRow:)]
        pub unsafe fn itemIdentifierForRow(
            &self,
            row: NSInteger,
        ) -> Option<Id<ItemIdentifierType, Shared>>;

        #[method(rowForItemIdentifier:)]
        pub unsafe fn rowForItemIdentifier(&self, identifier: &ItemIdentifierType) -> NSInteger;

        #[method_id(sectionIdentifierForRow:)]
        pub unsafe fn sectionIdentifierForRow(
            &self,
            row: NSInteger,
        ) -> Option<Id<SectionIdentifierType, Shared>>;

        #[method(rowForSectionIdentifier:)]
        pub unsafe fn rowForSectionIdentifier(
            &self,
            identifier: &SectionIdentifierType,
        ) -> NSInteger;

        #[method(rowViewProvider)]
        pub unsafe fn rowViewProvider(&self) -> NSTableViewDiffableDataSourceRowProvider;

        #[method(setRowViewProvider:)]
        pub unsafe fn setRowViewProvider(
            &self,
            rowViewProvider: NSTableViewDiffableDataSourceRowProvider,
        );

        #[method(sectionHeaderViewProvider)]
        pub unsafe fn sectionHeaderViewProvider(
            &self,
        ) -> NSTableViewDiffableDataSourceSectionHeaderViewProvider;

        #[method(setSectionHeaderViewProvider:)]
        pub unsafe fn setSectionHeaderViewProvider(
            &self,
            sectionHeaderViewProvider: NSTableViewDiffableDataSourceSectionHeaderViewProvider,
        );

        #[method(defaultRowAnimation)]
        pub unsafe fn defaultRowAnimation(&self) -> NSTableViewAnimationOptions;

        #[method(setDefaultRowAnimation:)]
        pub unsafe fn setDefaultRowAnimation(
            &self,
            defaultRowAnimation: NSTableViewAnimationOptions,
        );
    }
);
