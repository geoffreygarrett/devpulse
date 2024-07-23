use std::path::Path;
use std::process::Command;

use git2::{Error, Repository};
use url::Url;

/// Manages cloning or opening a Git repository.
pub struct RepositoryManager {
    repository_url: String,
    local_path: String,
}

impl RepositoryManager {
    /// Creates a new `RepositoryManager` instance.
    ///
    /// # Arguments
    ///
    /// * `repository_url` - The URL of the repository to manage.
    ///
    /// # Returns
    ///
    /// A `RepositoryManager` instance.
    ///
    /// # Errors
    ///
    /// Returns an error if the URL cannot be parsed.
    pub fn new(repository_url: &str) -> Result<Self, url::ParseError> {
        let local_path = Self::url_to_directory_name(repository_url)?;
        Ok(Self {
            repository_url: repository_url.to_string(),
            local_path,
        })
    }

    /// Converts a URL into a directory-friendly name.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to convert.
    ///
    /// # Returns
    ///
    /// A directory-friendly name.
    ///
    /// # Errors
    ///
    /// Returns an error if the URL cannot be parsed.
    fn url_to_directory_name(url: &str) -> Result<String, url::ParseError> {
        let parsed_url = Url::parse(url)?;
        let host = parsed_url.host_str().unwrap_or_default().replace(".", "_");
        let path = parsed_url.path().replace("/", "_");
        Ok(format!("./.cache/{}_{}", host, path.trim_matches('_')))
    }

    /// Opens an existing repository or clones it if it doesn't exist.
    ///
    /// # Returns
    ///
    /// A `Repository` instance.
    ///
    /// # Errors
    ///
    /// Returns an error if the repository cannot be opened or cloned.
    pub async fn open_or_clone(&self) -> Result<Repository, Error> {
        let local_path = Path::new(&self.local_path);
        if local_path.exists() {
            Repository::open(local_path)
        } else {
            let repository_url = self.repository_url.clone();
            let local_path = self.local_path.clone();
            Repository::clone(&repository_url, &local_path)
        }
    }

    /// Fetches updates from the remote repository.
    ///
    /// # Returns
    ///
    /// A result indicating success or failure.
    ///
    /// # Errors
    ///
    /// Returns an error if the fetch operation fails.
    pub async fn fetch_updates(&self) -> Result<(), Error> {
        let local_path = Path::new(&self.local_path);
        if local_path.exists() {
            let output = Command::new("git")
                .arg("-C")
                .arg(&self.local_path)
                .arg("fetch")
                .output()
                .expect("Failed to execute git fetch command");

            if output.status.success() {
                Ok(())
            } else {
                Err(Error::from_str("Failed to fetch updates from remote repository"))
            }
        } else {
            Err(Error::from_str("Repository does not exist locally"))
        }
    }

    /// Get local path
    ///
    /// # Returns
    ///
    /// A string containing the local path of the repository
    ///
    /// # Errors
    ///
    /// Returns an error if the local path is not a valid string
    pub fn get_local_path(&self) -> String {
        self.local_path.clone()
    }
}
