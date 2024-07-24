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

# Create a test result file
cat <<EOL > "$(pwd)/output/testresult.trx"
<?xml version="1.0" encoding="utf-8"?>
<TestRun id="00000000-0000-0000-0000-000000000000" name="user@machine 2024-07-23 12:37:00" runUser="user@machine">
  <TestSettings name="TestSettings1" id="00000000-0000-0000-0000-000000000000"/>
  <Results>
    <UnitTestResult executionId="00000000-0000-0000-0000-000000000000" testId="00000000-0000-0000-0000-000000000000" testName="TestMethod1" computerName="machine" duration="00:00:00.1234567" startTime="2024-07-23T12:37:00.1234567+00:00" endTime="2024-07-23T12:37:00.1234567+00:00" testType="13cdc9d9-ddb5-4fa4-a97d-d965ccfc6d4b" outcome="Passed" testListId="00000000-0000-0000-0000-000000000000" relativeResultsDirectory="00000000-0000-0000-0000-000000000000"/>
  </Results>
</TestRun>
EOL

# Print a message indicating that the files have been created
echo "Test files created successfully in $(pwd)/output"
