#[cfg(test)]
mod tests;

mod cli;
mod operands;
mod operations;
mod vm;

use clap::Parser;
use cli::{Cli, Verbosity};
use std::{env::set_var, fs::read};

#[cfg(debug_assertions)]
extern crate log;
#[cfg(debug_assertions)]
extern crate pretty_env_logger;

macro_rules! fatal {
    ($($arg:tt)*) => {
        use log;
        log::error!($($arg)*);
        std::process::exit(1);
    }
}

pub(crate) use fatal;

fn main() {
    let cli = Cli::parse();

    match cli.verbosity {
        Some(v) => match v {
            Verbosity::Info => set_var("RUST_LOG", "info"),
            Verbosity::Debug => set_var("RUST_LOG", "debug"),
            Verbosity::Trace => set_var("RUST_LOG", "trace"),
            Verbosity::Warn => set_var("RUST_LOG", "warn"),
            Verbosity::Quiet => set_var("RUST_LOG", "off"),
        },
        None => set_var("RUST_LOG", "info"),  // Default verbosity if none specified.
    }

    pretty_env_logger::init();

    if !cli.input.exists() {
        fatal!("Provided input file does not exist!");
    }

    if !cli.input.is_file() {
        fatal!("Provided input path is not a file!");
    }

    let program: Vec<u8> = read(&cli.input).expect("Failed to read input file: ");

    let mut vm = vm::VM::new();

    vm.run(program);
}
