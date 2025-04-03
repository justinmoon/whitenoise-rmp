#!/usr/bin/env bash
set -ex
cd android

# Build the debug version of the app
./gradlew assembleDebug

# Check if build was successful
if [ $? -eq 0 ]; then
    echo "Build successful."
else
    echo "Build failed."
    exit 1
fi

