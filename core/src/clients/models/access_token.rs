use expunge::Expunge;
use nject::injectable;
use serde::{Deserialize, Serialize};

/// A trait for tokens with common operations.
trait AccessToken: Clone + Default + Serialize + for<'a> Deserialize<'a> + Expunge {
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
pub(crate) struct GitHubAccessToken(#[expunge(with = GitHubAccessToken::redact)] Option<String>);

#[allow(unused)]
impl AccessToken for GitHubAccessToken {
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
        GitHubAccessToken(token)
    }

    /// Gets the token from the concrete implementation.
    fn inner(&self) -> &Option<String> {
        &self.0
    }
}

#[injectable]
#[derive(Clone, Default, Serialize, Deserialize, Expunge)]
pub(crate) struct AzureAccessToken(#[expunge(with = AzureAccessToken::redact)] Option<String>);

#[allow(unused)]
impl AccessToken for AzureAccessToken {
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
        AzureAccessToken(token)
    }

    /// Gets the token from the concrete implementation.
    fn inner(&self) -> &Option<String> {
        &self.0
    }
}
