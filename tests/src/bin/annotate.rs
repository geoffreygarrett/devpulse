use std::env;

use devpulse_core::annotations::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let annotations = vec![
        // Notice annotation
        Annotation {
            file: "tests/src/main.rs".to_string(),
            line: 10, // This should have a notice
            end_line: 10,
            start_column: 5,
            end_column: 30,
            message: "Consider refactoring this function to improve readability.".to_string(),
            annotation_type: AnnotationType::Notice,
        },
        // Warning annotation
        Annotation {
            file: "tests/src/main.rs".to_string(),
            line: 15, // This should have a warning
            end_line: 15,
            start_column: 5,
            end_column: 30,
            message: "Ensure the command format is correct.".to_string(),
            annotation_type: AnnotationType::Warning,
        },
        // Error annotation // WORKS BUT COMMENTED OUT
        // Annotation {
        //     file: "tests/src/main.rs".to_string(),
        //     line: 20, // This should have an error
        //     end_line: 20,
        //     start_column: 5,
        //     end_column: 30,
        //     message: "This function implementation is incorrect.".to_string(),
        //     annotation_type: AnnotationType::Error,
        // },
        // Debug annotation
        Annotation {
            file: "tests/src/main.rs".to_string(),
            line: 25, // This should have a debug message
            end_line: 25,
            start_column: 5,
            end_column: 30,
            message: "Debugging this line for performance.".to_string(),
            annotation_type: AnnotationType::Debug,
        },
    ];

    let platform = env::var("CI_PLATFORM").unwrap_or_else(|_| "github".to_string());
    let platform = match platform.as_str() {
        "github" => Platform::GitHub,
        "azure" => Platform::Azure,
        "gitlab" => Platform::GitLab,
        "bitbucket" => Platform::Bitbucket,
        _ => {
            eprintln!("Unknown platform: {}", platform);
            return Err(anyhow::anyhow!("Unknown platform").into());
        }
    };

    let annotation_service = AnnotationService::new_with_platform(platform)?;
    annotation_service.issue_annotations(annotations)?;

    Ok(())
}
