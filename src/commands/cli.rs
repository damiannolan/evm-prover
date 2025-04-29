use crate::commands::config::ConfigCommands;
use crate::commands::version::VERSION;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "evm-prover", version = VERSION, about = "EVM Prover CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize configuration and home directory
    Init {},

    /// Start the gRPC server
    Start {},

    /// Show the service version
    Version {},

    /// Manage configuration
    Config {
        #[command(subcommand)]
        config_command: ConfigCommands,
    },
}
