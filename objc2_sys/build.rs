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
/// Anyhow, it's not ultra important, but enables some optimizations if this
/// is specified. In the future, Rust will hopefully get something similar to
/// clang's `-mmacosx-version-min`, and then we won't need this for the
/// Apple runtimes.
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

impl AppleRuntime {
    fn get_default(target_os: &str) -> Option<Self> {
        match target_os {
            "macos" => Some(Self::MacOS(None)),
            "ios" => Some(Self::IOS(None)),
            "watchos" => Some(Self::WatchOS(None)),
            "tvos" => Some(Self::TvOS(None)),
            _ => None,
        }
    }
}

enum Runtime {
    Apple(AppleRuntime),
    GNUStep(Version),
    WinObjc(Version),
    ObjFW(Version),
}
use Runtime::*;

fn main() {
    // The script doesn't depend on our code
    println!("cargo:rerun-if-changed=build.rs");

    println!("cargo:rerun-if-env-changed=OBJC_RUNTIME");

    // Used to figure out when BOOL should be i8 vs. bool
    if env::var("TARGET").unwrap().ends_with("macabi") {
        println!("cargo:rustc-cfg=target_abi_macabi");
    }

    // TODO: Figure out when to enable this
    // println!("cargo:rustc-cfg=libobjc2_strict_apple_compat");

    let target_vendor = env::var("CARGO_CFG_TARGET_VENDOR").unwrap();
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    // OBJC_RUNTIME syntax: `RUNTIME ("-" VERSION)?`
    let runtime = if let Ok(runtime) = env::var("OBJC_RUNTIME") {
        let (runtime, version) = if let Some((runtime, version)) = runtime.split_once('-') {
            (runtime, Some(version.into()))
        } else {
            (&*runtime, None)
        };

        match runtime {
            "apple" => {
                if version.is_some() {
                    panic!("Invalid OBJC_RUNTIME: Version doesn't make sense for the `apple` runtime; use `macos`, `ios`, `tvos` or `watchos`");
                }
                Apple(AppleRuntime::get_default(&target_os).expect("Invalid OBJC_RUNTIME: Target OS invalid for the `apple` runtime; specify manually with `macos`, `ios`, `tvos` or `watchos`"))
            }
            "gnustep" => GNUStep(version),
            "winobjc" => WinObjc(version),
            "objfw" => ObjFW(version),
            // Support clang "syntax" (`macosx`)
            "macos" | "macosx" => Apple(MacOS(version)),
            "ios" => Apple(IOS(version)),
            "tvos" => Apple(TvOS(version)),
            "watchos" => Apple(WatchOS(version)),
            _ => {
                panic!("Invalid OBJC_RUNTIME: {}", runtime)
            }
        }
    } else {
        if target_vendor == "apple" {
            Apple(AppleRuntime::get_default(&target_os).unwrap())
        } else if target_os == "windows" {
            WinObjc(None)
        } else {
            GNUStep(None)
        }
    };

    // Add `#[cfg(RUNTIME)]` directive
    let runtime_cfg = match runtime {
        Apple(_) => "apple",
        GNUStep(_) => "gnustep",
        WinObjc(_) => "winobjc",
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
        GNUStep(version) => {
            // GNUStep default in clang is 1.6; we require at least 1.7
            let version = version.as_deref().unwrap_or("1.7");
            format!("gnustep-{}", version)
        }
        WinObjc(version) => {
            // WinObjc use a small fork of GNUStep version 1.8; so lower
            // versions doesn't make sense.
            let version = version.as_deref().unwrap_or("1.8");
            format!("gnustep-{}", version)
        }
        ObjFW(version) => {
            // Default in clang
            let _version = version.as_deref().unwrap_or("0.8");
            todo!()
        }
    };

    // Add clang arguments
    println!(
        "cargo:clang_args=-fobjc-arc -fobjc-arc-exceptions -fobjc-exceptions -fobjc-weak -fobjc-runtime={}",
        // -fobjc-arc implies -fobjc-link-runtime, so people actually don't
        // even need to specify `-lobjc` (though they probably still should).
        clang_runtime
    ); // DEP_OBJC_CLANG_ARGS

    // Add GCC arguments. Not really supported
    match runtime {
        Apple(_) => {
            println!("cargo:gcc_args=-fnext-runtime -fobjc-exceptions -fobjc-abi-version=2")
        }
        _ => println!("cargo:gcc_args=-fgnu-runtime -fobjc-exceptions"),
    } // DEP_OBJC_GCC_ARGS

    // Link to libobjc
    println!("cargo:rustc-link-lib=dylib=objc");
}
