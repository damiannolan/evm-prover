use crate::config::config::{APP_HOME_DIR, CONFIG_FILE, Config};
use crate::grpc::server::create_grpc_server;
use std::fs;

pub async fn start() -> anyhow::Result<()> {
    let config_path = dirs::home_dir()
        .expect("Cannot find home directory")
        .join(APP_HOME_DIR)
        .join(CONFIG_FILE);

    let config_yaml = fs::read_to_string(&config_path)?;
    let config: Config = serde_yaml::from_str(&config_yaml)?;

    println!("Starting gRPC server at {}", config.grpc_address);
    create_grpc_server(&config.grpc_address).await?;

    Ok(())
}
