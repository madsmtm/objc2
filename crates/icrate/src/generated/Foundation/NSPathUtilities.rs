#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
#[doc = "NSStringPathExtensions"]
impl NSString {
    pub unsafe fn pathWithComponents(components: TodoGenerics) -> Id<NSString, Shared> {
        msg_send_id![Self::class(), pathWithComponents: components]
    }
    pub fn stringByAppendingPathComponent(&self, str: &NSString) -> Id<NSString, Shared> {
        msg_send_id![self, stringByAppendingPathComponent: str]
    }
    pub unsafe fn stringByAppendingPathExtension(
        &self,
        str: &NSString,
    ) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, stringByAppendingPathExtension: str]
    }
    pub unsafe fn stringsByAppendingPaths(&self, paths: TodoGenerics) -> TodoGenerics {
        msg_send![self, stringsByAppendingPaths: paths]
    }
    pub unsafe fn completePathIntoString_caseSensitive_matchesIntoArray_filterTypes(
        &self,
        outputName: *mut Option<&NSString>,
        flag: bool,
        outputArray: *mut TodoGenerics,
        filterTypes: TodoGenerics,
    ) -> NSUInteger {
        msg_send![
            self,
            completePathIntoString: outputName,
            caseSensitive: flag,
            matchesIntoArray: outputArray,
            filterTypes: filterTypes
        ]
    }
    pub unsafe fn getFileSystemRepresentation_maxLength(
        &self,
        cname: NonNull<c_char>,
        max: NSUInteger,
    ) -> bool {
        msg_send![self, getFileSystemRepresentation: cname, maxLength: max]
    }
    pub unsafe fn pathComponents(&self) -> TodoGenerics {
        msg_send![self, pathComponents]
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
    pub unsafe fn pathExtension(&self) -> Id<NSString, Shared> {
        msg_send_id![self, pathExtension]
    }
    pub unsafe fn stringByDeletingPathExtension(&self) -> Id<NSString, Shared> {
        msg_send_id![self, stringByDeletingPathExtension]
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
    pub unsafe fn fileSystemRepresentation(&self) -> NonNull<c_char> {
        msg_send![self, fileSystemRepresentation]
    }
}
#[doc = "NSArrayPathExtensions"]
impl NSArray {
    pub unsafe fn pathsMatchingExtensions(&self, filterTypes: TodoGenerics) -> TodoGenerics {
        msg_send![self, pathsMatchingExtensions: filterTypes]
    }
}
