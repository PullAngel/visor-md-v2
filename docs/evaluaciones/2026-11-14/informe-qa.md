# Informe de QA de UI/UX: Visor MD (build de evaluación)

Evaluadora: QA manual sénior de UI/UX. Fecha: 2026-11-14. Copia evaluada: `qa`.
Capturas en `capturas\` (los nombres `z-*` son ampliaciones y los `m-*` son montajes de varias capturas).

---

## 1. Resumen ejecutivo

**Calificación global de UX: 6/10.** Visor MD tiene una base visual cuidada: buena tipografía de lectura, tema de noche con contraste alto, un diálogo de cierre ejemplar, una paleta de acciones potente y un foco de teclado visible en las barras. Sin embargo, la interacción falla en varios puntos básicos y el lenguaje no está pensado para su público, que son estudiantes que no saben Markdown.

Los 5 problemas principales:
1. **QA-01 (S1).** No pude escribir en los campos de búsqueda, ni en el documento ni en la carpeta. R12 y la mitad de R13 quedaron sin poder completarse.
2. **QA-03 (S2).** La rueda del mouse sobre menús, la paleta o el índice lateral mueve el documento de fondo en vez de la lista. Las listas largas solo se recorren con flechas.
3. **QA-02 (S2).** El botón Índice no cierra su panel con un segundo clic y deja de mostrar que está activo (N3).
4. **QA-04 y QA-05 (S2).** Los enlaces web no se abren con clic ni con Enter y la app no indica cómo abrirlos. Los enlaces rotos se ven igual que los válidos.
5. **QA-14 y QA-15 (S3).** El microcopy es técnico o está en inglés («Workspace indexado… presupuesto de memoria», «guardado de forma atómica», «enlace de bóveda», «callout», «Backlinks»). Además, muchos avisos son redundantes y tapan contenido.

**Hallazgos:** 41 en total (S1: 1, S2: 6, S3: 29, S4: 5).

---

## 2. Alcance y entorno

- **Tamaños.** 1100×720 (base), 640×480 (mínimo) y maximizada con el botón propio de la app (1366×738).
- **Temas.** Noche (el que trae por defecto) y día. Los dos se revisaron en los tres tamaños.
- **Contenido.** `inicio.md`, `historia.md`, `biologia\celula.md`, `manual-largo.md` (49 títulos) y documentos nuevos (`prueba-qa.md` y `prueba-qa-copia.md`, creados en `apuntes\`). Exporté `inicio-export.pdf` e `inicio-export2.pdf`.
- **Volumen.** Unas 210 interacciones (clics, movimientos del puntero, teclas, rueda, arrastres y escritura) y 123 capturas principales más ampliaciones.
- **Limitaciones de la herramienta (no se reportan como defectos):**
  - No llegan Ctrl, Alt ni Shift, así que los atajos solo se verificaron como comunicados en la interfaz.
  - No se pueden escribir mayúsculas ni tildes.
  - Los diálogos de archivo están simulados.
  - El hover queda en el último punto, por lo que algunos tooltips persistentes pueden deberse en parte a esto. Solo reporto los casos en que el tooltip siguió visible tras mover el puntero a otra zona de la ventana.
  - No se pudo provocar un cierre abrupto real (R21).
- **Contraste.** Lo estimé leyendo los píxeles de las capturas: el color de fondo dominante frente al píxel de texto más contrastado. Es una cota superior aproximada, porque el antialias puede reducir el contraste real.

---

## 3. Matriz de requerimientos

| ID | Dictamen | Evidencia | Nota |
| --- | --- | --- | --- |
| R1 | Cumple | 09-abrir-inicio | Abre en Leer con buen formato. Ver QA-10 y QA-11 por defectos de render. |
| R2 | Cumple | 43-editor-type, 88-guardado, 123-guardar-como | Guardar con nombre y Guardar como funcionan. El aviso de Guardar como es técnico (QA-14). |
| R3 | Cumple | 67-comparar, 68-comparar-rueda, 96-640-editar | Los tres modos existen. Comparar se sincroniza con la rueda. |
| R4 | Cumple parcialmente | 32, 33, 77-pestana-fijada, 78-reordenar, 79/80 (posición) | Abrir, cambiar, cerrar, reordenar y recordar la posición funcionan. La pestaña fijada casi no se distingue y no se ancla al inicio (QA-35). Las pestañas inactivas no tienen hover (QA-19). |
| R5 | Cumple parcialmente | 69-paleta-dividir, 70, 71-dividido2 | Verifiqué «Dividir a la derecha» con otra pestaña y el cierre del panel. No verifiqué «abajo» ni el límite de 4. |
| R6 | Cumple parcialmente | 45-formato, 46/50-menu-*, 115-emoji, 116-tabla-insertada | Están todas las ayudas. Las inserciones no respetan el contexto (QA-23) y algunos íconos son ambiguos (QA-22). |
| R7 | Cumple | 44-listas | Enter continúa la lista y, en un elemento vacío, la termina. |
| R8 | Cumple | 18, 19-copiar-click (portapapeles = solo las 2 líneas del bloque) | Confirma con «Copiado» y además un aviso (doble, QA-15). Sin hover (QA-20). Bloques fusionados (QA-11). |
| R9 | Cumple | 20-tarea-marcada | Un clic marca la casilla y el documento pasa a «sin guardar». |
| R10 | Cumple | 66-plegado | El triángulo cambia ▾/▸. El objetivo de clic es muy chico (unos 8 px). |
| R11 | Cumple parcialmente | 10-indice, 62/64-indice-flotante, 76 | Los saltos funcionan. El panel no se cierra con el botón (QA-02), el resaltado del flotante va desfasado (QA-12) y el texto desborda (QA-13). |
| R12 | No cumple (no pude completar la tarea) | 41-buscar-doc, 42-buscar-doc2, 108/109 | La caja aparece, pero no recibe texto. La misma herramienta sí escribe en el editor y en la paleta. QA-01. |
| R13 | Cumple parcialmente | 29-carpeta, 30-archivos, 33-click-celula, 34-backlinks, 38/39/40 | Elegir carpeta, árbol, Backlinks y `[[nota]]` funcionan. Buscar en la carpeta no acepta texto (QA-01). Sin carpeta, `[[nota]]` no hace nada (QA-06). |
| R14 | Cumple parcialmente | 21-hover-wiki, 25-rclick-link, 26/27-roto | Al pasar el puntero se muestra el destino real. Los enlaces rotos no se distinguen (QA-05) y los web no se abren ni indican cómo (QA-04). |
| R15 | Cumple | 50-menu-acciones, 52-paleta-tema, 69 | Tiene 69 acciones y se filtra escribiendo. Problemas de rueda y paginación (QA-03, QA-41) y «1 resultados». |
| R16 | Cumple parcialmente | 25, 76-menu-pestana, 103-ctx-tabla, 104-ctx-texto, 107 | El menú cambia según texto, tabla y pestaña, pero en lectura solo ofrece copiar. No hay acciones de tarea ni de tabla para editar, y en el editor no hay Copiar ni Cortar (QA-32). |
| R17 | Cumple parcialmente | 53/54 (día), 81-tecla-t, 117-mov-reducido | El cambio de tema funciona. «Movimiento reducido» no muestra estado ni confirmación (QA-27). No hay control visible de tema (QA-36). |
| R18 | Cumple | 74-fijar, z-74-fijar, 75 | El aviso es claro. El estado activo se confunde con el hover (QA-21). |
| R19 | Cumple | 93/94-pdf, inicio-export2.pdf | El PDF se ve bien. La capa de texto sale desordenada (QA-37). |
| R20 | Cumple | 90-cerrar-sucio, 92-esc-dialogo, 120-cerrar-app | Diálogo propio, verbos claros y Esc elige la opción segura. El diálogo de la app es incoherente con el de la pestaña (QA-25). |
| R21 | Cumple parcialmente / no verificable del todo | 01-primer-arranque, 118, 119 | El aviso aparece al primer arranque, se puede desactivar desde la paleta y existe «Abrir recuperación». No pude provocar un cierre abrupto. La restauración es opaca (QA-26). |
| R22 | Cumple parcialmente | 43, 44, 88, z-106-contador, 99 | Están todos los campos, pero palabras y caracteres no se actualizan al editar (QA-07) y un documento nuevo figura como «guardado» (QA-28). |
| R23 | Cumple parcialmente | m-tooltips (atajos), m-f6, m-teclado, 82-tab-3 | Los atajos se comunican en tooltips y en la paleta. El foco es visible en las barras. El foco sobre enlaces es débil y F6 no llega a las pestañas. Los atajos con Ctrl no se pudieron verificar. |
| N1 | Cumple parcialmente | 95-640-leer, 96, 97, 99, 61-maximizada | A 640×480 no hay controles ocultos ni encimados. Las etiquetas pasan a íconos y las pestañas se truncan. Maximizada deja un gran vacío a la derecha (QA-33). |
| N2 | Cumple parcialmente | m-tooltips, z-100-hover-tab, z-18-hover-copiar | Los botones de barra tienen hover. Pestañas inactivas y Copiar no (QA-19, QA-20). |
| N3 | Cumple parcialmente | 46/47 (Bloques sí cierra), 10–16 (Índice no) | Los desplegables se cierran con un segundo clic. El panel Índice no, y el botón pierde el estado activo (QA-02). |
| N4 | No cumple | 48-bloques-rueda, 57/58-paleta, 111-largo-rueda-indice | La rueda no recorre las listas. Las flechas sí, pero PgDn y Fin no (QA-03). |
| N5 | Cumple parcialmente | medición de contraste (sección 5, QA-18) | Noche: cuerpo 14,9:1 y secundario unos 6,4–7,4:1. Día: cuerpo 13,1:1, pero secundario e íconos de 3,9 a 4,7:1 y enlaces 5,5:1. |
| N6 | Cumple parcialmente | 29, 27, 50-menu-insertar, 123, 35 | Jerga, mezcla de inglés, de voseo y tuteo, y errores de concordancia (QA-14). |
| N7 | Cumple parcialmente | 03, 10, 46, 81, 113, 96 | Avisos redundantes que se superponen a texto o código (QA-15). |
| N8 | Cumple parcialmente | 46 frente a 76, 90 frente a 120, 71 | Sombras, menús, diálogos y selectores de modo no son coherentes entre sí (QA-16, QA-25, QA-34). |

---

## 4. Evaluación heurística (Nielsen)

| # | Heurística | Puntaje (1–5) |
| --- | --- | --- |
| H1 | Visibilidad del estado del sistema | 3 |
| H2 | Relación entre el sistema y el mundo real | 2 |
| H3 | Control y libertad del usuario | 4 |
| H4 | Consistencia y estándares | 2 |
| H5 | Prevención de errores | 3 |
| H6 | Reconocer antes que recordar | 3 |
| H7 | Flexibilidad y eficiencia de uso | 4 |
| H8 | Diseño estético y minimalista | 3 |
| H9 | Ayudar a reconocer, diagnosticar y recuperarse de errores | 2 |
| H10 | Ayuda y documentación | 3 |
| | **Promedio** | **2,9** |

**H1. Visibilidad del estado (3).**
- Lo bueno: el asterisco en las pestañas modificadas, «guardado» y «sin guardar» en la barra de estado, la ruta tipo migas de pan, el aviso de fijado y el botón «Copiado».
- Los problemas:
  - El contador no se actualiza al editar (QA-07).
  - Un documento nuevo figura como «guardado» (QA-28).
  - No hay barra de desplazamiento en documentos largos (QA-24).
  - El índice flotante marca la sección equivocada (QA-12).
  - El botón Índice no refleja que el panel está abierto (QA-02).
  - Las alternancias no muestran su estado (QA-27).

**H2. Mundo real (2).**
- El público no sabe Markdown, pero la interfaz dice «callout», «bóveda», «Workspace», «atómica», «TSV», «Backlinks», «tabla Markdown» y «presupuesto de memoria» (QA-14).
- Los íconos de resaltado (H) y de código en línea (N tachada) no remiten a nada conocido (QA-22).

**H3. Control y libertad (4).**
- Esc cierra menús, paneles y diálogos de forma consistente. El diálogo de cambios sin guardar tiene «Cancelar» como salida segura, y existe una recuperación local.
- Resta puntos que el panel Índice no se cierre desde su botón y que la tecla T cambie el tema sin querer.

**H4. Consistencia (2).**
- Las sombras se aplican de forma desigual (QA-16).
- Hay dos colores de enlace (QA-17).
- Los menús no se anclan a su botón (QA-08).
- Los rótulos cambian según dónde aparezcan: «Bloques y estructura» frente a «Bloques y encabezados», e «Insertar» frente a «Insertar enlaces y bloques».
- Hay dos diálogos de cierre distintos (QA-25).
- Las rutas usan barra invertida o «›» según el lugar.
- Se mezcla voseo con tuteo.

**H5. Prevención de errores (3).**
- Lo bueno: el diálogo de cierre deja «Guardar» como primario y «Cerrar sin guardar» con color de peligro.
- Los problemas: las inserciones de la barra rompen la estructura (QA-23) y una sola tecla («T») cambia el tema en lectura (QA-36).

**H6. Reconocer antes que recordar (3).**
- Los tooltips incluyen el atajo y la paleta lista todas las acciones con sus atajos.
- En contra: íconos sin texto ambiguos o repetidos para acciones distintas (QA-22), sin botón visible de tema y sin pista de cómo abrir un enlace web.

**H7. Flexibilidad (4).**
- Paleta filtrable, F2, F3 y F6, flechas en las listas, paneles divididos, índice flotante y menús contextuales para copiar en varios formatos.
- Falta la rueda en las listas, y Fin y PgDn en los menús.

**H8. Estética (3).**
- Lo bueno: buena tipografía serif para leer y una paleta sobria.
- Los problemas: barras con muchos íconos pequeños, marcas de desplegable diminutas encimadas a las palabras, avisos de más, callout con la etiqueta pisada, columna izquierda con vacío a la derecha al maximizar, y tablas en monoespaciada dentro de un cuerpo serif.

**H9. Errores (2).**
- Hacer clic en un enlace roto, en un enlace web o en `[[nota]]` sin carpeta no produce respuesta alguna.
- La búsqueda dice «Sin resultados» antes de que se escriba nada.
- No hay mensajes que expliquen por qué algo no ocurrió.

**H10. Ayuda (3).**
- Los textos de ayuda en los avisos y al pie de las listas («flechas eligen, Enter abre, Escape cierra») orientan, aunque a veces se repiten.
- El primer arranque no guía: abre en Editar con un documento vacío y un aviso técnico (QA-40).

---

## 5. Hallazgos

### 5.1 Tabla resumen

| ID | Título | Área | Severidad | Frecuencia |
| --- | --- | --- | --- | --- |
| QA-01 | Los campos de búsqueda no aceptan texto | Búsqueda | S1 | siempre |
| QA-02 | El botón Índice no cierra su panel y pierde el estado activo | Paneles | S2 | siempre |
| QA-03 | La rueda desplaza el documento, no la lista abierta; PgDn y Fin no funcionan | Menús y listas | S2 | siempre |
| QA-04 | Los enlaces web no se abren y no se indica cómo | Enlaces | S2 | siempre |
| QA-05 | Los enlaces rotos no se distinguen y no dan respuesta | Enlaces | S2 | siempre |
| QA-06 | `[[nota]]` sin carpeta de trabajo no hace nada, sin explicación | Enlaces | S2 | siempre |
| QA-07 | Palabras y caracteres no se actualizan al editar | Barra de estado | S2 | siempre |
| QA-08 | Los desplegables no se anclan a su botón | Menús | S3 | siempre |
| QA-09 | Tooltips persistentes que tapan contenido o quedan detrás del panel | Tooltips | S3 | a veces |
| QA-10 | La etiqueta «Nota» del callout queda encimada con la barra | Render | S3 | siempre |
| QA-11 | Dos bloques de código seguidos se ven fusionados | Render | S3 | siempre |
| QA-12 | El índice flotante resalta la sección equivocada | Índice | S3 | a veces |
| QA-13 | El texto del índice flotante desborda la caja | Índice | S3 | siempre |
| QA-14 | Microcopy técnico, en inglés o inconsistente | Microcopy | S3 | siempre |
| QA-15 | Avisos redundantes que tapan contenido | Avisos | S3 | siempre |
| QA-16 | Sombras inconsistentes entre elementos flotantes | Visual | S3 | siempre |
| QA-17 | Segundo color de acento (enlaces externos azules) | Visual | S3 | siempre |
| QA-18 | Contraste insuficiente de secundarios e íconos en tema de día | Contraste | S3 | siempre |
| QA-19 | Pestañas inactivas sin hover, sin X y sin nombre completo | Pestañas | S3 | siempre |
| QA-20 | El botón Copiar no tiene hover y su estilo es distinto | Botones | S3 | siempre |
| QA-21 | «Fijar encima» activo se confunde con el hover | Ventana | S3 | siempre |
| QA-22 | Iconografía ambigua o repetida; marcas de desplegable encimadas | Iconos | S3 | siempre |
| QA-23 | Las inserciones de la barra no respetan el contexto | Edición | S3 | siempre |
| QA-24 | No hay barra de desplazamiento en el documento | Navegación | S3 | siempre |
| QA-25 | Diálogo de cierre de la app incoherente con el de pestaña | Diálogos | S3 | siempre |
| QA-26 | «Abrir recuperación» es opaco | Recuperación | S3 | siempre |
| QA-27 | Alternancias sin estado ni confirmación | Paleta | S3 | siempre |
| QA-28 | Un documento nuevo aparece como «guardado» | Barra de estado | S3 | siempre |
| QA-29 | Árbol de archivos y Backlinks inconsistentes | Carpeta | S3 | siempre |
| QA-30 | La búsqueda muestra «Sin resultados» antes de escribir; el campo no parece un campo | Búsqueda | S3 | siempre |
| QA-31 | La caja «Buscar en documento» tapa el texto | Búsqueda | S3 | siempre |
| QA-32 | Menú contextual del editor: desborda y le faltan acciones | Menú contextual | S3 | siempre |
| QA-33 | Maximizada: gran vacío a la derecha; el ícono no pasa a «restaurar» | Layout | S4 | siempre |
| QA-34 | El selector de modo tiene padding desigual y se duplica con otro estilo | Visual | S4 | siempre |
| QA-35 | La pestaña fijada casi no se distingue | Pestañas | S4 | siempre |
| QA-36 | La tecla «T» cambia el tema; no hay control visible de tema | Tema | S3 | siempre |
| QA-37 | El PDF exportado tiene la capa de texto desordenada | Exportar | S3 | siempre |
| QA-38 | Lista de símbolos con rótulos desalineados | Menús | S4 | siempre |
| QA-39 | Jerarquía débil de títulos en el índice | Índice | S4 | siempre |
| QA-40 | El primer arranque no orienta | Onboarding | S3 | una vez |
| QA-41 | Menús de solo 9 filas, sin barra y con paginación a saltos | Menús | S3 | siempre |

### 5.2 Detalle

**QA-01: Los campos de búsqueda no aceptan texto escrito**
- **Área / referencia:** búsqueda; R12, R13, H9.
- **Severidad / frecuencia:** S1, siempre.
- **Pasos:**
  1. Pulsar la lupa «Buscar en documento» (arriba) o abrir Buscar en el panel lateral con la carpeta activa.
  2. Hacer clic en el campo.
  3. Escribir «celula» o «nota».
- **Esperado:** el texto aparece en el campo, se resaltan las coincidencias y se puede ir de una a otra.
- **Observado:** el campo sigue mostrando «Buscar en documento…» o «Buscar en apuntes…» y «Sin resultados». La misma herramienta de escritura sí funciona en el editor (43-editor-type) y en el filtro de la paleta (52-paleta-tema), por lo que el campo no parece recibir el foco ni los caracteres. Lo probé en lectura y en edición, abriendo desde el botón y desde la pestaña del panel, y también con teclas sueltas.
- **Evidencia:** 36, 38-buscar-celula2, 39-buscar-celula3, z-40-buscar-key, 41-buscar-doc, 42-buscar-doc2, 109-buscar-editar-nota.
- **Recomendación:**
  - Dar foco automático al campo al abrir la búsqueda y mostrar un cursor parpadeante con un borde de campo visible.
  - Verificar que el campo recibe los caracteres igual que la paleta.
  - Mientras tanto, la paleta podría ofrecer «Buscar: …» como alternativa.

**QA-02: El botón Índice no cierra su panel y pierde el estado activo**
- **Área / referencia:** paneles; N3, H1.
- **Severidad / frecuencia:** S2, siempre.
- **Pasos:** en Leer, pulsar «Índice del documento» (≡); se abre el panel. Volver a pulsar ≡ dos o tres veces.
- **Esperado:** el segundo clic cierra el panel y el botón se ve presionado mientras el panel está abierto.
- **Observado:** el panel sigue abierto. Tras el primer uso, el botón ya no se ve activo aunque el panel esté abierto. Solo cierran la X, Esc o elegir un elemento. Los botones Archivos y Backlinks del mismo grupo siguen la misma lógica.
- **Evidencia:** 10-indice, 11-indice-2click, 13-indice-3click, 15-indice-click4, 16-indice-click5.
- **Recomendación:** convertir los cinco botones de paneles en interruptores: el segundo clic cierra y el estado activo usa el acento verde de forma persistente.

**QA-03: La rueda desplaza el documento, no la lista abierta**
- **Área / referencia:** menús y listas; N4, H7.
- **Severidad / frecuencia:** S2, siempre.
- **Pasos:**
  1. Abrir la paleta «···» (69 acciones) o «Bloques» (10).
  2. Girar la rueda sobre la lista.
  3. Repetir con el panel Índice de `manual-largo.md` (49 títulos).
  4. Probar PgDn y Fin.
- **Esperado:** la lista se desplaza con la rueda, y PgDn, PgUp, Inicio y Fin funcionan.
- **Observado:** el documento de fondo se desplaza bajo la paleta abierta y la lista queda igual. PgDn y Fin no hacen nada. Solo funcionan las flechas, que van de 9 en 9 o de 17 en 17.
- **Evidencia:** 48-bloques-rueda, 57-paleta-pgdn, 58-paleta-rueda, 60-paleta-end, 111-largo-rueda-indice.
- **Recomendación:** capturar la rueda dentro del elemento flotante o panel que esté bajo el puntero, bloquear el desplazamiento del fondo mientras hay un menú abierto y soportar PgUp, PgDn, Inicio y Fin.

**QA-04: Los enlaces web no se abren y no se indica cómo**
- **Área / referencia:** enlaces; R14, H6, H9.
- **Severidad / frecuencia:** S2, siempre.
- **Pasos:** en `inicio.md` (Leer), hacer clic en «Wikipedia», pulsar Enter con el enlace enfocado y luego clic derecho.
- **Esperado:** se abre el navegador, o una confirmación con el destino; si hace falta Ctrl+clic, se indica.
- **Observado:** el enlace solo queda enfocado (con un recuadro). Ni el clic ni Enter abren nada, y no aparece ningún mensaje. El menú contextual solo ofrece «Copiar texto / Markdown / para Discord», sin «Abrir enlace» ni «Copiar dirección».
- **Evidencia:** 21-hover-wiki, 22, 23-tras-click-wiki, 24-enter-wiki, 25-rclick-link.
- **Recomendación:**
  - Al hacer clic, mostrar una confirmación breve: «Abrir es.wikipedia.org/wiki/Markdown en el navegador · Abrir / Cancelar».
  - Si se requiere Ctrl+clic, decirlo en el aviso de destino («Ctrl+clic para abrir»).
  - Agregar «Abrir enlace» y «Copiar dirección» al menú contextual.

**QA-05: Los enlaces rotos no se distinguen y no dan respuesta**
- **Área / referencia:** enlaces; R14, H9.
- **Severidad / frecuencia:** S2, siempre.
- **Pasos:** ver «nota-que-no-existe» en `inicio.md`, en Leer y con la carpeta activa; pasar el puntero y hacer clic.
- **Esperado:** estilo diferenciado (por ejemplo, subrayado punteado, otro tono o un ícono) y un aviso tipo «La nota no existe · Crearla».
- **Observado:** tiene el mismo verde subrayado que «Historia», que sí existe. Al pasar el puntero dice «enlace de bóveda: nota-que-no-existe» sin advertir nada. El clic no hace nada.
- **Evidencia:** 17-salto-tareas, 26-click-roto, z-27-hover-roto, 55-dia-inicio.
- **Recomendación:** dar un estilo propio a los enlaces rotos (punteado más un ícono de advertencia, en un color que siga cumpliendo el contraste), avisar al pasar el puntero («No existe esta nota») y ofrecer «Crear nota» al hacer clic.

**QA-06: `[[nota]]` sin carpeta de trabajo no hace nada y no se explica**
- **Área / referencia:** enlaces; R13, H9.
- **Severidad / frecuencia:** S2, siempre (hasta que se elige una carpeta).
- **Pasos:** abrir `inicio.md` con «Abrir documento» (sin carpeta de trabajo) y hacer clic o doble clic en «Historia» o en «biologia/celula».
- **Esperado:** abre la nota, o explica «Elegí la carpeta de apuntes para seguir enlaces» con un botón.
- **Observado:** no hay respuesta. Tras abrir la carpeta, el mismo clic abre la nota en una pestaña nueva (33).
- **Evidencia:** 28-dbl-historia, 33-click-celula.
- **Recomendación:** resolver los enlaces relativos a la carpeta del archivo abierto o, si no es posible, mostrar un aviso que ofrezca «Usar esta carpeta como carpeta de apuntes».

**QA-07: Palabras y caracteres no se actualizan al editar**
- **Área / referencia:** barra de estado; R22, H1.
- **Severidad / frecuencia:** S2, siempre.
- **Pasos:**
  1. En un documento nuevo, en Editar, escribir «hola mundo» y listas: la barra dice «0 palabras · 0 caracteres», incluso después de guardar.
  2. En `inicio.md` (102 palabras), en Editar, escribir «dos palabras»: sigue en 102.
- **Esperado:** recuento en vivo.
- **Observado:** solo se recalcula al pasar a Leer (89: 9 palabras). Línea y columna sí se actualizan.
- **Evidencia:** 43-editor-type, 44-listas, 88-guardado, 89-prueba-leer, z-106-contador.
- **Recomendación:** recalcular al escribir, con un pequeño retraso de 200–300 ms.

**QA-08: Los desplegables no se anclan a su botón**
- **Área / referencia:** menús; N8, H4.
- **Severidad / frecuencia:** S3, siempre.
- **Pasos:** abrir Bloques, Listas, Insertar, Estudio, Más y «···».
- **Esperado:** el menú se abre debajo del botón que lo invoca, alineado con él.
- **Observado:** todos se abren en la misma posición (x≈48, debajo de «B»). El de «Más» aparece a 740 px de su botón. Además, la cabecera del menú repite el tooltip, que queda encima.
- **Evidencia:** 46-bloques, 50-menu-listas, 50-menu-insertar, 50-menu-estudio, 50-menu-mas, 98-640-mas.
- **Recomendación:** posicionar el menú relativo al botón, con reglas de volteo en los bordes, y ocultar el tooltip mientras el menú esté abierto.

**QA-09: Tooltips persistentes que tapan contenido o quedan detrás del panel**
- **Área / referencia:** tooltips; N7, N8.
- **Severidad / frecuencia:** S3, a veces.
- **Pasos:**
  1. Pulsar un botón de barra (Abrir, Abrir carpeta, «···», Resaltar).
  2. Mover el puntero al documento.
  3. Con el panel lateral abierto, pasar el puntero por la barra de lectura.
- **Esperado:** el tooltip se oculta al pulsar o al salir del botón y siempre queda por encima de todo.
- **Observado:** el tooltip siguió visible sobre títulos y menús tras pulsar. Con el panel lateral abierto, el tooltip queda detrás del panel y se lee recortado («a · Ctrl+Shift+F», «ajo», «rl+Shift+T»). Los tooltips de la barra superior aparecen a 100 px del botón, debajo de la barra de formato.
- **Evidencia:** 09-abrir-inicio, 29-carpeta, 39-buscar-celula3, 45-formato, 50-menu-espacio, 122-max-noche-archivos, 05-tooltip-top.
- **Recomendación:** ocultar el tooltip al pulsar, darle la capa más alta (por encima de los paneles) y anclarlo junto al botón.

**QA-10: La etiqueta «Nota» del callout queda encimada con la barra lateral**
- **Área / referencia:** render; estilo «Papel y tinta», H8.
- **Severidad / frecuencia:** S3, siempre (lectura, ambos temas, 640 px, maximizada y también en el PDF).
- **Pasos:** abrir `inicio.md` en Leer.
- **Esperado:** etiqueta «Nota» dentro del bloque, legible.
- **Observado:** la barra verde atraviesa la palabra («No|ta») y la etiqueta queda fuera del margen del texto.
- **Evidencia:** z-09-abrir-inicio, 55-dia-inicio, 97-640-comparar, inicio-export2.pdf.
- **Recomendación:** poner la etiqueta dentro del bloque, en la primera línea, con un ícono y en negrita pequeña, o a la izquierda con una separación mínima de 8 px de la barra.

**QA-11: Dos bloques de código seguidos se ven fusionados**
- **Área / referencia:** render; R8, H8.
- **Severidad / frecuencia:** S3, siempre.
- **Pasos:** ver «Comandos útiles» en `inicio.md`: en la fuente son dos bloques (bash y python).
- **Esperado:** dos tarjetas separadas, cada una con su lenguaje y su botón Copiar.
- **Observado:** se ve un único fondo continuo con dos botones Copiar. Parece que el segundo copia «todo lo de abajo», y el lenguaje no se muestra.
- **Evidencia:** 09-abrir-inicio, 17, 67-comparar.
- **Recomendación:** dejar un margen entre bloques (12–16 px) y mostrar una etiqueta de lenguaje junto a Copiar.

**QA-12: El índice flotante resalta la sección equivocada**
- **Área / referencia:** índice; R11, H1.
- **Severidad / frecuencia:** S3, a veces.
- **Pasos:** maximizar, activar el índice flotante y pulsar «Tareas».
- **Esperado:** se resalta «Tareas».
- **Observado:** el documento llega al final, pero se resalta «Plan de la semana». En otras posiciones va una sección atrasado (79: se ve «Plan de la semana» y se resalta «Mis apuntes»).
- **Evidencia:** 64-flotante-tareas, 66-plegado, 79-inicio-pos.
- **Recomendación:** calcular la sección activa con el título más cercano al tercio superior de la vista y forzar la última sección al llegar al final del documento.

**QA-13: El texto del índice flotante desborda la caja**
- **Área / referencia:** índice; H8.
- **Severidad / frecuencia:** S3, siempre (con títulos largos a 1100 px).
- **Pasos:** abrir `historia.md` a 1100×720 con el índice flotante activado.
- **Observado:** «Asamblea Nacional (1789–1791)» sobrepasa el borde derecho de la caja.
- **Evidencia:** 76-menu-pestana, z-76-menu-pestana, 77.
- **Recomendación:** truncar con «…» y mostrar el título completo al pasar el puntero, o ensanchar la caja hasta un máximo.

**QA-14: Microcopy técnico, en inglés o inconsistente**
- **Área / referencia:** microcopy; N6, H2.
- **Severidad / frecuencia:** S3, siempre.
- **Observado:**
  - «Workspace indexado: 5 notas, 0 omitidas; búsqueda limitada por presupuesto de memoria» (29).
  - «Documento creado y guardado de forma atómica» (123).
  - «enlace de bóveda» (27), «Insertar callout de nota» (49, 50-menu-insertar).
  - «Backlinks», «Copiar tabla como TSV» (103), «Insertar tabla Markdown», «Más formato Markdown».
  - «Acciones: tema · 1 resultados» (52).
  - Se mezcla el voseo («Escribí aquí», «elegí una variante», «Si cerrás») con el tuteo («escribe para filtrar», «escribe texto»).
  - El mismo grupo tiene nombres distintos: «Bloques y estructura» (tooltip) frente a «Bloques y encabezados» (menú contextual), e «Insertar» frente a «Insertar enlaces y bloques».
  - Espacio, carpeta y bóveda se usan para el mismo concepto: «Espacio de trabajo», «carpeta de trabajo», «bóveda» y «Workspace».
- **Evidencia:** las capturas citadas.
- **Recomendación:** hacer un glosario único. Propuestas:
  - «Carpeta de apuntes» en vez de Workspace, espacio o bóveda.
  - «Notas que enlazan aquí» en vez de Backlinks.
  - «Recuadro de nota» en vez de callout.
  - «Enlace a otra nota» en vez de enlace de bóveda.
  - «Guardado» en vez de «de forma atómica».
  - «Copiar tabla para hoja de cálculo» en vez de TSV.
  - Elegir voseo en todo y corregir la concordancia («1 resultado»).

**QA-15: Avisos redundantes que tapan contenido**
- **Área / referencia:** avisos; N7.
- **Severidad / frecuencia:** S3, siempre.
- **Observado:**
  - Tras «Entendido» en el diálogo de recuperación aparece «Recuperación local temporal activada» (03).
  - «Tema actualizado» (81) y «Encabezado enfocado» (113).
  - «Bloques y estructura · elegí una variante» al abrir un menú (46).
  - «Código copiado» además del botón «Copiado» (19).
  - Avisos de ayuda en cada apertura de panel (10, 35, 50-menu-espacio).
  - Varios aparecen centrados sobre código o sobre líneas del editor (10, 96).
- **Recomendación:** eliminar los avisos que repiten lo que el propio control ya muestra. Mostrar las ayudas de teclado solo la primera vez. Ubicar los avisos en una esquina inferior, lejos del texto en edición.

**QA-16: Sombras inconsistentes entre elementos flotantes**
- **Área / referencia:** visual; estilo declarado («menús, diálogos y avisos flotan con sombra suave»), N8.
- **Severidad / frecuencia:** S3, siempre.
- **Observado:**
  - Con sombra: menús contextuales (76, 103), diálogos (90, 120), avisos en tema de día (62) e índice flotante en tema de día.
  - Sin sombra perceptible: desplegables de barra y paleta (z-56-dia-paleta, 46) y tooltips.
  - El índice flotante en tema de noche tampoco muestra sombra (122).
  - En tema de día la paleta casi se confunde con el papel.
- **Recomendación:** definir un único token de elevación para todo lo flotante y aplicarlo también a desplegables y paleta. En noche, compensar con un borde de 1 px más claro.

**QA-17: Segundo color de acento (enlaces externos azules)**
- **Área / referencia:** visual; estilo «un único color de acento verde».
- **Severidad / frecuencia:** S3, siempre.
- **Observado:** «Wikipedia» se ve en azul (#7DB3FF en noche, azul medio en día) mientras los enlaces internos son verdes.
- **Evidencia:** 21-hover-wiki, 55-dia-inicio.
- **Recomendación:** distinguir el enlace externo con un ícono ↗ y el mismo verde, no con otro color.

**QA-18: Contraste insuficiente de secundarios e íconos en tema de día**
- **Área / referencia:** contraste; N5.
- **Severidad / frecuencia:** S3, siempre.
- **Observado (estimación por píxeles):**
  - Tema de día:
    - íconos de la barra de formato: 3,86:1;
    - íconos de la barra de lectura: 4,41:1;
    - pestañas inactivas: 4,50:1;
    - segmento inactivo y barra de estado: 4,72:1;
    - números de lista y de línea: 5,23:1;
    - enlaces: 5,48:1;
    - el cuerpo (13,1:1) está bien.
  - Tema de noche: cuerpo 14,9:1, secundarios 6,4–7,4:1 y enlaces 8,9:1; cumple.
- **Evidencia:** 53-tema-dia-editor, 54-tema-dia-leer.
- **Recomendación:** oscurecer el gris secundario de día hasta llegar a ≥4,5:1 con margen (por ejemplo, de #6C6151 a un tono ~20 % más oscuro), llevar los íconos a ≥4,5:1 y los enlaces a ≥7:1, o compensar con subrayado y peso.

**QA-19: Pestañas inactivas sin hover, sin X y sin nombre completo**
- **Área / referencia:** pestañas; N2, R4.
- **Severidad / frecuencia:** S3, siempre.
- **Pasos:** pasar el puntero por una pestaña inactiva (y truncada a 640 px).
- **Observado:** sin cambio visual, sin botón de cerrar y sin tooltip con el nombre completo («prueba-qa.…», «historia…»).
- **Evidencia:** z-100-hover-tab, 99-640-dia-nuevo.
- **Recomendación:** mostrar fondo de hover, la X al pasar el puntero y un tooltip con el nombre y la ruta.

**QA-20: El botón Copiar no tiene hover y su estilo es distinto**
- **Área / referencia:** botones; R8, N2, N8.
- **Severidad / frecuencia:** S3, siempre.
- **Observado:** al pasar el puntero no cambia (z-18-hover-copiar). Es un rectángulo verde sólido sin borde, distinto de los botones de los diálogos, de la barra y del selector de modo.
- **Recomendación:** usar el estilo del botón secundario con un estado de hover y otro de pulsado. Confirmar solo en el propio botón (✓ Copiado, durante 1,5 s).

**QA-21: «Fijar encima» activo se confunde con el hover**
- **Área / referencia:** ventana; R18, H1.
- **Severidad / frecuencia:** S3, siempre.
- **Observado:** el estado fijado es un fondo claro idéntico al hover de minimizar (z-74-fijar frente a z-121). El ícono de la chincheta no cambia.
- **Recomendación:** en el estado activo, usar la chincheta rellena en verde de acento y un tooltip «Fijada · clic para soltar».

**QA-22: Iconografía ambigua o repetida; marcas de desplegable encimadas**
- **Área / referencia:** iconos; H6, N8.
- **Severidad / frecuencia:** S3, siempre.
- **Observado:**
  - «Resaltar texto» usa una H igual a la de «Bloques» o títulos.
  - «Código en línea» usa una N tachada en lugar de `</>` o de un backtick.
  - «Abrir carpeta» (barra de lectura) tiene el mismo ícono que «Abrir documento».
  - «Buscar en carpeta» tiene el mismo que «Buscar en documento».
  - «Plegar» es un «>» de texto.
  - Las marcas de desplegable son puntos de 2–3 px que se pegan a la última letra («Bloques̡», «Listas ̡», «Insertaṛ», «Estudiọ»), más una marca suelta tras «Más».
  - Las cinco funciones de la barra de lectura no tienen texto.
- **Evidencia:** z-03-tras-esc, m-tooltips, m-tooltips-leer.
- **Recomendación:**
  - Usar un ícono de marcador para resaltar y `</>` para código en línea.
  - Crear variantes «carpeta + lupa» y «carpeta abierta».
  - Usar un chevron real separado al menos 4 px del rótulo.
  - En la barra de lectura, mostrar rótulos cuando el ancho lo permita.

**QA-23: Las inserciones de la barra no respetan el contexto**
- **Área / referencia:** edición; R6, H5.
- **Severidad / frecuencia:** S3, siempre.
- **Pasos:**
  1. En un documento con un símbolo «→» al inicio, pulsar «Insertar tabla»: la tabla queda pegada al símbolo en la misma línea y no se renderiza como tabla.
  2. Después de una lista numerada, en una línea nueva, pulsar Negrita y Resaltar: en Leer, «negrita» y «resaltado» quedan dentro del ítem 3 («3. tarea **negrita** resaltado»).
- **Evidencia:** 116-tabla-insertada, 45-formato, 89-prueba-leer.
- **Recomendación:** las inserciones de bloque (tabla, código, cita, regla) deben ir en una línea propia, con una línea en blanco antes y después. Al salir de una lista con Enter en un elemento vacío, dejar una línea en blanco de separación.

**QA-24: No hay barra de desplazamiento en el documento**
- **Área / referencia:** navegación; H1.
- **Severidad / frecuencia:** S3, siempre.
- **Observado:** en `manual-largo.md` (2036 palabras) no hay ninguna barra ni indicador de posición, ni en Leer ni en Editar.
- **Evidencia:** 110–114, 122.
- **Recomendación:** agregar una barra delgada que se vea al pasar el puntero o al desplazarse, o un indicador de progreso de lectura.

**QA-25: Diálogo de cierre de la app incoherente con el de pestaña**
- **Área / referencia:** diálogos; R20, N8.
- **Severidad / frecuencia:** S3, siempre.
- **Observado:**
  - El de pestaña ofrece «Cerrar sin guardar / Cancelar / Guardar» en tema de noche.
  - El de la app ofrece «Cerrar de todos modos / Seguir editando», dice «Hay 3 documentos…» sin nombrarlos y no ofrece «Guardar todo».
  - Los botones y la forma de redactar la salida segura son distintos.
  - Esc sí elige la opción segura en ambos.
- **Evidencia:** 90-cerrar-sucio, 120-cerrar-app.
- **Recomendación:** usar el mismo patrón: lista de documentos con cambios, «Guardar todo» como primario, «Cerrar sin guardar» como peligroso y «Cancelar».

**QA-26: «Abrir recuperación» es opaco**
- **Área / referencia:** recuperación; R21, H1, H10.
- **Severidad / frecuencia:** S3, siempre.
- **Pasos:** en la paleta, elegir «Abrir recuperación».
- **Observado:** se abre directamente una pestaña «recuperación sin guardar.md» con el contenido de otro documento abierto. No indica de qué documento es, de qué hora, ni si hay otras recuperaciones.
- **Evidencia:** 118-paleta-recupera, 119-abrir-recuperacion.
- **Recomendación:** mostrar una lista de recuperaciones con documento de origen, fecha y vista previa, y las acciones «Restaurar», «Comparar» y «Descartar».

**QA-27: Alternancias sin estado ni confirmación**
- **Área / referencia:** paleta; R17, R21, H1.
- **Severidad / frecuencia:** S3, siempre.
- **Observado:** «Activar o desactivar movimiento reducido» y «Activar o desactivar recuperación local» no muestran el estado actual. Tras ejecutar la primera no vi ninguna confirmación (117).
- **Recomendación:** rótulos que digan el estado y la acción, por ejemplo «Movimiento reducido: desactivado · Activar», o una casilla ✓ en la paleta, más una confirmación breve.

**QA-28: Un documento nuevo aparece como «guardado»**
- **Área / referencia:** barra de estado; R22, H1.
- **Severidad / frecuencia:** S3, siempre.
- **Observado:** «sin título.md» recién creado (y nunca guardado) dice «Edición · guardado».
- **Evidencia:** 03-tras-esc, 99-640-dia-nuevo.
- **Recomendación:** mostrar «nuevo, sin guardar», o no mostrar estado hasta el primer cambio.

**QA-29: Árbol de archivos y Backlinks inconsistentes**
- **Área / referencia:** carpeta; R13, N8.
- **Severidad / frecuencia:** S3, siempre.
- **Observado:**
  - El árbol no resalta la nota abierta: la selección queda en «- biologia/» (30, 122).
  - La carpeta se marca con un guion de texto en lugar de un ícono o un chevron.
  - La cabecera dice «5 notas» y el pie «1–6 de 6».
  - Backlinks muestra «biologia\fotosintesis.md», con barra invertida, mientras la barra de estado usa «apuntes › biologia › celula.md».
  - Backlinks no muestra el contexto de la mención.
- **Recomendación:** resaltar el archivo activo, usar un chevron para plegar carpetas, unificar las rutas con «›» y mostrar debajo de cada backlink la línea donde aparece el enlace.

**QA-30: La búsqueda muestra «Sin resultados» antes de escribir y el campo no parece un campo**
- **Área / referencia:** búsqueda; H9, N2.
- **Severidad / frecuencia:** S3, siempre.
- **Observado:** el panel Buscar dice «Sin resultados» de entrada. El campo es solo texto gris, sin borde ni cursor.
- **Evidencia:** 35-tab-buscar, z-40-buscar-key.
- **Recomendación:** estado vacío del tipo «Escribí para buscar en tus apuntes» y un campo con borde, cursor y botón de limpiar.

**QA-31: La caja «Buscar en documento» tapa el texto**
- **Área / referencia:** búsqueda; N7.
- **Severidad / frecuencia:** S3, siempre.
- **Observado:** la caja aparece sobre las líneas 3 y 4 del editor (y sobre el texto en Leer), y el tooltip queda encima de la caja.
- **Evidencia:** 41-buscar-doc, 109-buscar-editar-nota.
- **Recomendación:** mostrarla como una barra que empuja el contenido, o anclada arriba a la derecha y sin tooltip mientras está abierta.

**QA-32: Menú contextual del editor: desborda y le faltan acciones**
- **Área / referencia:** menú contextual; R16, N1.
- **Severidad / frecuencia:** S3, siempre.
- **Observado:**
  - Al hacer clic derecho en la línea 13 del editor, el menú se extiende por debajo de la barra de estado (hasta y≈718 en una ventana de 720).
  - Empieza por «Pegar», pero no tiene Copiar, Cortar ni Deshacer.
  - Sobre una tabla no ofrece acciones de tabla (agregar fila o columna).
  - En lectura, sobre texto, tabla o enlace, solo ofrece variantes de copiar.
- **Evidencia:** 107-ctx-tabla-editar, 103, 104, 25.
- **Recomendación:** voltear el menú hacia arriba cuando no entra, agregar Cortar, Copiar, Pegar y Deshacer, y acciones según el contexto (tabla: fila o columna; tarea: marcar o desmarcar; enlace: abrir o copiar dirección).

**QA-33: Maximizada: gran vacío a la derecha; el ícono no pasa a «restaurar»**
- **Área / referencia:** layout; N1.
- **Severidad / frecuencia:** S4, siempre.
- **Observado:** a 1366 px la columna de lectura queda pegada a la izquierda (unos 780 px) y deja unos 580 px vacíos. El botón maximizar sigue mostrando □.
- **Evidencia:** 61-maximizada, z-61-maximizada.
- **Recomendación:** centrar la columna de lectura y alternar el ícono a «restaurar» (dos cuadrados).

**QA-34: El selector de modo tiene padding desigual y se duplica con otro estilo**
- **Área / referencia:** visual; N8.
- **Severidad / frecuencia:** S4, siempre.
- **Observado:** «Comparar» queda pegado al borde derecho del control segmentado (z-03-tras-esc). En la vista dividida, cada panel repite su propio Leer, Editar y Comparar con otro estilo (71).
- **Recomendación:** igualar el padding y usar un solo selector, el del panel activo, o el mismo componente en ambos lugares.

**QA-35: La pestaña fijada casi no se distingue**
- **Área / referencia:** pestañas; R4.
- **Severidad / frecuencia:** S4, siempre.
- **Observado:** se marca con un «▪» de unos 4 px, conserva la X de cerrar y, al reordenar, quedó última en vez de anclada al inicio.
- **Evidencia:** 77-pestana-fijada, 78-reordenar.
- **Recomendación:** usar un ícono de chincheta, anclar la pestaña a la izquierda, compactarla y ocultar la X.

**QA-36: La tecla «T» cambia el tema y no hay control visible de tema**
- **Área / referencia:** tema; R17, H5, H6.
- **Severidad / frecuencia:** S3, siempre.
- **Observado:** en Leer, la tecla T sola cambia entre día y noche (81, 99), lo que facilita activarlo sin querer. El único acceso visible es la paleta.
- **Recomendación:** un botón sol/luna en la barra superior y que el atajo requiera un modificador.

**QA-37: El PDF exportado tiene la capa de texto desordenada**
- **Área / referencia:** exportar; R19.
- **Severidad / frecuencia:** S3, siempre.
- **Observado:** visualmente el PDF es correcto, pero al extraer el texto aparece duplicado y entremezclado («Bbiioelnovgeina/icdeolualala…», «Mar t es Mar t es | Fo Mar»), así que copiar o buscar dentro del PDF no sirve. También se reproduce el defecto de QA-10.
- **Evidencia:** inicio-export2.pdf.
- **Recomendación:** generar el PDF con una sola capa de texto por glifo, sin duplicados de enlaces ni de celdas.

**QA-38: Lista de símbolos con rótulos desalineados**
- **Área / referencia:** menús; H8.
- **Severidad / frecuencia:** S4, siempre.
- **Observado:** el ancho variable de los símbolos (→, ✓, ≡, …, —) desplaza los rótulos.
- **Evidencia:** 115-emoji.
- **Recomendación:** dar a la columna del símbolo un ancho fijo de unos 24 px.

**QA-39: Jerarquía débil de títulos en el índice**
- **Área / referencia:** índice; H8.
- **Severidad / frecuencia:** S4, siempre.
- **Observado:** en el panel Índice, el nivel 3 («Ejercicio 5») solo tiene 6 px más de sangría que el nivel 2. En el documento, H2 y H3 se parecen mucho («Etapas» frente a «Asamblea Nacional»).
- **Evidencia:** 110-largo-indice, 32-abre-historia.
- **Recomendación:** sangría de 12–16 px por nivel y una escala tipográfica más marcada entre H2 y H3.

**QA-40: El primer arranque no orienta**
- **Área / referencia:** onboarding; H10, H2.
- **Severidad / frecuencia:** S3, una vez.
- **Observado:**
  - La primera pantalla es un documento vacío en Editar, con una barra de más de 15 íconos.
  - Se abre un diálogo técnico («Es texto sin cifrar, no reemplaza a Guardar…»), seguido de un aviso que repite lo mismo.
  - No hay accesos a «Abrir carpeta de apuntes», «Abrir archivo» ni un ejemplo.
  - El modo inicial (Editar) contradice que la app se presente como lectora.
- **Evidencia:** 01-primer-arranque, 03-tras-esc.
- **Recomendación:** una pantalla de bienvenida con tres acciones grandes (Abrir carpeta de apuntes, Abrir archivo, Nuevo apunte) y una nota breve sobre la recuperación, con un enlace a «más detalles».

**QA-41: Menús de solo 9 filas, sin barra y con paginación a saltos**
- **Área / referencia:** menús; N4, H1.
- **Severidad / frecuencia:** S3, siempre.
- **Observado:**
  - La paleta (69 acciones), «Más» (16) y Bloques (10) muestran 9 filas aunque haya espacio libre.
  - No tienen barra de desplazamiento; solo el texto «1–9 de 69».
  - Al bajar con flechas la vista salta por páginas («9–17 de 69»), así que se pierde el contexto.
  - El ítem 10 de Bloques solo se descubre con la flecha hacia arriba (49).
- **Evidencia:** 46-bloques, 49-bloques-up, 59-paleta-down12.
- **Recomendación:** mostrar hasta el alto disponible (unas 12–14 filas), agregar una barra delgada y desplazar fila por fila manteniendo visible el elemento seleccionado.

---

## 6. Fortalezas (conservar)

- **Diálogo de cambios sin guardar de la pestaña:** título en forma de pregunta concreta, verbos explícitos, «Guardar» como primario, «Cerrar sin guardar» con color de peligro y Esc igual a Cancelar. Además, confirma que no se cerró nada (90, 92).
- **Tipografía de lectura:** serif cómoda, buen interlineado y contraste alto en noche (14,9:1).
- **Foco de teclado en las barras:** subrayado verde más tooltip con atajo, F6 entre regiones y flechas en las listas (m-f6, m-teclado).
- **Paleta de acciones:** completa, filtra al instante y muestra los atajos.
- **Tooltips con atajo** en casi todos los botones.
- **Destino del enlace visible** al pasar el puntero o al enfocar (R14).
- **Adaptación a 640×480:** las etiquetas pasan a íconos, las pestañas se truncan y no hay controles encimados.
- **Modo Comparar** con desplazamiento sincronizado.
- **Recordar la posición por pestaña** y una vista dividida simple.
- **Copiar del bloque de código** que copia exactamente el código.
- **Índice flotante** con sombra y seguimiento del documento, aunque desfasado.
- **Estética del PDF** exportado.

---

## 7. Recomendaciones priorizadas

### Quick wins (bajo costo, alto impacto)
1. Dar foco y recibir texto en los campos de búsqueda (QA-01).
2. Convertir los botones de paneles en interruptores con estado activo visible (QA-02).
3. Hacer que la rueda desplace el elemento flotante o la lista bajo el puntero y soportar PgUp, PgDn, Inicio y Fin (QA-03, QA-41).
4. Dar estilo propio a los enlaces rotos y un aviso al hacer clic en enlaces web, rotos o sin carpeta (QA-04 a QA-06).
5. Actualizar el contador de palabras en vivo y corregir «guardado» en documentos nuevos (QA-07, QA-28).
6. Pasada de microcopy con glosario, voseo único y concordancia (QA-14).
7. Eliminar los avisos redundantes y moverlos lejos del texto (QA-15).
8. Corregir la etiqueta del callout, la separación entre bloques de código y el desborde del índice flotante (QA-10, QA-11, QA-13).
9. Ocultar los tooltips al pulsar, darles la capa superior y anclarlos (QA-09).
10. Agregar hover a pestañas y a Copiar, y un estado activo con acento a la chincheta (QA-19 a QA-21).
11. Oscurecer los grises secundarios y los íconos del tema de día (QA-18).

### Cambios mayores
1. Sistema de elevación y de menús unificado: anclaje al botón, sombra común, alto adaptativo y barra de desplazamiento (QA-08, QA-16, QA-41).
2. Inserciones de formato con conciencia de contexto, para un público que no sabe Markdown (QA-23).
3. Menús contextuales realmente contextuales: tabla, tarea, enlace y edición básica (QA-32, QA-04).
4. Recuperación con una lista navegable, origen y fecha, y un diálogo de cierre de la app unificado (QA-25, QA-26).
5. Primer arranque guiado e indicadores de posición: barra de desplazamiento y índice preciso (QA-40, QA-24, QA-12).
6. Revisión de la iconografía y de los rótulos de la barra de lectura (QA-22).
7. Pulir el layout al maximizar, la pestaña fijada, el selector de modo único y la capa de texto del PDF (QA-33 a QA-35, QA-37).
