#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSTreeNode;
    unsafe impl ClassType for NSTreeNode {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSTreeNode {
        #[method_id(treeNodeWithRepresentedObject:)]
        pub unsafe fn treeNodeWithRepresentedObject(
            modelObject: Option<&Object>,
        ) -> Id<Self, Shared>;
        #[method_id(initWithRepresentedObject:)]
        pub unsafe fn initWithRepresentedObject(
            &self,
            modelObject: Option<&Object>,
        ) -> Id<Self, Shared>;
        #[method_id(representedObject)]
        pub unsafe fn representedObject(&self) -> Option<Id<Object, Shared>>;
        #[method_id(indexPath)]
        pub unsafe fn indexPath(&self) -> Id<NSIndexPath, Shared>;
        #[method(isLeaf)]
        pub unsafe fn isLeaf(&self) -> bool;
        #[method_id(childNodes)]
        pub unsafe fn childNodes(&self) -> Option<Id<NSArray<NSTreeNode>, Shared>>;
        #[method_id(mutableChildNodes)]
        pub unsafe fn mutableChildNodes(&self) -> Id<NSMutableArray<NSTreeNode>, Shared>;
        #[method_id(descendantNodeAtIndexPath:)]
        pub unsafe fn descendantNodeAtIndexPath(
            &self,
            indexPath: &NSIndexPath,
        ) -> Option<Id<NSTreeNode, Shared>>;
        #[method_id(parentNode)]
        pub unsafe fn parentNode(&self) -> Option<Id<NSTreeNode, Shared>>;
        #[method(sortWithSortDescriptors:recursively:)]
        pub unsafe fn sortWithSortDescriptors_recursively(
            &self,
            sortDescriptors: &NSArray<NSSortDescriptor>,
            recursively: bool,
        );
    }
);