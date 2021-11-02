use std::env;
use std::path::Path;

fn main() {
    // Only rerun if this file changes; the script doesn't depend on our code
    println!("cargo:rerun-if-changed=build.rs");

    let mut apple = env::var_os("CARGO_FEATURE_APPLE").is_some();
    let mut compiler_rt = env::var_os("CARGO_FEATURE_COMPILER_RT").is_some();
    let gnustep = env::var_os("CARGO_FEATURE_GNUSTEP_1_7").is_some();
    let objfw = env::var_os("CARGO_FEATURE_OBJFW").is_some();

    if let (false, false, false, false) = (apple, compiler_rt, gnustep, objfw) {
        let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
        if let "macos" | "ios" | "tvos" | "watchos" = &*target_os {
            apple = true;
            // Add cheaty #[cfg(feature = "apple")] directive
            println!("cargo:rustc-cfg=feature=\"apple\"");
        } else {
            compiler_rt = true;
            // Add cheaty #[cfg(feature = "compiler-rt")] directive
            println!("cargo:rustc-cfg=feature=\"compiler-rt\"");
        }
    }

    let mut cc_args = "-fblocks".to_owned();

    match (apple, compiler_rt, gnustep, objfw) {
        (true, false, false, false) => {
            // Link to libclosure (internally called libsystem_blocks), which is
            // exported by libSystem.dylib.
            //
            // Note that System.framework is just a deprecated wrapper over the
            // dynamic library.
            println!("cargo:rustc-link-lib=dylib=System");
            // Alternative: Only link to libsystem_blocks.dylib
            // println!("cargo:rustc-link-search=native=/usr/lib/system");
            // println!("cargo:rustc-link-lib=dylib=system_blocks");
        }
        (false, true, false, false) => {
            println!("cargo:rustc-link-lib=dylib=BlocksRuntime");
        }
        (false, false, true, false) => {
            // Don't link to anything; objc-sys already does that for us!

            // Add GNUStep compability headers to make `#include <Block.h>`
            // work (on newer GNUStep versions these headers are present)
            if env::var_os("CARGO_FEATURE_GNUSTEP_2_0").is_none() {
                let compat_headers =
                    Path::new(env!("CARGO_MANIFEST_DIR")).join("gnustep-compat-headers");
                cc_args.push_str(" -I");
                cc_args.push_str(compat_headers.to_str().unwrap());
            }
        }
        (false, false, false, true) => unimplemented!(),
        // Checked in if-let above
        (false, false, false, false) => unreachable!(),
        (_, _, _, _) => panic!("Invalid feature combination; only one runtime may be selected!"),
    }

    // Add DEP_BLOCK_CC_ARGS
    println!("cargo:cc_args={}", cc_args);
}
