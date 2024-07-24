# Note on GitHub Workflow Annotations

## Overview

GitHub Actions allows the use of workflow commands to communicate with the runner machine for various tasks such as
setting environment variables, output values, and creating log messages. These commands help in automating workflows and
can also be used to create annotations in the repository.

## Key Workflow Commands for Annotations

Workflow commands typically use the `echo` command with a specific format to interact with the GitHub Actions toolkit
functions.

### Setting Messages

- **Debug Message**:
  ```bash
  echo "::debug::Set the Octocat variable"
  ```

- **Notice Message**:
  ```bash
  echo "::notice file=app.js,line=1,col=5,endColumn=7::Missing semicolon"
  ```

- **Warning Message**:
  ```bash
  echo "::warning file=app.js,line=1,col=5,endColumn=7::Missing semicolon"
  ```

- **Error Message**:
  ```bash
  echo "::error file=app.js,line=1,col=5,endColumn=7::Missing semicolon"
  ```

### Grouping Log Lines

- **Group Log Lines**:
  ```yaml
  jobs:
    bash-example:
      runs-on: ubuntu-latest
      steps:
        - name: Group of log lines
          run: |
            echo "::group::My title"
            echo "Inside group"
            echo "::endgroup::"
  ```

### Masking Values

- **Mask a Value**:
  ```bash
  echo "::add-mask::Mona The Octocat"
  ```

### Environment Files

- **Set an Environment Variable**:
  ```bash
  echo "MY_ENV_VAR=myValue" >> $GITHUB_ENV
  ```

- **Set an Output Parameter**:
  ```bash
  echo "SELECTED_COLOR=green" >> $GITHUB_OUTPUT
  ```

- **Add a Job Summary**:
  ```bash
  echo "### Hello world! :rocket:" >> $GITHUB_STEP_SUMMARY
  ```

- **Add a System Path**:
  ```bash
  echo "$HOME/.local/bin" >> $GITHUB_PATH
  ```

## Example Annotation Script in Python

The following Python script demonstrates how to create annotations in a GitHub Actions workflow by using
self-referencing annotations.

```python
import os

# Example annotation data
annotations = [
    {
        "file": "annotate.py",  # The script is annotating itself
        "line": 10,  # Line number where the message should appear
        "end_line": 10,  # End line number for the annotation
        "start_column": 5,  # Starting column for the annotation
        "end_column": 20,  # Ending column for the annotation
        "message": "Consider refactoring this function to improve readability.",  # Annotation message
        "annotation_type": "notice",  # Annotation type, can be 'error', 'warning', or 'notice'
    },
    {
        "file": "annotate.py",
        "line": 15,
        "end_line": 15,
        "start_column": 5,
        "end_column": 20,
        "message": "Ensure the command format is correct.",
        "annotation_type": "notice",
    }
]

# Loop over each annotation in the list
for annotation in annotations:
    file = annotation["file"]
    line = annotation["line"]
    end_line = annotation["end_line"]
    start_column = annotation["start_column"]
    end_column = annotation["end_column"]
    message = annotation["message"]
    annotation_type = annotation["annotation_type"]

    # Construct the GitHub Actions annotation command
    command = (
        f"echo '::{annotation_type} file={file},line={line},endLine={end_line},"
        f"col={start_column},endColumn={end_column}::{message}'"
    )
    
    # Execute the command to create the annotation
    os.system(command)
```

## Workflow Example with Annotations

Below is an example of how you can include the above Python script in a GitHub Actions workflow to create annotations:

```yaml
name: Annotate Script

on: [ push ]

jobs:
  annotate:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout code
        uses: actions/checkout@v2

      - name: Set up Python
        uses: actions/setup-python@v2
        with:
          python-version: 3.8

      - name: Run Annotation Script
        run: python annotate.py
```

## Conclusion

GitHub Actions provides powerful tools to automate workflows, and using workflow commands, you can create detailed
annotations in your repository. These annotations can help identify and address issues directly within the codebase,
improving collaboration and code quality.