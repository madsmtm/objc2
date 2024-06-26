use std::{env, path::Path};

/// The selected runtime (and runtime version).
enum Runtime {
    Apple,
    GNUStep(u8, u8),
    WinObjC,
    #[allow(dead_code)]
    ObjFW(Option<String>),
}

fn main() {
    // The script doesn't depend on our code
    println!("cargo:rerun-if-changed=build.rs");

    let target = env::var("TARGET").unwrap();
    let target_vendor = env::var("CARGO_CFG_TARGET_VENDOR").unwrap();
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    // Used to figure out when BOOL should be i8 vs. bool
    // Matches:
    // aarch64-apple-ios-macabi
    // x86_64-apple-ios-macabi
    println!("cargo:rustc-check-cfg=cfg(target_abi_macabi)");
    if std::env::var("CARGO_CFG_TARGET_ABI").as_deref() == Ok("macabi")
        // Legacy check for Rust versions < 1.78 (still within MSRV)
        || target.ends_with("macabi")
    {
        println!("cargo:rustc-cfg=target_abi_macabi");
    }

    // Used to set correct image info in `objc2`
    // Matches:
    // aarch64-apple-ios-sim
    // aarch64-apple-watchos-sim
    // x86_64-apple-watchos-sim
    // i386-apple-ios
    // x86_64-apple-ios
    println!("cargo:rustc-check-cfg=cfg(target_simulator)");
    if std::env::var("CARGO_CFG_TARGET_ABI").as_deref() == Ok("sim")
        // Legacy check for Rust versions < 1.78 (still within MSRV)
        || target.ends_with("sim")
        || target == "i386-apple-ios"
        || target == "x86_64-apple-ios"
    {
        println!("cargo:rustc-cfg=target_simulator");
    }

    let runtime = match (
        env::var_os("CARGO_FEATURE_GNUSTEP_1_7").is_some(),
        env::var_os("CARGO_FEATURE_UNSTABLE_OBJFW").is_some(),
    ) {
        (true, true) => panic!("Invalid feature combination; only one runtime may be selected!"),
        (true, false) => {
            if env::var_os("CARGO_FEATURE_UNSTABLE_WINOBJC").is_some() {
                Runtime::WinObjC
            } else if env::var_os("CARGO_FEATURE_GNUSTEP_2_1").is_some() {
                Runtime::GNUStep(2, 1)
            } else if env::var_os("CARGO_FEATURE_GNUSTEP_2_0").is_some() {
                Runtime::GNUStep(2, 0)
            } else if env::var_os("CARGO_FEATURE_GNUSTEP_1_9").is_some() {
                Runtime::GNUStep(1, 9)
            } else if env::var_os("CARGO_FEATURE_GNUSTEP_1_8").is_some() {
                Runtime::GNUStep(1, 8)
            } else {
                // CARGO_FEATURE_GNUSTEP_1_7
                Runtime::GNUStep(1, 7)
            }
        }
        (false, true) => {
            // For now
            unimplemented!("ObjFW is not yet supported")
            // ObjFW(None)
        }
        (false, false) if target_vendor == "apple" => Runtime::Apple,
        // Choose defaults when generating docs
        (false, false) if std::env::var("DOCS_RS").is_ok() => {
            if target_os == "windows" {
                Runtime::WinObjC
            } else {
                Runtime::GNUStep(1, 7)
            }
        }
        (false, false) => {
            panic!("Must specify the desired runtime using Cargo features on non-Apple platforms")
        }
    };

    let clang_objc_runtime = match &runtime {
        // Default to `clang`'s own heuristics.
        //
        // Note that the `cc` crate forwards the correct deployment target to clang as well.
        Runtime::Apple => "".into(),
        // Default in clang is 1.6
        // GNUStep's own default is 1.8
        Runtime::GNUStep(major, minor) => format!(" -fobjc-runtime=gnustep-{major}.{minor}"),
        // WinObjC's libobjc2 is a fork of gnustep's from version 1.8
        Runtime::WinObjC => " -fobjc-runtime=gnustep-1.8".into(),
        Runtime::ObjFW(version) => {
            // Default in clang
            let version = version.as_deref().unwrap_or("0.8");
            format!(" -fobjc-runtime=objfw-{version}")
        }
    };

    // let gcc_args = match &runtime {
    //     Apple(_) => "-fnext-runtime -fobjc-abi-version=2",
    //     _ => "-fgnu-runtime",
    // };

    // Add CC arguments
    // Assume the compiler is clang; if it isn't, this is probably going to
    // fail anyways, since we're using newer runtimes than GCC supports.
    //
    // TODO: -fobjc-weak ?
    let mut cc_args =
        format!("-fobjc-exceptions{clang_objc_runtime} -fobjc-arc -fobjc-arc-exceptions");

    if let Runtime::ObjFW(_) = &runtime {
        // Add compability headers to make `#include <objc/objc.h>` work.
        let compat_headers = Path::new(env!("CARGO_MANIFEST_DIR")).join("compat-headers-objfw");
        cc_args.push_str(" -I");
        cc_args.push_str(compat_headers.to_str().unwrap());
    }

    println!("cargo:cc_args={cc_args}"); // DEP_OBJC_[version]_CC_ARGS
}
