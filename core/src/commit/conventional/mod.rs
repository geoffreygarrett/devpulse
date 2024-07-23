use std::fmt;

pub(crate) mod template;

/// Represents the types of commits recognized by the Conventional Commits 1.0.0-beta.2 specification.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
enum CommitType {
    Feat,
    Fix,
    Chore,
    Docs,
    Style,
    Refactor,
    Perf,
    Test,
    Build,
    Ops,
    Improvement,
}

impl CommitType {
    /// Retrieves the string representation of the commit type.
    fn as_str(&self) -> &'static str {
        match self {
            CommitType::Feat => "feat",
            CommitType::Fix => "fix",
            CommitType::Chore => "chore",
            CommitType::Docs => "docs",
            CommitType::Style => "style",
            CommitType::Refactor => "refactor",
            CommitType::Perf => "perf",
            CommitType::Test => "test",
            CommitType::Build => "build",
            CommitType::Ops => "ops",
            CommitType::Improvement => "improvement",
        }
    }

    /// Returns the corresponding emoji for each commit type.
    fn emoji(&self) -> Emoji {
        match self {
            CommitType::Feat => Emoji::Feat,
            CommitType::Fix => Emoji::Fix,
            CommitType::Docs => Emoji::Docs,
            CommitType::Style => Emoji::Style,
            CommitType::Refactor => Emoji::Refactor,
            CommitType::Perf => Emoji::Perf,
            CommitType::Test => Emoji::Test,
            CommitType::Build => Emoji::Build,
            CommitType::Chore => Emoji::Chore,
            // CommitType::Revert => Emoji::Revert,
            CommitType::Improvement => Emoji::Feat,
            CommitType::Ops => Emoji::Ci,
        }
    }
}

/// Represents a conventional commit, adhering to the Conventional Commits 1.0.0-beta.2 specification.
/// This structure encapsulates all possible elements of a conventional commit message.
#[derive(Debug, serde::Serialize)]
struct Commit {
    /// The type of commit, indicating the nature of the changes.
    commit_type: CommitType,
    /// Optional scope providing additional context on the area of the codebase affected.
    scope: Option<String>,
    /// A concise description of the changes, following the type and scope.
    description: String,
    /// An optional detailed description of the changes, providing additional context.
    body: Option<String>,
    /// An optional footer containing additional metadata, such as issue references or breaking changes.
    footer: Option<String>,
    /// Indicates if the commit introduces breaking changes, correlating with MAJOR in semantic versioning.
    breaking_change: Option<String>,
    /// Optional bool for emoji
    use_emoji: bool,
}

impl Commit {
    fn new(
        commit_type: CommitType,
        scope: Option<String>,
        description: String,
        body: Option<String>,
        footer: Option<String>,
        breaking_change: Option<String>,
        use_emoji: bool, // New parameter to specify emoji usage
    ) -> Self {
        Commit {
            commit_type,
            scope,
            description,
            body,
            footer,
            breaking_change,
            use_emoji,
        }
    }
}

/// Represents emoji symbols associated with different types of conventional commits.
#[derive(Debug, Clone, Copy)]
enum Emoji {
    Feat,
    Fix,
    Docs,
    Style,
    Refactor,
    Perf,
    Test,
    Build,
    Ci,
    Chore,
    Revert,
}

impl Emoji {
    /// Retrieves the character representation of the emoji.
    fn as_char(self) -> char {
        match self {
            Emoji::Feat => '✨',
            Emoji::Fix => '🐛',
            Emoji::Docs => '📚',
            Emoji::Style => '💎',
            Emoji::Refactor => '📦',
            Emoji::Perf => '🚀',
            Emoji::Test => '🚨',
            Emoji::Build => '🛠',
            Emoji::Ci => '🔧',
            Emoji::Chore => '🧹',
            Emoji::Revert => '🗑',
        }
    }
}

impl fmt::Display for Commit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut handlebars = template::setup_templates();
        handlebars.register_escape_fn(handlebars::no_escape);
        let commit_message = template::render_commit_message(self, &handlebars);
        write!(f, "{}", commit_message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests a simple feature commit without any optional fields.
    #[test]
    fn test_feature_commit() {
        let commit = Commit::new(
            CommitType::Feat,
            None,
            "Add new user login API".to_string(),
            None,
            None,
            None,
            false, // Disable emoji
        );
        assert_eq!(commit.to_string(), "feat: Add new user login API");
    }

    /// Tests a bug fix commit with a scope and body.
    #[test]
    fn test_bug_fix_commit() {
        let commit = Commit::new(
            CommitType::Fix,
            Some("auth".to_string()),
            "Fix CSRF vulnerability".to_string(),
            Some("Adds token validation on user session".to_string()),
            None,
            None,
            false, // Disable emoji
        );
        let expected_message = "\
fix(auth): Fix CSRF vulnerability
Adds token validation on user session";
        assert_eq!(commit.to_string(), expected_message);
    }

    /// Tests a commit that introduces a breaking change.
    #[test]
    fn test_breaking_change_commit() {
        let commit = Commit::new(
            CommitType::Refactor,
            Some("database".to_string()),
            "Change database schema for users".to_string(),
            None,
            Some("Related to issue #123".to_string()),
            Some("Change column 'name' to 'username'".to_string()),
            false, // Disable emoji
        );
        let expected_message = "\
refactor(database): Change database schema for users
BREAKING CHANGE: Change column 'name' to 'username'
Related to issue #123";
        assert_eq!(commit.to_string(), expected_message);
    }

    /// Tests a commit with all fields utilized.
    #[test]
    fn test_full_commit() {
        let commit = Commit::new(
            CommitType::Perf,
            Some("rendering".to_string()),
            "Improve shadow rendering performance".to_string(),
            Some("Optimizes shadow rendering for mobile devices".to_string()),
            Some("See PR #456 for more details".to_string()),
            None,
            false, // Disable emoji
        );
        let expected_message = "\
perf(rendering): Improve shadow rendering performance
Optimizes shadow rendering for mobile devices
See PR #456 for more details";
        assert_eq!(commit.to_string(), expected_message);
    }
}
