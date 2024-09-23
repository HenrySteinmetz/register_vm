#[cfg(test)]
mod tests;

mod cli;
mod operands;
mod operations;
mod vm;

use clap::Parser;
use cli::Cli;
use std::fs::read;

fn main() {
    let cli = Cli::parse();

    if !cli.input.exists() {
        panic!("Provided input file does not exist!");
    }

    if !cli.input.is_file() {
        panic!("Provided input path is not a file!");
    }

    let program: Vec<u8> = read(&cli.input).expect("Failed to read input file!");

    let mut vm = vm::VM::new();
    vm.run(program, cli.verbose);
}
