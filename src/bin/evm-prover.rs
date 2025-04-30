use clap::Parser;
use evm_prover::commands::cli::{CLI, Commands};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = CLI::parse();

    match cli.command {
        Commands::Init {} => evm_prover::commands::command::init()?,
        Commands::Start {} => evm_prover::commands::command::start().await?,
        Commands::Version {} => evm_prover::commands::command::version(),
    }

    Ok(())
}
