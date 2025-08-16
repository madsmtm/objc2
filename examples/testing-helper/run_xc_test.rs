//! Helper tool for running a test built with XCTest.
//!
//! See `../testing/README.md` for usage.
use objc2::rc::{autoreleasepool, Retained};
use objc2::runtime::AnyClass;
use objc2::{msg_send, AnyThread};
use objc2_foundation::{
    ns_string, NSKeyedArchiver, NSNumber, NSObject, NSObjectNSKeyValueCoding, NSPropertyListFormat,
    NSString, NSURL, NSUUID,
};
use std::ffi::{CStr, CString};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    // TODO: Figure this path out in a better way.
    let target_dir = Path::new("./target/debug");

    let mut args = std::env::args().skip(1);
    let example = args
        .next()
        .expect("must specify app under test as first argument");
    let test = args.next().expect("must specify test as first argument");
    assert_eq!(args.count(), 0, "only pass two arguments");

    // xcode-select --print-path
    let developer_dir = xcode_select_developer_dir().expect("failed finding Xcode developer dir");

    // cargo bundle --package examples-app --example default_xcode_app
    println!(
        "Building and bundling application binary... ({})",
        target_dir
            .join("examples")
            .join(&example)
            .with_extension("app")
            .display()
    );
    let app_bundle = build_and_bundle(
        &target_dir.join("examples"),
        "--example",
        &example,
        "app",
        false,
    );

    // cargo bundle --package objc2-xc-test --example foo
    // /usr/libexec/PlistBuddy -c "Add :XCTContainsUITests bool true" target/debug/examples/bundle/osx/objc2-xc-test.app/Contents/Info.plist
    println!(
        "Building and bundling test binary... ({})",
        target_dir.join(&test).with_extension("xctest").display()
    );
    let test_bundle = build_and_bundle(target_dir, "--bin", &test, "xctest", true);

    let test_configuration = target_dir.join(&test).with_extension("xctestconfiguration");
    println!(
        "Writing test configuration... ({})",
        test_configuration.display()
    );
    write_xctestconfiguration(
        &developer_dir,
        &app_bundle,
        Some(&format!("com.example.{example}")),
        &test_bundle,
        true,
        &test_configuration,
    );

    // XCTestConfigurationFilePath=xyz.xctestconfiguration /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/Library/Xcode/Agents/xctest
    println!("Running test...");
    run_test(&developer_dir, &test_configuration);
}

fn xcode_select_developer_dir() -> Option<String> {
    let output = Command::new("xcode-select")
        .arg("--print-path")
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let mut stdout = output.stdout;
    if let Some(b'\n') = stdout.last() {
        let _ = stdout.pop().unwrap();
    }
    Some(String::from_utf8(stdout).unwrap())
}

fn build_and_bundle(
    target_dir: &Path,
    flag: &str,
    example: &str,
    extension: &str,
    ui: bool,
) -> PathBuf {
    let app_dir = target_dir.join(example).with_extension(extension);
    std::fs::create_dir_all(app_dir.join("Contents/MacOS")).unwrap();

    let status = Command::new("cargo")
        .args(["build", flag, example])
        .status()
        .unwrap();
    assert!(status.success());
    std::fs::copy(
        target_dir.join(example),
        app_dir.join("Contents/MacOS").join(example),
    )
    .unwrap();

    let app_type = match extension {
        "app" => "APPL",
        "xctest" => "BNDL",
        _ => unreachable!("{extension:?}"),
    };
    write_plist(app_dir.join("Contents/Info.plist"), app_type, example, ui);

    app_dir
}

