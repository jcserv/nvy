use anyhow::Result;
use clap::{Parser, Subcommand};

use nvy::config::TARGET_SHELL;
use nvy::init::run_init;
use nvy::export::run_export;
use nvy::r#use::run_use;

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
    /// Set the target destination for environment variables
    Export {
        #[arg(default_value = TARGET_SHELL)]
        target: String,
    },
    /// Output the specified profile(s) to the target destination
    Use {
        /// The profiles to use. If overlapping environment variables are defined, the last one wins.
        #[arg(num_args = 1..)] 
        #[arg(default_values_t = vec!["default".to_string()])]
        profiles: Vec<String>,
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => {
            run_init()?;
        },
        Commands::Export { target } => {    
            run_export(target)?;
        },
        Commands::Use { profiles } => {
            run_use(profiles)?;
        },
    }

    Ok(())
}
