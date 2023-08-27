#!/bin/bash

# Note: This file is run in CI as well as manually by the user

set -euxo pipefail

ARGS="run --features=run --bin=test-assembly --  -Zbuild-std --target"

# Add `+nightly` when using `-Zbuild-std` if the user's `cargo` is not a
# `nightly` by default
if grep -q nightly <<< $(cargo --version); then
    NIGHTLY=""
else
    NIGHTLY="+nightly"
fi

echo "Apple"
cargo $NIGHTLY $ARGS x86_64-apple-darwin
cargo $NIGHTLY $ARGS aarch64-apple-darwin
cargo $NIGHTLY $ARGS armv7s-apple-ios
cargo $NIGHTLY $ARGS armv7-apple-ios
cargo $NIGHTLY $ARGS i386-apple-ios

echo "Old Apple"
cargo $NIGHTLY $ARGS i686-apple-darwin

echo "GNUStep"
cargo $NIGHTLY $ARGS x86_64-unknown-linux-gnu --no-default-features --features=std,gnustep-2-1
cargo $NIGHTLY $ARGS i686-unknown-linux-gnu --no-default-features --features=std,gnustep-2-1

echo "Old GNUStep"
cargo $NIGHTLY $ARGS x86_64-unknown-linux-gnu --no-default-features --features=std,gnustep-1-7
cargo $NIGHTLY $ARGS i686-unknown-linux-gnu --no-default-features --features=std,gnustep-1-7
