# Rust File Storage gRPC Service

## Summary

-   [Documentación en Español](./docs/README-ES.md)
-   [Informativo de seguridad en Español](./docs/SECURITY-ES.md)

---

-   [Documentação em Português](./docs/README-PT.md)
-   [Informativo de segurança em Português](./docs/SECURITY-PT.md)

## Description

gRPC service in Rust for secure storage of files segmented by `user_id`. Executable files are neutralized using GZIP compression, while non-executable files are stored in their original format.

## Architecture

-   **gRPC Service:** `StorageService`
-   **Endpoints:**
-   `SaveFile`
-   `GetFile`
-   `DeleteFile`
-   **Functionality:**
-   Save files.
-   Retrieve files.
-   Delete files.
-   Executable files are neutralized using compression (`.neutralized.gz`).
-   Non-executable files are stored in their original format.

## Project Structure

```
.
├── build.rs
├── proto/
│   └── storage.proto
├── src/
│   ├── generated/
│   │   └── storage.rs (auto-generated)
│   ├── main.rs
│   └── storage.rs
└── Cargo.toml
```

> The `src/generated` directory is generated automatically and should not be included in the repository.

## gRPC Protocol

### File: `proto/storage.proto`

### Service: `StorageService`

### Methods:

---

### ✅ SaveFile

-   **Request:**

```proto
message SaveFileRequest {
string user_id = 1;
string filename = 2;
bytes file_content = 3;
}
```

-   **Response:**

```proto
message SaveFileResponse {
bool success = 1;
string message = 2;
string stored_path = 3;
}
```

-   **Description:** Saves a file for the specified `user_id`. Executable files are neutralized as `.neutralized.gz`. If the file already exists, it is overwritten.

---

### ✅ GetFile

-   **Request:**

```proto
message GetFileRequest {
string user_id = 1;
string filename = 2;
}
```

-   **Response:**

```proto
message GetFileResponse {
bool success = 1;
string message = 2;
bytes file_content = 3;
}
```

-   **Description:** Retrieves a file from the specified `user_id`. If it is a neutralized executable, it is automatically decompressed before sending.

---

### ✅ DeleteFile

-   **Request:**

```proto
message DeleteFileRequest {
string user_id = 1;
string filename = 2;
}
```

-   **Response:**

```proto
message DeleteFileResponse {
bool success = 1;
string message = 2;
}
```

-   **Description:** Deletes a specific file from the `user_id`. If it is a neutralized executable, the compressed version is also deleted.

## File Handling

-   Executable files (`exe`, `bat`, `sh`, `msi`, `cmd`, `apk`, `bin`, `com`) are:
-   Compressed with GZIP.
-   Renamed with the extension `.neutralized.gz`.
-   Non-executable files:
-   Stored in their original format.
-   Save operations overwrite if the file already exists.

## Logging

-   Implemented with `tracing`.
-   Minimum level: `info`.
-   Critical errors are logged and responded to as `Status::internal`.

## Usage

### Compilation

```bash
cargo build --release
```

### Execution

```bash
cargo run --release
```

### Endpoints

-   **Host:** `[::1]:50051`
-   **Service:** `StorageService`

### Examples with grpcurl

#### ✔️ Save file

```bash
grpcurl -plaintext -proto proto/storage.proto \
  -d '{"user_id": "123", "filename":"file.txt", "file_content":"aGVsbG8gd29ybGQ="}' \
  [::1]:50051 storage.StorageService/SaveFile
```

#### ✔️ Get file

```bash
grpcurl -plaintext -proto proto/storage.proto \
  -d '{"user_id": "123", "filename":"file.txt"}' \
  [::1]:50051 storage.StorageService/GetFile
```

#### ✔️ Delete file

```bash
grpcurl -plaintext -proto proto/storage.proto \
  -d '{"user_id": "123", "filename":"file.txt"}' \
  [::1]:50051 storage.StorageService/DeleteFile
```

## License

This project is licensed under the **MIT License**.  
See the [LICENSE](./LICENSE) file for more details.

---

**Author:** HormigaDev  
**Email:** hormigadev7@gmail.com
