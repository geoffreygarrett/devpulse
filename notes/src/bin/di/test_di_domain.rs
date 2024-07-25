use std::sync::Arc;

use expunge::Expunge;
use nject::{injectable, provider};
use reqwest::Client;
use serde::{Deserialize, Serialize};

/// A trait for tokens with common operations.
trait Token: Clone + Default + Serialize + for<'a> Deserialize<'a> + Expunge {
    /// List of environment variables to check for the token.
    const ENV_VARS: &'static [&'static str];

    /// Creates a token instance from an option.
    fn from_option(token: Option<String>) -> Self;

    /// Gets the token from the concrete implementation.
    fn inner(&self) -> &Option<String>;

    /// Creates an `AzureToken` instance from the first available environment variable.
    /// If none of the environment variables are set, it defaults to an empty string.
    fn from_env() -> Self {
        // Iterate through the list of environment variables and return the first one that's set.
        let token = Self::ENV_VARS
            .iter()
            .find_map(|&var| std::env::var(var).ok())
            .or_else(|| Some(String::new())); // Default to empty string if no variable is set

        Self::from_option(token)
    }

    /// Returns the token as a reference, if present.
    fn as_ref(&self) -> Option<&str> {
        self.inner().as_deref()
    }

    /// Redacts sensitive information.
    fn redact(s: Option<String>) -> Option<String> {
        s.as_ref().map(|a| a.chars().map(|_| '*').collect())
    }
}

#[injectable]
#[derive(Clone, Default, Serialize, Deserialize, Expunge)]
struct GitHubToken(#[expunge(with = GitHubToken::redact)] Option<String>);

#[allow(unused)]
impl Token for GitHubToken {
    /// List of environment variables to check for the GitHub token.
    const ENV_VARS: &'static [&'static str] = &[
        "DEVPULSE_GITHUB_TOKEN",
        "GITHUB_TOKEN",
        "GH_TOKEN",
        "GITHUB_PAT",
        "GH_PAT",
    ];

    /// Creates a token instance from an option.
    fn from_option(token: Option<String>) -> Self {
        GitHubToken(token)
    }

    /// Gets the token from the concrete implementation.
    fn inner(&self) -> &Option<String> {
        &self.0
    }
}

#[injectable]
#[derive(Clone, Default, Serialize, Deserialize, Expunge)]
struct AzureToken(#[expunge(with = AzureToken::redact)] Option<String>);

#[allow(unused)]
impl Token for AzureToken {
    /// List of environment variables to check for the Azure token.
    const ENV_VARS: &'static [&'static str] = &[
        "DEVPULSE_AZURE_TOKEN",
        "AZURE_TOKEN",
        "AZURE_PAT",
        "AZURE_ACCESS_TOKEN",
        "AZURE_CLIENT_SECRET",
    ];

    /// Creates a token instance from an option.
    fn from_option(token: Option<String>) -> Self {
        AzureToken(token)
    }

    /// Gets the token from the concrete implementation.
    fn inner(&self) -> &Option<String> {
        &self.0
    }
}

#[injectable]
struct GitHubClient<'a> {
    client: &'a Client,
    token: &'a GitHubToken,
}

impl GitHubClient<'_> {
    #[allow(unused)]
    fn new<'b>(client: &'b Arc<Client>, token: &'b GitHubToken) -> GitHubClient<'b> {
        GitHubClient { client, token }
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
    fn test_github_token_redaction() {
        // Arrange: Create a GitHubToken instance with a specific token
        let token = GitHubToken(Some("secret_github_token".to_string()));

        // Act: Expunge the token
        let expunged_token = token.expunge();

        // Serialize the expunged token to JSON
        let json = serde_json::to_string(&expunged_token).expect("should serialize");

        // Assert: Check that the JSON output matches the expected redacted format
        // Here, we compare the length of the redacted token to the original token length
        let expected_redacted = "*******************"; // Length should match the length of the original token
        assert_eq!(json, format!("\"{}\"", expected_redacted));
    }

    #[test]
    fn test_azure_token_redaction() {
        // Arrange: Create an AzureToken instance with a specific token
        let token = AzureToken(Some("secret_azure_token".to_string()));

        // Act: Expunge the token
        let expunged_token = token.expunge();

        // Serialize the expunged token to JSON
        let json = serde_json::to_string(&expunged_token).expect("should serialize");

        // Assert: Check that the JSON output matches the expected redacted format
        let expected_redacted = "******************"; // Length should match the length of the original token
        assert_eq!(json, format!("\"{}\"", expected_redacted));
    }

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
