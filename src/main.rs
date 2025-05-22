mod storage;

mod generated {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/generated/storage.rs"
    ));
}

use dotenv::dotenv;
use generated::storage_service_server::StorageServiceServer;
use storage::StorageServiceImpl;
use tonic::transport::Server;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Err(error) = dotenv() {
        eprintln!("Error loading environment variables: {}", error);
        std::process::exit(1);
    }

    tracing_subscriber::fmt().with_env_filter("info").init();

    let addr = "[::1]:50051".parse()?;
    tracing::info!("Starting gRPC server at {}", addr);

    Server::builder()
        .add_service(StorageServiceServer::new(StorageServiceImpl::default()))
        .serve(addr)
        .await?;

    Ok(())
}
