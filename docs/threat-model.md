# Modelo de amenaza

Última revisión: 25 de agosto de 2026.

## Cómo leer este documento

Un threat model describe qué se protege, de quién, por dónde puede entrar un
ataque y qué riesgo queda después de aplicar controles. No es una lista genérica
de miedos. Cada amenaza debe conectarse con una decisión y una prueba.

## Alcance

Incluye:

- aplicación de escritorio;
- parser y modelo documental;
- renderer;
- edición y guardado;
- rutas, imágenes y enlaces;
- workspace e índice;
- sidecars;
- exportadores;
- instaladores y actualizaciones futuras;
- dependencias y fuentes.

No asume que el contenido Markdown sea confiable.

## Activos

Un activo es algo cuyo daño importa.

| Activo | Daño a evitar |
| --- | --- |
| Documentos | Pérdida, corrupción o modificación inesperada |
| Archivos vecinos | Lectura o escritura provocada por contenido |
| Privacidad | Envío de contenido, rutas, IP o hábitos sin consentimiento |
| Disponibilidad | Bloqueo, crash o agotamiento de recursos |
| Integridad de la app | Ejecución de código o alteración de configuración |
| Confianza del usuario | Phishing, avisos engañosos o permisos ambiguos |
| Supply chain | Binarios o dependencias comprometidas |
| Evidencia del proyecto | Métricas o tests que afirman más de lo demostrado |

## Atacantes considerados

- autor de un `.md` malicioso;
- repositorio o bóveda descargada de terceros;
- enlace o imagen controlados por un atacante;
- archivo local preparado para agotar recursos;
- dependencia comprometida o abandonada;
- proceso local no privilegiado que cambia archivos durante una operación;
- servidor remoto que registra o redirige solicitudes confirmadas.

## Fuera del modelo

- administrador local malicioso;
- kernel o sistema operativo comprometido;
- hardware comprometido;
- usuario que ejecuta voluntariamente otro programa malicioso;
- protección criptográfica de todo el disco.

Excluirlos no significa que sean imposibles. Significa que Visor MD no puede
resolverlos dentro de su alcance.

## Entradas

- archivo principal;
- nombre y ruta;
- argumentos de proceso y asociación de archivos;
- Markdown y HTML embebido;
- imágenes;
- enlaces web y locales;
- wikilinks y frontmatter;
- sidecars;
- índice persistido;
- portapapeles;
- drag and drop futuro;
- resultados de exportación;
- archivos de configuración;
- paquetes y fuentes de build.

## Fronteras de confianza

```text
Documento hostil
      |
      v
Decodificación y parser
      |
      v
Modelo validado
      |
      +-----------> Renderer sin permisos de disco o red
      |
      +-----------> VFS con política
      |
      +-----------> Editor y guardado atómico

Internet <---- componente de red opcional y consentido ----> modelo validado
```

Cada flecha es un lugar donde validar datos, limitar capacidades y crear tests.

## Escenarios principales

| Amenaza | Ejemplo | Control previsto | Evidencia requerida | Riesgo residual |
| --- | --- | --- | --- | --- |
| Ejecución | `<script>` o handler HTML | Sin DOM, allowlist y texto inerte | Corpus HTML y revisión de display list | Bug en parser o dependencia |
| Stack overflow | Miles de citas anidadas | Profundidad, recorrido iterativo y cancelación | Test adversarial y medición | Nuevas rutas recursivas |
| Agotamiento de memoria | Tabla, línea o imagen enorme | Presupuestos, tope de 16 KiB por línea y fallback | Benchmark y límites simulados | Coste previo a detectar formato |
| Path traversal | `../../secreto` | VFS, canonicalización y contención | Pruebas de rutas, prefijos y streams alternativos | TOCTOU o diferencias de plataforma |
| Escape por symlink | Recurso relativo que cambia destino | Identidad y validación sobre handle | Tests de carrera | Limitaciones de API |
| Acceso UNC | Markdown apunta a un share | Solo archivo principal manual | Tests UNC | Intención ambigua en asociación externa |
| Portapapeles | Documento intenta inducir copia o contenido queda expuesto | Copia o pegado solo tras gesto explícito; sin observador, historial ni red | Tests de selección y QA de atajos | La persona puede pegar texto voluntariamente en el editor |
| Filtración por imagen | Pixel remoto registra IP | Bloqueo y consentimiento aislado | Monitor de sockets | IP revelada tras consentimiento |
| SSRF | Imagen apunta al router local | Bloqueo de red privada y redirects | Servidor de prueba local | Variantes de resolución DNS |
| Phishing | Texto dice un dominio y URL abre otro | Destino real visible y esquema permitido | QA visual y casos Unicode | Usuario acepta destino malicioso |
| Corrupción al guardar | Corte durante escritura | Temporal y reemplazo atómico | Fault injection | Fallos del filesystem |
| Pérdida por round-trip | Sintaxis desconocida desaparece | Parches sobre rangos | Property tests | Ediciones estructurales complejas |
| Índice malicioso | Bóveda enorme o cambiante | Límites, incremental y cancelación | Corpus de workspace | Alto coste legítimo |
| Sidecar desalineado | Nota cambia y resaltado apunta a otro texto | Versión, contexto y detección | Tests de edición | Ambigüedad de texto repetido |
| Supply chain | Crate o fuente comprometidos | Lock, audit, hashes, SBOM y revisión | Pipeline de release | Vulnerabilidad desconocida |
| Confusión de permisos | Bóveda confiable parece habilitar todo | Alcance visible y garantías fijas | QA de configuración | Error humano |

## STRIDE aplicado

STRIDE es una lista de categorías para no olvidar clases de amenaza:

- Suplantación: enlace o archivo aparenta otra identidad.
- Manipulación: documento, índice o sidecar se modifica sin detectarlo.
- Repudio: falta evidencia para saber qué acción ocurrió.
- Divulgación: contenido, path o IP salen del ámbito esperado.
- Denegación de servicio: CPU, memoria, pila o disco se agotan.
- Elevación de privilegios: contenido obtiene permisos de disco, red o ejecución.

No se usa como sustituto de escenarios concretos. Sirve como control de
completitud.

## Abuso de funciones legítimas

Algunas amenazas aparecen sin explotar un bug:

- un link legítimo lleva a una página de phishing;
- una imagen confirmada revela IP;
- una bóveda legítima contiene millones de archivos;
- aumentar el límite de tamaño hace lenta una operación;
- exportar un documento crea un archivo enorme.

Los avisos, presupuestos y permisos deben cubrir también abuso de funciones
correctas.

## Riesgos abiertos durante la recuperación

- VFS conectado a navegación e indexado, con QA adversarial de junctions y
  diferencias de plataforma todavía pendiente;
- los límites absolutos están implementados; falta evidencia sostenida bajo
  presión de memoria y cancelación cooperativa del parseo de un solo documento;
- round-trip y rangos finos todavía no demostrados para toda sintaxis;
- modo seguro sin validación visual end to end;
- red no verificada mediante monitor;
- renderer sin accesibilidad demostrada;
- los resultados de parseo viejos se descartan por identidad y revisión; falta
  cancelación cooperativa para ahorrar el trabajo de una tarea ya obsoleta;
- dependencia transitiva no mantenida;
- Linux sin evidencia equivalente a Windows.

## Criterio de aceptación

Una amenaza pasa de abierta a controlada cuando:

1. existe una política clara;
2. la arquitectura tiene un punto responsable;
3. hay prueba que intenta evadirla;
4. la evidencia corre en las plataformas relevantes;
5. el riesgo residual está documentado.

Ver [`test-matrix.md`](test-matrix.md).
