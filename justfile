default:
    just --list

# Opens the android emulator (installs one on first run)
run-emulator:
    bash scripts/run-emulator.sh

# Cross-compile rust uniffi code for Android
cross-android: 
    bash scripts/cross-android.sh

# Install Android APK
build-apk: cross-android
    bash scripts/build-apk.sh

# Install Android APK
install-apk: build-apk
    adb install -r android/app/build/outputs/apk/debug/app-debug.apk

# Run the android app
run-android: install-apk
    bash scripts/run-android.sh

# Run E2E tests with Appium (assumes app is built and emulator is running)
ui-tests: install-apk
    bash scripts/ui-tests.sh

# Lint all source files
lint:
    cd rust
    cargo check
    cargo clippy
    cd ..

# Lint all source file lint errors
lint-fix:
    cd rust
    cargo fix --allow-dirty
    cargo clippy --fix --allow-dirty
    cd ..

# Hit the "home" button on android emulator (sometimes broken)
adb-home:
    adb shell input keyevent 3

# Delete all build artifacts and revert to basically fresh git checkout
clean:
    cd rust
    cargo clean
    cd ..
    rm -rf android/.gradle

# Auto-format rust and nix files (Kotlin soon, too!)
format:
    cargo fmt --all
    nixfmt $(git ls-files | grep "\.nix$")

