# Fuzzing `objc2`

Run using `cargo-fuzz` (note: you must specify the `--fuzz-dir` argument).

```sh
cargo fuzz run --fuzz-dir=./crates/test-fuzz/ --features=fuzz-all $fuzz_target
```

## Fuzzing with `afl`

Fuzz with AFL++ by doing:

(This is probably not the optimal settings, AFL has a lot of configuration options).

```sh
cargo afl build --bin $fuzz_target --features=afl,fuzz-all --release
cargo afl fuzz -i crates/test-fuzz/corpus/$fuzz_target -o crates/test-fuzz/afl -- target/release/$fuzz_target
```
