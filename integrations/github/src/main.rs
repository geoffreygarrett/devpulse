// use actions_toolkit::{core, github};
// use octocrab::{models::checks::{CheckRunOutput, CheckStatus, Conclusion, Annotation}, Octocrab};
// use tokio;
// use chrono::Utc;
//
// #[tokio::main]
// async fn main() {
//     if let Err(e) = run().await {
//         core::set_failed(e.to_string().as_str());
//     }
// }
//
fn main() {
    println!("Hello, world!");
}
// async fn run() -> Result<(), Box<dyn std::error::Error>> {
//     let token = core::get_input("repo-token", Some(true))?;
//     let octocrab = Octocrab::builder().personal_token(token).build()?;
//     let context = github::context()?;
//     let repo = context.repo();
//     let sha = context.sha();
//
//     let check_run = octocrab.checks().create(
//         &repo.owner,
//         &repo.repo,
//         "Repository Analysis Check",
//         &sha,
//         octocrab::models::checks::CheckRunCreate::builder()
//             .started_at(Utc::now())
//             .status(CheckStatus::InProgress)
//             .build(),
//     ).await?;
//
//     // Perform repository analysis
//     let output = CheckRunOutput {
//         title: "Repository Analysis Completed".to_string(),
//         summary: "All checks passed".to_string(),
//         text: Some("Detailed message".to_string()),
//         annotations: Some(vec![
//             Annotation {
//                 path: "./dummyfile.txt".to_string(),
//                 start_line: 1,
//                 end_line: 1,
//                 annotation_level: octocrab::models::checks::AnnotationLevel::Failure,
//                 message: "missing value".to_string(),
//                 title: Some("Annotation Title".to_string()),
//                 start_column: None,
//                 end_column: None,
//                 raw_details: None,
//             }
//         ]),
//     };
//
//     let _ = octocrab.checks().update(
//         &repo.owner,
//         &repo.repo,
//         check_run.id,
//         octocrab::models::checks::CheckRunUpdate::builder()
//             .completed_at(Some(Utc::now()))
//             .status(Some(CheckStatus::Completed))
//             .conclusion(Some(Conclusion::Success))
//             .output(Some(output))
//             .build(),
//     ).await?;
//
//     Ok(())
// }
