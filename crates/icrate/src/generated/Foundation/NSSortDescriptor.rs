use crate::Foundation::generated::NSArray::*;
use crate::Foundation::generated::NSOrderedSet::*;
use crate::Foundation::generated::NSSet::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSSortDescriptor;
    unsafe impl ClassType for NSSortDescriptor {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSSortDescriptor {
        # [method_id (sortDescriptorWithKey : ascending :)]
        pub unsafe fn sortDescriptorWithKey_ascending(
            key: Option<&NSString>,
            ascending: bool,
        ) -> Id<Self, Shared>;
        # [method_id (sortDescriptorWithKey : ascending : selector :)]
        pub unsafe fn sortDescriptorWithKey_ascending_selector(
            key: Option<&NSString>,
            ascending: bool,
            selector: Option<Sel>,
        ) -> Id<Self, Shared>;
        # [method_id (initWithKey : ascending :)]
        pub unsafe fn initWithKey_ascending(
            &self,
            key: Option<&NSString>,
            ascending: bool,
        ) -> Id<Self, Shared>;
        # [method_id (initWithKey : ascending : selector :)]
        pub unsafe fn initWithKey_ascending_selector(
            &self,
            key: Option<&NSString>,
            ascending: bool,
            selector: Option<Sel>,
        ) -> Id<Self, Shared>;
        # [method_id (initWithCoder :)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>>;
        #[method_id(key)]
        pub unsafe fn key(&self) -> Option<Id<NSString, Shared>>;
        #[method(ascending)]
        pub unsafe fn ascending(&self) -> bool;
        #[method(selector)]
        pub unsafe fn selector(&self) -> Option<Sel>;
        #[method(allowEvaluation)]
        pub unsafe fn allowEvaluation(&self);
        # [method_id (sortDescriptorWithKey : ascending : comparator :)]
        pub unsafe fn sortDescriptorWithKey_ascending_comparator(
            key: Option<&NSString>,
            ascending: bool,
            cmptr: NSComparator,
        ) -> Id<Self, Shared>;
        # [method_id (initWithKey : ascending : comparator :)]
        pub unsafe fn initWithKey_ascending_comparator(
            &self,
            key: Option<&NSString>,
            ascending: bool,
            cmptr: NSComparator,
        ) -> Id<Self, Shared>;
        #[method(comparator)]
        pub unsafe fn comparator(&self) -> NSComparator;
        # [method (compareObject : toObject :)]
        pub unsafe fn compareObject_toObject(
            &self,
            object1: &Object,
            object2: &Object,
        ) -> NSComparisonResult;
        #[method_id(reversedSortDescriptor)]
        pub unsafe fn reversedSortDescriptor(&self) -> Id<Object, Shared>;
    }
);
extern_methods!(
    #[doc = "NSSortDescriptorSorting"]
    unsafe impl<ObjectType: Message> NSSet<ObjectType> {
        # [method_id (sortedArrayUsingDescriptors :)]
        pub unsafe fn sortedArrayUsingDescriptors(
            &self,
            sortDescriptors: &NSArray<NSSortDescriptor>,
        ) -> Id<NSArray<ObjectType>, Shared>;
    }
);
extern_methods!(
    #[doc = "NSSortDescriptorSorting"]
    unsafe impl<ObjectType: Message> NSArray<ObjectType> {
        # [method_id (sortedArrayUsingDescriptors :)]
        pub unsafe fn sortedArrayUsingDescriptors(
            &self,
            sortDescriptors: &NSArray<NSSortDescriptor>,
        ) -> Id<NSArray<ObjectType>, Shared>;
    }
);
extern_methods!(
    #[doc = "NSSortDescriptorSorting"]
    unsafe impl<ObjectType: Message> NSMutableArray<ObjectType> {
        # [method (sortUsingDescriptors :)]
        pub unsafe fn sortUsingDescriptors(&self, sortDescriptors: &NSArray<NSSortDescriptor>);
    }
);
extern_methods!(
    #[doc = "NSKeyValueSorting"]
    unsafe impl<ObjectType: Message> NSOrderedSet<ObjectType> {
        # [method_id (sortedArrayUsingDescriptors :)]
        pub unsafe fn sortedArrayUsingDescriptors(
            &self,
            sortDescriptors: &NSArray<NSSortDescriptor>,
        ) -> Id<NSArray<ObjectType>, Shared>;
    }
);
extern_methods!(
    #[doc = "NSKeyValueSorting"]
    unsafe impl<ObjectType: Message> NSMutableOrderedSet<ObjectType> {
        # [method (sortUsingDescriptors :)]
        pub unsafe fn sortUsingDescriptors(&self, sortDescriptors: &NSArray<NSSortDescriptor>);
    }
);
