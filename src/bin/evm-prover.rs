use clap::Parser;
use evm_prover::commands::cli::{Cli, Commands};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init {} => evm_prover::commands::init::init()?,
        Commands::Start {} => evm_prover::commands::start::start().await?,
        Commands::Version {} => evm_prover::commands::version::version(),
        Commands::Config { config_command } => {
            evm_prover::commands::config::handle_config(config_command)?
        }
    }

    Ok(())
}
