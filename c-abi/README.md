https://github.com/robo9k/rust-magic-sys/blob/main/src/lib.rs

```sh
cargo build --release
nm --dynamic --defined-only ../target/release/libmagic.so | awk '$2=="T"'
```
