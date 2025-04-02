#!/usr/bin/env bash
set -ex
cd android

# Variables
PACKAGE_NAME="com.rmp.bar"
ACTIVITY_NAME=".MainActivity"

# Launch the app
adb shell am start -n $PACKAGE_NAME/$ACTIVITY_NAME

# Check if launch was successful
if [ $? -eq 0 ]; then
    echo "App launched successfully."
else
    echo "App launch failed."
    exit 1
fi
