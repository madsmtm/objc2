# Fuzzing `objc2`

Run using `cargo-fuzz` (note: you must specify the `--fuzz-dir` argument).

```sh
cargo +nightly fuzz run --fuzz-dir=crates/test-fuzz/ $fuzz_target
```
