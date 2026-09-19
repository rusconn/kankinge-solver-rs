use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::commands::bfs;

#[derive(Debug, Parser)]
#[command(version, about)]
pub(crate) struct Args {
    /// Path to stage.json
    pub(crate) stage_json: PathBuf,

    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    Bfs(bfs::Args),
    Iddfs,
}
