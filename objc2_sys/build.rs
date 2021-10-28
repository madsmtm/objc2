use std::env;

/// TODO: Better validation of this
///
/// The version is used for providing different behaviour when:
/// - CGException.cpp getObjCPersonality (GNUStep >= 1.7)
/// - Clang.cpp Clang::AddObjCRuntimeArgs (GNUStep >= 2.0)
/// - isLegacyDispatchDefaultForArch (macOS < 10.6, GNUStep < 1.6)
/// - hasNativeARC (macOS < 10.7, iOS < 5)
/// - shouldUseARCFunctionsForRetainRelease (macOS < 10.10, iOS < 8)
/// - shouldUseRuntimeFunctionsForAlloc (macOS < 10.10, iOS < 8)
/// - shouldUseRuntimeFunctionForCombinedAllocInit (macOS >= 10.14.4, iOS >= 12.2, watchOS >= 5.2)
/// - hasOptimizedSetter (macOS >= 10.8, iOS >= 6, GNUStep >= 1.7)
/// - hasSubscripting (macOS < 10.11, iOS < 9)
/// - hasTerminate (macOS < 10.8, iOS < 5)
/// - hasARCUnsafeClaimAutoreleasedReturnValue (macOS >= 10.11, iOS >= 9, watchOS >= 2)
/// - hasEmptyCollections (macOS >= 10.11, iOS >= 9, watchOS >= 2)
/// - ... (incomplete)
///
/// `macosx-fragile` and `gcc` was not considered in this analysis, made on
/// clang version 13's source code:
/// https://github.com/llvm/llvm-project/blob/llvmorg-13.0.0/clang/include/clang/Basic/ObjCRuntime.h
///
/// In short, it's not ultra important, but enables some optimizations if this
/// is specified.
type Version = Option<String>;

// For clang "-fobjc-runtime" support
enum AppleRuntime {
    MacOS(Version),
    IOS(Version),
    TvOS(Version),
    WatchOS(Version),
    // BridgeOS,
}
use AppleRuntime::*;

enum Runtime {
    Apple(AppleRuntime),
    GNUStep(u8, u8),
    WinObjc,
    #[allow(dead_code)]
    ObjFW(Option<String>),
}
use Runtime::*;

fn get_env(env: &str) -> Option<String> {
    println!("cargo:rerun-if-env-changed={}", env);
    match env::var(env) {
        Ok(var) => Some(var),
        Err(env::VarError::NotPresent) => None,
        Err(env::VarError::NotUnicode(var)) => panic!("Invalid unicode for {}: {:?}", env, var),
    }
}

fn main() {
    // The script doesn't depend on our code
    println!("cargo:rerun-if-changed=build.rs");

    // Used to figure out when BOOL should be i8 vs. bool
    if env::var("TARGET").unwrap().ends_with("macabi") {
        println!("cargo:rustc-cfg=target_abi_macabi");
    }

    // TODO: Figure out when to enable this
    // println!("cargo:rustc-cfg=libobjc2_strict_apple_compat");

    let runtime = match get_env("RUNTIME_VERSION").as_deref() {
        // Force using GNUStep on all platforms
        Some("gnustep-1.7") => GNUStep(1, 7),
        Some("gnustep-1.8") => GNUStep(1, 8),
        Some("gnustep-1.9") => GNUStep(1, 9),
        Some("gnustep-2.0") => GNUStep(2, 0),
        Some("gnustep-2.1") => GNUStep(2, 1),

        // Pick the relevant runtime to use, and find target versions
        None => match &*env::var("CARGO_CFG_TARGET_OS").unwrap() {
            "macos" => Apple(MacOS(Some(
                get_env("MACOSX_DEPLOYMENT_TARGET").unwrap_or_else(|| "10.7".into()),
            ))),
            "ios" => Apple(IOS(Some(
                get_env("IPHONEOS_DEPLOYMENT_TARGET").unwrap_or_else(|| "7.0".into()),
            ))),
            "tvos" => Apple(TvOS(get_env("TVOS_DEPLOYMENT_TARGET"))),
            "watchos" => Apple(WatchOS(get_env("WATCHOS_DEPLOYMENT_TARGET"))),
            "windows" => WinObjc,
            _ => GNUStep(1, 8), // GNUStep's own default
        },

        Some(runtime) => panic!("Invalid RUNTIME_VERSION: {}", runtime),
    };

    // Add `#[cfg(RUNTIME)]` directive
    let runtime_cfg = match runtime {
        Apple(_) => "apple",
        GNUStep(_, _) => "gnustep",
        WinObjc => "winobjc",
        ObjFW(_) => "objfw",
    };
    println!("cargo:rustc-cfg={}", runtime_cfg);
    // Allow downstream build scripts to do the same
    println!("cargo:runtime={}", runtime_cfg); // DEP_OBJC_RUNTIME

    let clang_runtime = match &runtime {
        Apple(runtime) => {
            let clang_runtime_str = match runtime {
                MacOS(_) => "macosx",
                IOS(_) => "ios",
                WatchOS(_) => "watchos",
                TvOS(_) => "ios", // ??
            };
            match runtime {
                MacOS(version) | IOS(version) | WatchOS(version) | TvOS(version) => {
                    if let Some(version) = version {
                        format!("{}-{}", clang_runtime_str, version)
                    } else {
                        clang_runtime_str.into()
                    }
                }
            }
        }
        // Default in clang is 1.6
        GNUStep(major, minor) => format!("gnustep-{}.{}", major, minor),
        // WinObjC's libobjc2 is just a fork of gnustep's from version 1.8
        WinObjc => "gnustep-1.8".into(),
        ObjFW(version) => {
            // Default in clang
            let _version = version.as_deref().unwrap_or("0.8");
            todo!()
        }
    };

    // let gcc_args = match &runtime {
    //     Apple(_) => "-fnext-runtime -fobjc-abi-version=2",
    //     _ => "-fgnu-runtime",
    // };

    // Add CC arguments
    // Assume the compiler is clang; if it isn't, this is probably going to
    // fail anyways, since we're using newer runtimes than GCC supports.
    println!(
        "cargo:cc_args=-fobjc-arc -fobjc-arc-exceptions -fobjc-exceptions -fobjc-runtime={}",
        // TODO: -fobjc-weak ?
        clang_runtime
    ); // DEP_OBJC_CC_ARGS

    // Link to libobjc
    println!("cargo:rustc-link-lib=dylib=objc");
}
