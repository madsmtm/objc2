#!/bin/bash

# Note: This file is run in CI as well as manually by the user

set -euxo pipefail

RUN="cargo run --bin=test-assembly --  -Zbuild-std -Zbuild-std --target"

echo "Apple"
$RUN x86_64-apple-darwin --features=assembly-features
$RUN aarch64-apple-darwin --features=assembly-features
$RUN armv7s-apple-ios --features=assembly-features
$RUN i386-apple-ios --features=assembly-features

echo "Old Apple"
$RUN i686-apple-darwin --features=assembly-features

echo "GNUStep"
$RUN x86_64-unknown-linux-gnu --features=gnustep-2-1
$RUN i686-unknown-linux-gnu --features=gnustep-2-1

echo "Old GNUStep"
$RUN x86_64-unknown-linux-gnu --features=gnustep-1-7
$RUN i686-unknown-linux-gnu --features=gnustep-1-7
