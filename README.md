# RUST NOTE

```text
please make sure your ~/.cargo/config.toml settings the right linker
[target.aarch64-unknown-linux-musl]
linker = "lld"
```

```text
build .o: clang -target aarch64-linux-musl -fuse-ld=lld -nostdinc -nostdlib -isystem
build .a: llvm-ar r libhello.a hello.o
build .o + .a: clang -target aarch64-linux-musl -fuse-ld=lld -static -o libhello.a // this is wrong!
build rs: cargo build --target=aarch64-unknown-linux-musl

```
