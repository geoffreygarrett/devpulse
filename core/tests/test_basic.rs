use devpulse_core::services::{analyze_commit_range_service};


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_basic() {
        let url = "https://github.com/tudat-team/tudatpy";
        let old_commit = "469fddb7d66f06981596c051cf7cca2681e55f55";  // Earlier commit
        let new_commit = "60719404b37c8cfb747e9d4cd7ef6f865b148acf";  // Later commit

        assert!(analyze_commit_range_service(url, old_commit, new_commit).await.is_ok());
    }
}
