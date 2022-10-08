use crate::Foundation::generated::NSArray::*;
use crate::Foundation::generated::NSString::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_methods!(
    #[doc = "NSStringPathExtensions"]
    unsafe impl NSString {
        # [method_id (pathWithComponents :)]
        pub unsafe fn pathWithComponents(components: &NSArray<NSString>) -> Id<NSString, Shared>;
        #[method_id(pathComponents)]
        pub unsafe fn pathComponents(&self) -> Id<NSArray<NSString>, Shared>;
        #[method(isAbsolutePath)]
        pub unsafe fn isAbsolutePath(&self) -> bool;
        #[method_id(lastPathComponent)]
        pub unsafe fn lastPathComponent(&self) -> Id<NSString, Shared>;
        #[method_id(stringByDeletingLastPathComponent)]
        pub unsafe fn stringByDeletingLastPathComponent(&self) -> Id<NSString, Shared>;
        # [method_id (stringByAppendingPathComponent :)]
        pub fn stringByAppendingPathComponent(&self, str: &NSString) -> Id<NSString, Shared>;
        #[method_id(pathExtension)]
        pub unsafe fn pathExtension(&self) -> Id<NSString, Shared>;
        #[method_id(stringByDeletingPathExtension)]
        pub unsafe fn stringByDeletingPathExtension(&self) -> Id<NSString, Shared>;
        # [method_id (stringByAppendingPathExtension :)]
        pub unsafe fn stringByAppendingPathExtension(
            &self,
            str: &NSString,
        ) -> Option<Id<NSString, Shared>>;
        #[method_id(stringByAbbreviatingWithTildeInPath)]
        pub unsafe fn stringByAbbreviatingWithTildeInPath(&self) -> Id<NSString, Shared>;
        #[method_id(stringByExpandingTildeInPath)]
        pub unsafe fn stringByExpandingTildeInPath(&self) -> Id<NSString, Shared>;
        #[method_id(stringByStandardizingPath)]
        pub unsafe fn stringByStandardizingPath(&self) -> Id<NSString, Shared>;
        #[method_id(stringByResolvingSymlinksInPath)]
        pub unsafe fn stringByResolvingSymlinksInPath(&self) -> Id<NSString, Shared>;
        # [method_id (stringsByAppendingPaths :)]
        pub unsafe fn stringsByAppendingPaths(
            &self,
            paths: &NSArray<NSString>,
        ) -> Id<NSArray<NSString>, Shared>;
        # [method (completePathIntoString : caseSensitive : matchesIntoArray : filterTypes :)]
        pub unsafe fn completePathIntoString_caseSensitive_matchesIntoArray_filterTypes(
            &self,
            outputName: Option<&mut Option<Id<NSString, Shared>>>,
            flag: bool,
            outputArray: Option<&mut Option<Id<NSArray<NSString>, Shared>>>,
            filterTypes: Option<&NSArray<NSString>>,
        ) -> NSUInteger;
        #[method(fileSystemRepresentation)]
        pub unsafe fn fileSystemRepresentation(&self) -> NonNull<c_char>;
        # [method (getFileSystemRepresentation : maxLength :)]
        pub unsafe fn getFileSystemRepresentation_maxLength(
            &self,
            cname: NonNull<c_char>,
            max: NSUInteger,
        ) -> bool;
    }
);
extern_methods!(
    #[doc = "NSArrayPathExtensions"]
    unsafe impl<ObjectType: Message> NSArray<ObjectType> {
        # [method_id (pathsMatchingExtensions :)]
        pub unsafe fn pathsMatchingExtensions(
            &self,
            filterTypes: &NSArray<NSString>,
        ) -> Id<NSArray<NSString>, Shared>;
    }
);
