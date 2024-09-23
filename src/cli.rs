use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    pub input: PathBuf,
    #[arg(short, long)]
    pub verbose: bool,
}
