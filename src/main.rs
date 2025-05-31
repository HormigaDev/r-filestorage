mod storage;

mod generated {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/generated/storage.rs"
    ));
}

use dotenv::dotenv;
use generated::storage_service_server::StorageServiceServer;
use std::{env, process};
use storage::StorageServiceImpl;
use tonic::transport::Server;
use tracing_subscriber;

const ADDR_IPV4: &str = "ADDR_IPV4";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Err(error) = dotenv() {
        eprintln!("Error loading environment variables: {}", error);
        std::process::exit(1);
    }

    tracing_subscriber::fmt().with_env_filter("info").init();

    let addr_ipv4 = match env::var(ADDR_IPV4) {
        Ok(addr) => addr.parse()?,
        Err(e) => {
            eprintln!("Error loading {} env var: {}", ADDR_IPV4, e);
            process::exit(1);
        }
    };
    tracing::info!("Starting gRPC server at {}", addr_ipv4);

    Server::builder()
        .add_service(StorageServiceServer::new(StorageServiceImpl::default()))
        .serve(addr_ipv4)
        .await?;

    Ok(())
}