fn write_xctestconfiguration(
    developer_dir: &str,
    app_path: &Path,
    app_bundle_id: Option<&str>,
    test_bundle: &Path,
    ui: bool,
    out: &Path,
) {
    // Dynamically load XCTest.
    //
    // We do this because it lives in a special path under Xcode, and if we
    // wanted to link to it normally, the user would have to specify something
    // like:
    // DYLD_FRAMEWORK_PATH=/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/Library/Frameworks
    // RUSTFLAGS=-Clink-args=-F/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/Library/Frameworks
    //
    // And that's kinda cumbersome.
    let xctest_path = CString::new(format!("{developer_dir}/Platforms/MacOSX.platform/Developer/Library/Frameworks/XCTest.framework/XCTest")).unwrap();
    let handle = unsafe { libc::dlopen(xctest_path.as_ptr(), libc::RTLD_LAZY) };
    if handle.is_null() {
        let err = unsafe { CStr::from_ptr(libc::dlerror()) };
        panic!("could not open XCTest.framework: {err:?}");
    }

    autoreleasepool(|_| {
        // Load the private XCTestConfiguration class from XCTestCore, and
        // configure an instance of it.
        //
        // This is somewhat of a hack, but there is no public API for creating
        // test configurations.
        let cls = AnyClass::get(c"XCTestConfiguration")
            .or_else(|| AnyClass::get(c"_XCTestConfiguration"))
            .expect("failed finding XCTestConfiguration class");
        let cfg: Retained<NSObject> = unsafe { msg_send![cls, new] };

        unsafe {
            // Set target application.
            let app_path = app_path.canonicalize().unwrap();
            cfg.setValue_forKey(
                Some(&NSString::from_str(app_path.as_os_str().to_str().unwrap())),
                ns_string!("targetApplicationPath"),
            );
            if let Some(bid) = &app_bundle_id {
                let bid_ns = NSString::from_str(bid);
                cfg.setValue_forKey(Some(&bid_ns), ns_string!("targetApplicationBundleID"));
            } else if ui {
                panic!("must have application bundle ID when testing UI applications");
            }

            // Set target test.
            let test_bundle = test_bundle.canonicalize().unwrap();
            cfg.setValue_forKey(
                Some(&NSURL::from_file_path(&test_bundle).unwrap()),
                ns_string!("testBundleURL"),
            );
            cfg.setValue_forKey(
                Some(&NSString::from_str(
                    test_bundle.file_stem().unwrap().to_str().unwrap(),
                )),
                ns_string!("productModuleName"),
            );

            // Set a random session identifier.
            cfg.setValue_forKey(Some(&NSUUID::new()), ns_string!("sessionIdentifier"));

            cfg.setValue_forKey(
                Some(&NSNumber::new_bool(ui)),
                ns_string!("initializeForUITesting"),
            );
            cfg.setValue_forKey(
                Some(&NSNumber::new_bool(ui)),
                ns_string!("testsMustRunOnMainThread"),
            );

            // TODO: Do we need these?
            cfg.setValue_forKey(
                Some(&NSNumber::new_bool(false)),
                ns_string!("treatMissingBaselinesAsFailures"),
            );
            cfg.setValue_forKey(
                Some(&NSNumber::new_bool(false)),
                ns_string!("reportResultsToIDE"),
            );
            cfg.setValue_forKey(
                Some(&NSNumber::new_bool(false)),
                ns_string!("reportActivities"),
            );
            cfg.setValue_forKey(
                Some(&NSNumber::new_bool(false)),
                ns_string!("disablePerformanceMetrics"),
            );
        }

        // Serialize the `XCTestConfiguration` instance.
        unsafe {
            let archiver = NSKeyedArchiver::alloc();
            let archiver = NSKeyedArchiver::initRequiringSecureCoding(archiver, true);
            archiver.setOutputFormat(NSPropertyListFormat::XMLFormat_v1_0);
            archiver.encodeObject_forKey(Some(&cfg), ns_string!("root"));
            archiver.finishEncoding();
            let res = archiver.encodedData().writeToFile_atomically(
                &NSString::from_str(out.as_os_str().to_str().unwrap()),
                true,
            );
            assert!(res, "failed writing {out:?}");
        }
    });

    let _ = unsafe { libc::dlclose(handle) };
}

fn run_test(developer_dir: &str, test_configuration: &Path) {
    let xctest = PathBuf::from(developer_dir)
        .join("Platforms/MacOSX.platform/Developer/Library/Xcode/Agents/xctest");
    let status = Command::new(xctest)
        .env("XCTestConfigurationFilePath", test_configuration)
        .status()
        .unwrap();
    assert!(status.success());
}

fn write_plist(path: impl AsRef<Path>, ty: &str, name: &str, contains_ui_tests: bool) {
    let contents = format!(
        r#"
            <?xml version="1.0" encoding="UTF-8"?>
            <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
            <plist version="1.0">
            <dict>
                <key>CFBundleDevelopmentRegion</key>
                <string>en</string>
                <key>CFBundleExecutable</key>
                <string>{name}</string>
                <key>CFBundleIdentifier</key>
                <string>com.example.{name}</string>
                <key>CFBundleInfoDictionaryVersion</key>
                <string>6.0</string>
                <key>CFBundleName</key>
                <string>{name}</string>
                <key>CFBundlePackageType</key>
                <string>{ty}</string>
                <key>CFBundleShortVersionString</key>
                <string>1.0</string>
                <key>CFBundleSupportedPlatforms</key>
                <array>
                    <string>MacOSX</string>
                </array>
                <key>CFBundleVersion</key>
                <string>1</string>
                <key>XCTContainsUITests</key>
                <{contains_ui_tests}/>
            </dict>
            </plist>
        "#
    );
    fs::write(path, &contents).expect("failed to write PList");
}
