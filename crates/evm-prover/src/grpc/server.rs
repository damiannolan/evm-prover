use std::fs;

use anyhow::Result;
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::Server;
use tonic_reflection::server::Builder as ReflectionBuilder;

use crate::config::config::Config;
use crate::evm::prover::ProverService;
use crate::proto::celestia::prover::v1::prover_server::ProverServer;

pub async fn create_grpc_server(config: Config) -> Result<()> {
    let listener = TcpListener::bind(config.grpc_address).await?;

    let descriptor_bytes = fs::read("src/proto/descriptor.bin")?;
    let reflection_service = ReflectionBuilder::configure()
        .register_encoded_file_descriptor_set(&descriptor_bytes)
        .build()
        .unwrap();

    let prover_serivce = ProverService::default();

    Server::builder()
        .add_service(reflection_service)
        .add_service(ProverServer::new(prover_serivce))
        .serve_with_incoming(TcpListenerStream::new(listener))
        .await?;

    Ok(())
}
