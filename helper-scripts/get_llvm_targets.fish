set APPLE_TARGETS x86_64-apple-darwin aarch64-apple-darwin aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios aarch64-apple-ios-macabi aarch64-apple-tvos aarch64-apple-watchos-sim arm64_32-apple-watchos armv7k-apple-watchos armv7s-apple-ios i386-apple-ios i686-apple-darwin x86_64-apple-ios-macabi x86_64-apple-tvos x86_64-apple-watchos-sim

for TARGET in $APPLE_TARGETS
    echo ""
    echo "$TARGET"
    rustc +nightly --print target-spec-json --target=$TARGET | jq '.["llvm-target"]'
end
