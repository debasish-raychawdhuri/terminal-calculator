#!/bin/bash

# Start the calculator
cargo run &
CALC_PID=$!

# Wait for the calculator to start
sleep 2

# Send keystrokes to demonstrate features
xdotool type "5 + 3"
sleep 1
xdotool key Return
sleep 2

xdotool type "(2 + 3) * (4 - 1)"
sleep 1
xdotool key Return
sleep 2

xdotool type "(10 + (5 * 2)) / 4"
sleep 1
xdotool key Return
sleep 2

# Exit the calculator
xdotool key Escape

# Clean up
kill $CALC_PID
