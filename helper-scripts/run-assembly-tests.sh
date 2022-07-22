#!/bin/bash

set -euxo pipefail

export HELPER="cargo run --features=run --bin=test-assembly -- --target"
export NIGHTLY_HELPER="cargo +nightly run --features=run --bin=test-assembly -- -Zbuild-std --target"

echo "Apple"
$HELPER x86_64-apple-darwin
$HELPER aarch64-apple-darwin
$NIGHTLY_HELPER armv7s-apple-ios
$NIGHTLY_HELPER armv7-apple-ios
$NIGHTLY_HELPER i386-apple-ios

echo "Old Apple"
$NIGHTLY_HELPER i686-apple-darwin

echo "GNUStep"
$HELPER x86_64-unknown-linux-gnu --no-default-features --features=std,gnustep-2-1
$HELPER i686-unknown-linux-gnu --no-default-features --features=std,gnustep-2-1

echo "Old GNUStep"
$HELPER x86_64-unknown-linux-gnu --no-default-features --features=std,gnustep-1-7
$HELPER i686-unknown-linux-gnu --no-default-features --features=std,gnustep-1-7
