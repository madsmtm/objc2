# Objective-C header translator

For use in making `icrate`.

```console
cargo run --bin header-translator -- /Applications/Xcode.app/Contents/Developer
```

## SDKs

We do not redistribute the relevant SDKs, to hopefully avoid a license violation. You can download the SDKs yourself (they're bundled in XCode) from [Apple's website](https://developer.apple.com/download/all/?q=xcode) (requires an Apple ID).

The following diffs are applied to silence warnings when compiling the headers using `clang 11.0.0`:

```diff
--- a/XYZ.sdk/System/Library/Frameworks/Foundation.framework/Versions/C/Headers/NSBundle.h
+++ b/XYZ.sdk/System/Library/Frameworks/Foundation.framework/Versions/C/Headers/NSBundle.h
@@ -88,7 +88,7 @@ NS_ASSUME_NONNULL_BEGIN

 /* Methods for retrieving localized strings. */
 - (NSString *)localizedStringForKey:(NSString *)key value:(nullable NSString *)value table:(nullable NSString *)tableName NS_FORMAT_ARGUMENT(1);
-- (NSAttributedString *)localizedAttributedStringForKey:(NSString *)key value:(nullable NSString *)value table:(nullable NSString *)tableName NS_FORMAT_ARGUMENT(1) NS_REFINED_FOR_SWIFT API_AVAILABLE(macos(12.0), ios(15.0), watchos(8.0), tvos(15.0));
+- (NSString *)localizedAttributedStringForKey:(NSString *)key value:(nullable NSString *)value table:(nullable NSString *)tableName NS_FORMAT_ARGUMENT(1) NS_REFINED_FOR_SWIFT API_AVAILABLE(macos(12.0), ios(15.0), watchos(8.0), tvos(15.0));

 /* Methods for obtaining various information about a bundle. */
 @property (nullable, readonly, copy) NSString *bundleIdentifier;
--- a/XYZ.sdk/System/Library/Frameworks/Foundation.framework/Versions/C/Headers/NSURLSession.h
+++ b/XYZ.sdk/System/Library/Frameworks/Foundation.framework/Versions/C/Headers/NSURLSession.h
@@ -497,7 +497,7 @@ API_AVAILABLE(macos(10.11), ios(9.0), watchos(2.0), tvos(9.0))
  * If an error occurs, any outstanding reads will also fail, and new
  * read requests will error out immediately.
  */
-- (void)readDataOfMinLength:(NSUInteger)minBytes maxLength:(NSUInteger)maxBytes timeout:(NSTimeInterval)timeout completionHandler:(void (^) (NSData * _Nullable_result data, BOOL atEOF, NSError * _Nullable error))completionHandler;
+- (void)readDataOfMinLength:(NSUInteger)minBytes maxLength:(NSUInteger)maxBytes timeout:(NSTimeInterval)timeout completionHandler:(void (^) (NSData * _Nullable data, BOOL atEOF, NSError * _Nullable error))completionHandler;

 /* Write the data completely to the underlying socket.  If all the
  * bytes have not been written by the timeout, a timeout error will
```
