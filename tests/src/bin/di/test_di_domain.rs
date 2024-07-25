use std::sync::Arc;

use nject::{injectable, provider};
use reqwest::Client;

#[injectable]
struct GitHubToken(Option<String>);

#[injectable]
struct AzureToken(Option<String>);

#[injectable]
struct GitHubClient<'a> {
    client: &'a Client,
    token: &'a GitHubToken,
}

impl GitHubClient<'_> {
    #[allow(unused)]
    fn new<'b>(client: &'b Arc<Client>, token: &'b GitHubToken) -> GitHubClient<'b> {
        GitHubClient {
            client,
            token,
        }
    }
}

impl GitHubClient<'_> {
    fn get_token(&self) -> Option<&str> {
        self.token.0.as_deref()
    }

    fn get_client(&self) -> &Client {
        self.client
    }
}

#[injectable]
struct AzureClient<'a> {
    client: &'a Client,
    token: &'a AzureToken,
}

impl AzureClient<'_> {
    fn get_token(&self) -> Option<&str> {
        self.token.0.as_deref()
    }

    fn get_client(&self) -> &Client {
        self.client
    }
}

// Define the provider for App
#[provider]
struct AppProvider {
    #[provide]
    client: Client,
    #[provide]
    github_token: GitHubToken,
    #[provide]
    azure_token: AzureToken,
}

fn main() {
    // Initialize the provider
    let provider = AppProvider {
        client: Client::new(),
        github_token: GitHubToken(Some("github_token_value".to_string())),
        azure_token: AzureToken(Some("azure_token_value".to_string())),
    };

    // Use the provider to create instances of the services
    let github_client: GitHubClient = provider.provide();
    let azure_client: AzureClient = provider.provide();

    println!("GitHub Token: {}", github_client.get_token().unwrap());
    println!("Azure Token: {}", azure_client.get_token().unwrap());

    println!("Services have been initialized and actions have been performed.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_github_client() {
        let provider = AppProvider {
            client: Client::new(),
            github_token: GitHubToken(Some("github_test_token".to_string())),
            azure_token: AzureToken(Some("azure_test_token".to_string())),
        };

        let github_client: GitHubClient = provider.provide();

        assert_eq!(github_client.get_token(), Some("github_test_token"));
        assert_eq!(github_client.get_client() as *const _, &provider.client as *const _);
    }

    #[test]
    fn test_azure_client() {
        let provider = AppProvider {
            client: Client::new(),
            github_token: GitHubToken(Some("github_test_token".to_string())),
            azure_token: AzureToken(Some("azure_test_token".to_string())),
        };

        let azure_client: AzureClient = provider.provide();

        assert_eq!(azure_client.get_token(), Some("azure_test_token"));
        assert_eq!(azure_client.get_client() as *const _, &provider.client as *const _);
    }

    #[test]
    fn test_shared_client() {
        let provider = AppProvider {
            client: Client::new(),
            github_token: GitHubToken(Some("github_test_token".to_string())),
            azure_token: AzureToken(Some("azure_test_token".to_string())),
        };

        let github_client: GitHubClient = provider.provide();
        let azure_client: AzureClient = provider.provide();

        assert_eq!(github_client.get_client() as *const _, azure_client.get_client() as *const _);
    }
}
