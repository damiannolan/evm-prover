use crate::config::config::{APP_HOME_DIR, CONFIG_FILE, Config};
use std::fs;
use tokio::net::TcpListener;
use tonic::transport::Server;

pub async fn start() -> anyhow::Result<()> {
    let config_path = dirs::home_dir()
        .expect("Cannot find home directory")
        .join(APP_HOME_DIR)
        .join(CONFIG_FILE);

    let config_yaml = fs::read_to_string(&config_path)?;
    let config: Config = serde_yaml::from_str(&config_yaml)?;

    println!("Starting gRPC server at {}", config.grpc_address);

    let listener = TcpListener::bind(&config.grpc_address).await?;

    Server::builder()
        .add_service(tonic::codegen::server::Router::new()) // Placeholder for gRPC service
        .serve_with_incoming(tokio_stream::wrappers::TcpListenerStream::new(listener))
        .await?;

    Ok(())
}
