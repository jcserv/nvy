use anyhow::Result;
use clap::{Parser, Subcommand};

use nv::init::run_init;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize nv configuration in the current directory
    Init,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => {
            run_init()?;
        }
    }

    Ok(())
}
