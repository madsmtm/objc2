use crate::Foundation::generated::NSArray::*;
use crate::Foundation::generated::NSString::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, msg_send, msg_send_id, ClassType};
extern_methods!(
    #[doc = "NSStringPathExtensions"]
    unsafe impl NSString {
        pub unsafe fn pathWithComponents(components: &NSArray<NSString>) -> Id<NSString, Shared> {
            msg_send_id![Self::class(), pathWithComponents: components]
        }
        pub unsafe fn pathComponents(&self) -> Id<NSArray<NSString>, Shared> {
            msg_send_id![self, pathComponents]
        }
        pub unsafe fn isAbsolutePath(&self) -> bool {
            msg_send![self, isAbsolutePath]
        }
        pub unsafe fn lastPathComponent(&self) -> Id<NSString, Shared> {
            msg_send_id![self, lastPathComponent]
        }
        pub unsafe fn stringByDeletingLastPathComponent(&self) -> Id<NSString, Shared> {
            msg_send_id![self, stringByDeletingLastPathComponent]
        }
        pub fn stringByAppendingPathComponent(&self, str: &NSString) -> Id<NSString, Shared> {
            msg_send_id![self, stringByAppendingPathComponent: str]
        }
        pub unsafe fn pathExtension(&self) -> Id<NSString, Shared> {
            msg_send_id![self, pathExtension]
        }
        pub unsafe fn stringByDeletingPathExtension(&self) -> Id<NSString, Shared> {
            msg_send_id![self, stringByDeletingPathExtension]
        }
        pub unsafe fn stringByAppendingPathExtension(
            &self,
            str: &NSString,
        ) -> Option<Id<NSString, Shared>> {
            msg_send_id![self, stringByAppendingPathExtension: str]
        }
        pub unsafe fn stringByAbbreviatingWithTildeInPath(&self) -> Id<NSString, Shared> {
            msg_send_id![self, stringByAbbreviatingWithTildeInPath]
        }
        pub unsafe fn stringByExpandingTildeInPath(&self) -> Id<NSString, Shared> {
            msg_send_id![self, stringByExpandingTildeInPath]
        }
        pub unsafe fn stringByStandardizingPath(&self) -> Id<NSString, Shared> {
            msg_send_id![self, stringByStandardizingPath]
        }
        pub unsafe fn stringByResolvingSymlinksInPath(&self) -> Id<NSString, Shared> {
            msg_send_id![self, stringByResolvingSymlinksInPath]
        }
        pub unsafe fn stringsByAppendingPaths(
            &self,
            paths: &NSArray<NSString>,
        ) -> Id<NSArray<NSString>, Shared> {
            msg_send_id![self, stringsByAppendingPaths: paths]
        }
        pub unsafe fn completePathIntoString_caseSensitive_matchesIntoArray_filterTypes(
            &self,
            outputName: Option<&mut Option<Id<NSString, Shared>>>,
            flag: bool,
            outputArray: Option<&mut Option<Id<NSArray<NSString>, Shared>>>,
            filterTypes: Option<&NSArray<NSString>>,
        ) -> NSUInteger {
            msg_send![
                self,
                completePathIntoString: outputName,
                caseSensitive: flag,
                matchesIntoArray: outputArray,
                filterTypes: filterTypes
            ]
        }
        pub unsafe fn fileSystemRepresentation(&self) -> NonNull<c_char> {
            msg_send![self, fileSystemRepresentation]
        }
        pub unsafe fn getFileSystemRepresentation_maxLength(
            &self,
            cname: NonNull<c_char>,
            max: NSUInteger,
        ) -> bool {
            msg_send![self, getFileSystemRepresentation: cname, maxLength: max]
        }
    }
);
extern_methods!(
    #[doc = "NSArrayPathExtensions"]
    unsafe impl<ObjectType: Message> NSArray<ObjectType> {
        pub unsafe fn pathsMatchingExtensions(
            &self,
            filterTypes: &NSArray<NSString>,
        ) -> Id<NSArray<NSString>, Shared> {
            msg_send_id![self, pathsMatchingExtensions: filterTypes]
        }
    }
);
