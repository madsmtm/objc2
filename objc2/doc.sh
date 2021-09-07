#! /usr/bin/env sh

set -ev

mkdir -p doc
cd doc

git clone -b gh-pages git@github.com:SSheldon/rust-objc.git out
pushd out
git rm -r .
git reset HEAD .
popd

CARGO_TOML='[package]
name = "doc"
version = "0.0.0"

[lib]
name = "doc"
path = "doc.rs"

[dependencies]
block = "*"
dispatch = "*"
objc = "*"
objc-encode = "*"
objc_exception = "*"
objc_test_utils = "*"
objc_id = "*"
objc-foundation = "*"'

echo "$CARGO_TOML" > Cargo.toml
touch doc.rs
cargo doc

cp -r target/doc/ out/
pushd out
rm .lock
git add .
git commit -m "Updated documentation."
popd
