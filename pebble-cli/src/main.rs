mod build;

use crate::build::run_build;
use clap::{Parser, Subcommand};

/// The top-level Cargo command parser
#[derive(Parser)]
#[command(name = "cargo", bin_name = "cargo")]
struct Cli {
    #[command(subcommand)]
    command: CargoCmd,
}

/// Captures the `pebble` keyword passed by Cargo
#[derive(Subcommand)]
enum CargoCmd {
    Pebble(PebbleArgs),
}

/// The actual arguments and subcommands for our tool
#[derive(clap::Args)]
struct PebbleArgs {
    #[command(subcommand)]
    command: PebbleSubCommand,
}

/// The available actions we can perform
#[derive(Subcommand)]
enum PebbleSubCommand {
    /// Build the Pebble project for all target platforms
    Build,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        CargoCmd::Pebble(args) => match args.command {
            PebbleSubCommand::Build => run_build(),
        },
    }
}
