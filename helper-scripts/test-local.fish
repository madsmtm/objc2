#!/usr/local/bin/fish
# A test script I use to test on my local devices

echo 'Add this to objc2/Cargo.toml: `default = ["exception", "verify_message", "catch-all"]`'
read

export MACOSX_DEPLOYMENT_TARGET=10.7
export IPHONEOS_DEPLOYMENT_TARGET=7.0

# Use custom branch of cargo-dinghy that fixes an issue with iOS 9.3
cargo install --git https://github.com/madsmtm/dinghy --branch fix-active-session --bin cargo-dinghy

# Start the simulator
open /Applications/Xcode.app/Contents/Developer/Applications/Simulator.app

# Test on the simulator
cargo dinghy --device sim test
cargo dinghy --device sim test --release

# Test on my iPad mini 1st generation iOS 9.3
# Followed this guide to set it up:
# https://github.com/sonos/dinghy/blob/main/docs/ios.md
#
# We use build-std because the target is armv7-apple-ios
mkdir .cargo
echo '[unstable]' > .cargo/config.toml
echo 'build-std = ["core", "alloc", "std"]' >> .cargo/config.toml
cargo +nightly dinghy --device ipad test
cargo +nightly dinghy --device ipad test --release
rm .cargo/config.toml
rm -d .cargo

# Test on macOS 32bit
export SDKROOT=(pwd)/ideas/MacOSX10.13.sdk
export CARGO_BUILD_TARGET=i686-apple-darwin
cargo +nightly test -Zbuild-std
cargo +nightly test -Zbuild-std --features malloc,block,exception,catch-all,verify_message
cargo +nightly test -Zbuild-std --release
