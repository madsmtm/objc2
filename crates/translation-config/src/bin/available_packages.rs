//! Query the available public crates on the given target.
//!
//! Use as:
//! ```sh
//! # Test on host macOS.
//! cargo test $(cargo run --bin=available_packages -- $(rustc --print host-tuple) $(sw_vers --productVersion))
//!
//! # Test 26.3 simulators.
//! cargo test $(cargo run --bin=available_packages -- aarch64-apple-ios-sim 26.3) --target aarch64-apple-ios-sim
//! cargo test $(cargo run --bin=available_packages -- aarch64-apple-tvos-sim 26.3) --target aarch64-apple-tvos-sim
//! cargo test $(cargo run --bin=available_packages -- aarch64-apple-watchos-sim 26.3) --target aarch64-apple-watchos-sim
//! cargo test $(cargo run --bin=available_packages -- aarch64-apple-visionos-sim 26.3) --target aarch64-apple-visionos-sim
//! ```

use semver::{Version, VersionReq};
use translation_config::{Config, LibraryConfig};

fn main() {
    let mut args = std::env::args();
    let _ = args.next();

    let Some(target) = args.next() else {
        panic!("must provide target as first argument")
    };

    let req = args
        .next()
        .map(|arg| {
            let version = lenient_semver_parser::parse::<Version>(&arg)
                .expect("second argument must be a valid version");
            VersionReq::parse(&format!("<={version}")).unwrap()
        })
        .unwrap_or_default();

    // Include public packages.
    print!("--package=objc2-encode");
    print!(" --package=objc2-proc-macros");
    print!(" --package=objc2-exception-helper");

    let config = Config::load().expect("failed loading config");
    for (_, lib) in config.to_parse() {
        if library_available(lib, &target, &req) {
            print!(" --package {}", lib.krate);
        }
    }

    println!();
}

fn library_available(lib: &LibraryConfig, target: &str, req: &VersionReq) -> bool {
    if lib.located_outside_sdk {
        // Cannot easily link to these.
        return false;
    }

    if target.contains("linux-gnu") {
        // HACK: CoreFoundation uses mach types that GNUStep doesn't support
        if lib.framework == "CoreFoundation" {
            return false;
        }

        return lib.gnustep;
    }

    let version = match target {
        _ if target.contains("macos") || target.contains("darwin") => &lib.macos,
        _ if target.contains("macabi") => &lib.maccatalyst,
        _ if target.contains("ios") => &lib.ios,
        _ if target.contains("tvos") => &lib.tvos,
        _ if target.contains("watchos") => &lib.watchos,
        _ if target.contains("visionos") => &lib.visionos,
        _ => panic!("unknown target: {target}"),
    };

    let Some(version) = version else {
        return false;
    };

    if !req.matches(version) {
        return false;
    }

    if is_simulator(target) {
        // HACK: These aren't available in the simulator.
        if matches!(
            &*lib.framework,
            "Cinematic" | "MediaSetup" | "ThreadNetwork"
        ) {
            return false;
        }

        // HACK: This is not available on the tvOS simulator.
        if target.contains("tvos") && lib.framework == "MetalPerformanceShadersGraph" {
            return false;
        }
    }

    // HACK: These are only available on non-simulator Aarch64.
    if matches!(&*lib.framework, "MLCompute" | "MetalFX")
        && (is_simulator(target) || !is_aarch64(target))
    {
        return false;
    }

    // HACK: Linking `objc2-avf-audio` on older systems is not possible
    // without an SDK that's new enough.
    // TODO: Unsure which SDK version exactly?
    let uses_avf_audio = matches!(
        &*lib.framework,
        "AVFAudio"
            | "AVFoundation"
            | "AVKit"
            | "MediaPlayer"
            | "Photos"
            | "PhotosUI"
            | "SpriteKit"
            | "SceneKit"
    );
    if !req.matches(&Version::new(11, 0, 0)) && uses_avf_audio {
        return false;
    }

    // HACK: PDFKit requires linking Quartz on older systems.
    // TODO: Unsure which version exactly?
    if !req.matches(&Version::new(11, 0, 0)) && lib.framework == "PDFKit" {
        return false;
    }

    // HACK: iTunesLibrary has a different install name on older systems.
    // TODO: Unsure which version exactly?
    if !req.matches(&Version::new(11, 0, 0)) && lib.framework == "iTunesLibrary" {
        return false;
    }

    true
}

fn is_simulator(target: &str) -> bool {
    target.contains("-sim") || matches!(target, "i386-apple-ios" | "x86_64-apple-ios")
}

fn is_aarch64(target: &str) -> bool {
    target.contains("aarch64") || target.contains("arm64")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn available() {
        let config = Config::load().expect("failed loading config");

        let available = |library_name: &str, target: &str, version: &str| -> bool {
            let lib = config.try_library(library_name).unwrap();
            let req = VersionReq::parse(&format!("<={version}")).unwrap();
            library_available(lib, target, &req)
        };

        assert!(available("Cinematic", "aarch64-apple-ios", "26.0"));
        assert!(!available("Cinematic", "aarch64-apple-ios-sim", "26.0"));

        assert!(!available("GameSave", "aarch64-apple-darwin", "14.0"));
        assert!(!available("GameSave", "aarch64-apple-darwin", "15.0"));
        assert!(available("GameSave", "aarch64-apple-darwin", "26.0"));
        assert!(available("GameSave", "aarch64-apple-darwin", "27.0"));

        assert!(!available("MLCompute", "x86_64-apple-ios", "26.0"));
        assert!(!available("MLCompute", "aarch64-apple-ios-sim", "26.0"));
        assert!(available("MLCompute", "aarch64-apple-ios", "26.0"));
    }
}
