# Roadmap

Fases concretas y en orden. No hay fechas: es planificación, y el ritmo lo pone
el tiempo real disponible. Cada fase termina con algo verificable antes de
pasar a la siguiente.

## Fase 0 — Prototipo y validación del presupuesto

**La fase que puede cambiar todo el plan, por eso va primero.**

- Prototipo mínimo en Rust: abrir un `.md`, parsear con `comrak`, dibujar
  encabezados, párrafos, listas y tablas con `parley` + `tiny-skia`.
- Medir contra `calculos.md`: tamaño del binario, arranque, primer pintado, RAM.
- Probar con el corpus de la v1 (`tests/estres.md`, `conversion-defectuosa.md`).
- Decidir con números reales: backend de dibujo (software vs GPU), toolkit de
  chrome (sí o dibujo propio), estrategia de resaltado de sintaxis.

**Criterio de salida:** un binario que abre un documento simple, medido, por
debajo de 7 MB con margen. Si no se llega, se replantea alcance o presupuesto
acá, antes de construir nada grande encima.

## Fase 1 — Lector completo

- CommonMark + GFM completo sobre la vista de documento.
- Resaltado de sintaxis en bloques de código.
- Alertas de GitHub y callouts de Obsidian.
- Imágenes locales (remotas bloqueadas por defecto).
- Temas claro/oscuro, tipografía ajustable, índice lateral.
- Suite de seguridad de la v1 portada y en verde.

**Criterio de salida:** abre y muestra fielmente el corpus de la v1, con las
cuatro propiedades de seguridad verificadas por pruebas.

## Fase 2 — Chrome y workspace

- Pestañas y ventanas.
- Abrir una carpeta como workspace, barra lateral con el árbol.
- Recientes y favoritos persistentes.
- Búsqueda en toda la carpeta.
- Menú contextual, configuración.

**Criterio de salida:** usable como herramienta de trabajo diaria, no solo como
visor de un archivo suelto.

## Fase 3 — Conexión con segundos cerebros

- Wikilinks de Obsidian: resolver, navegar, marcar rotos.
- Enlaces a encabezados y bloques; embeds como enlace destacado.
- Backlinks (qué notas enlazan a la actual).
- Repo de GitHub: enlaces relativos, raíz del repo, README automático.

**Criterio de salida:** abrir una bóveda de Obsidian real y navegarla con los
wikilinks funcionando.

## Fase 4 — Edición

- Editor de texto plano con barra de ayudas y atajos (base de la v1).
- Vista dividida con scroll sincronizado.
- Ayudas al escribir: listas automáticas, indentado, pegar URL sobre selección.
- Edición estructural incremental: renombrar encabezado actualiza enlaces
  internos; pegar imagen la guarda y arma el enlace.

**Criterio de salida:** paridad de edición con la v1, más las ayudas
estructurales que la v1 no tiene.

## Fase 5 — Distribución

- Empaquetado portable <7 MB.
- Registro como programa predeterminado de Windows para `.md`.
- Instalación en el equipo, desinstalación limpia.
- Evaluar firma vía Microsoft Store para evitar SmartScreen (lección de Tinta).

**Criterio de salida:** un release descargable, verificable por hash, que se
fija como predeterminado igual que la v1.

## Más allá — sujeto a que las fases anteriores cierren bien

- Repaso espaciado desde el documento (ver `brainstorm-estudio.md`).
- Modo estudio / foco, resaltado persistente.
- Componente opcional de Mermaid nativo, si aparece una vía local viable.
- Componente opcional de IA local (ver `inference.md`).
- Grafo de notas, solo si la pila de dibujo demostró que rinde.

## Regla que gobierna el roadmap

Ninguna fase avanza sin que la anterior tenga su criterio de salida cumplido y
sus pruebas en verde. Es la disciplina que evita el destino de simpler-paper
(archivado por un solo mantenedor sin red de seguridad): alcance acotado por
fase, pruebas por fase, y honestidad para frenar en la Fase 0 si los números no
dan.
