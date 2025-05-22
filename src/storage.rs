use crate::generated::{
    storage_service_server::StorageService, DeleteFileRequest, DeleteFileResponse, GetFileRequest,
    GetFileResponse, SaveFileRequest, SaveFileResponse,
};

use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use std::{
    env,
    io::{Read, Write},
    path::Path,
    sync::Arc,
};
use tokio::{
    fs::{self, File},
    io::{AsyncReadExt, AsyncWriteExt},
    sync::Semaphore,
    task,
};
use tonic::{Request, Response, Status};
use tracing::{error, info};

#[derive(Clone)]
pub struct StorageServiceImpl {
    semaphore: Arc<Semaphore>,
}

impl Default for StorageServiceImpl {
    fn default() -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(10)),
        }
    }
}

#[tonic::async_trait]
impl StorageService for StorageServiceImpl {
    async fn save_file(
        &self,
        request: Request<SaveFileRequest>,
    ) -> Result<Response<SaveFileResponse>, Status> {
        let _permit = self.semaphore.acquire().await.map_err(internal)?;

        let req = request.into_inner();
        let user_id = req.user_id;
        let filename = req.filename;
        let content = req.file_content;

        if filename.contains("..") {
            return Err(Status::internal("Invalid filename"));
        }

        let base_path = Path::new(&get_storage_dir()).join(&user_id);
        fs::create_dir_all(&base_path).await.map_err(internal)?;

        let final_path = base_path.join(&filename);

        if is_executable_file(&filename) {
            let compressed_path = final_path.with_extension("neutralized.gz");
            compress_and_save(&compressed_path, &content)
                .await
                .map_err(internal)?;
            info!(
                "File saved (compressed executable) at {:?}",
                compressed_path
            );

            Ok(Response::new(SaveFileResponse {
                success: true,
                message: "Executable file compressed and saved.".into(),
                stored_path: compressed_path.to_string_lossy().into(),
            }))
        } else {
            let mut file = File::create(&final_path).await.map_err(internal)?;
            file.write_all(&content).await.map_err(internal)?;
            file.flush().await.map_err(internal)?;
            info!("File saved at {:?}", final_path);

            Ok(Response::new(SaveFileResponse {
                success: true,
                message: "File saved successfully.".into(),
                stored_path: final_path.to_string_lossy().into(),
            }))
        }
    }

    async fn get_file(
        &self,
        request: Request<GetFileRequest>,
    ) -> Result<Response<GetFileResponse>, Status> {
        let _permit = self.semaphore.acquire().await.map_err(internal)?;

        let req = request.into_inner();
        let base_path = Path::new(&get_storage_dir()).join(&req.user_id);
        let filename = &req.filename;
        let file_path = base_path.join(filename);

        if filename.contains("..") {
            return Err(Status::internal("Invalid filename"));
        }

        if is_executable_file(filename) {
            let compressed_path = file_path.with_extension("neutralized.gz");
            let data = task::spawn_blocking(move || -> anyhow::Result<Vec<u8>> {
                let file = std::fs::File::open(&compressed_path)?;
                let mut decoder = GzDecoder::new(file);
                let mut buf = Vec::new();
                decoder.read_to_end(&mut buf)?;
                Ok(buf)
            })
            .await
            .map_err(internal)?
            .map_err(internal)?;

            Ok(Response::new(GetFileResponse {
                success: true,
                message: "File retrieved and decompressed.".into(),
                file_content: data,
            }))
        } else {
            let mut file = File::open(&file_path).await.map_err(internal)?;
            let mut buf = Vec::new();
            file.read_to_end(&mut buf).await.map_err(internal)?;

            Ok(Response::new(GetFileResponse {
                success: true,
                message: "File retrieved successfully.".into(),
                file_content: buf,
            }))
        }
    }

    async fn delete_file(
        &self,
        request: Request<DeleteFileRequest>,
    ) -> Result<Response<DeleteFileResponse>, Status> {
        let _permit = self.semaphore.acquire().await.map_err(internal)?;

        let req = request.into_inner();

        if req.filename.contains("..") {
            return Err(Status::internal("Invalid filename"));
        }

        let base_path = Path::new(&get_storage_dir()).join(&req.user_id);
        let file_path = base_path.join(&req.filename);

        if is_executable_file(&req.filename) {
            let compressed_path = file_path.with_extension("neutralized.gz");
            if let Err(e) = fs::remove_file(&compressed_path).await {
                if e.kind() != tokio::io::ErrorKind::NotFound {
                    return Err(internal(e));
                }
            }
        }

        if let Err(e) = fs::remove_file(&file_path).await {
            if e.kind() == tokio::io::ErrorKind::NotFound {
                return Err(Status::not_found("File not found"));
            } else {
                return Err(internal(e));
            }
        }

        info!("File deleted at {:?}", file_path);

        Ok(Response::new(DeleteFileResponse {
            success: true,
            message: "File deleted successfully.".into(),
        }))
    }
}

/// Ejecuta la compresión en un hilo bloqueante para no afectar Tokio.
async fn compress_and_save(path: &Path, data: &[u8]) -> anyhow::Result<()> {
    let path = path.to_path_buf();
    let data = data.to_vec();

    task::spawn_blocking(move || {
        let file = std::fs::File::create(&path)?;
        let mut encoder = GzEncoder::new(file, Compression::default());
        encoder.write_all(&data)?;
        encoder.finish()?;
        Ok::<(), anyhow::Error>(())
    })
    .await?
}

/// Detecta si es un archivo ejecutable para neutralizarlo.
fn is_executable_file(filename: &str) -> bool {
    matches!(
        filename
            .rsplit('.')
            .next()
            .unwrap_or("")
            .to_lowercase()
            .as_str(),
        "exe" | "bat" | "sh" | "msi" | "cmd" | "apk" | "bin" | "com"
    )
}

/// Conversión interna de errores a Status gRPC.
fn internal<E: std::fmt::Debug>(e: E) -> Status {
    error!("Internal error: {:?}", e);
    Status::internal(format!("Internal error: {:?}", e))
}

fn get_storage_dir() -> String {
    env::var("STORAGE_DIR").unwrap_or_else(|_| "./storage".to_string())
}
