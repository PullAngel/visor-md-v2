# Producto

Qué hace la v2 y dónde termina el alcance de la primera versión. El pecado que
este documento evita es prometer todo lo de la competencia junta y no entregar
nada terminado.

## Los cuatro modos de vista

| Modo | Qué es | En la v2.0 |
| --- | --- | --- |
| **Lectura** | Documento renderizado, con resaltado y anotaciones | Sí |
| **Fuente** | Texto plano con las marcas visibles | Sí |
| **Dividido** | Fuente y lectura lado a lado, scroll sincronizado | Sí |
| **Edición en vivo** | Escribir sobre el documento renderizado | **No** — primer objetivo grande después |

El razonamiento sobre la edición en vivo está en `architecture.md`: es
técnicamente el modo más difícil y hacerlo a medias es peor que no tenerlo.

## Alcance de la v2.0

### Lectura
CommonMark + GFM completo. Alertas de GitHub y callouts de Obsidian. Resaltado
de sintaxis. Imágenes locales, remotas bloqueadas por defecto. Índice lateral
con contador de palabras y tiempo estimado al pie. Plegado de secciones. Temas
claro y oscuro. Tipografía ajustable.

Mermaid: la fuente del diagrama se muestra en un bloque con estilo. El render
nativo es el segundo objetivo grande después de la v2.0 —y la única función por
la que vale subir el presupuesto de tamaño.

### Edición
Editor de texto plano con barra de ayudas y atajos. Vista dividida con scroll
sincronizado. Listas automáticas, indentado, pegar URL sobre selección. Pegar
imagen del portapapeles. Guardado atómico preservando codificación y fin de
línea.

**Sin autoguardado por defecto.** Las modificaciones no tocan el original hasta
que guardás. La recuperación ante cierre inesperado usa un archivo temporal
aparte. Se puede activar el autoguardado desde configuración avanzada.

### Ventanas y pestañas
Pestañas y ventanas al estilo navegador, con arrastre entre ellas. Dividir a la
derecha y abajo, abriendo una pestaña **nueva** con la opción de crear archivo,
abrir archivo o cerrar —no un duplicado. Menú contextual por zona. Siempre
encima. Pantalla completa.

### Workspace
Abrir una carpeta como espacio de trabajo, con árbol lateral. Recientes y
favoritos persistentes. Búsqueda en toda la carpeta. Sesión restaurada.

### Segundos cerebros
Wikilinks, backlinks, callouts, `==resaltado==`, frontmatter. Repos clonados con
enlaces relativos y README automático. Ver `connectivity.md`.

### Estudio
Resaltado persistente desde modo lectura. Repaso espaciado en su forma simple.
Pomodoro. Exportar a PDF directo.

### Seguridad
Las cuatro propiedades de la v1, ahora por construcción. Ver `security.md`.

## Anotaciones: el formato

El sidecar es un archivo junto a la nota, con nombre `nota.md.anot`. Guarda:

- Resaltados: rango de caracteres, color, y el texto resaltado como respaldo.
- Estado de repaso: qué se marcó, cuándo se vio, cuándo toca de nuevo.

**Por qué guarda el texto además del rango:** si la nota se edita por fuera, el
rango deja de servir. Con el texto se puede reubicar el resaltado; si no se
encuentra, se avisa en vez de perderlo en silencio.

Es texto plano, tipado y simple, sin rutas adentro. Se trata como entrada no
confiable igual que el `.md`.

## Fuera del alcance de la v2.0

| Función | Por qué |
| --- | --- |
| Edición en vivo | El modo más difícil; a medias es peor que nada |
| Mermaid nativo | El ítem más caro del proyecto |
| KaTeX nativo | Igual de pesado; iría como plugin descargable |
| Corrector ortográfico | 2 MB en disco y ~4,5 MB de RAM por idioma. Componente aparte |
| IA local | Componente opt-in aparte |
| Grafo de notas | Más bonito que útil |
| Crear referencias de bloque | Cambia el modelo de datos |
| Colaboración en vivo | Rompe la política de red |
| Plugins de terceros | Multiplica la superficie de confianza |
| macOS publicado | Se compila y prueba, pero no se publicita todavía |

## Renombrar un encabezado y sus enlaces

Pediste revisar si choca con Obsidian. **Sí choca, parcialmente**, y por eso se
pospone: Obsidian tiene su propia lógica de actualización de enlaces al
renombrar, y dos herramientas reescribiendo los mismos archivos con criterios
distintos es una receta para corromper una bóveda.

**Decisión: fuera de la v2.0.** Si entra después, será con confirmación
explícita y mostrando qué archivos va a tocar antes de tocarlos.

## Comparación de posición

| | Tinta | ThisIs-Dev | Obsidian | **Visor MD v2** |
| --- | --- | --- | --- | --- |
| Nativo y liviano | Sí (1,9 MB) | No (web) | No (Electron) | **Sí (<7 MB)** |
| Seguridad declarada | No documentada | Parcial | Buena | **Por construcción** |
| Workspace | Sí | Sí | Sí | **Sí** |
| Wikilinks | No | No | Es el creador | **Sí, sin ser Obsidian** |
| Herramientas de estudio | No | No | Vía plugins | **Integradas** |
| Mermaid nativo | Sí | Vía servicios externos | Plugin | **No en 2.0** |
| Predeterminado del sistema | Sí | No | No | **Sí** |
| Linux | No | Vía web | Sí | **Sí** |

La casilla donde la v2 está sola: **liviano + seguro por construcción + entiende
Obsidian + sirve para estudiar**.
