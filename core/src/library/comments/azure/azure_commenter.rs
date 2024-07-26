struct AzureDevOpsCommenter {
    api_token: String,
    organization: String,
    project: String,
    repository: String,
}

#[async_trait]
impl Commenter for AzureDevOpsCommenter {
    async fn post_comment(&self, pr_id: &str, comment: Comment) -> Result<()> {
        if comment.file.is_some() && comment.line.is_some() {
            self.post_inline_comment(pr_id, comment).await
        } else {
            self.post_general_comment(pr_id, &comment.message).await
        }
    }

    async fn post_inline_comment(&self, pr_id: &str, comment: Comment) -> Result<()> {
        let url = format!(
            "https://dev.azure.com/{}/{}/_apis/git/repositories/{}/pullRequests/{}/threads",
            self.organization, self.project, self.repository, pr_id
        );

        let payload = serde_json::json!({
            "comments": [{
                "parentCommentId": 0,
                "content": comment.message,
                "commentType": 1
            }],
            "status": 1,
            "threadContext": {
                "filePath": comment.file.unwrap(),
                "rightFileStart": { "line": comment.line.unwrap(), "offset": 1 },
                "rightFileEnd": { "line": comment.end_line.unwrap_or(comment.line.unwrap()), "offset": 1 }
            }
        });

        let client = Client::new();
        let response = client
            .post(&url)
            .bearer_auth(&self.api_token)
            .json(&payload)
            .header("Content-Type", "application/json")
            .send()
            .await;

        map_response(response).await
    }

    async fn post_general_comment(&self, pr_id: &str, message: &str) -> Result<()> {
        let url = format!(
            "https://dev.azure.com/{}/{}/_apis/git/repositories/{}/pullRequests/{}/threads",
            self.organization, self.project, self.repository, pr_id
        );

        let payload = serde_json::json!({
            "comments": [{
                "parentCommentId": 0,
                "content": message,
                "commentType": 1
            }],
            "status": 1
        });

        let client = Client::new();
        let response = client
            .post(&url)
            .bearer_auth(&self.api_token)
            .json(&payload)
            .header("Content-Type", "application/json")
            .send()
            .await;

        map_response(response).await
    }
}

async fn map_response(response: Result<reqwest::Response, ReqwestError>) -> Result<()> {
    match response {
        Ok(res) if res.status().is_success() => Ok(()),
        Ok(res) => Err(CommenterError::Other(format!("HTTP Error: {}", res.status()))),
        Err(e) => Err(CommenterError::NetworkError(e.to_string())),
    }
}
