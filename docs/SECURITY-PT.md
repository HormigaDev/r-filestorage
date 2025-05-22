# Política de Segurança

## Versões Suportadas

| Versão | Suportada |
| ------ | --------- |
| Última | ✅        |

Apenas a última versão está ativamente suportada e mantida.

---

## Reporte de Vulnerabilidades

Se você encontrar uma vulnerabilidade de segurança neste repositório, reporte-a de forma responsável.

-   Contato: [hormigadev7@gmail.com](mailto:hormigadev7@gmail.com)
-   Assunto: `Vulnerabilidade de segurança no rust-storage-service`

Investigaremos e responderemos o mais rápido possível.

---

## Escopo de Responsabilidade

Este projeto é um **microserviço de armazenamento de arquivos** que fornece funcionalidades de upload, download e exclusão de arquivos via gRPC. **Não implementa mecanismos de autenticação, autorização ou controle de acesso.**

### ⚠️ O seguinte está **fora do escopo** deste repositório:

-   Autenticação de usuários.
-   Gerenciamento de autorização ou permissões.
-   Limitação de taxa (rate limiting) ou prevenção de abuso.
-   Criptografia em repouso ou em trânsito além do que o gRPC provê por padrão.

### ✅ O seguinte está **dentro do escopo** deste repositório:

-   Integridade no manuseio de arquivos.
-   Neutralização de arquivos executáveis (compressão GZIP).
-   Proteção contra ataques de path traversal (e.g., validação contra `"../"`).
-   Proteção contra injeção de caminhos.
-   Tratamento correto de erros e resiliência interna.

---

## Recomendações para Uso em Produção

-   **Deve implementar autenticação e autorização** no API Gateway, proxy reverso ou no cliente/servidor que integra este microserviço.
-   Implante este serviço em uma **rede privada**, atrás de firewalls ou malhas de serviço internas.
-   Não exponha este serviço diretamente para a internet pública sem camadas adequadas de segurança.
-   Realize auditorias e monitoramento regular do uso do armazenamento.

---

## Aviso Legal

Este projeto é fornecido "como está" sob licença MIT. É responsabilidade da organização que o implanta implementar os controles de segurança adequados para seu caso de uso.

---

© HormigaDev
