use std::env;

use anyhow::Result;

use devpulse_core::annotations::prelude::*;

fn main() -> Result<()> {
    let annotations = vec![
        // Notice annotation
        Annotation::builder()
            .file("tests/src/main.rs".to_string())
            .line(Some(10))
            .end_line(Some(10))
            .col(Some(5))
            .end_col(Some(30))
            .message("Consider refactoring this function to improve readability.".to_string())
            .level(AnnotationLevel::Notice)
            .build()
            .unwrap(),
        // Warning annotation
        Annotation::builder()
            .file("tests/src/main.rs".to_string())
            .line(Some(15))
            .end_line(Some(15))
            .col(Some(5))
            .end_col(Some(30))
            .message("Ensure the command format is correct.".to_string())
            .level(AnnotationLevel::Warning)
            .build()
            .unwrap(),
        // Error annotation (commented out)
        Annotation::builder()
            .file("tests/src/main.rs".to_string())
            .line(Some(20))
            .end_line(Some(20))
            .col(Some(5))
            .end_col(Some(30))
            .message("This function implementation is incorrect.".to_string())
            .level(AnnotationLevel::Error)
            .build()
            .unwrap(),
        // Debug annotation
        Annotation::builder()
            .file("tests/src/main.rs".to_string())
            .line(Some(25))
            .end_line(Some(25))
            .col(Some(5))
            .end_col(Some(30))
            .message("Debugging this line for performance.".to_string())
            .level(AnnotationLevel::Debug)
            .build()
            .unwrap(),
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
