#!/usr/bin/env bash
# Set log file path
LOG_FILE="android-avd/emulator.log"

# Parse command line arguments
HEADLESS=false
while [ "$#" -gt 0 ]; do
    case "$1" in
        --headless) HEADLESS=true; shift ;;
        *) echo "Unknown parameter: $1"; exit 1 ;;
    esac
done

# Reset the log file
> "$LOG_FILE"

# Create emulator if it doesn't exist yet
if [ ! -d "$ANDROID_AVD_HOME/emulator.avd" ]; then
    echo "Creating emulator..." | tee -a "$LOG_FILE"
    bash scripts/create-emulator.sh 2>&1 | tee -a "$LOG_FILE"
fi

# Record start time
START_TIME=$(date +%s)

# Run the emulator in the background and redirect output to log file
echo "Starting emulator..." | tee -a "$LOG_FILE"
if [ "$HEADLESS" = true ]; then
    # -no-accel and -no-snapshot were to get this running in github actions
    emulator -avd emulator -no-window -read-only -no-accel -no-snapshot >> "$LOG_FILE" 2>&1 &
else
    emulator -avd emulator -read-only >> "$LOG_FILE" 2>&1 &
fi
EMULATOR_PID=$!

# Wait for the emulator to boot completely
echo "Waiting for emulator to start..." | tee -a "$LOG_FILE"
timeout 180 bash -c 'until adb shell getprop sys.boot_completed 2>/dev/null | grep -q "1"; do 
    echo "Still waiting for boot completion..." >> "'"$LOG_FILE"'"
    sleep 2
done'

if [ $? -eq 0 ]; then
    # Calculate elapsed time
    END_TIME=$(date +%s)
    ELAPSED_TIME=$((END_TIME - START_TIME))
    echo "Emulator started successfully in ${ELAPSED_TIME} seconds" | tee -a "$LOG_FILE"
else
    echo "printing log file"
    cat $LOG_FILE
    echo "Emulator startup timed out or failed" | tee -a "$LOG_FILE"
    kill $EMULATOR_PID
    exit 1
fi
