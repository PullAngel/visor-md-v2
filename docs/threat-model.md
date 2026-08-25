# Auditoría y modelo de amenaza

Este documento traslada la frontera de seguridad de la v1
(`docs/frontera-de-seguridad.md` en el repo de la v1) al diseño nativo, y
muestra dónde el enfoque nativo mejora la postura y dónde abre riesgos nuevos
que hay que cubrir.

## El supuesto no cambia

Todo `.md` es contenido ajeno y potencialmente hostil hasta demostrar lo
contrario. Llega de un repo, de un conversor de PDF, de una IA. Puede estar
diseñado para atacar al lector, o simplemente malformado por un conversor
defectuoso.

## Las cuatro propiedades, en versión nativa

| Propiedad (v1) | Cómo se cumple en la v2 nativa |
| --- | --- |
| 1. Abrir no genera peticiones de red | La capa de archivos es la única que podría tocar la red, y no lo hace. No hay CSS con `url()`, ni `srcset`, ni SVG con `<image href>` que dispare una petición: nada de eso existe porque no hay motor web. La propiedad pasa de "garantizada por sanitización cuidadosa" a "imposible por ausencia de la capacidad" |
| 2. Nada del documento se ejecuta | No hay intérprete de JavaScript en el proceso. Un `<script>` en el `.md` es texto inerte porque no hay dónde ejecutarlo. La propiedad pasa de "sanitizamos scripts" a "no existe un motor de scripts" |
| 3. Un documento no puede leer archivos ajenos | Se hereda `safe_media_path`: rutas canonizadas, red rechazada, ADS de NTFS rechazado. Idéntico a la v1 |
| 4. Un documento no puede disfrazarse de la app | La superficie del documento se dibuja en su propia región; no puede pintar sobre el chrome porque no comparte un DOM con él. Más fuerte que el `contain: paint` de la v1 |

## Dónde el enfoque nativo es estrictamente mejor

- **Desaparece la superficie de scripts.** La mitad del trabajo de seguridad de
  la v1 (DOMPurify, CSP, allowlist de protocolos, aislar Mermaid) existía para
  contener un motor de scripts. Sin ese motor, esa mitad no hace falta: no se
  contiene lo que no está.
- **Desaparece la clase "fuga de red por recurso indirecto".** Los tres
  hallazgos que la suite de la v1 encontró en su primera corrida (`srcset`,
  SVG inline, `background-image`) eran vías de red abiertas por el motor web.
  En nativo no existen: no hay CSS ni HTML que las active.

## Dónde el enfoque nativo abre riesgos NUEVOS que hay que cubrir

Esto es lo que un enfoque nativo ingenuo pasaría por alto, y por lo que la
elección de lenguaje importa:

- **Corrupción de memoria en el parser.** Un lector nativo en C/C++ que parsea
  entrada no confiable es terreno clásico de desbordamientos de búfer: el
  CVE-2026-5525 de Notepad++ es exactamente esto. **Mitigación: Rust.** La
  seguridad de memoria del lenguaje elimina la clase entera. Es la razón de
  seguridad detrás del ADR-2, no una preferencia estética.
- **Bombas de recursos.** Un `.md` con anidamiento patológico, una tabla de un
  millón de celdas, o un documento de gigabytes puede agotar CPU o RAM.
  **Mitigación:** parseo en hilo aparte con límites, y los topes de la v1
  (máximo de diagramas, etc.) trasladados a topes de nodos.
- **Rutas y enlaces.** Igual que la v1: cada ruta que propone el documento se
  valida canonizada antes de tocar disco. Los enlaces `file://`, `\\servidor`,
  `javascript:` y esquemas raros se rechazan por allowlist.
- **Dependencias nativas.** Cada crate de Rust es código de terceros en el
  binario. **Mitigación:** `cargo audit` en el pipeline, y minimizar el árbol
  de dependencias (que además ayuda al presupuesto de 7 MB).

## Lo que la suite de pruebas debe demostrar

Igual que en la v1, se afirman **propiedades**, no ausencia de crash. El corpus
de ataque de la v1 (`tests/security/`) se traslada tal cual, y las
comprobaciones se reescriben para el renderizador nativo:

- Ningún `.md` del corpus produce una petición de red (se observa el socket, no
  se confía en la ausencia).
- Ninguna ruta del corpus de traversal se resuelve fuera de su carpeta.
- El corpus de "conversión defectuosa" renderiza entero sin panic.
- Un documento con anidamiento patológico se corta en el tope, no cuelga.
- El parser sobrevive a entrada aleatoria (fuzzing con `cargo-fuzz`), posible en
  Rust de forma barata, y algo que la v1 no podía hacer sobre su pipeline JS con
  la misma facilidad.

## Riesgo residual

- Dependiente del sistema operativo: la seguridad de las llamadas de dibujo y
  de las fuentes del sistema queda en manos de Windows. Fuera del modelo.
- Dependiente de crate externo: un fallo en `comrak`, `parley` o `tiny-skia`.
  `cargo audit` lo detecta; Rust acota el daño posible.
- Agotar memoria con un documento gigante sigue siendo posible pese a los
  límites; el usuario cierra la pestaña. Aceptado, como en la v1.
