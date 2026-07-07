#[cfg(target_pointer_width = "64")] // #if TARGET_RT_64_BIT
type Inner = core::ffi::c_int;
#[cfg(not(target_pointer_width = "64"))]
type Inner = i16; // SInt16

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfbundlerefnum?language=objc)
pub type CFBundleRefNum = Inner;

#[cfg(test)]
#[cfg(all(feature = "CFString", feature = "CFURL"))]
mod tests {
    use alloc::string::ToString;

    use crate::{CFBundle, CFRetained, CFString, CFURLPathStyle, CFURL};

    fn url_from_str(s: &str, is_dir: bool) -> CFRetained<CFURL> {
        CFURL::with_file_system_path(
            None,
            &CFString::from_str(s),
            CFURLPathStyle::CFURLPOSIXPathStyle,
            is_dir as _,
        )
        .unwrap()
    }

    #[test]
    fn safari_executable_url() {
        let path = url_from_str("/Applications/Safari.app", true);
        let bundle = CFBundle::new(None, &path).expect("Safari not present");
        let executable = bundle.executable_url().unwrap();
        assert_eq!(
            CFURL::file_system_path(
                &CFURL::absolute_url(&executable).unwrap(),
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
        let bundle = CFBundle::new(None, &path).expect("Safari not present");
        let frameworks = bundle.private_frameworks_url();
        assert_eq!(
            frameworks
                .absolute_url()
                .unwrap()
                .file_system_path(CFURLPathStyle::CFURLPOSIXPathStyle,)
                .unwrap()
                .to_string(),
            "/Applications/Safari.app/Contents/Frameworks"
        );
    }

    #[test]
    fn non_existent_bundle() {
        let path = url_from_str("/usr/local/non_existent", true);
        assert_eq!(CFBundle::new(None, &path), None);
    }

    #[test]
    fn main() {
        let bundle = CFBundle::main_bundle();

        assert_eq!(
            bundle.executable_url().unwrap().to_file_path().unwrap(),
            std::env::current_exe().unwrap(),
        );

        assert_eq!(
            bundle.bundle_url().to_file_path().unwrap(),
            std::env::current_exe().unwrap().parent().unwrap()
        );
    }
}
