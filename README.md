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


#fails
cargo ndk --target armv7-linux-androideabi --platform 33 build

```
