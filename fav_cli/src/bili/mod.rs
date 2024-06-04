//! The CLI for [bilibili](https://www.bilibili.com)

mod action;

use action::*;
use clap::{error::ErrorKind, CommandFactory as _, Parser, Subcommand, ValueHint};
use fav_core::{local::ProtoLocal as _, FavCoreResult};
use fav_utils::bili::BiliSets;
use tracing::{error, info};

const VERSION: &str = const_format::formatcp!(
    "{} {}\nRUSTC: {} {} {}",
    match option_env!("VERGEN_GIT_DESCRIBE") {
        Some(var) => var,
        _ => concat!(env!("CARGO_PKG_VERSION"), "(CARGO_PKG_VERSION)"),
    },
    match option_env!("VERGEN_GIT_COMMIT_TIMESTAMP") {
        Some(var) => var,
        _ => "unknown-timestamp",
    },
    env!("VERGEN_RUSTC_HOST_TRIPLE"),
    env!("VERGEN_RUSTC_CHANNEL"),
    env!("VERGEN_RUSTC_SEMVER")
);

/// The main CLI entry point.
#[derive(Parser)]
#[command(author, version = VERSION, about)]
pub struct Cli {
    #[clap(subcommand)]
    subcmd: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize the folder for fav
    Init,
    /// Login your account
    Auth {
        /// Login method
        #[clap(subcommand)]
        subcmd: AuthCommand,
    },
    /// Fetch from remote
    Fetch,
    /// Show status of local, default to show resource status
    Status {
        /// Show resource status
        #[arg(value_hint = ValueHint::Other)]
        id: Option<String>,
        /// Show all sets(lists) status
        #[arg(long, short)]
        sets: bool,
        /// Show all resource(video) status
        #[arg(long, short)]
        res: bool,
        /// Show tracked only
        #[arg(long, short)]
        track: bool,
    },
    /// Track a remote source
    Track {
        /// The id of the source to track
        #[arg(value_hint = ValueHint::Other)]
        id: Vec<String>,
    },
    /// Untrack a remote source
    Untrack {
        /// The id of the source to untrack
        #[arg(value_hint = ValueHint::Other)]
        id: Vec<String>,
    },
    /// Pull remote resource to local.  If no id provided, then pull all and skip those having been saved
    Pull {
        /// Optional. The id of the sources to pull forcely or the sets to pull normally
        #[arg(value_hint = ValueHint::Other)]
        id: Option<Vec<String>>,
    },
    /// Interval fetch and pull
    Daemon {
        /// The interval to fetch and pull (in minutes, greater than 15)
        #[arg(value_hint = ValueHint::Other)]
        interval: u64,
    },
    /// Completions for the shell
    Completion {
        /// The shell to generate completions for
        #[arg(value_enum)]
        shell: clap_complete::Shell,
    },
}

#[derive(Subcommand)]
enum AuthCommand {
    /// Login with password
    Login,
    /// Login with QR code
    Logout,
    /// Reuse the login info
    Reuse {
        /// The path of .fav folder, example: /path/to/.fav
        #[arg(value_hint = ValueHint::DirPath)]
        path: std::path::PathBuf,
    },
}

impl Cli {
    /// Run the CLI.
    pub async fn run() -> FavCoreResult<()> {
        let args = Self::parse();
        match args.subcmd {
            Commands::Init => init()?,
            Commands::Auth {
                subcmd: AuthCommand::Logout,
            } => logout().await?,
            Commands::Completion { shell } => {
                let mut cmd = Cli::command();
                clap_complete::generate(shell, &mut cmd, "fav", &mut std::io::stdout());
            }
            subcmd => {
                let mut sets = BiliSets::read()?;
                let res = match subcmd {
                    Commands::Auth { subcmd: authcmd } => {
                        match authcmd {
                            AuthCommand::Login => login().await?,
                            AuthCommand::Reuse { path } => {
                                std::fs::hard_link(path.join("bili"), ".fav/bili")?;
                                info!("Reuse the login info from {}", path.display());
                            }
                            _ => unreachable!(),
                        }
                        fetch(&mut sets).await
                    }
                    Commands::Status {
                        id,
                        sets: show_sets,
                        res: show_res,
                        track: only_track,
                    } => match id {
                        Some(id) => {
                            if show_sets | show_res | only_track {
                                Cli::command()
                                    .error(
                                        ErrorKind::ArgumentConflict,
                                        "The id to 'fav status' does not take -s, -r, -t, options.",
                                    )
                                    .exit();
                            }
                            status(&mut sets, id)
                        }
                        None => match (show_sets, show_res) {
                            (false, false) => status_all(&mut sets, true, false, only_track),
                            _ => status_all(&mut sets, show_sets, show_res, only_track),
                        },
                    },
                    Commands::Fetch => fetch(&mut sets).await,
                    Commands::Track { id } => {
                        for id in id {
                            if let Err(e) = track(&mut sets, id) {
                                error!("{e}");
                            }
                        }
                        Ok(())
                    }
                    Commands::Untrack { id } => {
                        for id in id {
                            if let Err(e) = untrack(&mut sets, id) {
                                error!("{e}");
                            }
                        }
                        Ok(())
                    }
                    Commands::Pull { id } => match id {
                        Some(id) => {
                            for id in id {
                                if let Err(e) = pull(&mut sets, id).await {
                                    error!("{e}");
                                }
                            }
                            Ok(())
                        }
                        None => pull_all(&mut sets).await,
                    },
                    Commands::Daemon { interval } => daemon(&mut sets, interval).await,
                    _ => unreachable!(),
                };
                sets.write()?;
                res?;
            }
        }
        Ok(())
    }
}
