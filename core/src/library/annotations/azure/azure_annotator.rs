/// https://learn.microsoft.com/en-gb/azure/devops/pipelines/scripts/logging-commands?view=azure-devops&tabs=bash#task-commands
use super::super::prelude::*;

pub struct AzureAnnotator;

/// ##[command]message
// To invoke a logging command, echo the command via standard output.
//
// Bash
// PowerShell
// Bash
//
// Copy
// #!/bin/bash
// echo "##vso[task.setvariable variable=testvar;]testvalue"
// File paths should be given as absolute paths: rooted to a drive on Windows, or beginning with / on Linux and macOS.
//
//  Note
//
// Please note that you can't use the set -x command before a logging command when you are using Linux or macOS. See troubleshooting, to learn how to disable set -x temporarily for Bash.
//
// Formatting commands
//  Note
//
// Use UTF-8 encoding for logging commands.
//
// These commands are messages to the log formatter in Azure Pipelines. They mark specific log lines as errors, warnings, collapsible sections, and so on.
//
// The formatting commands are:
//
//
// Copy
// ##[group]Beginning of a group
// ##[warning]Warning message
// ##[error]Error message
// ##[section]Start of a section
// ##[debug]Debug text
// ##[command]Command-line being run
// ##[endgroup]
// You can use the formatting commands in a bash or PowerShell task.
//
// Bash
// PowerShell
// YAML
//
// Copy
// steps:
// - bash: |
//     echo "##[group]Beginning of a group"
//     echo "##[warning]Warning message"
//     echo "##[error]Error message"
//     echo "##[section]Start of a section"
//     echo "##[debug]Debug text"
//     echo "##[command]Command-line being run"
//     echo "##[endgroup]"
// Those commands will render in the logs like this:
//
// Screenshot of logs with custom formatting options
//
// That block of commands can also be collapsed, and looks like this:
//
// Screenshot of collapsed section of logs
//
// Task commands
// LogIssue: Log an error or warning
// ##vso[task.logissue]error/warning message
//
// Usage
// Log an error or warning message in the timeline record of the current task.
//
// Properties
// type = error or warning (Required)
// sourcepath = source file location
// linenumber = line number
// columnnumber = column number
// code = error or warning code
// Example: Log an error
// Bash
// PowerShell
// Bash
//
// Copy
// #!/bin/bash
// echo "##vso[task.logissue type=error]Something went very wrong."
// exit 1
//  Tip
//
// exit 1 is optional, but is often a command you'll issue soon after an error is logged. If you select Control Options: Continue on error, then the exit 1 will result in a partially successful build instead of a failed build. As an alternative, you can also use task.logissue type=error.
//
// Example: Log a warning about a specific place in a file
// Bash
// PowerShell
// Bash
//
// Copy
// #!/bin/bash
// echo "##vso[task.logissue type=warning;sourcepath=consoleapp/main.cs;linenumber=1;columnnumber=1;code=100;]Found something that could be a problem."

impl Annotator for AzureAnnotator {
    fn get_annotation_string(&self, annotation: &Annotation) -> String {
        format!(
            "##vso[task.logissue type={};sourcepath={};linenumber={};columnnumber={};endcolumnnumber={};]{}",
            annotation.annotation_type,
            annotation.file,
            annotation.line,
            annotation.start_column,
            annotation.end_column,
            annotation.message
        )
    }
}
