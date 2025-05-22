# Security Policy

## Supported Versions

| Version | Supported |
| ------- | --------- |
| Latest  | ✅        |

Only the latest version is actively supported and maintained.

---

## Reporting a Vulnerability

If you discover a security vulnerability in this repository, please report it responsibly.

-   Contact: [hormigadev7@gmail.com](mailto:hormigadev7@gmail.com)
-   Subject: `Security vulnerability in rust-storage-service`

We will investigate and respond as soon as possible.

---

## Scope of Responsibility

This project is a **file storage microservice** that provides file upload, download, and deletion functionality over gRPC. **It does not implement authentication, authorization, or access control mechanisms.**

### ⚠️ The following are **out of scope** for this repository:

-   User authentication.
-   Authorization or permission handling.
-   Rate limiting or abuse prevention.
-   Encryption at rest or in transit beyond what gRPC inherently provides.

### ✅ The following are **in scope** for this repository:

-   File handling integrity.
-   File type neutralization for executables (via GZIP compression).
-   Handling of directory traversal attacks (e.g., `"../"` checks).
-   Protection against path injection vulnerabilities.
-   Correct error handling and internal resilience.

---

## Recommendations for Production Use

-   You **must implement authentication and authorization** at the API Gateway, Reverse Proxy, or within the client/server that integrates this microservice.
-   Deploy this service within a **private network**, behind firewalls or internal service meshes.
-   Do not expose this service directly to public internet without proper security layers.
-   Regularly audit and monitor file storage usage.

---

## Disclaimer

This project is provided "as is" under the MIT License. It is the responsibility of the deploying organization to implement the necessary security controls suitable for their use case.

---

© HormigaDev
