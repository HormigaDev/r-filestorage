# Rust File Storage gRPC Service

## Descripción

Servicio gRPC en Rust para almacenamiento seguro de archivos segmentados por `user_id`. Los archivos ejecutables se neutralizan mediante compresión GZIP, mientras que los archivos no ejecutables se almacenan en su formato original.

## Arquitectura

-   **gRPC Service:** `StorageService`
-   **Endpoints:**
    -   `SaveFile`
    -   `GetFile`
    -   `DeleteFile`
-   **Funcionalidad:**
    -   Guardar archivos.
    -   Recuperar archivos.
    -   Eliminar archivos.
    -   Archivos ejecutables son neutralizados mediante compresión (`.neutralized.gz`).
    -   Archivos no ejecutables se almacenan en su formato original.

## Estructura del Proyecto

```
.
├── build.rs
├── proto/
│   └── storage.proto
├── src/
│   ├── generated/
│   │   └── storage.rs (autogenerado)
│   ├── main.rs
│   └── storage.rs
└── Cargo.toml
```

> El directorio `src/generated` se genera automáticamente y no debe incluirse en el repositorio.

## Protocolo gRPC

### Archivo: `proto/storage.proto`

### Servicio: `StorageService`

### Métodos:

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

-   **Descripción:** Guarda un archivo para el `user_id` especificado. Archivos ejecutables se neutralizan como `.neutralized.gz`. Si el archivo ya existe, se sobrescribe.

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

-   **Descripción:** Recupera un archivo del `user_id` especificado. Si es un ejecutable neutralizado, es descomprimido automáticamente antes de enviarlo.

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

-   **Descripción:** Elimina un archivo específico del `user_id`. Si es un ejecutable neutralizado, también se elimina la versión comprimida.

## Manejo de Archivos

-   Archivos ejecutables (`exe`, `bat`, `sh`, `msi`, `cmd`, `apk`, `bin`, `com`) son:
    -   Comprimidos con GZIP.
    -   Renombrados con la extensión `.neutralized.gz`.
-   Archivos no ejecutables:
    -   Almacenados en su formato original.
-   Las operaciones de guardado sobrescriben si el archivo ya existe.

## Logging

-   Implementado con `tracing`.
-   Nivel mínimo: `info`.
-   Errores críticos se loguean y responden como `Status::internal`.

## Uso

### Compilación

```bash
cargo build --release
```

### Ejecución

```bash
cargo run --release
```

### Endpoints

-   **Host:** `[::1]:50051`
-   **Servicio:** `StorageService`

### Ejemplos con grpcurl

#### ✔️ Guardar archivo

```bash
grpcurl -plaintext -proto proto/storage.proto \
    -d '{"user_id": "123", "filename":"file.txt", "file_content":"aGVsbG8gd29ybGQ="}' \
    [::1]:50051 storage.StorageService/SaveFile
```

#### ✔️ Obtener archivo

```bash
grpcurl -plaintext -proto proto/storage.proto \
    -d '{"user_id": "123", "filename":"file.txt"}' \
    [::1]:50051 storage.StorageService/GetFile
```

#### ✔️ Eliminar archivo

```bash
grpcurl -plaintext -proto proto/storage.proto \
    -d '{"user_id": "123", "filename":"file.txt"}' \
    [::1]:50051 storage.StorageService/DeleteFile
```

## Licencia

Este proyecto está licenciado bajo la **Licencia MIT**.  
Consulta el archivo [LICENSE](./LICENSE) para más detalles.

---

**Autor:** HormigaDev  
**Email:** hormigadev7@gmail.com
