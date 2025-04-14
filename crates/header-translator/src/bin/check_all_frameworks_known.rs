//! Check that all of Apple's frameworks are known to us, and are either
//! mapped or documented as skipped.
//!
//! Run with:
//! ```
//! cargo run --bin check_all_frameworks_known
//! ```
use std::fs;
use std::os::unix::ffi::OsStrExt;
use std::{collections::BTreeSet, process::ExitCode};

use apple_sdk::{AppleSdk, DeveloperDirectory, Platform, SimpleSdk};
use header_translator::{load_config, load_skipped};

fn all_frameworks() -> BTreeSet<String> {
    let developer_dir = DeveloperDirectory::from_env()
        .unwrap()
        .or_else(|| DeveloperDirectory::from_xcode_select_paths().unwrap())
        .or_else(DeveloperDirectory::default_xcode)
        .expect("could not find developer directory. Pass DEVELOPER_DIR=...");

    let mut res = BTreeSet::new();

    for sdk in developer_dir.sdks::<SimpleSdk>().unwrap() {
        if *sdk.platform() == Platform::DriverKit {
            // Ignore DriverKit for now
            continue;
        }

        for dir in fs::read_dir(sdk.path().join("System/Library/Frameworks")).unwrap() {
            let path = dir.unwrap().path();

            if path.file_name().unwrap().as_bytes().starts_with(b"_") {
                // Ignore private/underscored frameworks
                continue;
            }

            if path.extension().unwrap() != "framework" {
                continue;
            }

            res.insert(path.file_stem().unwrap().to_string_lossy().into_owned());
        }
    }

    res
}

fn known_frameworks() -> BTreeSet<String> {
    load_config()
        .unwrap()
        .to_parse()
        .filter(|(_, config)| !config.is_library)
        .map(|(name, _)| name.to_string())
        .chain(load_skipped().unwrap().into_keys())
        .collect()
}

fn main() -> ExitCode {
    let mut exit_code = ExitCode::SUCCESS;

    for framework in all_frameworks().difference(&known_frameworks()) {
        eprintln!(
            "framework {framework} was in SDKs, but wasn't present in header-translator's configs"
        );
        exit_code = ExitCode::FAILURE;
    }

    for framework in known_frameworks().difference(&all_frameworks()) {
        eprintln!("framework {framework} was in header-translator config, but not in any SDK");
        exit_code = ExitCode::FAILURE;
    }

    exit_code
}
