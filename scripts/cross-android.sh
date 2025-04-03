#!/bin/bash
set -ex
cd rust

# Build the Android libraries in jniLibs
# armeabi-v7a needed by xiaomi a3
# arm64-v8a needed by pixel 3a emulator
# x86_64 needed for Linux emulators

# Determine target architecture based on OS
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    TARGET_ARCH="x86_64"
else
    TARGET_ARCH="arm64-v8a"
fi

echo "Building for target architecture: $TARGET_ARCH"

cargo ndk -o ../android/app/src/main/jniLibs \
        --manifest-path ./Cargo.toml \
        -t $TARGET_ARCH \
        build --release

ls ../android/app/src/main
ls ../android/app/src/main/jniLibs

if [[ "$OSTYPE" == "darwin"* ]]; then
    LIB_EXT="dylib"
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    LIB_EXT="so" 
else
    echo "Unsupported OS"
    exit 1
fi

# testing out the libbar.so from android ...
# --library ../target/debug/libbar.$LIB_EXT \

# Create Kotlin bindings
cargo run --bin uniffi-bindgen generate \
    --library ../target/aarch64-linux-android/release/libbar.so \
    --language kotlin \
    --out-dir ../android/app/src/main/java/com/rmp/bar

