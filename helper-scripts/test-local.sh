#!/bin/bash
# A test script I use to test on my local devices

set -euxo pipefail

export CARGO_TARGET_DIR=$HOME/Desktop/objc2-target
export MACOSX_DEPLOYMENT_TARGET=10.12
export CRATES='--package=block2 --package=objc-sys --package=objc2 --package=objc2-encode --package=objc2-proc-macros --package=tests'
export FRAMEWORKS_MACOS_10_13='--package=objc2-app-kit --package=objc2-automator --package=objc2-cloud-kit --package=objc2-contacts --package=objc2-core-data --package=objc2-core-wlan --package=objc2-event-kit --package=objc2-exception-handling --package=objc2-external-accessory --package=objc2-foundation --package=objc2-game-controller --package=objc2-game-kit --package=objc2-input-method-kit --package=objc2-local-authentication --package=objc2-map-kit --package=objc2-media-player --package=objc2-metal --package=objc2-metal-kit --package=objc2-osa-kit --package=objc2-quartz-core --package=objc2-service-management --package=objc2-store-kit --package=objc2-web-kit'
export FRAMEWORKS_IOS_9='--package=objc2-foundation --package=objc2-metal'

# Test on macOS 32bit
export SDKROOT=$HOME/Desktop/MacOSX10.13.sdk
cargo test $CRATES
cargo test $CRATES --features=unstable-static-class,unstable-static-sel
cargo test $CRATES $FRAMEWORKS_MACOS_10_13 --features=block2,exception,catch-all,all
cargo test $CRATES --release
cargo test -Zbuild-std --target=i686-apple-darwin $CRATES
cargo test -Zbuild-std --target=i686-apple-darwin $CRATES $FRAMEWORKS_MACOS_10_13 --features=block2,exception,catch-all,all
cargo test -Zbuild-std --target=i686-apple-darwin $CRATES --release
unset SDKROOT

# Start the simulator
open /Applications/Xcode.app/Contents/Developer/Applications/Simulator.app

# Test on the simulator
IPHONEOS_DEPLOYMENT_TARGET=10.0 cargo dinghy --device sim test $CRATES
IPHONEOS_DEPLOYMENT_TARGET=10.0 cargo dinghy --device sim test $CRATES --features=unstable-static-class,unstable-static-sel
IPHONEOS_DEPLOYMENT_TARGET=10.0 cargo dinghy --device sim test $CRATES $FRAMEWORKS_IOS_9 --features=block2,exception,catch-all,all
IPHONEOS_DEPLOYMENT_TARGET=10.0 cargo dinghy --device sim test $CRATES --release

# Test on my iPad mini 1st generation iOS 9.3
# Followed this guide to set it up:
# https://github.com/sonos/dinghy/blob/main/docs/ios.md
#
# We use build-std and earlier nightly because the target is armv7-apple-ios, which was removed in:
# https://github.com/rust-lang/rust/pull/104385
#
# Requires: `cargo install cargo-dinghy@0.6.8`
export DINGHY_LOG=trace
IPHONEOS_DEPLOYMENT_TARGET=9.0 cargo +nightly-2023-09-23 dinghy --device ipad test -Zbuild-std $CRATES
IPHONEOS_DEPLOYMENT_TARGET=9.0 cargo +nightly-2023-09-23 dinghy --device ipad test -Zbuild-std $CRATES --features=unstable-static-class,unstable-static-sel
IPHONEOS_DEPLOYMENT_TARGET=9.0 cargo +nightly-2023-09-23 dinghy --device ipad test -Zbuild-std $CRATES $FRAMEWORKS_IOS_9 --features=block2,exception,catch-all,all
IPHONEOS_DEPLOYMENT_TARGET=9.0 cargo +nightly-2023-09-23 dinghy --device ipad test -Zbuild-std $CRATES --release
