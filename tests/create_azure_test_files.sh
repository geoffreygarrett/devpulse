#!/bin/bash

# Create the necessary directories
mkdir -p "$(pwd)/output"

# Create a Markdown summary file
cat <<EOL > "$(pwd)/output/summary.md"
# Build Summary

## Summary
This is a summary of the build process.

## Details
- Step 1: Checkout repository
- Step 2: Install dependencies
- Step 3: Run tests
- Step 4: Upload artifacts

## Results
- All tests passed successfully.
EOL

# Create a log file
cat <<EOL > "$(pwd)/output/logfile.log"
2024-07-23 12:34:56 - INFO - Starting build process...
2024-07-23 12:35:00 - INFO - Checking out repository...
2024-07-23 12:35:05 - INFO - Installing dependencies...
2024-07-23 12:36:00 - INFO - Running tests...
2024-07-23 12:37:00 - INFO - Uploading artifacts...
2024-07-23 12:37:30 - INFO - Build process completed successfully.
EOL

# Print a message indicating that the files have been created
echo "Test files created successfully in $(pwd)/output"
