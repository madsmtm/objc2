#!/bin/bash

# Note: This file is run in CI as well as manually by the user

set -euxo pipefail

export ARGS="run --features=run --bin=test-assembly -- --target"

# Add `+nightly` when using `-Zbuild-std` if the user's `cargo` is not a
# `nightly` by default
if grep -q nightly <<< $(cargo --version); then
    export NIGHTLY=""
else
    export NIGHTLY="+nightly"
fi

echo "Apple"
cargo $NIGHTLY $ARGS x86_64-apple-darwin
cargo $NIGHTLY $ARGS aarch64-apple-darwin -Zbuild-std
cargo $NIGHTLY $ARGS armv7s-apple-ios -Zbuild-std
cargo $NIGHTLY $ARGS armv7-apple-ios -Zbuild-std
cargo $NIGHTLY $ARGS i386-apple-ios -Zbuild-std

echo "Old Apple"
cargo $NIGHTLY $ARGS i686-apple-darwin -Zbuild-std

echo "GNUStep"
cargo $NIGHTLY $ARGS x86_64-unknown-linux-gnu -Zbuild-std --no-default-features --features=std,gnustep-2-1
cargo $NIGHTLY $ARGS i686-unknown-linux-gnu -Zbuild-std --no-default-features --features=std,gnustep-2-1

echo "Old GNUStep"
cargo $NIGHTLY $ARGS x86_64-unknown-linux-gnu -Zbuild-std --no-default-features --features=std,gnustep-1-7
cargo $NIGHTLY $ARGS i686-unknown-linux-gnu -Zbuild-std --no-default-features --features=std,gnustep-1-7
