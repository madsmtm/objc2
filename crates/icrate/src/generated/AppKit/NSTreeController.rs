use super::__exported::NSSortDescriptor;
use super::__exported::NSTreeNode;
use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSObjectController::*;
use crate::Foundation::generated::NSArray::*;
use crate::Foundation::generated::NSIndexPath::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSTreeController;
    unsafe impl ClassType for NSTreeController {
        type Super = NSObjectController;
    }
);
extern_methods!(
    unsafe impl NSTreeController {
        #[method(rearrangeObjects)]
        pub unsafe fn rearrangeObjects(&self);
        #[method_id(arrangedObjects)]
        pub unsafe fn arrangedObjects(&self) -> Id<NSTreeNode, Shared>;
        #[method_id(childrenKeyPath)]
        pub unsafe fn childrenKeyPath(&self) -> Option<Id<NSString, Shared>>;
        #[method(setChildrenKeyPath:)]
        pub unsafe fn setChildrenKeyPath(&self, childrenKeyPath: Option<&NSString>);
        #[method_id(countKeyPath)]
        pub unsafe fn countKeyPath(&self) -> Option<Id<NSString, Shared>>;
        #[method(setCountKeyPath:)]
        pub unsafe fn setCountKeyPath(&self, countKeyPath: Option<&NSString>);
        #[method_id(leafKeyPath)]
        pub unsafe fn leafKeyPath(&self) -> Option<Id<NSString, Shared>>;
        #[method(setLeafKeyPath:)]
        pub unsafe fn setLeafKeyPath(&self, leafKeyPath: Option<&NSString>);
        #[method_id(sortDescriptors)]
        pub unsafe fn sortDescriptors(&self) -> Id<NSArray<NSSortDescriptor>, Shared>;
        #[method(setSortDescriptors:)]
        pub unsafe fn setSortDescriptors(&self, sortDescriptors: &NSArray<NSSortDescriptor>);
        #[method_id(content)]
        pub unsafe fn content(&self) -> Option<Id<Object, Shared>>;
        #[method(setContent:)]
        pub unsafe fn setContent(&self, content: Option<&Object>);
        #[method(add:)]
        pub unsafe fn add(&self, sender: Option<&Object>);
        #[method(remove:)]
        pub unsafe fn remove(&self, sender: Option<&Object>);
        #[method(addChild:)]
        pub unsafe fn addChild(&self, sender: Option<&Object>);
        #[method(insert:)]
        pub unsafe fn insert(&self, sender: Option<&Object>);
        #[method(insertChild:)]
        pub unsafe fn insertChild(&self, sender: Option<&Object>);
        #[method(canInsert)]
        pub unsafe fn canInsert(&self) -> bool;
        #[method(canInsertChild)]
        pub unsafe fn canInsertChild(&self) -> bool;
        #[method(canAddChild)]
        pub unsafe fn canAddChild(&self) -> bool;
        #[method(insertObject:atArrangedObjectIndexPath:)]
        pub unsafe fn insertObject_atArrangedObjectIndexPath(
            &self,
            object: Option<&Object>,
            indexPath: &NSIndexPath,
        );
        #[method(insertObjects:atArrangedObjectIndexPaths:)]
        pub unsafe fn insertObjects_atArrangedObjectIndexPaths(
            &self,
            objects: &NSArray,
            indexPaths: &NSArray<NSIndexPath>,
        );
        #[method(removeObjectAtArrangedObjectIndexPath:)]
        pub unsafe fn removeObjectAtArrangedObjectIndexPath(&self, indexPath: &NSIndexPath);
        #[method(removeObjectsAtArrangedObjectIndexPaths:)]
        pub unsafe fn removeObjectsAtArrangedObjectIndexPaths(
            &self,
            indexPaths: &NSArray<NSIndexPath>,
        );
        #[method(avoidsEmptySelection)]
        pub unsafe fn avoidsEmptySelection(&self) -> bool;
        #[method(setAvoidsEmptySelection:)]
        pub unsafe fn setAvoidsEmptySelection(&self, avoidsEmptySelection: bool);
        #[method(preservesSelection)]
        pub unsafe fn preservesSelection(&self) -> bool;
        #[method(setPreservesSelection:)]
        pub unsafe fn setPreservesSelection(&self, preservesSelection: bool);
        #[method(selectsInsertedObjects)]
        pub unsafe fn selectsInsertedObjects(&self) -> bool;
        #[method(setSelectsInsertedObjects:)]
        pub unsafe fn setSelectsInsertedObjects(&self, selectsInsertedObjects: bool);
        #[method(alwaysUsesMultipleValuesMarker)]
        pub unsafe fn alwaysUsesMultipleValuesMarker(&self) -> bool;
        #[method(setAlwaysUsesMultipleValuesMarker:)]
        pub unsafe fn setAlwaysUsesMultipleValuesMarker(
            &self,
            alwaysUsesMultipleValuesMarker: bool,
        );
        #[method_id(selectedObjects)]
        pub unsafe fn selectedObjects(&self) -> Id<NSArray, Shared>;
        #[method(setSelectionIndexPaths:)]
        pub unsafe fn setSelectionIndexPaths(&self, indexPaths: &NSArray<NSIndexPath>) -> bool;
        #[method_id(selectionIndexPaths)]
        pub unsafe fn selectionIndexPaths(&self) -> Id<NSArray<NSIndexPath>, Shared>;
        #[method(setSelectionIndexPath:)]
        pub unsafe fn setSelectionIndexPath(&self, indexPath: Option<&NSIndexPath>) -> bool;
        #[method_id(selectionIndexPath)]
        pub unsafe fn selectionIndexPath(&self) -> Option<Id<NSIndexPath, Shared>>;
        #[method(addSelectionIndexPaths:)]
        pub unsafe fn addSelectionIndexPaths(&self, indexPaths: &NSArray<NSIndexPath>) -> bool;
        #[method(removeSelectionIndexPaths:)]
        pub unsafe fn removeSelectionIndexPaths(&self, indexPaths: &NSArray<NSIndexPath>) -> bool;
        #[method_id(selectedNodes)]
        pub unsafe fn selectedNodes(&self) -> Id<NSArray<NSTreeNode>, Shared>;
        #[method(moveNode:toIndexPath:)]
        pub unsafe fn moveNode_toIndexPath(&self, node: &NSTreeNode, indexPath: &NSIndexPath);
        #[method(moveNodes:toIndexPath:)]
        pub unsafe fn moveNodes_toIndexPath(
            &self,
            nodes: &NSArray<NSTreeNode>,
            startingIndexPath: &NSIndexPath,
        );
        #[method_id(childrenKeyPathForNode:)]
        pub unsafe fn childrenKeyPathForNode(
            &self,
            node: &NSTreeNode,
        ) -> Option<Id<NSString, Shared>>;
        #[method_id(countKeyPathForNode:)]
        pub unsafe fn countKeyPathForNode(&self, node: &NSTreeNode)
            -> Option<Id<NSString, Shared>>;
        #[method_id(leafKeyPathForNode:)]
        pub unsafe fn leafKeyPathForNode(&self, node: &NSTreeNode) -> Option<Id<NSString, Shared>>;
    }
);
