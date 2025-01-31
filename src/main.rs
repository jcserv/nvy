use anyhow::Result;
use clap::{Parser, Subcommand};

use nv::init::run_init;
use nv::r#use::run_use;

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
    /// Export the specific profile to the current shell
    Use {
        /// The profile to use
        #[arg(default_value = "default")]
        profile: String,
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => {
            run_init()?;
        },
        Commands::Use { profile } => {
            run_use(profile)?;
        },
    }

    Ok(())
}
