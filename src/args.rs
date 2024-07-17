use std::path::PathBuf;

use crate::{config::ConfigFile, query::Query};

#[derive(Debug, clap::Parser)]
pub struct Args {
    /// Log file to use
    #[clap(short, long)]
    pub file: Option<PathBuf>,

    #[clap(subcommand)]
    pub command: ArgsCommand,
}

impl Args {
    pub fn file(&self, cfg: &ConfigFile) -> PathBuf {
        if let Some(f) = &self.file {
            f.clone()
        } else {
            cfg.normalized_path()
        }
    }
}

#[derive(Debug, clap::Subcommand)]
pub enum ArgsCommand {
    /// Create a default config file
    Init,

    /// Log a message to the given file
    Log {
        /// Message to log
        log: String,
    },

    /// Command to easily query the slf file
    Query(#[command(flatten)] Query),
}
