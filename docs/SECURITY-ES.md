# Política de Seguridad

## Versiones Soportadas

| Versión | Soportada |
| ------- | --------- |
| Última  | ✅        |

Solo la última versión está activamente soportada y mantenida.

---

## Reporte de Vulnerabilidades

Si detectas una vulnerabilidad de seguridad en este repositorio, por favor repórtala de forma responsable.

-   Contacto: [hormigadev7@gmail.com](mailto:hormigadev7@gmail.com)
-   Asunto: `Vulnerabilidad de seguridad en rust-storage-service`

Investigaremos y responderemos lo antes posible.

---

## Alcance de Responsabilidad

Este proyecto es un **microservicio de almacenamiento de archivos** que provee funcionalidades de subida, descarga y eliminación de archivos vía gRPC. **No implementa mecanismos de autenticación, autorización ni control de acceso.**

### ⚠️ Lo siguiente está **fuera del alcance** de este repositorio:

-   Autenticación de usuarios.
-   Manejo de autorización o permisos.
-   Limitación de tasa (rate limiting) o prevención de abuso.
-   Cifrado en reposo o en tránsito más allá de lo que gRPC provee por defecto.

### ✅ Lo siguiente está **dentro del alcance** de este repositorio:

-   Integridad en el manejo de archivos.
-   Neutralización de tipos ejecutables (compresión GZIP).
-   Protección contra ataques de recorrido de directorios (e.g., validación contra `"../"`).
-   Protección contra inyección de rutas.
-   Manejo correcto de errores y resiliencia interna.

---

## Recomendaciones para Uso en Producción

-   **Debe implementar autenticación y autorización** en el API Gateway, proxy inverso o en el cliente/servidor que integre este microservicio.
-   Despliegue este servicio en una **red privada**, detrás de firewalls o mallas de servicio internas.
-   No exponga este servicio directamente a internet público sin capas adecuadas de seguridad.
-   Realice auditorías y monitoreo regular del uso del almacenamiento.

---

## Descargo de Responsabilidad

Este proyecto se proporciona "tal cual" bajo licencia MIT. Es responsabilidad de la organización que despliega implementar los controles de seguridad adecuados para su caso de uso.

---

© HormigaDev
