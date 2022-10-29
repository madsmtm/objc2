#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSArrayController;
    unsafe impl ClassType for NSArrayController {
        type Super = NSObjectController;
    }
);
extern_methods!(
    unsafe impl NSArrayController {
        #[method(rearrangeObjects)]
        pub unsafe fn rearrangeObjects(&self);
        #[method(automaticallyRearrangesObjects)]
        pub unsafe fn automaticallyRearrangesObjects(&self) -> bool;
        #[method(setAutomaticallyRearrangesObjects:)]
        pub unsafe fn setAutomaticallyRearrangesObjects(
            &self,
            automaticallyRearrangesObjects: bool,
        );
        #[method_id(automaticRearrangementKeyPaths)]
        pub unsafe fn automaticRearrangementKeyPaths(
            &self,
        ) -> Option<Id<NSArray<NSString>, Shared>>;
        #[method(didChangeArrangementCriteria)]
        pub unsafe fn didChangeArrangementCriteria(&self);
        #[method_id(sortDescriptors)]
        pub unsafe fn sortDescriptors(&self) -> Id<NSArray<NSSortDescriptor>, Shared>;
        #[method(setSortDescriptors:)]
        pub unsafe fn setSortDescriptors(&self, sortDescriptors: &NSArray<NSSortDescriptor>);
        #[method_id(filterPredicate)]
        pub unsafe fn filterPredicate(&self) -> Option<Id<NSPredicate, Shared>>;
        #[method(setFilterPredicate:)]
        pub unsafe fn setFilterPredicate(&self, filterPredicate: Option<&NSPredicate>);
        #[method(clearsFilterPredicateOnInsertion)]
        pub unsafe fn clearsFilterPredicateOnInsertion(&self) -> bool;
        #[method(setClearsFilterPredicateOnInsertion:)]
        pub unsafe fn setClearsFilterPredicateOnInsertion(
            &self,
            clearsFilterPredicateOnInsertion: bool,
        );
        #[method_id(arrangeObjects:)]
        pub unsafe fn arrangeObjects(&self, objects: &NSArray) -> Id<NSArray, Shared>;
        #[method_id(arrangedObjects)]
        pub unsafe fn arrangedObjects(&self) -> Id<Object, Shared>;
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
        #[method(setSelectionIndexes:)]
        pub unsafe fn setSelectionIndexes(&self, indexes: &NSIndexSet) -> bool;
        #[method_id(selectionIndexes)]
        pub unsafe fn selectionIndexes(&self) -> Id<NSIndexSet, Shared>;
        #[method(setSelectionIndex:)]
        pub unsafe fn setSelectionIndex(&self, index: NSUInteger) -> bool;
        #[method(selectionIndex)]
        pub unsafe fn selectionIndex(&self) -> NSUInteger;
        #[method(addSelectionIndexes:)]
        pub unsafe fn addSelectionIndexes(&self, indexes: &NSIndexSet) -> bool;
        #[method(removeSelectionIndexes:)]
        pub unsafe fn removeSelectionIndexes(&self, indexes: &NSIndexSet) -> bool;
        #[method(setSelectedObjects:)]
        pub unsafe fn setSelectedObjects(&self, objects: &NSArray) -> bool;
        #[method_id(selectedObjects)]
        pub unsafe fn selectedObjects(&self) -> Id<NSArray, Shared>;
        #[method(addSelectedObjects:)]
        pub unsafe fn addSelectedObjects(&self, objects: &NSArray) -> bool;
        #[method(removeSelectedObjects:)]
        pub unsafe fn removeSelectedObjects(&self, objects: &NSArray) -> bool;
        #[method(add:)]
        pub unsafe fn add(&self, sender: Option<&Object>);
        #[method(remove:)]
        pub unsafe fn remove(&self, sender: Option<&Object>);
        #[method(insert:)]
        pub unsafe fn insert(&self, sender: Option<&Object>);
        #[method(canInsert)]
        pub unsafe fn canInsert(&self) -> bool;
        #[method(selectNext:)]
        pub unsafe fn selectNext(&self, sender: Option<&Object>);
        #[method(selectPrevious:)]
        pub unsafe fn selectPrevious(&self, sender: Option<&Object>);
        #[method(canSelectNext)]
        pub unsafe fn canSelectNext(&self) -> bool;
        #[method(canSelectPrevious)]
        pub unsafe fn canSelectPrevious(&self) -> bool;
        #[method(addObject:)]
        pub unsafe fn addObject(&self, object: &Object);
        #[method(addObjects:)]
        pub unsafe fn addObjects(&self, objects: &NSArray);
        #[method(insertObject:atArrangedObjectIndex:)]
        pub unsafe fn insertObject_atArrangedObjectIndex(&self, object: &Object, index: NSUInteger);
        #[method(insertObjects:atArrangedObjectIndexes:)]
        pub unsafe fn insertObjects_atArrangedObjectIndexes(
            &self,
            objects: &NSArray,
            indexes: &NSIndexSet,
        );
        #[method(removeObjectAtArrangedObjectIndex:)]
        pub unsafe fn removeObjectAtArrangedObjectIndex(&self, index: NSUInteger);
        #[method(removeObjectsAtArrangedObjectIndexes:)]
        pub unsafe fn removeObjectsAtArrangedObjectIndexes(&self, indexes: &NSIndexSet);
        #[method(removeObject:)]
        pub unsafe fn removeObject(&self, object: &Object);
        #[method(removeObjects:)]
        pub unsafe fn removeObjects(&self, objects: &NSArray);
    }
);
