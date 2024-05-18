#!/bin/bash

# Note: This file is run in CI as well as manually by the user

set -euxo pipefail

RUN="cargo run --bin=test-assembly --  -Zbuild-std -Zbuild-std --target"

echo "Apple"
$RUN x86_64-apple-darwin
$RUN aarch64-apple-darwin
$RUN armv7s-apple-ios
$RUN i386-apple-ios

echo "Old Apple"
$RUN i686-apple-darwin

echo "GNUStep"
$RUN x86_64-unknown-linux-gnu --no-default-features --features=gnustep-2-1
$RUN i686-unknown-linux-gnu --no-default-features --features=gnustep-2-1

echo "Old GNUStep"
$RUN x86_64-unknown-linux-gnu --no-default-features --features=gnustep-1-7
$RUN i686-unknown-linux-gnu --no-default-features --features=gnustep-1-7
