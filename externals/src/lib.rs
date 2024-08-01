#![allow(non_snake_case)]

/// Configures the conditional compilation for OpenAI API bindings.
///
/// This module is only included if the "use_openai" feature is enabled in your Cargo.toml.
/// It re-exports the contents of `external_openai` for convenience.
#[cfg(feature = "use_openai")]
pub mod external_openai {
    pub use external_openai::*;
}

/// Configures the conditional compilation for GitHub API bindings.
///
/// This module is only included if the "use_github" feature is enabled. It allows
/// easy access to GitHub-specific functionality encapsulated in the `external_github` module,
/// simplifying the import paths in the consumer code.
#[cfg(feature = "use_github")]
pub mod external_github {
    pub use external_github::*;
}

/// Configures the conditional compilation for Azure DevOps API bindings.
///
/// When the "use_azure" feature is enabled, this module becomes available. It primarily
/// re-exports all public members of `external_azure`, ensuring that these can be
/// accessed directly through this module.
#[cfg(feature = "use_azure")]
pub mod external_azure {
    pub use external_azure::*;
}
