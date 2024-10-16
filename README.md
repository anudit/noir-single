# Noir

With Noir/Nargo & Rust installed run,
```
nargo compile
cargo run --release
sh ./set-env.sh
```

Build Test
```
#works
cargo ndk --target aarch64-linux-android --platform 33 build
cargo build --release --target aarch64-apple-ios

#fails
cargo ndk --target armv7-linux-androideabi --platform 33 build
cargo build --release --target aarch64-apple-ios-sim # not supported
cargo build --release --target x86_64-apple-ios # unknown type name '__int64_t' '__int32_t'
```
