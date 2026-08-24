# Producto

Qué hace la v2 y, sobre todo, dónde termina el alcance de la primera versión.
El pecado que este documento existe para evitar es prometer todo lo de la
competencia junta y no entregar nada terminado.

## Alcance de la v2.0 (lo que se construye primero)

### Lectura
- CommonMark + GFM completo: encabezados, listas anidadas y numeradas, tablas
  con alineación, tareas, notas al pie, tachado, autolinks, citas, bloques de
  código con resaltado de sintaxis.
- Alertas de GitHub (`> [!NOTE]`, `TIP`, `IMPORTANT`, `WARNING`, `CAUTION`).
- Callouts de Obsidian (`> [!info]`, etc.), que son casi los mismos.
- Imágenes locales (remotas bloqueadas por defecto, como en la v1).
- Índice lateral desde los encabezados.
- Temas claro y oscuro; tipografía y tamaño ajustables.

### Edición
- Editor de texto plano con barra de ayudas y atajos (la base de la v1).
- Vista dividida con scroll sincronizado.
- Continuación de listas, indentado con Tab, pegar URL sobre selección.

### Workspace (lo nuevo frente a la v1)
- Abrir una **carpeta** como espacio de trabajo, no solo archivos sueltos.
- Barra lateral con el árbol de archivos.
- Recientes y favoritos que sobreviven entre sesiones.
- Pestañas y ventanas (heredado de la v1).

### Conexión con segundos cerebros (ver `conectividad.md`)
- Abrir una bóveda de Obsidian: entender `[[wikilinks]]`, navegarlos, y avisar
  cuando un enlace apunta a una nota que no existe.
- Abrir un repo de GitHub clonado: enlaces relativos correctos, GFM fiel.

### Seguridad (heredada y reforzada)
- Las cuatro propiedades de la v1, ahora por construcción (ver `audit.md`).

## Fuera del alcance de la v2.0 (con motivo)

| Función | Por qué se pospone |
| --- | --- |
| Mermaid nativo | El ítem más caro del proyecto (ADR-5). La v2.0 muestra la fuente del diagrama en un bloque con estilo. Render real, como componente opcional posterior |
| Matemática (KaTeX) nativo | Igual de pesado, sin librería madura clara. Mismo trato que Mermaid |
| IA de estudio | Componente opt-in aparte, no núcleo (ver `inference.md`). No entra en la v2.0 |
| Repaso espaciado / tarjetas | Depende de que el workspace y los wikilinks estén sólidos primero (ver `brainstorm-estudio.md`). Fase posterior |
| Colaboración / compartir en vivo | Rompe la política de red si no es estrictamente aislado. No es el problema que resolvemos |
| Plugins de terceros | Multiplica la superficie de confianza justo donde la queremos angosta |

## El principio de alcance

Cada función de la columna "fuera de alcance" tiene un motivo escrito, no es un
"quizás algún día" vago. La v2.0 hace **menos** que ThisIs-Developer/Markdown-Viewer
a propósito: hace lo esencial, terminado y liviano, en vez de todo a medias.
Lo que sí tiene y ellos no —wikilinks de Obsidian de primera clase, seguridad
por construcción, <7 MB— es lo que la vuelve una elección real, no una copia
más chica.

## Comparación de posición

| | Tinta | ThisIs-Dev | Obsidian | **Visor MD v2** |
| --- | --- | --- | --- | --- |
| Nativo / liviano | Sí (1,9 MB) | No (web) | No (Electron) | **Sí (<7 MB)** |
| Seguridad declarada | No documentada | Parcial (manda a servicios externos) | Buena | **Por construcción** |
| Workspace persistente | Sí | Sí | Sí | **Sí** |
| Wikilinks de Obsidian | No | No | Es el creador | **Sí, sin ser Obsidian** |
| Mermaid nativo | Sí (22 tipos) | Sí (vía externos) | Sí (plugin) | **No en 2.0 (fuente visible)** |
| Predeterminado de Windows | Sí | No | No | **Sí** |

La casilla donde la v2 es única: **liviano + seguro por construcción +
entiende Obsidian sin ser Obsidian**. Ninguna otra la ocupa.
