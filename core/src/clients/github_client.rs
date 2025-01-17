use nject::injectable;

use external_github;

use crate::clients::models::access_token::GitHubAccessToken;
use crate::clients::models::arc_client::ArcClient;

#[injectable]
struct GithubClient {
    client: ArcClient,
    access_token: GitHubAccessToken,
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_github_client() {
        let config = external_github::apis::configuration::Configuration::builder()
            .base_path("https://api.github.com".to_string())
            .user_agent("OpenAPI-Generator/1.1.4/rust".to_string())
            .client(reqwest::Client::new())
            .build()
            .unwrap();

        let params = external_github::apis::repos_api::ReposSlashGetCommitParams::builder()
            .owner("geoffreygarrett".to_string())
            .repo("devpulse".to_string())
            .r#ref("1e14522488cf65e0e7e9142fae7a8a395414b424".to_string())
            .build()
            .unwrap();

        let result = external_github::apis::repos_api::repos_slash_get_commit(&config, params)
            .await
            .expect("TODO: panic message");

        println!("Result: {:#?}", result);
        // let github_client = GithubClient {
        //     client: ArcClient::new(),
        //     access_token: GitHubAccessToken::new("github_token_value".to_string()),
        // };

        // println!("GitHub Token: {}", github_client.access_token.0);
        // println!("Client: {:?}", github_client.client);
    }

    #[tokio::test]
    async fn test_github_client_2() {
        use external_github::{
            apis::configuration::Configuration, apis::repos_api::ReposSlashGetCommitParams,
            apis::GithubClient,
        };
        let config = Configuration::builder()
            .base_path("https://api.github.com".to_string())
            .user_agent("OpenAPI-Generator/1.1.4/rust".to_string())
            .client(reqwest::Client::new())
            .build()
            .unwrap();

        let params = external_github::apis::repos_api::ReposSlashGetCommitParams::builder()
            .owner("geoffreygarrett".to_string())
            .repo("devpulse".to_string())
            .r#ref("1e14522488cf65e0e7e9142fae7a8a395414b424".to_string())
            .build()
            .unwrap();

        let result = GithubClient::new(config)
            .repos_slash_get_commit(params)
            .await
            .expect("TODO: panic message");

        println!("Result: {:#?}", result);
    }
}
