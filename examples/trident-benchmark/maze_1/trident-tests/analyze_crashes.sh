#!/bin/bash

# Directory containing crashes
CRASH_DIR="out/default/crashes"
BINARY="target/debug/fuzz_0"

# Check if crash directory exists
if [ ! -d "$CRASH_DIR" ]; then
    echo "Error: Crash directory $CRASH_DIR not found"
    exit 1
fi

# Count crash files (excluding README)
crash_count=$(find "$CRASH_DIR" -type f ! -name "README*" | wc -l | tr -d ' ')
echo "Found $crash_count crash files"
echo "================================"

# Temporary file to collect assertion numbers
ASSERTIONS_FILE=$(mktemp)

# Process each crash file
for crash_file in "$CRASH_DIR"/id:*; do
    if [ -f "$crash_file" ]; then
        filename=$(basename "$crash_file")
        echo -n "Processing: $filename ... "
        
        # Run the fuzzer with this crash and capture output
        output=$(timeout 30s cargo afl run "$BINARY" < "$crash_file" 2>&1)
        
        # Extract AssertionFailed number
        assertion=$(echo "$output" | grep -oE 'AssertionFailed: [0-9]+' | head -1)
        
        if [ -n "$assertion" ]; then
            num=$(echo "$assertion" | grep -oE '[0-9]+')
            echo "$num" >> "$ASSERTIONS_FILE"
            echo "$assertion"
        else
            # Check for other panic messages
            panic=$(echo "$output" | grep -oE "panicked at.*" | head -1)
            if [ -n "$panic" ]; then
                echo "Other panic: $panic"
            else
                echo "No assertion found"
            fi
        fi
    fi
done

echo ""
echo "================================"
echo "SUMMARY: Unique AssertionFailed values"
echo "================================"

if [ -s "$ASSERTIONS_FILE" ]; then
    echo "Assertion Number | Count"
    echo "-----------------|------"
    sort "$ASSERTIONS_FILE" | uniq -c | sort -rn | while read count num; do
        printf "%-17s| %d\n" "$num" "$count"
    done
    echo ""
    echo "Total unique assertions: $(sort -u "$ASSERTIONS_FILE" | wc -l | tr -d ' ')"
    echo "Total crashes analyzed: $(wc -l < "$ASSERTIONS_FILE" | tr -d ' ')"
else
    echo "No AssertionFailed errors found"
fi

rm -f "$ASSERTIONS_FILE"
