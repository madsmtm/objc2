#! /usr/bin/env sh

set -eu

rust_ios_install() {
    ios_stdlib="rust-std-nightly-${1}-apple-ios"
    curl -O "http://static.rust-lang.org/dist/${ios_stdlib}.tar.gz"
    tar xzf "${ios_stdlib}.tar.gz"
    "./${ios_stdlib}/install.sh" --prefix=$(rustc --print sysroot)
}

for arch in $IOS_ARCHS; do
    rust_ios_install "$arch"
done

if [ -n "$IOS_ARCHS" ]; then
    curl -LO https://github.com/SSheldon/rust-test-ios/releases/download/0.1.1/rust-test-ios
    chmod +x rust-test-ios
fi
