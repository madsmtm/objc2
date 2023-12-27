#!/bin/bash
# A test script I use to test on my local devices

export MACOSX_DEPLOYMENT_TARGET=10.12
export IPHONEOS_DEPLOYMENT_TARGET=10.0
export FEATURES=std,block,exception,catch-all,verify,unstable-static-class,unstable-static-sel

# Start the simulator
open /Applications/Xcode.app/Contents/Developer/Applications/Simulator.app

# Test on the simulator
cargo dinghy --device sim test
cargo dinghy --device sim test --features=$FEATURES
cargo dinghy --device sim test --release

# Test on my iPad mini 1st generation iOS 9.3
# Followed this guide to set it up:
# https://github.com/sonos/dinghy/blob/main/docs/ios.md
#
# We use build-std and earlier nightly because the target is armv7-apple-ios, which was removed in:
# https://github.com/rust-lang/rust/pull/104385
IPHONEOS_DEPLOYMENT_TARGET=9.0 cargo +nightly-2023-09-23 dinghy --device ipad test -Zbuild-std
IPHONEOS_DEPLOYMENT_TARGET=9.0 cargo +nightly-2023-09-23 dinghy --device ipad test -Zbuild-std --features=$FEATURES
IPHONEOS_DEPLOYMENT_TARGET=9.0 cargo +nightly-2023-09-23 dinghy --device ipad test -Zbuild-std --release

# Test on macOS 32bit
export SDKROOT=(pwd)/ideas/MacOSX10.13.sdk
export CARGO_BUILD_TARGET=i686-apple-darwin
cargo +nightly test -Zbuild-std
cargo +nightly test -Zbuild-std --features=$FEATURES
cargo +nightly test -Zbuild-std --release
