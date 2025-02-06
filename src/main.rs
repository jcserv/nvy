use anyhow::Result;
use clap::{Parser, Subcommand};

use nvy::config::run_config;
use nvy::nvy_config::TARGET_SHELL;
use nvy::init::run_init;
use nvy::profiles::{run_profiles, run_profiles_remove, run_profiles_set};
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
    #[clap(alias = "i")]
    Init,
    /// Output the specified profile(s) to the target destination
    #[clap(alias = "u")]
    Use {
        /// The profiles to use. If overlapping environment variables are defined, the last one wins.
        #[arg(num_args = 1..)] 
        #[arg(default_values_t = vec!["default".to_string()])]
        profiles: Vec<String>,
    },
    /// View the nvy configuration
    #[clap(alias = "c")]
    Config,
    /// View or modify the target destination in the nvy configuration
    #[clap(alias = "t")]
    Target {
        #[command(subcommand)]
        command: Option<TargetCommands>,
    },
    /// View or modify the profile(s) in the nvy configuration
    #[clap(alias = "p")]
    Profiles {
        #[command(subcommand)]
        command: Option<ProfileCommands>,
    },
}

#[derive(Subcommand)]
enum TargetCommands {
    /// Set the target destination for environment variables
    Set {
        #[arg(default_value = TARGET_SHELL)]
        file: String,
    },
}

#[derive(Subcommand)]
enum ProfileCommands {
    /// Set the file path for a given profile. If the profile does not exist, it will be created.
    Set {
        profile: String,
        file: String,
    },
    /// Remove the provided profile
    Remove {
        profile: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => {
            run_init()?;
        },
        Commands::Use { profiles } => {
            run_use(profiles)?;
        },
        Commands::Config => {
            run_config()?;
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
        Commands::Profiles { command } => {
            match command {
                None => {
                    run_profiles()?;
                }
                Some(ProfileCommands::Set { profile, file }) => {
                    run_profiles_set(profile, file)?;
                },
                Some(ProfileCommands::Remove { profile }) => {
                    run_profiles_remove(profile)?;
                },
            }
        }
    }

    Ok(())
}
