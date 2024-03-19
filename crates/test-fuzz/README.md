# Fuzzing `objc2`

Run using `cargo-fuzz` (note: you must specify the `--fuzz-dir` argument).

```sh
cargo fuzz run --fuzz-dir=./crates/test-fuzz/ --features=fuzz-all $fuzz_target
```

## Fuzzing with `afl`

The `mut_while_iter` fuzzing target AFL++, by doing:

```sh
mkdir target/afl
mkdir target/afl/in
mkdir target/afl/out
echo "" > target/afl/in/empty
cargo afl build --bin mut_while_iter --features=afl,fuzz-all --release
cargo afl fuzz -i target/afl/in -o target/afl/out target/release/mut_while_iter
```
