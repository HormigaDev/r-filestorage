# Rust File Storage gRPC Service

## Descrição

Serviço gRPC em Rust para armazenamento seguro de arquivos segmentados por `user_id`. Arquivos executáveis são neutralizados por meio de compressão GZIP, enquanto arquivos não executáveis são armazenados em seu formato original.

## Arquitetura

-   **gRPC Service:** `StorageService`
-   **Endpoints:**
    -   `SaveFile`
    -   `GetFile`
    -   `DeleteFile`
-   **Funcionalidades:**
    -   Salvar arquivos.
    -   Recuperar arquivos.
    -   Excluir arquivos.
    -   Arquivos executáveis são neutralizados com compressão (`.neutralized.gz`).
    -   Arquivos não executáveis são armazenados no formato original.

## Estrutura do Projeto

```
.
├── build.rs
├── proto/
│   └── storage.proto
├── src/
│   ├── generated/
│   │   └── storage.rs (gerado automaticamente)
│   ├── main.rs
│   └── storage.rs
└── Cargo.toml
```

> O diretório `src/generated` é gerado automaticamente e não deve ser incluído no repositório.

## Protocolo gRPC

### Arquivo: `proto/storage.proto`

### Serviço: `StorageService`

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

-   **Descrição:** Salva um arquivo para o `user_id` especificado. Arquivos executáveis são neutralizados como `.neutralized.gz`. Se o arquivo já existir, ele será sobrescrito.

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

-   **Descrição:** Recupera um arquivo do `user_id` especificado. Se for um executável neutralizado, ele será descomprimido automaticamente antes de ser enviado.

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

-   **Descrição:** Exclui um arquivo específico do `user_id`. Se for um executável neutralizado, a versão comprimida também será excluída.

## Gerenciamento de Arquivos

-   Arquivos executáveis (`exe`, `bat`, `sh`, `msi`, `cmd`, `apk`, `bin`, `com`) são:
    -   Comprimidos com GZIP.
    -   Renomeados com a extensão `.neutralized.gz`.
-   Arquivos não executáveis:
    -   Armazenados no formato original.
-   As operações de salvamento sobrescrevem o arquivo se ele já existir.

## Logging

-   Implementado com `tracing`.
-   Nível mínimo: `info`.
-   Erros críticos são registrados e respondidos como `Status::internal`.

## Uso

### Compilar

```bash
cargo build --release
```

### Executar

```bash
cargo run --release
```

### Endpoints

-   **Host:** `[::1]:50051`
-   **Serviço:** `StorageService`

### Exemplos com grpcurl

#### ✔️ Salvar arquivo

```bash
grpcurl -plaintext -proto proto/storage.proto     -d '{"user_id": "123", "filename":"file.txt", "file_content":"aGVsbG8gd29ybGQ="}'     [::1]:50051 storage.StorageService/SaveFile
```

#### ✔️ Obter arquivo

```bash
grpcurl -plaintext -proto proto/storage.proto     -d '{"user_id": "123", "filename":"file.txt"}'     [::1]:50051 storage.StorageService/GetFile
```

#### ✔️ Excluir arquivo

```bash
grpcurl -plaintext -proto proto/storage.proto     -d '{"user_id": "123", "filename":"file.txt"}'     [::1]:50051 storage.StorageService/DeleteFile
```

## Licença

Este projeto está licenciado sob a **Licença MIT**.  
Consulte o arquivo [LICENSE](./LICENSE) para mais detalhes.

---

**Autor:** HormigaDev  
**Email:** hormigadev7@gmail.com
