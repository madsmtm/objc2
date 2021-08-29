#! /usr/bin/env sh

set -eu

gnustep_install() {
    git clone -b 1.9 https://github.com/gnustep/libobjc2.git
    mkdir libobjc2/build
    cd libobjc2/build
    export CC="clang"
    export CXX="clang++"
    cmake -DCMAKE_INSTALL_PREFIX:PATH=$HOME/libobjc2_staging ../
    make install
}

for arch in $IOS_ARCHS; do
    rustup target add "${arch}-apple-ios"
done

if [ -n "$IOS_ARCHS" ]; then
    curl -LO https://github.com/SSheldon/rust-test-ios/releases/download/0.1.1/rust-test-ios
    chmod +x rust-test-ios
fi

if [ "$TRAVIS_OS_NAME" = "linux" ]; then
    gnustep_install
fi
