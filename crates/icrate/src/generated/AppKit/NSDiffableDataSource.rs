use crate::AppKit::generated::NSCollectionView::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
__inner_extern_class!(
    #[derive(Debug)]
    pub struct NSDiffableDataSourceSnapshot<
        SectionIdentifierType: Message,
        ItemIdentifierType: Message,
    >;
    unsafe impl<SectionIdentifierType: Message, ItemIdentifierType: Message> ClassType
        for NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>
    {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl<SectionIdentifierType: Message, ItemIdentifierType: Message>
        NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>
    {
        #[method(numberOfItems)]
        pub unsafe fn numberOfItems(&self) -> NSInteger;
        #[method(numberOfSections)]
        pub unsafe fn numberOfSections(&self) -> NSInteger;
        #[method_id(sectionIdentifiers)]
        pub unsafe fn sectionIdentifiers(&self) -> Id<NSArray<SectionIdentifierType>, Shared>;
        #[method_id(itemIdentifiers)]
        pub unsafe fn itemIdentifiers(&self) -> Id<NSArray<ItemIdentifierType>, Shared>;
        #[method(numberOfItemsInSection:)]
        pub unsafe fn numberOfItemsInSection(
            &self,
            sectionIdentifier: &SectionIdentifierType,
        ) -> NSInteger;
        #[method_id(itemIdentifiersInSectionWithIdentifier:)]
        pub unsafe fn itemIdentifiersInSectionWithIdentifier(
            &self,
            sectionIdentifier: &SectionIdentifierType,
        ) -> Id<NSArray<ItemIdentifierType>, Shared>;
        #[method_id(sectionIdentifierForSectionContainingItemIdentifier:)]
        pub unsafe fn sectionIdentifierForSectionContainingItemIdentifier(
            &self,
            itemIdentifier: &ItemIdentifierType,
        ) -> Option<Id<SectionIdentifierType, Shared>>;
        #[method(indexOfItemIdentifier:)]
        pub unsafe fn indexOfItemIdentifier(
            &self,
            itemIdentifier: &ItemIdentifierType,
        ) -> NSInteger;
        #[method(indexOfSectionIdentifier:)]
        pub unsafe fn indexOfSectionIdentifier(
            &self,
            sectionIdentifier: &SectionIdentifierType,
        ) -> NSInteger;
        #[method(appendItemsWithIdentifiers:)]
        pub unsafe fn appendItemsWithIdentifiers(&self, identifiers: &NSArray<ItemIdentifierType>);
        #[method(appendItemsWithIdentifiers:intoSectionWithIdentifier:)]
        pub unsafe fn appendItemsWithIdentifiers_intoSectionWithIdentifier(
            &self,
            identifiers: &NSArray<ItemIdentifierType>,
            sectionIdentifier: &SectionIdentifierType,
        );
        #[method(insertItemsWithIdentifiers:beforeItemWithIdentifier:)]
        pub unsafe fn insertItemsWithIdentifiers_beforeItemWithIdentifier(
            &self,
            identifiers: &NSArray<ItemIdentifierType>,
            itemIdentifier: &ItemIdentifierType,
        );
        #[method(insertItemsWithIdentifiers:afterItemWithIdentifier:)]
        pub unsafe fn insertItemsWithIdentifiers_afterItemWithIdentifier(
            &self,
            identifiers: &NSArray<ItemIdentifierType>,
            itemIdentifier: &ItemIdentifierType,
        );
        #[method(deleteItemsWithIdentifiers:)]
        pub unsafe fn deleteItemsWithIdentifiers(&self, identifiers: &NSArray<ItemIdentifierType>);
        #[method(deleteAllItems)]
        pub unsafe fn deleteAllItems(&self);
        #[method(moveItemWithIdentifier:beforeItemWithIdentifier:)]
        pub unsafe fn moveItemWithIdentifier_beforeItemWithIdentifier(
            &self,
            fromIdentifier: &ItemIdentifierType,
            toIdentifier: &ItemIdentifierType,
        );
        #[method(moveItemWithIdentifier:afterItemWithIdentifier:)]
        pub unsafe fn moveItemWithIdentifier_afterItemWithIdentifier(
            &self,
            fromIdentifier: &ItemIdentifierType,
            toIdentifier: &ItemIdentifierType,
        );
        #[method(reloadItemsWithIdentifiers:)]
        pub unsafe fn reloadItemsWithIdentifiers(&self, identifiers: &NSArray<ItemIdentifierType>);
        #[method(appendSectionsWithIdentifiers:)]
        pub unsafe fn appendSectionsWithIdentifiers(&self, sectionIdentifiers: &NSArray);
        #[method(insertSectionsWithIdentifiers:beforeSectionWithIdentifier:)]
        pub unsafe fn insertSectionsWithIdentifiers_beforeSectionWithIdentifier(
            &self,
            sectionIdentifiers: &NSArray<SectionIdentifierType>,
            toSectionIdentifier: &SectionIdentifierType,
        );
        #[method(insertSectionsWithIdentifiers:afterSectionWithIdentifier:)]
        pub unsafe fn insertSectionsWithIdentifiers_afterSectionWithIdentifier(
            &self,
            sectionIdentifiers: &NSArray<SectionIdentifierType>,
            toSectionIdentifier: &SectionIdentifierType,
        );
        #[method(deleteSectionsWithIdentifiers:)]
        pub unsafe fn deleteSectionsWithIdentifiers(
            &self,
            sectionIdentifiers: &NSArray<SectionIdentifierType>,
        );
        #[method(moveSectionWithIdentifier:beforeSectionWithIdentifier:)]
        pub unsafe fn moveSectionWithIdentifier_beforeSectionWithIdentifier(
            &self,
            fromSectionIdentifier: &SectionIdentifierType,
            toSectionIdentifier: &SectionIdentifierType,
        );
        #[method(moveSectionWithIdentifier:afterSectionWithIdentifier:)]
        pub unsafe fn moveSectionWithIdentifier_afterSectionWithIdentifier(
            &self,
            fromSectionIdentifier: &SectionIdentifierType,
            toSectionIdentifier: &SectionIdentifierType,
        );
        #[method(reloadSectionsWithIdentifiers:)]
        pub unsafe fn reloadSectionsWithIdentifiers(
            &self,
            sectionIdentifiers: &NSArray<SectionIdentifierType>,
        );
    }
);
__inner_extern_class!(
    #[derive(Debug)]
    pub struct NSCollectionViewDiffableDataSource<
        SectionIdentifierType: Message,
        ItemIdentifierType: Message,
    >;
    unsafe impl<SectionIdentifierType: Message, ItemIdentifierType: Message> ClassType
        for NSCollectionViewDiffableDataSource<SectionIdentifierType, ItemIdentifierType>
    {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl<SectionIdentifierType: Message, ItemIdentifierType: Message>
        NSCollectionViewDiffableDataSource<SectionIdentifierType, ItemIdentifierType>
    {
        #[method_id(initWithCollectionView:itemProvider:)]
        pub unsafe fn initWithCollectionView_itemProvider(
            &self,
            collectionView: &NSCollectionView,
            itemProvider: NSCollectionViewDiffableDataSourceItemProvider,
        ) -> Id<Self, Shared>;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
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
        #[method_id(itemIdentifierForIndexPath:)]
        pub unsafe fn itemIdentifierForIndexPath(
            &self,
            indexPath: &NSIndexPath,
        ) -> Option<Id<ItemIdentifierType, Shared>>;
        #[method_id(indexPathForItemIdentifier:)]
        pub unsafe fn indexPathForItemIdentifier(
            &self,
            identifier: &ItemIdentifierType,
        ) -> Option<Id<NSIndexPath, Shared>>;
        #[method(supplementaryViewProvider)]
        pub unsafe fn supplementaryViewProvider(
            &self,
        ) -> NSCollectionViewDiffableDataSourceSupplementaryViewProvider;
        #[method(setSupplementaryViewProvider:)]
        pub unsafe fn setSupplementaryViewProvider(
            &self,
            supplementaryViewProvider: NSCollectionViewDiffableDataSourceSupplementaryViewProvider,
        );
    }
);
