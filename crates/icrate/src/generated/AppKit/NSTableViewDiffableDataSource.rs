#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
__inner_extern_class!(
    #[derive(Debug)]
    pub struct NSTableViewDiffableDataSource<
        SectionIdentifierType: Message,
        ItemIdentifierType: Message,
    >;
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
            &self,
            tableView: &NSTableView,
            cellProvider: NSTableViewDiffableDataSourceCellProvider,
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
