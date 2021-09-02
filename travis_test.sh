#! /usr/bin/env sh

set -eu

if [ -z "$IOS_ARCHS" ]; then
    cargo build --verbose
    cargo test --verbose
else
    ./rust-test-ios
fi
