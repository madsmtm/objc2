#! /usr/bin/env sh

set -eu

for arch in $IOS_ARCHS; do
    rustup target add "${arch}-apple-ios"
done

if [ -n "$IOS_ARCHS" ]; then
    curl -LO https://github.com/SSheldon/rust-test-ios/releases/download/0.1.1/rust-test-ios
    chmod +x rust-test-ios
fi
