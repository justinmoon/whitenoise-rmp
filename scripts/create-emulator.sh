#!/usr/bin/env bash

mkdir -p $ANDROID_AVD_HOME/emulator.avd

avdmanager create avd --force --name emulator --package 'system-images;android-35;google_apis;arm64-v8a' --path $ANDROID_AVD_HOME/emulator.avd -d pixel_6

echo "avd.ini.encoding=UTF-8" > $ANDROID_AVD_HOME/emulator.ini
echo "path=$ANDROID_AVD_HOME/emulator.avd" >> android-avd/emulator.ini
echo "path.rel=emulator.avd" >> $ANDROID_AVD_HOME/emulator.ini
echo "target=android-35" >> $ANDROID_AVD_HOME/emulator.ini

