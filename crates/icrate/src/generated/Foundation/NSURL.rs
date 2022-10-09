use super::__exported::NSArray;
use super::__exported::NSData;
use super::__exported::NSDictionary;
use super::__exported::NSNumber;
use crate::Foundation::generated::NSCharacterSet::*;
use crate::Foundation::generated::NSItemProvider::*;
use crate::Foundation::generated::NSObject::*;
use crate::Foundation::generated::NSString::*;
use crate::Foundation::generated::NSURLHandle::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSURLResourceKey = NSString;
pub type NSURLFileResourceType = NSString;
pub type NSURLThumbnailDictionaryItem = NSString;
pub type NSURLFileProtectionType = NSString;
pub type NSURLUbiquitousItemDownloadingStatus = NSString;
pub type NSURLUbiquitousSharedItemRole = NSString;
pub type NSURLUbiquitousSharedItemPermissions = NSString;
pub type NSURLBookmarkFileCreationOptions = NSUInteger;
extern_class!(
    #[derive(Debug)]
    pub struct NSURL;
    unsafe impl ClassType for NSURL {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSURL {
        #[method_id(initWithScheme:host:path:)]
        pub unsafe fn initWithScheme_host_path(
            &self,
            scheme: &NSString,
            host: Option<&NSString>,
            path: &NSString,
        ) -> Option<Id<Self, Shared>>;
        #[method_id(initFileURLWithPath:isDirectory:relativeToURL:)]
        pub unsafe fn initFileURLWithPath_isDirectory_relativeToURL(
            &self,
            path: &NSString,
            isDir: bool,
            baseURL: Option<&NSURL>,
        ) -> Id<Self, Shared>;
        #[method_id(initFileURLWithPath:relativeToURL:)]
        pub unsafe fn initFileURLWithPath_relativeToURL(
            &self,
            path: &NSString,
            baseURL: Option<&NSURL>,
        ) -> Id<Self, Shared>;
        #[method_id(initFileURLWithPath:isDirectory:)]
        pub unsafe fn initFileURLWithPath_isDirectory(
            &self,
            path: &NSString,
            isDir: bool,
        ) -> Id<Self, Shared>;
        #[method_id(initFileURLWithPath:)]
        pub unsafe fn initFileURLWithPath(&self, path: &NSString) -> Id<Self, Shared>;
        #[method_id(fileURLWithPath:isDirectory:relativeToURL:)]
        pub unsafe fn fileURLWithPath_isDirectory_relativeToURL(
            path: &NSString,
            isDir: bool,
            baseURL: Option<&NSURL>,
        ) -> Id<NSURL, Shared>;
        #[method_id(fileURLWithPath:relativeToURL:)]
        pub unsafe fn fileURLWithPath_relativeToURL(
            path: &NSString,
            baseURL: Option<&NSURL>,
        ) -> Id<NSURL, Shared>;
        #[method_id(fileURLWithPath:isDirectory:)]
        pub unsafe fn fileURLWithPath_isDirectory(
            path: &NSString,
            isDir: bool,
        ) -> Id<NSURL, Shared>;
        #[method_id(fileURLWithPath:)]
        pub unsafe fn fileURLWithPath(path: &NSString) -> Id<NSURL, Shared>;
        #[method_id(initFileURLWithFileSystemRepresentation:isDirectory:relativeToURL:)]
        pub unsafe fn initFileURLWithFileSystemRepresentation_isDirectory_relativeToURL(
            &self,
            path: NonNull<c_char>,
            isDir: bool,
            baseURL: Option<&NSURL>,
        ) -> Id<Self, Shared>;
        #[method_id(fileURLWithFileSystemRepresentation:isDirectory:relativeToURL:)]
        pub unsafe fn fileURLWithFileSystemRepresentation_isDirectory_relativeToURL(
            path: NonNull<c_char>,
            isDir: bool,
            baseURL: Option<&NSURL>,
        ) -> Id<NSURL, Shared>;
        #[method_id(initWithString:)]
        pub unsafe fn initWithString(&self, URLString: &NSString) -> Option<Id<Self, Shared>>;
        #[method_id(initWithString:relativeToURL:)]
        pub unsafe fn initWithString_relativeToURL(
            &self,
            URLString: &NSString,
            baseURL: Option<&NSURL>,
        ) -> Option<Id<Self, Shared>>;
        #[method_id(URLWithString:)]
        pub unsafe fn URLWithString(URLString: &NSString) -> Option<Id<Self, Shared>>;
        #[method_id(URLWithString:relativeToURL:)]
        pub unsafe fn URLWithString_relativeToURL(
            URLString: &NSString,
            baseURL: Option<&NSURL>,
        ) -> Option<Id<Self, Shared>>;
        #[method_id(initWithDataRepresentation:relativeToURL:)]
        pub unsafe fn initWithDataRepresentation_relativeToURL(
            &self,
            data: &NSData,
            baseURL: Option<&NSURL>,
        ) -> Id<Self, Shared>;
        #[method_id(URLWithDataRepresentation:relativeToURL:)]
        pub unsafe fn URLWithDataRepresentation_relativeToURL(
            data: &NSData,
            baseURL: Option<&NSURL>,
        ) -> Id<NSURL, Shared>;
        #[method_id(initAbsoluteURLWithDataRepresentation:relativeToURL:)]
        pub unsafe fn initAbsoluteURLWithDataRepresentation_relativeToURL(
            &self,
            data: &NSData,
            baseURL: Option<&NSURL>,
        ) -> Id<Self, Shared>;
        #[method_id(absoluteURLWithDataRepresentation:relativeToURL:)]
        pub unsafe fn absoluteURLWithDataRepresentation_relativeToURL(
            data: &NSData,
            baseURL: Option<&NSURL>,
        ) -> Id<NSURL, Shared>;
        #[method_id(dataRepresentation)]
        pub unsafe fn dataRepresentation(&self) -> Id<NSData, Shared>;
        #[method_id(absoluteString)]
        pub unsafe fn absoluteString(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(relativeString)]
        pub unsafe fn relativeString(&self) -> Id<NSString, Shared>;
        #[method_id(baseURL)]
        pub unsafe fn baseURL(&self) -> Option<Id<NSURL, Shared>>;
        #[method_id(absoluteURL)]
        pub unsafe fn absoluteURL(&self) -> Option<Id<NSURL, Shared>>;
        #[method_id(scheme)]
        pub unsafe fn scheme(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(resourceSpecifier)]
        pub unsafe fn resourceSpecifier(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(host)]
        pub unsafe fn host(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(port)]
        pub unsafe fn port(&self) -> Option<Id<NSNumber, Shared>>;
        #[method_id(user)]
        pub unsafe fn user(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(password)]
        pub unsafe fn password(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(path)]
        pub unsafe fn path(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(fragment)]
        pub unsafe fn fragment(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(parameterString)]
        pub unsafe fn parameterString(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(query)]
        pub unsafe fn query(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(relativePath)]
        pub unsafe fn relativePath(&self) -> Option<Id<NSString, Shared>>;
        #[method(hasDirectoryPath)]
        pub unsafe fn hasDirectoryPath(&self) -> bool;
        #[method(getFileSystemRepresentation:maxLength:)]
        pub unsafe fn getFileSystemRepresentation_maxLength(
            &self,
            buffer: NonNull<c_char>,
            maxBufferLength: NSUInteger,
        ) -> bool;
        #[method(fileSystemRepresentation)]
        pub unsafe fn fileSystemRepresentation(&self) -> NonNull<c_char>;
        #[method(isFileURL)]
        pub unsafe fn isFileURL(&self) -> bool;
        #[method_id(standardizedURL)]
        pub unsafe fn standardizedURL(&self) -> Option<Id<NSURL, Shared>>;
        #[method(checkResourceIsReachableAndReturnError:)]
        pub unsafe fn checkResourceIsReachableAndReturnError(
            &self,
        ) -> Result<(), Id<NSError, Shared>>;
        #[method(isFileReferenceURL)]
        pub unsafe fn isFileReferenceURL(&self) -> bool;
        #[method_id(fileReferenceURL)]
        pub unsafe fn fileReferenceURL(&self) -> Option<Id<NSURL, Shared>>;
        #[method_id(filePathURL)]
        pub unsafe fn filePathURL(&self) -> Option<Id<NSURL, Shared>>;
        #[method(getResourceValue:forKey:error:)]
        pub unsafe fn getResourceValue_forKey_error(
            &self,
            value: &mut Option<Id<Object, Shared>>,
            key: &NSURLResourceKey,
        ) -> Result<(), Id<NSError, Shared>>;
        #[method_id(resourceValuesForKeys:error:)]
        pub unsafe fn resourceValuesForKeys_error(
            &self,
            keys: &NSArray<NSURLResourceKey>,
        ) -> Result<Id<NSDictionary<NSURLResourceKey, Object>, Shared>, Id<NSError, Shared>>;
        #[method(setResourceValue:forKey:error:)]
        pub unsafe fn setResourceValue_forKey_error(
            &self,
            value: Option<&Object>,
            key: &NSURLResourceKey,
        ) -> Result<(), Id<NSError, Shared>>;
        #[method(setResourceValues:error:)]
        pub unsafe fn setResourceValues_error(
            &self,
            keyedValues: &NSDictionary<NSURLResourceKey, Object>,
        ) -> Result<(), Id<NSError, Shared>>;
        #[method(removeCachedResourceValueForKey:)]
        pub unsafe fn removeCachedResourceValueForKey(&self, key: &NSURLResourceKey);
        #[method(removeAllCachedResourceValues)]
        pub unsafe fn removeAllCachedResourceValues(&self);
        #[method(setTemporaryResourceValue:forKey:)]
        pub unsafe fn setTemporaryResourceValue_forKey(
            &self,
            value: Option<&Object>,
            key: &NSURLResourceKey,
        );
        #[method_id(bookmarkDataWithOptions:includingResourceValuesForKeys:relativeToURL:error:)]
        pub unsafe fn bookmarkDataWithOptions_includingResourceValuesForKeys_relativeToURL_error(
            &self,
            options: NSURLBookmarkCreationOptions,
            keys: Option<&NSArray<NSURLResourceKey>>,
            relativeURL: Option<&NSURL>,
        ) -> Result<Id<NSData, Shared>, Id<NSError, Shared>>;
        #[method_id(initByResolvingBookmarkData:options:relativeToURL:bookmarkDataIsStale:error:)]
        pub unsafe fn initByResolvingBookmarkData_options_relativeToURL_bookmarkDataIsStale_error(
            &self,
            bookmarkData: &NSData,
            options: NSURLBookmarkResolutionOptions,
            relativeURL: Option<&NSURL>,
            isStale: *mut bool,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;
        #[method_id(URLByResolvingBookmarkData:options:relativeToURL:bookmarkDataIsStale:error:)]
        pub unsafe fn URLByResolvingBookmarkData_options_relativeToURL_bookmarkDataIsStale_error(
            bookmarkData: &NSData,
            options: NSURLBookmarkResolutionOptions,
            relativeURL: Option<&NSURL>,
            isStale: *mut bool,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;
        #[method_id(resourceValuesForKeys:fromBookmarkData:)]
        pub unsafe fn resourceValuesForKeys_fromBookmarkData(
            keys: &NSArray<NSURLResourceKey>,
            bookmarkData: &NSData,
        ) -> Option<Id<NSDictionary<NSURLResourceKey, Object>, Shared>>;
        #[method(writeBookmarkData:toURL:options:error:)]
        pub unsafe fn writeBookmarkData_toURL_options_error(
            bookmarkData: &NSData,
            bookmarkFileURL: &NSURL,
            options: NSURLBookmarkFileCreationOptions,
        ) -> Result<(), Id<NSError, Shared>>;
        #[method_id(bookmarkDataWithContentsOfURL:error:)]
        pub unsafe fn bookmarkDataWithContentsOfURL_error(
            bookmarkFileURL: &NSURL,
        ) -> Result<Id<NSData, Shared>, Id<NSError, Shared>>;
        #[method_id(URLByResolvingAliasFileAtURL:options:error:)]
        pub unsafe fn URLByResolvingAliasFileAtURL_options_error(
            url: &NSURL,
            options: NSURLBookmarkResolutionOptions,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;
        #[method(startAccessingSecurityScopedResource)]
        pub unsafe fn startAccessingSecurityScopedResource(&self) -> bool;
        #[method(stopAccessingSecurityScopedResource)]
        pub unsafe fn stopAccessingSecurityScopedResource(&self);
    }
);
extern_methods!(
    #[doc = "NSPromisedItems"]
    unsafe impl NSURL {
        #[method(getPromisedItemResourceValue:forKey:error:)]
        pub unsafe fn getPromisedItemResourceValue_forKey_error(
            &self,
            value: &mut Option<Id<Object, Shared>>,
            key: &NSURLResourceKey,
        ) -> Result<(), Id<NSError, Shared>>;
        #[method_id(promisedItemResourceValuesForKeys:error:)]
        pub unsafe fn promisedItemResourceValuesForKeys_error(
            &self,
            keys: &NSArray<NSURLResourceKey>,
        ) -> Result<Id<NSDictionary<NSURLResourceKey, Object>, Shared>, Id<NSError, Shared>>;
        #[method(checkPromisedItemIsReachableAndReturnError:)]
        pub unsafe fn checkPromisedItemIsReachableAndReturnError(
            &self,
        ) -> Result<(), Id<NSError, Shared>>;
    }
);
extern_methods!(
    #[doc = "NSItemProvider"]
    unsafe impl NSURL {}
);
extern_class!(
    #[derive(Debug)]
    pub struct NSURLQueryItem;
    unsafe impl ClassType for NSURLQueryItem {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSURLQueryItem {
        #[method_id(initWithName:value:)]
        pub unsafe fn initWithName_value(
            &self,
            name: &NSString,
            value: Option<&NSString>,
        ) -> Id<Self, Shared>;
        #[method_id(queryItemWithName:value:)]
        pub unsafe fn queryItemWithName_value(
            name: &NSString,
            value: Option<&NSString>,
        ) -> Id<Self, Shared>;
        #[method_id(name)]
        pub unsafe fn name(&self) -> Id<NSString, Shared>;
        #[method_id(value)]
        pub unsafe fn value(&self) -> Option<Id<NSString, Shared>>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSURLComponents;
    unsafe impl ClassType for NSURLComponents {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSURLComponents {
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(initWithURL:resolvingAgainstBaseURL:)]
        pub unsafe fn initWithURL_resolvingAgainstBaseURL(
            &self,
            url: &NSURL,
            resolve: bool,
        ) -> Option<Id<Self, Shared>>;
        #[method_id(componentsWithURL:resolvingAgainstBaseURL:)]
        pub unsafe fn componentsWithURL_resolvingAgainstBaseURL(
            url: &NSURL,
            resolve: bool,
        ) -> Option<Id<Self, Shared>>;
        #[method_id(initWithString:)]
        pub unsafe fn initWithString(&self, URLString: &NSString) -> Option<Id<Self, Shared>>;
        #[method_id(componentsWithString:)]
        pub unsafe fn componentsWithString(URLString: &NSString) -> Option<Id<Self, Shared>>;
        #[method_id(URL)]
        pub unsafe fn URL(&self) -> Option<Id<NSURL, Shared>>;
        #[method_id(URLRelativeToURL:)]
        pub unsafe fn URLRelativeToURL(&self, baseURL: Option<&NSURL>)
            -> Option<Id<NSURL, Shared>>;
        #[method_id(string)]
        pub unsafe fn string(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(scheme)]
        pub unsafe fn scheme(&self) -> Option<Id<NSString, Shared>>;
        #[method(setScheme:)]
        pub unsafe fn setScheme(&self, scheme: Option<&NSString>);
        #[method_id(user)]
        pub unsafe fn user(&self) -> Option<Id<NSString, Shared>>;
        #[method(setUser:)]
        pub unsafe fn setUser(&self, user: Option<&NSString>);
        #[method_id(password)]
        pub unsafe fn password(&self) -> Option<Id<NSString, Shared>>;
        #[method(setPassword:)]
        pub unsafe fn setPassword(&self, password: Option<&NSString>);
        #[method_id(host)]
        pub unsafe fn host(&self) -> Option<Id<NSString, Shared>>;
        #[method(setHost:)]
        pub unsafe fn setHost(&self, host: Option<&NSString>);
        #[method_id(port)]
        pub unsafe fn port(&self) -> Option<Id<NSNumber, Shared>>;
        #[method(setPort:)]
        pub unsafe fn setPort(&self, port: Option<&NSNumber>);
        #[method_id(path)]
        pub unsafe fn path(&self) -> Option<Id<NSString, Shared>>;
        #[method(setPath:)]
        pub unsafe fn setPath(&self, path: Option<&NSString>);
        #[method_id(query)]
        pub unsafe fn query(&self) -> Option<Id<NSString, Shared>>;
        #[method(setQuery:)]
        pub unsafe fn setQuery(&self, query: Option<&NSString>);
        #[method_id(fragment)]
        pub unsafe fn fragment(&self) -> Option<Id<NSString, Shared>>;
        #[method(setFragment:)]
        pub unsafe fn setFragment(&self, fragment: Option<&NSString>);
        #[method_id(percentEncodedUser)]
        pub unsafe fn percentEncodedUser(&self) -> Option<Id<NSString, Shared>>;
        #[method(setPercentEncodedUser:)]
        pub unsafe fn setPercentEncodedUser(&self, percentEncodedUser: Option<&NSString>);
        #[method_id(percentEncodedPassword)]
        pub unsafe fn percentEncodedPassword(&self) -> Option<Id<NSString, Shared>>;
        #[method(setPercentEncodedPassword:)]
        pub unsafe fn setPercentEncodedPassword(&self, percentEncodedPassword: Option<&NSString>);
        #[method_id(percentEncodedHost)]
        pub unsafe fn percentEncodedHost(&self) -> Option<Id<NSString, Shared>>;
        #[method(setPercentEncodedHost:)]
        pub unsafe fn setPercentEncodedHost(&self, percentEncodedHost: Option<&NSString>);
        #[method_id(percentEncodedPath)]
        pub unsafe fn percentEncodedPath(&self) -> Option<Id<NSString, Shared>>;
        #[method(setPercentEncodedPath:)]
        pub unsafe fn setPercentEncodedPath(&self, percentEncodedPath: Option<&NSString>);
        #[method_id(percentEncodedQuery)]
        pub unsafe fn percentEncodedQuery(&self) -> Option<Id<NSString, Shared>>;
        #[method(setPercentEncodedQuery:)]
        pub unsafe fn setPercentEncodedQuery(&self, percentEncodedQuery: Option<&NSString>);
        #[method_id(percentEncodedFragment)]
        pub unsafe fn percentEncodedFragment(&self) -> Option<Id<NSString, Shared>>;
        #[method(setPercentEncodedFragment:)]
        pub unsafe fn setPercentEncodedFragment(&self, percentEncodedFragment: Option<&NSString>);
        #[method(rangeOfScheme)]
        pub unsafe fn rangeOfScheme(&self) -> NSRange;
        #[method(rangeOfUser)]
        pub unsafe fn rangeOfUser(&self) -> NSRange;
        #[method(rangeOfPassword)]
        pub unsafe fn rangeOfPassword(&self) -> NSRange;
        #[method(rangeOfHost)]
        pub unsafe fn rangeOfHost(&self) -> NSRange;
        #[method(rangeOfPort)]
        pub unsafe fn rangeOfPort(&self) -> NSRange;
        #[method(rangeOfPath)]
        pub unsafe fn rangeOfPath(&self) -> NSRange;
        #[method(rangeOfQuery)]
        pub unsafe fn rangeOfQuery(&self) -> NSRange;
        #[method(rangeOfFragment)]
        pub unsafe fn rangeOfFragment(&self) -> NSRange;
        #[method_id(queryItems)]
        pub unsafe fn queryItems(&self) -> Option<Id<NSArray<NSURLQueryItem>, Shared>>;
        #[method(setQueryItems:)]
        pub unsafe fn setQueryItems(&self, queryItems: Option<&NSArray<NSURLQueryItem>>);
        #[method_id(percentEncodedQueryItems)]
        pub unsafe fn percentEncodedQueryItems(
            &self,
        ) -> Option<Id<NSArray<NSURLQueryItem>, Shared>>;
        #[method(setPercentEncodedQueryItems:)]
        pub unsafe fn setPercentEncodedQueryItems(
            &self,
            percentEncodedQueryItems: Option<&NSArray<NSURLQueryItem>>,
        );
    }
);
extern_methods!(
    #[doc = "NSURLUtilities"]
    unsafe impl NSCharacterSet {
        #[method_id(URLUserAllowedCharacterSet)]
        pub unsafe fn URLUserAllowedCharacterSet() -> Id<NSCharacterSet, Shared>;
        #[method_id(URLPasswordAllowedCharacterSet)]
        pub unsafe fn URLPasswordAllowedCharacterSet() -> Id<NSCharacterSet, Shared>;
        #[method_id(URLHostAllowedCharacterSet)]
        pub unsafe fn URLHostAllowedCharacterSet() -> Id<NSCharacterSet, Shared>;
        #[method_id(URLPathAllowedCharacterSet)]
        pub unsafe fn URLPathAllowedCharacterSet() -> Id<NSCharacterSet, Shared>;
        #[method_id(URLQueryAllowedCharacterSet)]
        pub unsafe fn URLQueryAllowedCharacterSet() -> Id<NSCharacterSet, Shared>;
        #[method_id(URLFragmentAllowedCharacterSet)]
        pub unsafe fn URLFragmentAllowedCharacterSet() -> Id<NSCharacterSet, Shared>;
    }
);
extern_methods!(
    #[doc = "NSURLUtilities"]
    unsafe impl NSString {
        #[method_id(stringByAddingPercentEncodingWithAllowedCharacters:)]
        pub unsafe fn stringByAddingPercentEncodingWithAllowedCharacters(
            &self,
            allowedCharacters: &NSCharacterSet,
        ) -> Option<Id<NSString, Shared>>;
        #[method_id(stringByRemovingPercentEncoding)]
        pub unsafe fn stringByRemovingPercentEncoding(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(stringByAddingPercentEscapesUsingEncoding:)]
        pub unsafe fn stringByAddingPercentEscapesUsingEncoding(
            &self,
            enc: NSStringEncoding,
        ) -> Option<Id<NSString, Shared>>;
        #[method_id(stringByReplacingPercentEscapesUsingEncoding:)]
        pub unsafe fn stringByReplacingPercentEscapesUsingEncoding(
            &self,
            enc: NSStringEncoding,
        ) -> Option<Id<NSString, Shared>>;
    }
);
extern_methods!(
    #[doc = "NSURLPathUtilities"]
    unsafe impl NSURL {
        #[method_id(fileURLWithPathComponents:)]
        pub unsafe fn fileURLWithPathComponents(
            components: &NSArray<NSString>,
        ) -> Option<Id<NSURL, Shared>>;
        #[method_id(pathComponents)]
        pub unsafe fn pathComponents(&self) -> Option<Id<NSArray<NSString>, Shared>>;
        #[method_id(lastPathComponent)]
        pub unsafe fn lastPathComponent(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(pathExtension)]
        pub unsafe fn pathExtension(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(URLByAppendingPathComponent:)]
        pub unsafe fn URLByAppendingPathComponent(
            &self,
            pathComponent: &NSString,
        ) -> Option<Id<NSURL, Shared>>;
        #[method_id(URLByAppendingPathComponent:isDirectory:)]
        pub unsafe fn URLByAppendingPathComponent_isDirectory(
            &self,
            pathComponent: &NSString,
            isDirectory: bool,
        ) -> Option<Id<NSURL, Shared>>;
        #[method_id(URLByDeletingLastPathComponent)]
        pub unsafe fn URLByDeletingLastPathComponent(&self) -> Option<Id<NSURL, Shared>>;
        #[method_id(URLByAppendingPathExtension:)]
        pub unsafe fn URLByAppendingPathExtension(
            &self,
            pathExtension: &NSString,
        ) -> Option<Id<NSURL, Shared>>;
        #[method_id(URLByDeletingPathExtension)]
        pub unsafe fn URLByDeletingPathExtension(&self) -> Option<Id<NSURL, Shared>>;
        #[method_id(URLByStandardizingPath)]
        pub unsafe fn URLByStandardizingPath(&self) -> Option<Id<NSURL, Shared>>;
        #[method_id(URLByResolvingSymlinksInPath)]
        pub unsafe fn URLByResolvingSymlinksInPath(&self) -> Option<Id<NSURL, Shared>>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSFileSecurity;
    unsafe impl ClassType for NSFileSecurity {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSFileSecurity {
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>>;
    }
);
extern_methods!(
    #[doc = "NSURLClient"]
    unsafe impl NSObject {
        #[method(URL:resourceDataDidBecomeAvailable:)]
        pub unsafe fn URL_resourceDataDidBecomeAvailable(&self, sender: &NSURL, newBytes: &NSData);
        #[method(URLResourceDidFinishLoading:)]
        pub unsafe fn URLResourceDidFinishLoading(&self, sender: &NSURL);
        #[method(URLResourceDidCancelLoading:)]
        pub unsafe fn URLResourceDidCancelLoading(&self, sender: &NSURL);
        #[method(URL:resourceDidFailLoadingWithReason:)]
        pub unsafe fn URL_resourceDidFailLoadingWithReason(
            &self,
            sender: &NSURL,
            reason: &NSString,
        );
    }
);
extern_methods!(
    #[doc = "NSURLLoading"]
    unsafe impl NSURL {
        #[method_id(resourceDataUsingCache:)]
        pub unsafe fn resourceDataUsingCache(
            &self,
            shouldUseCache: bool,
        ) -> Option<Id<NSData, Shared>>;
        #[method(loadResourceDataNotifyingClient:usingCache:)]
        pub unsafe fn loadResourceDataNotifyingClient_usingCache(
            &self,
            client: &Object,
            shouldUseCache: bool,
        );
        #[method_id(propertyForKey:)]
        pub unsafe fn propertyForKey(&self, propertyKey: &NSString) -> Option<Id<Object, Shared>>;
        #[method(setResourceData:)]
        pub unsafe fn setResourceData(&self, data: &NSData) -> bool;
        #[method(setProperty:forKey:)]
        pub unsafe fn setProperty_forKey(&self, property: &Object, propertyKey: &NSString) -> bool;
        #[method_id(URLHandleUsingCache:)]
        pub unsafe fn URLHandleUsingCache(
            &self,
            shouldUseCache: bool,
        ) -> Option<Id<NSURLHandle, Shared>>;
    }
);
