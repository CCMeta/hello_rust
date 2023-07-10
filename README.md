# RUST NOTE

```text
please make sure your ~/.cargo/config.toml settings the right linker
[target.aarch64-unknown-linux-musl]
linker = "lld"
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-none-linux-gnu-gcc"
rustflags = ["-C", "target-feature=+crt-static"]
```

```text
build .o: clang -target aarch64-linux-musl -fuse-ld=lld -nostdinc -nostdlib -isystem
build .a: llvm-ar r libhello.a hello.o
build .o + .a: clang -target aarch64-linux-musl -fuse-ld=lld -static -o libhello.a // this is wrong!
build rs: cargo build --target=aarch64-unknown-linux-musl
crate new lib: cargo new ffi-example --lib
build rs-static-lib: crate-type = ["staticlib"] && cargo build 
build rs-shared-lib: crate-type = ["dylib"] && cargo build 
get header file: cargo install --force cbindgen && $ cbindgen --config cbindgen.toml --crate rust_to_c --output sum.h


```
