# Política de seguridad

## Estado del proyecto

Visor MD v2 está en desarrollo temprano y todavía no tiene una release estable
soportada. Aun así, los reportes de seguridad son valiosos desde ahora.

## Cómo reportar

No publiques detalles explotables, documentos maliciosos o rutas privadas en un
issue público.

Usa el reporte privado de vulnerabilidades de GitHub si está habilitado en el
repositorio. Si no está disponible, abre un issue público mínimo indicando que
necesitas un canal privado, sin incluir payload, datos personales o instrucciones
de explotación.

Incluye por el canal privado:

- versión, commit o rama;
- sistema operativo;
- comportamiento observado;
- impacto posible;
- pasos mínimos de reproducción;
- archivo de prueba reducido y no sensible;
- si hubo red, escritura, crash o consumo de recursos;
- cualquier mitigación conocida.

## Qué se considera especialmente importante

- ejecución de código o contenido activo;
- conexiones sin consentimiento;
- lectura o escritura fuera del archivo o bóveda permitidos;
- corrupción o pérdida de documentos;
- bypass de límites de recursos;
- path traversal, symlinks, junctions o UNC inesperadas;
- phishing facilitado por destinos ocultos;
- vulnerabilidades de dependencias o del proceso de build;
- filtración mediante logs, exportación o imágenes.

## Tratamiento

El proyecto intentará:

1. confirmar recepción;
2. reproducir sin exponer el reporte;
3. evaluar impacto y versiones afectadas;
4. crear una prueba de regresión cuando sea seguro;
5. corregir en una rama privada o controlada;
6. publicar mitigación y crédito según lo acordado con quien reporta.

No se promete un SLA comercial. La prioridad se decide por impacto y
explotabilidad, no por cantidad de funciones afectadas.

## Divulgación responsable

Se solicita tiempo razonable para investigar y distribuir una corrección antes de
publicar detalles. No se pide ocultar problemas indefinidamente. La coordinación
busca proteger a usuarios mientras se produce evidencia y una solución.

## Documentación técnica

La postura y el modelo de amenaza viven en:

- [`docs/security.md`](docs/security.md);
- [`docs/threat-model.md`](docs/threat-model.md);
- [`docs/test-matrix.md`](docs/test-matrix.md).
