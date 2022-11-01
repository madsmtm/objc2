//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug)]
    pub struct NSSortDescriptor;

    unsafe impl ClassType for NSSortDescriptor {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSSortDescriptor {
        #[method_id(@__retain_semantics Other sortDescriptorWithKey:ascending:)]
        pub unsafe fn sortDescriptorWithKey_ascending(
            key: Option<&NSString>,
            ascending: bool,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other sortDescriptorWithKey:ascending:selector:)]
        pub unsafe fn sortDescriptorWithKey_ascending_selector(
            key: Option<&NSString>,
            ascending: bool,
            selector: Option<Sel>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithKey:ascending:)]
        pub unsafe fn initWithKey_ascending(
            this: Option<Allocated<Self>>,
            key: Option<&NSString>,
            ascending: bool,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithKey:ascending:selector:)]
        pub unsafe fn initWithKey_ascending_selector(
            this: Option<Allocated<Self>>,
            key: Option<&NSString>,
            ascending: bool,
            selector: Option<Sel>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Other key)]
        pub unsafe fn key(&self) -> Option<Id<NSString, Shared>>;

        #[method(ascending)]
        pub unsafe fn ascending(&self) -> bool;

        #[method(selector)]
        pub unsafe fn selector(&self) -> Option<Sel>;

        #[method(allowEvaluation)]
        pub unsafe fn allowEvaluation(&self);

        #[method_id(@__retain_semantics Other sortDescriptorWithKey:ascending:comparator:)]
        pub unsafe fn sortDescriptorWithKey_ascending_comparator(
            key: Option<&NSString>,
            ascending: bool,
            cmptr: NSComparator,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithKey:ascending:comparator:)]
        pub unsafe fn initWithKey_ascending_comparator(
            this: Option<Allocated<Self>>,
            key: Option<&NSString>,
            ascending: bool,
            cmptr: NSComparator,
        ) -> Id<Self, Shared>;

        #[method(comparator)]
        pub unsafe fn comparator(&self) -> NSComparator;

        #[method(compareObject:toObject:)]
        pub unsafe fn compareObject_toObject(
            &self,
            object1: &Object,
            object2: &Object,
        ) -> NSComparisonResult;

        #[method_id(@__retain_semantics Other reversedSortDescriptor)]
        pub unsafe fn reversedSortDescriptor(&self) -> Id<Object, Shared>;
    }
);

extern_methods!(
    /// NSSortDescriptorSorting
    unsafe impl<ObjectType: Message> NSSet<ObjectType> {
        #[method_id(@__retain_semantics Other sortedArrayUsingDescriptors:)]
        pub unsafe fn sortedArrayUsingDescriptors(
            &self,
            sortDescriptors: &NSArray<NSSortDescriptor>,
        ) -> Id<NSArray<ObjectType>, Shared>;
    }
);

extern_methods!(
    /// NSSortDescriptorSorting
    unsafe impl<ObjectType: Message> NSArray<ObjectType> {
        #[method_id(@__retain_semantics Other sortedArrayUsingDescriptors:)]
        pub unsafe fn sortedArrayUsingDescriptors(
            &self,
            sortDescriptors: &NSArray<NSSortDescriptor>,
        ) -> Id<NSArray<ObjectType>, Shared>;
    }
);

extern_methods!(
    /// NSSortDescriptorSorting
    unsafe impl<ObjectType: Message> NSMutableArray<ObjectType> {
        #[method(sortUsingDescriptors:)]
        pub unsafe fn sortUsingDescriptors(&self, sortDescriptors: &NSArray<NSSortDescriptor>);
    }
);

extern_methods!(
    /// NSKeyValueSorting
    unsafe impl<ObjectType: Message> NSOrderedSet<ObjectType> {
        #[method_id(@__retain_semantics Other sortedArrayUsingDescriptors:)]
        pub unsafe fn sortedArrayUsingDescriptors(
            &self,
            sortDescriptors: &NSArray<NSSortDescriptor>,
        ) -> Id<NSArray<ObjectType>, Shared>;
    }
);

extern_methods!(
    /// NSKeyValueSorting
    unsafe impl<ObjectType: Message> NSMutableOrderedSet<ObjectType> {
        #[method(sortUsingDescriptors:)]
        pub unsafe fn sortUsingDescriptors(&self, sortDescriptors: &NSArray<NSSortDescriptor>);
    }
);
