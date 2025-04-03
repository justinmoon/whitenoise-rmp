#!/usr/bin/env bash
# Set log file path
LOG_FILE="android-avd/emulator.log"

# Reset the log file
> "$LOG_FILE"

# Create emulator if it doesn't exist yet
if [ ! -d "$ANDROID_AVD_HOME/emulator.avd" ]; then
    echo "Creating emulator..." | tee -a "$LOG_FILE"
    bash scripts/create-emulator.sh 2>&1 | tee -a "$LOG_FILE"
fi

# Run the emulator in the background and redirect output to log file
echo "Starting emulator..." | tee -a "$LOG_FILE"
emulator -avd emulator -no-window >> "$LOG_FILE" 2>&1 &
EMULATOR_PID=$!

# Wait for the emulator to boot completely
echo "Waiting for emulator to start..." | tee -a "$LOG_FILE"
timeout 60 bash -c 'until adb shell getprop sys.boot_completed 2>/dev/null | grep -q "1"; do 
    echo "Still waiting for boot completion..." >> "'"$LOG_FILE"'"
    sleep 2
done'

if [ $? -eq 0 ]; then
    echo "Emulator started successfully" | tee -a "$LOG_FILE"
else
    echo "Emulator startup timed out or failed" | tee -a "$LOG_FILE"
    cat $LOG_FILE
    kill $EMULATOR_PID
    exit 1
fi
