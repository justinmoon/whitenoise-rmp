#!/bin/bash
set -ex
cd rust

# Build the dylib for local use
# FIXME: add debug / release flag
cargo build

# Build the Android libraries in jniLibs
# armeabi-v7a needed by xiaomi a3
# arm64-v8a needed by pixel 3a emulator
        # -t armeabi-v7a \
cargo ndk -o ../android/app/src/main/jniLibs \
        --manifest-path ./Cargo.toml \
        -t arm64-v8a \
        build --release

# Create Kotlin bindings
cargo run --bin uniffi-bindgen generate \
    --library ../target/debug/libbar.dylib \
    --language kotlin \
    --out-dir ../android/app/src/main/java/com/rmp/bar

