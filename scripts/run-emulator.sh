#!/usr/bin/env bash

# Create emulator if it doesn't exist yet
if [ ! -d "$ANDROID_AVD_HOME/emulator.avd" ]; then
    bash scripts/create-emulator.sh
fi

# Run the emulator
emulator -avd emulator

