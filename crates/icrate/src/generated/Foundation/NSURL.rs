#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSURL;
    unsafe impl ClassType for NSURL {
        type Super = NSObject;
    }
);
impl NSURL {
    pub unsafe fn initWithScheme_host_path(
        &self,
        scheme: &NSString,
        host: Option<&NSString>,
        path: &NSString,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithScheme: scheme, host: host, path: path]
    }
    pub unsafe fn initFileURLWithPath_isDirectory_relativeToURL(
        &self,
        path: &NSString,
        isDir: bool,
        baseURL: Option<&NSURL>,
    ) -> Id<Self, Shared> {
        msg_send_id![
            self,
            initFileURLWithPath: path,
            isDirectory: isDir,
            relativeToURL: baseURL
        ]
    }
    pub unsafe fn initFileURLWithPath_relativeToURL(
        &self,
        path: &NSString,
        baseURL: Option<&NSURL>,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initFileURLWithPath: path, relativeToURL: baseURL]
    }
    pub unsafe fn initFileURLWithPath_isDirectory(
        &self,
        path: &NSString,
        isDir: bool,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initFileURLWithPath: path, isDirectory: isDir]
    }
    pub unsafe fn initFileURLWithPath(&self, path: &NSString) -> Id<Self, Shared> {
        msg_send_id![self, initFileURLWithPath: path]
    }
    pub unsafe fn fileURLWithPath_isDirectory_relativeToURL(
        path: &NSString,
        isDir: bool,
        baseURL: Option<&NSURL>,
    ) -> Id<NSURL, Shared> {
        msg_send_id![
            Self::class(),
            fileURLWithPath: path,
            isDirectory: isDir,
            relativeToURL: baseURL
        ]
    }
    pub unsafe fn fileURLWithPath_relativeToURL(
        path: &NSString,
        baseURL: Option<&NSURL>,
    ) -> Id<NSURL, Shared> {
        msg_send_id![Self::class(), fileURLWithPath: path, relativeToURL: baseURL]
    }
    pub unsafe fn fileURLWithPath_isDirectory(path: &NSString, isDir: bool) -> Id<NSURL, Shared> {
        msg_send_id![Self::class(), fileURLWithPath: path, isDirectory: isDir]
    }
    pub unsafe fn fileURLWithPath(path: &NSString) -> Id<NSURL, Shared> {
        msg_send_id![Self::class(), fileURLWithPath: path]
    }
    pub unsafe fn initFileURLWithFileSystemRepresentation_isDirectory_relativeToURL(
        &self,
        path: NonNull<c_char>,
        isDir: bool,
        baseURL: Option<&NSURL>,
    ) -> Id<Self, Shared> {
        msg_send_id![
            self,
            initFileURLWithFileSystemRepresentation: path,
            isDirectory: isDir,
            relativeToURL: baseURL
        ]
    }
    pub unsafe fn fileURLWithFileSystemRepresentation_isDirectory_relativeToURL(
        path: NonNull<c_char>,
        isDir: bool,
        baseURL: Option<&NSURL>,
    ) -> Id<NSURL, Shared> {
        msg_send_id![
            Self::class(),
            fileURLWithFileSystemRepresentation: path,
            isDirectory: isDir,
            relativeToURL: baseURL
        ]
    }
    pub unsafe fn initWithString(&self, URLString: &NSString) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithString: URLString]
    }
    pub unsafe fn initWithString_relativeToURL(
        &self,
        URLString: &NSString,
        baseURL: Option<&NSURL>,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithString: URLString, relativeToURL: baseURL]
    }
    pub unsafe fn URLWithString(URLString: &NSString) -> Option<Id<Self, Shared>> {
        msg_send_id![Self::class(), URLWithString: URLString]
    }
    pub unsafe fn URLWithString_relativeToURL(
        URLString: &NSString,
        baseURL: Option<&NSURL>,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![
            Self::class(),
            URLWithString: URLString,
            relativeToURL: baseURL
        ]
    }
    pub unsafe fn initWithDataRepresentation_relativeToURL(
        &self,
        data: &NSData,
        baseURL: Option<&NSURL>,
    ) -> Id<Self, Shared> {
        msg_send_id![
            self,
            initWithDataRepresentation: data,
            relativeToURL: baseURL
        ]
    }
    pub unsafe fn URLWithDataRepresentation_relativeToURL(
        data: &NSData,
        baseURL: Option<&NSURL>,
    ) -> Id<NSURL, Shared> {
        msg_send_id![
            Self::class(),
            URLWithDataRepresentation: data,
            relativeToURL: baseURL
        ]
    }
    pub unsafe fn initAbsoluteURLWithDataRepresentation_relativeToURL(
        &self,
        data: &NSData,
        baseURL: Option<&NSURL>,
    ) -> Id<Self, Shared> {
        msg_send_id![
            self,
            initAbsoluteURLWithDataRepresentation: data,
            relativeToURL: baseURL
        ]
    }
    pub unsafe fn absoluteURLWithDataRepresentation_relativeToURL(
        data: &NSData,
        baseURL: Option<&NSURL>,
    ) -> Id<NSURL, Shared> {
        msg_send_id![
            Self::class(),
            absoluteURLWithDataRepresentation: data,
            relativeToURL: baseURL
        ]
    }
    pub unsafe fn getFileSystemRepresentation_maxLength(
        &self,
        buffer: NonNull<c_char>,
        maxBufferLength: NSUInteger,
    ) -> bool {
        msg_send![
            self,
            getFileSystemRepresentation: buffer,
            maxLength: maxBufferLength
        ]
    }
    pub unsafe fn checkResourceIsReachableAndReturnError(
        &self,
        error: *mut Option<&NSError>,
    ) -> bool {
        msg_send![self, checkResourceIsReachableAndReturnError: error]
    }
    pub unsafe fn isFileReferenceURL(&self) -> bool {
        msg_send![self, isFileReferenceURL]
    }
    pub unsafe fn fileReferenceURL(&self) -> Option<Id<NSURL, Shared>> {
        msg_send_id![self, fileReferenceURL]
    }
    pub unsafe fn getResourceValue_forKey_error(
        &self,
        value: NonNull<Option<&Object>>,
        key: NSURLResourceKey,
        error: *mut Option<&NSError>,
    ) -> bool {
        msg_send![self, getResourceValue: value, forKey: key, error: error]
    }
    pub unsafe fn resourceValuesForKeys_error(
        &self,
        keys: TodoGenerics,
        error: *mut Option<&NSError>,
    ) -> TodoGenerics {
        msg_send![self, resourceValuesForKeys: keys, error: error]
    }
    pub unsafe fn setResourceValue_forKey_error(
        &self,
        value: Option<&Object>,
        key: NSURLResourceKey,
        error: *mut Option<&NSError>,
    ) -> bool {
        msg_send![self, setResourceValue: value, forKey: key, error: error]
    }
    pub unsafe fn setResourceValues_error(
        &self,
        keyedValues: TodoGenerics,
        error: *mut Option<&NSError>,
    ) -> bool {
        msg_send![self, setResourceValues: keyedValues, error: error]
    }
    pub unsafe fn removeCachedResourceValueForKey(&self, key: NSURLResourceKey) {
        msg_send![self, removeCachedResourceValueForKey: key]
    }
    pub unsafe fn removeAllCachedResourceValues(&self) {
        msg_send![self, removeAllCachedResourceValues]
    }
    pub unsafe fn setTemporaryResourceValue_forKey(
        &self,
        value: Option<&Object>,
        key: NSURLResourceKey,
    ) {
        msg_send![self, setTemporaryResourceValue: value, forKey: key]
    }
    pub unsafe fn bookmarkDataWithOptions_includingResourceValuesForKeys_relativeToURL_error(
        &self,
        options: NSURLBookmarkCreationOptions,
        keys: TodoGenerics,
        relativeURL: Option<&NSURL>,
        error: *mut Option<&NSError>,
    ) -> Option<Id<NSData, Shared>> {
        msg_send_id![
            self,
            bookmarkDataWithOptions: options,
            includingResourceValuesForKeys: keys,
            relativeToURL: relativeURL,
            error: error
        ]
    }
    pub unsafe fn initByResolvingBookmarkData_options_relativeToURL_bookmarkDataIsStale_error(
        &self,
        bookmarkData: &NSData,
        options: NSURLBookmarkResolutionOptions,
        relativeURL: Option<&NSURL>,
        isStale: *mut bool,
        error: *mut Option<&NSError>,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![
            self,
            initByResolvingBookmarkData: bookmarkData,
            options: options,
            relativeToURL: relativeURL,
            bookmarkDataIsStale: isStale,
            error: error
        ]
    }
    pub unsafe fn URLByResolvingBookmarkData_options_relativeToURL_bookmarkDataIsStale_error(
        bookmarkData: &NSData,
        options: NSURLBookmarkResolutionOptions,
        relativeURL: Option<&NSURL>,
        isStale: *mut bool,
        error: *mut Option<&NSError>,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![
            Self::class(),
            URLByResolvingBookmarkData: bookmarkData,
            options: options,
            relativeToURL: relativeURL,
            bookmarkDataIsStale: isStale,
            error: error
        ]
    }
    pub unsafe fn resourceValuesForKeys_fromBookmarkData(
        keys: TodoGenerics,
        bookmarkData: &NSData,
    ) -> TodoGenerics {
        msg_send![
            Self::class(),
            resourceValuesForKeys: keys,
            fromBookmarkData: bookmarkData
        ]
    }
    pub unsafe fn writeBookmarkData_toURL_options_error(
        bookmarkData: &NSData,
        bookmarkFileURL: &NSURL,
        options: NSURLBookmarkFileCreationOptions,
        error: *mut Option<&NSError>,
    ) -> bool {
        msg_send![
            Self::class(),
            writeBookmarkData: bookmarkData,
            toURL: bookmarkFileURL,
            options: options,
            error: error
        ]
    }
    pub unsafe fn bookmarkDataWithContentsOfURL_error(
        bookmarkFileURL: &NSURL,
        error: *mut Option<&NSError>,
    ) -> Option<Id<NSData, Shared>> {
        msg_send_id![
            Self::class(),
            bookmarkDataWithContentsOfURL: bookmarkFileURL,
            error: error
        ]
    }
    pub unsafe fn URLByResolvingAliasFileAtURL_options_error(
        url: &NSURL,
        options: NSURLBookmarkResolutionOptions,
        error: *mut Option<&NSError>,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![
            Self::class(),
            URLByResolvingAliasFileAtURL: url,
            options: options,
            error: error
        ]
    }
    pub unsafe fn startAccessingSecurityScopedResource(&self) -> bool {
        msg_send![self, startAccessingSecurityScopedResource]
    }
    pub unsafe fn stopAccessingSecurityScopedResource(&self) {
        msg_send![self, stopAccessingSecurityScopedResource]
    }
    pub unsafe fn dataRepresentation(&self) -> Id<NSData, Shared> {
        msg_send_id![self, dataRepresentation]
    }
    pub unsafe fn absoluteString(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, absoluteString]
    }
    pub unsafe fn relativeString(&self) -> Id<NSString, Shared> {
        msg_send_id![self, relativeString]
    }
    pub unsafe fn baseURL(&self) -> Option<Id<NSURL, Shared>> {
        msg_send_id![self, baseURL]
    }
    pub unsafe fn absoluteURL(&self) -> Option<Id<NSURL, Shared>> {
        msg_send_id![self, absoluteURL]
    }
    pub unsafe fn scheme(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, scheme]
    }
    pub unsafe fn resourceSpecifier(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, resourceSpecifier]
    }
    pub unsafe fn host(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, host]
    }
    pub unsafe fn port(&self) -> Option<Id<NSNumber, Shared>> {
        msg_send_id![self, port]
    }
    pub unsafe fn user(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, user]
    }
    pub unsafe fn password(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, password]
    }
    pub unsafe fn path(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, path]
    }
    pub unsafe fn fragment(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, fragment]
    }
    pub unsafe fn parameterString(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, parameterString]
    }
    pub unsafe fn query(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, query]
    }
    pub unsafe fn relativePath(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, relativePath]
    }
    pub unsafe fn hasDirectoryPath(&self) -> bool {
        msg_send![self, hasDirectoryPath]
    }
    pub unsafe fn fileSystemRepresentation(&self) -> NonNull<c_char> {
        msg_send![self, fileSystemRepresentation]
    }
    pub unsafe fn isFileURL(&self) -> bool {
        msg_send![self, isFileURL]
    }
    pub unsafe fn standardizedURL(&self) -> Option<Id<NSURL, Shared>> {
        msg_send_id![self, standardizedURL]
    }
    pub unsafe fn filePathURL(&self) -> Option<Id<NSURL, Shared>> {
        msg_send_id![self, filePathURL]
    }
}
#[doc = "NSPromisedItems"]
impl NSURL {
    pub unsafe fn getPromisedItemResourceValue_forKey_error(
        &self,
        value: NonNull<Option<&Object>>,
        key: NSURLResourceKey,
        error: *mut Option<&NSError>,
    ) -> bool {
        msg_send![
            self,
            getPromisedItemResourceValue: value,
            forKey: key,
            error: error
        ]
    }
    pub unsafe fn promisedItemResourceValuesForKeys_error(
        &self,
        keys: TodoGenerics,
        error: *mut Option<&NSError>,
    ) -> TodoGenerics {
        msg_send![self, promisedItemResourceValuesForKeys: keys, error: error]
    }
    pub unsafe fn checkPromisedItemIsReachableAndReturnError(
        &self,
        error: *mut Option<&NSError>,
    ) -> bool {
        msg_send![self, checkPromisedItemIsReachableAndReturnError: error]
    }
}
#[doc = "NSItemProvider"]
impl NSURL {}
extern_class!(
    #[derive(Debug)]
    pub struct NSURLQueryItem;
    unsafe impl ClassType for NSURLQueryItem {
        type Super = NSObject;
    }
);
impl NSURLQueryItem {
    pub unsafe fn initWithName_value(
        &self,
        name: &NSString,
        value: Option<&NSString>,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initWithName: name, value: value]
    }
    pub unsafe fn queryItemWithName_value(
        name: &NSString,
        value: Option<&NSString>,
    ) -> Id<Self, Shared> {
        msg_send_id![Self::class(), queryItemWithName: name, value: value]
    }
    pub unsafe fn name(&self) -> Id<NSString, Shared> {
        msg_send_id![self, name]
    }
    pub unsafe fn value(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, value]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSURLComponents;
    unsafe impl ClassType for NSURLComponents {
        type Super = NSObject;
    }
);
impl NSURLComponents {
    pub unsafe fn init(&self) -> Id<Self, Shared> {
        msg_send_id![self, init]
    }
    pub unsafe fn initWithURL_resolvingAgainstBaseURL(
        &self,
        url: &NSURL,
        resolve: bool,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithURL: url, resolvingAgainstBaseURL: resolve]
    }
    pub unsafe fn componentsWithURL_resolvingAgainstBaseURL(
        url: &NSURL,
        resolve: bool,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![
            Self::class(),
            componentsWithURL: url,
            resolvingAgainstBaseURL: resolve
        ]
    }
    pub unsafe fn initWithString(&self, URLString: &NSString) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithString: URLString]
    }
    pub unsafe fn componentsWithString(URLString: &NSString) -> Option<Id<Self, Shared>> {
        msg_send_id![Self::class(), componentsWithString: URLString]
    }
    pub unsafe fn URLRelativeToURL(&self, baseURL: Option<&NSURL>) -> Option<Id<NSURL, Shared>> {
        msg_send_id![self, URLRelativeToURL: baseURL]
    }
    pub unsafe fn URL(&self) -> Option<Id<NSURL, Shared>> {
        msg_send_id![self, URL]
    }
    pub unsafe fn string(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, string]
    }
    pub unsafe fn scheme(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, scheme]
    }
    pub unsafe fn setScheme(&self, scheme: Option<&NSString>) {
        msg_send![self, setScheme: scheme]
    }
    pub unsafe fn user(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, user]
    }
    pub unsafe fn setUser(&self, user: Option<&NSString>) {
        msg_send![self, setUser: user]
    }
    pub unsafe fn password(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, password]
    }
    pub unsafe fn setPassword(&self, password: Option<&NSString>) {
        msg_send![self, setPassword: password]
    }
    pub unsafe fn host(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, host]
    }
    pub unsafe fn setHost(&self, host: Option<&NSString>) {
        msg_send![self, setHost: host]
    }
    pub unsafe fn port(&self) -> Option<Id<NSNumber, Shared>> {
        msg_send_id![self, port]
    }
    pub unsafe fn setPort(&self, port: Option<&NSNumber>) {
        msg_send![self, setPort: port]
    }
    pub unsafe fn path(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, path]
    }
    pub unsafe fn setPath(&self, path: Option<&NSString>) {
        msg_send![self, setPath: path]
    }
    pub unsafe fn query(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, query]
    }
    pub unsafe fn setQuery(&self, query: Option<&NSString>) {
        msg_send![self, setQuery: query]
    }
    pub unsafe fn fragment(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, fragment]
    }
    pub unsafe fn setFragment(&self, fragment: Option<&NSString>) {
        msg_send![self, setFragment: fragment]
    }
    pub unsafe fn percentEncodedUser(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, percentEncodedUser]
    }
    pub unsafe fn setPercentEncodedUser(&self, percentEncodedUser: Option<&NSString>) {
        msg_send![self, setPercentEncodedUser: percentEncodedUser]
    }
    pub unsafe fn percentEncodedPassword(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, percentEncodedPassword]
    }
    pub unsafe fn setPercentEncodedPassword(&self, percentEncodedPassword: Option<&NSString>) {
        msg_send![self, setPercentEncodedPassword: percentEncodedPassword]
    }
    pub unsafe fn percentEncodedHost(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, percentEncodedHost]
    }
    pub unsafe fn setPercentEncodedHost(&self, percentEncodedHost: Option<&NSString>) {
        msg_send![self, setPercentEncodedHost: percentEncodedHost]
    }
    pub unsafe fn percentEncodedPath(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, percentEncodedPath]
    }
    pub unsafe fn setPercentEncodedPath(&self, percentEncodedPath: Option<&NSString>) {
        msg_send![self, setPercentEncodedPath: percentEncodedPath]
    }
    pub unsafe fn percentEncodedQuery(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, percentEncodedQuery]
    }
    pub unsafe fn setPercentEncodedQuery(&self, percentEncodedQuery: Option<&NSString>) {
        msg_send![self, setPercentEncodedQuery: percentEncodedQuery]
    }
    pub unsafe fn percentEncodedFragment(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, percentEncodedFragment]
    }
    pub unsafe fn setPercentEncodedFragment(&self, percentEncodedFragment: Option<&NSString>) {
        msg_send![self, setPercentEncodedFragment: percentEncodedFragment]
    }
    pub unsafe fn rangeOfScheme(&self) -> NSRange {
        msg_send![self, rangeOfScheme]
    }
    pub unsafe fn rangeOfUser(&self) -> NSRange {
        msg_send![self, rangeOfUser]
    }
    pub unsafe fn rangeOfPassword(&self) -> NSRange {
        msg_send![self, rangeOfPassword]
    }
    pub unsafe fn rangeOfHost(&self) -> NSRange {
        msg_send![self, rangeOfHost]
    }
    pub unsafe fn rangeOfPort(&self) -> NSRange {
        msg_send![self, rangeOfPort]
    }
    pub unsafe fn rangeOfPath(&self) -> NSRange {
        msg_send![self, rangeOfPath]
    }
    pub unsafe fn rangeOfQuery(&self) -> NSRange {
        msg_send![self, rangeOfQuery]
    }
    pub unsafe fn rangeOfFragment(&self) -> NSRange {
        msg_send![self, rangeOfFragment]
    }
    pub unsafe fn queryItems(&self) -> TodoGenerics {
        msg_send![self, queryItems]
    }
    pub unsafe fn setQueryItems(&self, queryItems: TodoGenerics) {
        msg_send![self, setQueryItems: queryItems]
    }
    pub unsafe fn percentEncodedQueryItems(&self) -> TodoGenerics {
        msg_send![self, percentEncodedQueryItems]
    }
    pub unsafe fn setPercentEncodedQueryItems(&self, percentEncodedQueryItems: TodoGenerics) {
        msg_send![self, setPercentEncodedQueryItems: percentEncodedQueryItems]
    }
}
#[doc = "NSURLUtilities"]
impl NSCharacterSet {
    pub unsafe fn URLUserAllowedCharacterSet() -> Id<NSCharacterSet, Shared> {
        msg_send_id![Self::class(), URLUserAllowedCharacterSet]
    }
    pub unsafe fn URLPasswordAllowedCharacterSet() -> Id<NSCharacterSet, Shared> {
        msg_send_id![Self::class(), URLPasswordAllowedCharacterSet]
    }
    pub unsafe fn URLHostAllowedCharacterSet() -> Id<NSCharacterSet, Shared> {
        msg_send_id![Self::class(), URLHostAllowedCharacterSet]
    }
    pub unsafe fn URLPathAllowedCharacterSet() -> Id<NSCharacterSet, Shared> {
        msg_send_id![Self::class(), URLPathAllowedCharacterSet]
    }
    pub unsafe fn URLQueryAllowedCharacterSet() -> Id<NSCharacterSet, Shared> {
        msg_send_id![Self::class(), URLQueryAllowedCharacterSet]
    }
    pub unsafe fn URLFragmentAllowedCharacterSet() -> Id<NSCharacterSet, Shared> {
        msg_send_id![Self::class(), URLFragmentAllowedCharacterSet]
    }
}
#[doc = "NSURLUtilities"]
impl NSString {
    pub unsafe fn stringByAddingPercentEncodingWithAllowedCharacters(
        &self,
        allowedCharacters: &NSCharacterSet,
    ) -> Option<Id<NSString, Shared>> {
        msg_send_id![
            self,
            stringByAddingPercentEncodingWithAllowedCharacters: allowedCharacters
        ]
    }
    pub unsafe fn stringByAddingPercentEscapesUsingEncoding(
        &self,
        enc: NSStringEncoding,
    ) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, stringByAddingPercentEscapesUsingEncoding: enc]
    }
    pub unsafe fn stringByReplacingPercentEscapesUsingEncoding(
        &self,
        enc: NSStringEncoding,
    ) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, stringByReplacingPercentEscapesUsingEncoding: enc]
    }
    pub unsafe fn stringByRemovingPercentEncoding(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, stringByRemovingPercentEncoding]
    }
}
#[doc = "NSURLPathUtilities"]
impl NSURL {
    pub unsafe fn fileURLWithPathComponents(components: TodoGenerics) -> Option<Id<NSURL, Shared>> {
        msg_send_id![Self::class(), fileURLWithPathComponents: components]
    }
    pub unsafe fn URLByAppendingPathComponent(
        &self,
        pathComponent: &NSString,
    ) -> Option<Id<NSURL, Shared>> {
        msg_send_id![self, URLByAppendingPathComponent: pathComponent]
    }
    pub unsafe fn URLByAppendingPathComponent_isDirectory(
        &self,
        pathComponent: &NSString,
        isDirectory: bool,
    ) -> Option<Id<NSURL, Shared>> {
        msg_send_id![
            self,
            URLByAppendingPathComponent: pathComponent,
            isDirectory: isDirectory
        ]
    }
    pub unsafe fn URLByAppendingPathExtension(
        &self,
        pathExtension: &NSString,
    ) -> Option<Id<NSURL, Shared>> {
        msg_send_id![self, URLByAppendingPathExtension: pathExtension]
    }
    pub unsafe fn pathComponents(&self) -> TodoGenerics {
        msg_send![self, pathComponents]
    }
    pub unsafe fn lastPathComponent(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, lastPathComponent]
    }
    pub unsafe fn pathExtension(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, pathExtension]
    }
    pub unsafe fn URLByDeletingLastPathComponent(&self) -> Option<Id<NSURL, Shared>> {
        msg_send_id![self, URLByDeletingLastPathComponent]
    }
    pub unsafe fn URLByDeletingPathExtension(&self) -> Option<Id<NSURL, Shared>> {
        msg_send_id![self, URLByDeletingPathExtension]
    }
    pub unsafe fn URLByStandardizingPath(&self) -> Option<Id<NSURL, Shared>> {
        msg_send_id![self, URLByStandardizingPath]
    }
    pub unsafe fn URLByResolvingSymlinksInPath(&self) -> Option<Id<NSURL, Shared>> {
        msg_send_id![self, URLByResolvingSymlinksInPath]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSFileSecurity;
    unsafe impl ClassType for NSFileSecurity {
        type Super = NSObject;
    }
);
impl NSFileSecurity {
    pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithCoder: coder]
    }
}
#[doc = "NSURLClient"]
impl NSObject {
    pub unsafe fn URL_resourceDataDidBecomeAvailable(&self, sender: &NSURL, newBytes: &NSData) {
        msg_send![self, URL: sender, resourceDataDidBecomeAvailable: newBytes]
    }
    pub unsafe fn URLResourceDidFinishLoading(&self, sender: &NSURL) {
        msg_send![self, URLResourceDidFinishLoading: sender]
    }
    pub unsafe fn URLResourceDidCancelLoading(&self, sender: &NSURL) {
        msg_send![self, URLResourceDidCancelLoading: sender]
    }
    pub unsafe fn URL_resourceDidFailLoadingWithReason(&self, sender: &NSURL, reason: &NSString) {
        msg_send![self, URL: sender, resourceDidFailLoadingWithReason: reason]
    }
}
#[doc = "NSURLLoading"]
impl NSURL {
    pub unsafe fn resourceDataUsingCache(
        &self,
        shouldUseCache: bool,
    ) -> Option<Id<NSData, Shared>> {
        msg_send_id![self, resourceDataUsingCache: shouldUseCache]
    }
    pub unsafe fn loadResourceDataNotifyingClient_usingCache(
        &self,
        client: &Object,
        shouldUseCache: bool,
    ) {
        msg_send![
            self,
            loadResourceDataNotifyingClient: client,
            usingCache: shouldUseCache
        ]
    }
    pub unsafe fn propertyForKey(&self, propertyKey: &NSString) -> Option<Id<Object, Shared>> {
        msg_send_id![self, propertyForKey: propertyKey]
    }
    pub unsafe fn setResourceData(&self, data: &NSData) -> bool {
        msg_send![self, setResourceData: data]
    }
    pub unsafe fn setProperty_forKey(&self, property: &Object, propertyKey: &NSString) -> bool {
        msg_send![self, setProperty: property, forKey: propertyKey]
    }
    pub unsafe fn URLHandleUsingCache(
        &self,
        shouldUseCache: bool,
    ) -> Option<Id<NSURLHandle, Shared>> {
        msg_send_id![self, URLHandleUsingCache: shouldUseCache]
    }
}
