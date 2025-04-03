#!/usr/bin/env bash
set -ex

adb install -r app/build/outputs/apk/debug/app-debug.apk

# Check if install was successful
if [ $? -eq 0 ]; then
    echo "App installed successfully."
else
    echo "App installation failed."
    exit 1
fi

