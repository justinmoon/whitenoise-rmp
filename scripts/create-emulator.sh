#!/usr/bin/env bash

mkdir -p $ANDROID_AVD_HOME/emulator.avd

# Detect platform and choose the appropriate system image
if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS - use ARM64 image
    SYSTEM_IMAGE="system-images;android-35;google_apis;arm64-v8a"
    echo "Detected macOS, using ARM64 system image"
else
    # Linux/other - use x86_64 image
    SYSTEM_IMAGE="system-images;android-35;google_apis;x86_64"
    echo "Detected Linux/other, using x86_64 system image"
fi

avdmanager create avd --force --name emulator --package "$SYSTEM_IMAGE" --path $ANDROID_AVD_HOME/emulator.avd -d pixel_6

echo "avd.ini.encoding=UTF-8" > $ANDROID_AVD_HOME/emulator.ini
echo "path=$ANDROID_AVD_HOME/emulator.avd" >> android-avd/emulator.ini
echo "path.rel=emulator.avd" >> $ANDROID_AVD_HOME/emulator.ini
echo "target=android-35" >> $ANDROID_AVD_HOME/emulator.ini

