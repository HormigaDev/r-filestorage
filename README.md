# Rust File Storage gRPC Service

## Descripción

Este servicio implementa una API gRPC en Rust para almacenamiento seguro de archivos. Los archivos se segmentan por `user_id`. Los archivos ejecutables se neutralizan mediante compresión GZIP, mientras que los archivos de texto se almacenan en su formato plano.

## Arquitectura

-   **gRPC Service:** `StorageService`
-   **Endpoint:** `SaveFile`
-   **Funcionalidad:**
    -   Guarda archivos.
    -   Detecta archivos ejecutables y los neutraliza (`.neutralized.gz`).
    -   Archivos de texto se almacenan como texto plano.

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

> El directorio `src/generated` se genera automáticamente y debe incluirse en el repositorio.

## Protocolo gRPC

Archivo: `proto/storage.proto`

-   **Servicio:** `StorageService`
-   **Método:** `SaveFile`
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

## Uso

### Compilación

```bash
cargo build --release
```

### Ejecución

```bash
cargo run --release
```

### Endpoint

-   **Host:** `[::1]:50051`
-   **Servicio:** `StorageService`
-   **Método:** `SaveFile`

## Manejo de Archivos

-   Archivos ejecutables (`exe`, `bat`, `sh`, `msi`, `cmd`, `apk`, `bin`, `com`) son:

    -   Comprimidos con GZIP.
    -   Renombrados con la extensión `.neutralized.gz`.

-   Otros archivos son almacenados en su formato original.

## Logging

Se utiliza `tracing` con filtro `info`. Los errores se muestran en consola y son retornados al cliente como `Status::internal` en caso de fallo.

## Licencia

Licencia personalizada (CUSTOM).

---

**Autor:** HormigaDev
**Email:** hormigadev7@gmail.com
