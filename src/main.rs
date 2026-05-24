mod assembler;
mod commands;
mod constants;
mod emulator;
mod exercise;
mod state;
mod ui;
mod utils;

#[cfg(test)]
mod tests;

use clap::{Parser, Subcommand};
use commands::{init_mode, run_workflow, watch_mode};

#[derive(Parser)]
#[command(name = "asmlings")]
#[command(version, about = "x86 · 16-bit assembly exercises", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Creates a folder with all the blank exercises and needed files
    Init,
    /// Launches watch mode on the exercises folder
    Start,
    /// Runs the current exercise once (without watching)
    Run,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => init_mode(),
        Commands::Start => watch_mode(),
        Commands::Run => run_workflow(),
    }
}
