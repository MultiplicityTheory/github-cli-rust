mod iostreams;
mod config;
mod factory;
mod api;
mod cmd;
mod git;
mod utils;

use clap::Parser;
use factory::Factory;
use std::sync::Arc;
use crate::utils::telemetry::Telemetry;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(name = "gh-rs")]
#[command(bin_name = "gh-rs")]
#[command(version = VERSION)]
#[command(about = "GitHub CLI in Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Login to GitHub
    Auth {
        #[command(subcommand)]
        subcommand: AuthCommands,
    },
    /// Manage repositories
    Repo {
        #[command(subcommand)]
        subcommand: RepoCommands,
    },
    /// Manage issues
    Issue {
        #[command(subcommand)]
        subcommand: IssueCommands,
    },
    /// Manage pull requests
    Pr {
        #[command(subcommand)]
        subcommand: PrCommands,
    },
}

#[derive(clap::Subcommand)]
enum AuthCommands {
    /// Log in to a GitHub host
    Login,
    /// Log out of a GitHub host
    Logout,
    /// View authentication status
    Status,
}

#[derive(clap::Subcommand)]
enum RepoCommands {
    /// View a repository
    View {
        /// The repository to view [owner/repo]
        repository: Option<String>,
    },
    /// Clone a repository
    Clone {
        /// The repository to clone [owner/repo or URL]
        repository: String,
    },
}

#[derive(clap::Subcommand)]
enum IssueCommands {
    /// List issues in a repository
    List {
        /// The repository to list issues from [owner/repo]
        repository: Option<String>,
    },
    /// View an issue
    View {
        /// The issue number to view
        number: Option<u32>,
        /// The repository [owner/repo]
        #[arg(short, long)]
        repository: Option<String>,
    },
}

#[derive(clap::Subcommand)]
enum PrCommands {
    /// List pull requests in a repository
    List {
        /// The repository to list PRs from [owner/repo]
        repository: Option<String>,
    },
    /// View a pull request
    View {
        /// The PR number to view
        number: Option<u32>,
        /// The repository [owner/repo]
        #[arg(short, long)]
        repository: Option<String>,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let factory = Arc::new(Factory::new()?);
    let telemetry = Telemetry::new(factory.config.clone(), VERSION);

    // Update check in the background
    let update_config = factory.config.clone();
    let update_handle = tokio::spawn(async move {
        utils::update::check_for_update(update_config, VERSION).await
    });

    match &cli.command {
        Some(Commands::Auth { subcommand }) => {
            telemetry.log_command("auth").await;
            match subcommand {
                AuthCommands::Login => {
                    cmd::auth::login(factory).await?;
                }
                AuthCommands::Logout => {
                    println!("Logout not implemented yet");
                }
                AuthCommands::Status => {
                    cmd::auth::status(factory).await?;
                }
            }
        },
        Some(Commands::Repo { subcommand }) => {
            telemetry.log_command("repo").await;
            match subcommand {
                RepoCommands::View { repository } => {
                    cmd::repo::view(factory, repository.clone()).await?;
                }
                RepoCommands::Clone { repository } => {
                    cmd::repo::clone(factory, repository.clone()).await?;
                }
            }
        },
        Some(Commands::Issue { subcommand }) => {
            telemetry.log_command("issue").await;
            match subcommand {
                IssueCommands::List { repository } => {
                    cmd::issue::list(factory, repository.clone()).await?;
                }
                IssueCommands::View { number, repository } => {
                    cmd::issue::view(factory, repository.clone(), *number).await?;
                }
            }
        },
        Some(Commands::Pr { subcommand }) => {
            telemetry.log_command("pr").await;
            match subcommand {
                PrCommands::List { repository } => {
                    cmd::pr::list(factory, repository.clone()).await?;
                }
                PrCommands::View { number, repository } => {
                    cmd::pr::view(factory, repository.clone(), *number).await?;
                }
            }
        },
        None => {
            println!("Welcome to gh-rs! Use --help to see available commands.");
        }
    }

    // Report update if found
    if let Ok(Some(new_version)) = update_handle.await {
        eprintln!("\n\x1b[33mA new release of gh-rs is available: {} → {}\x1b[0", VERSION, new_version);
        eprintln!("https://github.com/cli/cli/releases/latest\n");
    }

    Ok(())
}
