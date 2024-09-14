use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = concat!(env!("CARGO_PKG_NAME"), " CLI"),
    about = cli::about(),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    long_about = cli::long_about(),
    styles = cli::get_styles(),
)]
pub(crate) struct Cli {
    /// Path to the configuration file
    #[arg(short, long, value_name = "FILE")]
    // #[arg(short, long, value_name = "FILE", env = "RUSTPROOF_CONFIG", action = ArgAction::Set)]
    config: Option<PathBuf>,

    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run the Rustproof service
    #[command(long_about = cli::run_about())]
    Run {
        /// Port to run the service on
        #[arg(short, long, default_value = "8080")]
        port: u16,
    },

    /// Generate a default configuration file
    #[command(long_about = cli::init_about())]
    Init {
        /// Path to write the generated configuration file
        #[arg(short, long, value_name = "FILE", default_value = "rustproof.toml")]
        output: PathBuf,
    },

    /// Perform database migrations
    #[command(long_about = cli::migrate_about())]
    Migrate {
        /// Run migrations without applying them
        #[arg(long)]
        dry_run: bool,
    },

    /// Manage users
    #[command(long_about = cli::user_about())]
    User {
        #[command(subcommand)]
        action: UserCommands,
    },

    /// Interact with the auth microservice
    #[command(long_about = cli::auth_about())]
    Auth {
        #[command(subcommand)]
        action: AuthCommands,
    },
}

#[derive(Subcommand, Debug)]
enum UserCommands {
    /// Create a new user
    #[command(long_about = cli::user_create_about())]
    Create {
        /// Username for the new user
        username: String,
        /// Email for the new user
        email: String,
    },
    /// List all users
    #[command(long_about = cli::user_list_about())]
    List,
    /// Delete a user
    #[command(long_about = cli::user_delete_about())]
    Delete {
        /// Username of the user to delete
        username: String,
    },
}

#[derive(Subcommand, Debug)]
enum AuthCommands {
    /// Verify a user's credentials
    #[command(long_about = cli::auth_verify_about())]
    Verify {
        /// Username to verify
        username: String,
        /// Password to verify
        password: String,
    },
    /// Generate a new authentication token
    #[command(long_about = cli::auth_token_about())]
    Token {
        /// Username to generate token for
        username: String,
    },
}

mod cli {
    use anstyle::{AnsiColor, Color, Style};

    pub(crate) fn get_styles() -> clap::builder::Styles {
        clap::builder::Styles::styled()
            .usage(Style::new().bold().underline().fg_color(Some(Color::Ansi(AnsiColor::Yellow))))
            .header(Style::new().bold().underline().fg_color(Some(Color::Ansi(AnsiColor::Yellow))))
            .literal(Style::new().fg_color(Some(Color::Ansi(AnsiColor::Green))))
            .invalid(Style::new().bold().fg_color(Some(Color::Ansi(AnsiColor::Red))))
            .error(Style::new().bold().fg_color(Some(Color::Ansi(AnsiColor::Red))))
            .valid(Style::new().bold().underline().fg_color(Some(Color::Ansi(AnsiColor::Green))))
            .placeholder(Style::new().fg_color(Some(Color::Ansi(AnsiColor::White))))
    }

    pub(crate) fn long_about() -> String {
        let mut about = String::new();
        about.push_str("Rustproof is a robust user registration and authentication API. ");
        about.push_str(&format!("You can configure {} using a configuration file (default: ", env!("CARGO_PKG_NAME")));
        let config_style = Style::new().fg_color(Some(Color::Ansi(AnsiColor::Green)));
        about.push_str(&format!("{config_style}rustproof.toml{config_style:#}"));
        about.push_str(") or by setting environment variables prefixed with ");
        let env_style = Style::new().fg_color(Some(Color::Ansi(AnsiColor::Yellow)));
        about.push_str(&format!("{env_style}RUSTPROOF_{env_style:#}"));
        about.push_str(".\nIf both are provided, environment variables take precedence.");
        about.push_str("\n\nUse the 'run' command to start the service, 'migrate' to manage database schemas,");
        about.push_str("\n'user' to manage users, and 'auth' to interact with the authentication microservice.");
        about.push_str("\n\nFor more information on a specific command, use the --help flag after the command name.");
        about
    }

    pub(crate) fn about() -> String {
        let title_style = Style::new().underline().bold().fg_color(Some(Color::Ansi(AnsiColor::BrightYellow)));
        let description_style = Style::new().fg_color(Some(Color::Ansi(AnsiColor::Green)));
        format!(
            "🛡️ {title_style}{title}{title_style:#} 🛡️\n{description_style}{description}{description_style:#} 🔐",
            title_style = title_style,
            title = env!("CARGO_PKG_NAME").to_uppercase(),
            description_style = description_style,
            description = env!("CARGO_PKG_DESCRIPTION"),
        )
    }

    pub(crate) fn run_about() -> &'static str {
        "Start the Rustproof service and listen for incoming connections. \
        This command initializes the database connection, sets up the HTTP server, \
        and begins processing authentication requests."
    }

    pub(crate) fn init_about() -> &'static str {
        "Create a new configuration file with default settings. \
        This file can be used as a starting point for customizing your Rustproof setup. \
        You can specify the output location or use the default 'rustproof.toml'."
    }

    pub(crate) fn migrate_about() -> &'static str {
        "Manage database schema migrations. This command allows you to \
        update your database structure to the latest version or rollback changes. \
        Use the --dry-run flag to preview changes without applying them."
    }

    pub(crate) fn user_about() -> &'static str {
        "Perform user management operations such as creating new users, \
        listing existing users, or deleting user accounts."
    }

    pub(crate) fn auth_about() -> &'static str {
        "Communicate with the authentication microservice to perform \
        operations such as verifying user credentials or generating authentication tokens."
    }

    pub(crate) fn user_create_about() -> &'static str {
        "Register a new user in the system. This command requires \
        a username and an email address for the new account."
    }

    pub(crate) fn user_list_about() -> &'static str {
        "Display a list of all registered users in the system. \
        This command provides an overview of user accounts."
    }

    pub(crate) fn user_delete_about() -> &'static str {
        "Remove a user account from the system. This operation is irreversible, \
        so use it with caution. You need to provide the username of the account to be deleted."
    }

    pub(crate) fn auth_verify_about() -> &'static str {
        "Check if the provided username and password combination is valid. \
        This command communicates with the authentication microservice to verify credentials."
    }

    pub(crate) fn auth_token_about() -> &'static str {
        "Create a new authentication token for a given user. \
        This token can be used for subsequent authenticated requests to the service."
    }
}