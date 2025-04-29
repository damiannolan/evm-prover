use anyhow::Result;
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::Server;
use tonic_reflection::server::Builder as ReflectionBuilder;

pub async fn create_grpc_server(addr: &str) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;

    let reflection_service = ReflectionBuilder::configure().build().unwrap();

    Server::builder()
        .add_service(reflection_service)
        .serve_with_incoming(TcpListenerStream::new(listener))
        .await?;

    Ok(())
}
