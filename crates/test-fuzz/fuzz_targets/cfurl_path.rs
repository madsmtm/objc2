#![cfg_attr(not(feature = "afl"), no_main)]
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

use objc2_core_foundation::CFURL;

fn trim_trailing_zeros(mut data: &[u8]) -> &[u8] {
    while let Some(res) = data.strip_suffix(b"\0") {
        data = res;
    }
    data
}

fn run(bytes: &[u8]) {
    let path = Path::new(OsStr::from_bytes(bytes));
    let trimmed_bytes = trim_trailing_zeros(bytes);
    let trimmed_path = Path::new(OsStr::from_bytes(trimmed_bytes));
    // Fails on empty paths, or paths with internal NUL bytes.
    let expected_failure = bytes.is_empty() || trimmed_bytes.contains(&0);

    // Test creation from file path.
    if let Some(url) = CFURL::from_file_path(path) {
        assert!(!expected_failure);

        // TODO: Verify the path here somehow.
        if let Some(_created_path) = url.to_file_path() {
            if trimmed_path.is_absolute() {
                // assert_eq!(created_path, trimmed_path, "failed on {path:?}");
            } else {

                // assert_eq!(
                //     created_path,
                //     current_dir().unwrap().join(trimmed_path),
                //     "failed on {path:?}"
                // );
            }
        } else {
            // panic!("{path:?}");
        }
    } else {
        assert!(expected_failure);
    }

    // Test creation from directory path.
    if let Some(_url) = CFURL::from_directory_path(path) {
        assert!(!expected_failure);
    } else {
        assert!(expected_failure);
    }
}

#[cfg(not(feature = "afl"))]
libfuzzer_sys::fuzz_target!(|bytes: &[u8]| run(bytes));

#[cfg(feature = "afl")]
fn main() {
    afl::fuzz!(|bytes: &[u8]| {
        run(bytes);
    });
}
