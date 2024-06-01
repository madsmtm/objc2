# Fuzzing `objc2`

Run using `cargo-fuzz` (note: you must specify the `--fuzz-dir` argument).

```sh
cargo fuzz run --fuzz-dir=crates/test-fuzz/ $fuzz_target
```
