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
    # The format follows the GitHub Actions annotation syntax
    command = (
        f"echo '::{annotation_type} file={file},line={line},endLine={end_line},"
        f"col={start_column},endColumn={end_column}::{message}'"
    )

    # Execute the command to create the annotation
    os.system(command)
