#!/usr/local/bin/fish

# Yup, this is terrible

# Source repo should be a path to https://github.com/gnustep/libobjc2.git
set source_repo $argv[1]
set out_file $argv[2]

cp gnustep-headers.h $source_repo/objc/headers.h

bindgen $source_repo/objc/headers.h \
    -o $out_file \
    --no-layout-tests \
    --no-doc-comments \
    --size_t-is-usize \
    --allowlist-function="(sel|object|class|objc|method|_?protocol|ivar|property|imp|_objc|)_.*" \
    --allowlist-var=".*OBJC.*" \
    --allowlist-var=".*objc.*" \
    # GNUStep-specific functions
    --blocklist-function=".*_np"

rm $source_repo/objc/headers.h
