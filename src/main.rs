mod assembler;
mod commands;
mod constants;
mod emulator;
mod exercise;
mod state;
mod ui;
mod utils;
mod harness;
mod exercise_tests;
mod hints;

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
    Init {
        /// Force initialization, overwriting existing exercises and resetting progress
        #[arg(short, long)]
        force: bool,
    },
    /// Launches watch mode on the exercises folder
    Start,
    /// Runs the current exercise once (without watching)
    Run,
    /// Dumps the assembled binary of the current exercise for debugger
    /// inspection
    Debug,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { force } => init_mode(force),
        Commands::Start => watch_mode(),
        Commands::Run => run_workflow(),
        Commands::Debug => commands::debug_exercise(),
    }
}
