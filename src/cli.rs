use std::path::PathBuf;

use clap::{Parser, ValueEnum};

#[derive(Clone, ValueEnum)]
pub enum Verbosity {
    Quiet,
    Warn,
    Info,
    Debug,
    Trace,
}

#[derive(Parser)]
pub struct Cli {
    pub input: PathBuf,
    #[arg(short, long)]
    pub verbosity: Option<Verbosity>,
}
