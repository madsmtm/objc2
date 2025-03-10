#[cfg(target_pointer_width = "64")] // #if TARGET_RT_64_BIT
type Inner = core::ffi::c_int;
#[cfg(not(target_pointer_width = "64"))]
type Inner = i16; // SInt16

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfbundlerefnum?language=objc)
pub type CFBundleRefNum = Inner;

#[cfg(test)]
#[cfg(all(feature = "CFBase", feature = "CFString", feature = "CFURL"))]
mod tests {
    use alloc::string::ToString;

    use crate::{
        CFBundleCopyExecutableURL, CFBundleCopyPrivateFrameworksURL, CFBundleCreate, CFRetained,
        CFString, CFURLCopyAbsoluteURL, CFURLCopyFileSystemPath, CFURLCreateWithFileSystemPath,
        CFURLPathStyle, CFURL,
    };

    fn url_from_str(s: &str, is_dir: bool) -> CFRetained<CFURL> {
        CFURLCreateWithFileSystemPath(
            None,
            Some(&CFString::from_str(s)),
            CFURLPathStyle::CFURLPOSIXPathStyle,
            is_dir as _,
        )
        .unwrap()
    }

    #[test]
    fn safari_executable_url() {
        let path = url_from_str("/Applications/Safari.app", true);
        let bundle = CFBundleCreate(None, Some(&path)).expect("Safari not present");
        let executable = CFBundleCopyExecutableURL(&bundle).unwrap();
        assert_eq!(
            CFURLCopyFileSystemPath(
                &CFURLCopyAbsoluteURL(&executable).unwrap(),
                CFURLPathStyle::CFURLPOSIXPathStyle,
            )
            .unwrap()
            .to_string(),
            "/Applications/Safari.app/Contents/MacOS/Safari"
        );
    }

    #[test]
    fn safari_private_frameworks_url() {
        let path = url_from_str("/Applications/Safari.app", true);
        let bundle = CFBundleCreate(None, Some(&path)).expect("Safari not present");
        let frameworks = CFBundleCopyPrivateFrameworksURL(&bundle).unwrap();
        assert_eq!(
            CFURLCopyFileSystemPath(
                &CFURLCopyAbsoluteURL(&frameworks).unwrap(),
                CFURLPathStyle::CFURLPOSIXPathStyle,
            )
            .unwrap()
            .to_string(),
            "/Applications/Safari.app/Contents/Frameworks"
        );
    }

    #[test]
    fn non_existent_bundle() {
        let path = url_from_str("/usr/local/non_existent", true);
        assert_eq!(CFBundleCreate(None, Some(&path)), None);
    }
}
