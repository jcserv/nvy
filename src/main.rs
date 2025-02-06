use anyhow::Result;
use clap::{Parser, Subcommand};

use nvy::config::run_config;
use nvy::nvy_config::TARGET_SHELL;
use nvy::init::run_init;
use nvy::target::{run_target, run_target_set};
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
    /// View or modify the target destination for environment variables
    Target {
        #[command(subcommand)]
        command: Option<TargetCommands>,
    },
    /// Output the specified profile(s) to the target destination
    Use {
        /// The profiles to use. If overlapping environment variables are defined, the last one wins.
        #[arg(num_args = 1..)] 
        #[arg(default_values_t = vec!["default".to_string()])]
        profiles: Vec<String>,
    },
    /// View the nvy configuration
    Config,
}

#[derive(Subcommand)]
enum TargetCommands {
    /// Set the target destination for environment variables
    Set {
        #[arg(default_value = TARGET_SHELL)]
        file: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => {
            run_init()?;
        },
        Commands::Target { command } => {
            match command {
                Some(TargetCommands::Set { file }) => {
                    run_target_set(file)?;
                },
                None => {
                    run_target()?;
                }
            }
        },
        Commands::Use { profiles } => {
            run_use(profiles)?;
        },
        Commands::Config => {
            run_config()?;
        }
    }

    Ok(())
}
