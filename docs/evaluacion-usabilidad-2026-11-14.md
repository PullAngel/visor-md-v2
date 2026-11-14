# Evaluación de usabilidad · 14 de noviembre de 2026

Esta evaluación se hizo con dos perfiles independientes, sobre el build
`58f904d`:

- una QA manual sénior de UI/UX;
- una usuaria final que usa la app por primera vez.

El documento consolida ambos informes, verifica en el código los hallazgos
dudosos, los compara con la revisión de interfaz de la mañana
([`revision-interfaz-2026-11-14.md`](revision-interfaz-2026-11-14.md)) y fija
el **plan definitivo de cambios**, que reemplaza las fases de esa revisión.

Material de origen, en
[`evaluaciones/2026-11-14/`](evaluaciones/2026-11-14/):

- [`hoja-requerimientos.md`](evaluaciones/2026-11-14/hoja-requerimientos.md):
  hoja de requerimientos para la QA (R1–R23, N1–N8), escrita sin detalles
  técnicos.
- [`informe-qa.md`](evaluaciones/2026-11-14/informe-qa.md): 41 hallazgos,
  matriz de requerimientos y evaluación heurística.
- [`informe-usuario.md`](evaluaciones/2026-11-14/informe-usuario.md): 16
  tareas, 23 problemas y cuestionario SUS.

## Resumen

| Indicador | Resultado |
| --- | --- |
| Calificación UX (QA) | **6 / 10** |
| Heurísticas de Nielsen (QA), promedio | **2,9 / 5**. Las más bajas son Mundo real (2), Consistencia (2) y Errores (2). |
| Requerimientos (QA) | 11 cumplen, 17 parciales, 2 no cumplen (R12 búsqueda, N4 listas con rueda) |
| Tareas de la usuaria | 14/15 completadas (93 %), pero solo **6 sin dificultad (40 %)**; 1 fracaso (búsqueda en la carpeta) |
| SUS (usuaria) | **35 / 100**, «pobre»; el umbral aceptable es 68 |
| Hallazgos únicos tras consolidar | 44 en 8 áreas (ver la tabla consolidada) |

**Diagnóstico.** Las dos evaluaciones coinciden: **leer es un placer y
escribir, buscar y encontrar funciones cuesta**. El valor de lectura está
logrado:

- tipografía cuidada;
- pestañas que recuerdan la posición;
- casillas de tarea, Comparar y plegado de secciones;
- un diálogo de cierre claro.

Lo que baja la nota es:

1. un defecto bloqueante: no se puede escribir en los buscadores;
2. listas y menús que no responden a la rueda;
3. botones que no se comportan como interruptores;
4. lenguaje técnico para un público que no sabe Markdown;
5. funciones que solo se encuentran en la paleta;
6. inserciones de formato que rompen el documento sin avisar.

## Método

- **Entorno aislado.** Cada perfil manejó su propia copia de la app, con perfil
  de configuración vacío (primer arranque real) y una carpeta de apuntes de
  muestra:
  - 5 notas;
  - enlaces `[[…]]` entre ellas;
  - tablas, código, tareas y recuadros;
  - un manual de 40 capítulos.

  La entrada se envió con mensajes de ventana, sin tocar el mouse ni el teclado
  reales. Las capturas se tomaron de la ventana.
- **Build de evaluación.** Tiene tres diferencias locales con `58f904d`; ninguna
  se publicó:
  1. Los diálogos de archivo de Windows se reemplazan por una respuesta
     preparada, para que no aparezcan en pantalla.
  2. El puntero simulado no se pierde cuando el cursor real está fuera de la
     ventana.
  3. Un clic con posición de puntero desconocida no inicia el redimensionado
     de la ventana. Esta corrección surge del defecto **E-01**, que pasa al
     plan.
- **Límites de la herramienta.** Ctrl, Alt y Shift no llegan a la app,
  así que los atajos se evaluaron solo como comunicados en la interfaz. Las
  mayúsculas y las vocales con tilde no se escriben. Ninguno de los dos perfiles
  reportó estos límites como defectos.
- **QA.** Unas 210 interacciones y más de 120 capturas, en tema de noche y de
  día y en tres tamaños (640×480, 1100×720 y maximizada). Incluye la matriz de
  requerimientos, las 10 heurísticas de Nielsen, una auditoría visual y de
  microcopy, y una revisión de accesibilidad básica.
- **Usuaria.** «Lucía», 21 años, estudiante, no conoce Markdown. Hizo un test
  de 5 segundos, 16 tareas con pensamiento en voz alta y el cuestionario SUS.

### Verificación de hallazgos dudosos

Cada hallazgo que podía deberse a la herramienta o a una decisión de diseño se
contrastó con el código o se reprodujo:

| Hallazgo | Veredicto | Evidencia |
| --- | --- | --- |
| No se puede escribir en Buscar (QA-01, US-01) | **Confirmado, defecto real** | En Windows, el texto escrito llega en `KeyEvent::text`. `handle_search_key` y `handle_workspace_search_key` solo aceptan `Ime::Commit`. El editor y la paleta sí leen `KeyEvent::text`, y por eso allí se puede escribir. Reproducido. |
| Los enlaces web no se abren (QA-04) | **Falso positivo, con un problema real** | El clic sí abre el navegador del sistema (`explorer.exe`), fuera de la ventana evaluada. Lo real es que la app no da ninguna señal propia al abrirlo. |
| Contador en 0 al editar (QA-07) | **Confirmado** | Reproducido: «0 palabras» con texto escrito en Editar. |
| El ícono de maximizar no cambia (QA-33) | **Confirmado** | El dibujo del control no consulta `is_maximized()`. |
| Sin emojis (US-03) | **Consecuencia de las listas** | Los 10 emojis existen, pero están en las posiciones 31 a 40. Con 9 filas visibles, sin rueda y sin filtro, no se descubren. |
| Franja negra abajo a 1280×760 (US-18) | **Artefacto del entorno** | La ventana excedía el alto del monitor de 768 px. La QA vio la barra de estado en todos los tamaños. |
| Letras de más al escribir y doble clic que no selecciona (US-17 y anomalía) | **Probable artefacto** | No se reprodujo con calma. Se verifica a mano en la fase 9. |
| Texto del PDF desordenado (QA-37) | **Plausible, a verificar** | El extractor que usó la QA no se pudo repetir aquí. Se verifica con un lector de PDF en la fase 8. |
| Sin respuesta al pulsar `[[nota]]` sin carpeta o un enlace roto (QA-05, QA-06) | **Parcial** | El código emite un aviso en varios casos, pero la usuaria lo vio y la QA no. Se unifica y verifica en la fase 6. |

**Nuevo defecto hallado al preparar la evaluación (E-01).** Si la app recibe
un clic sin conocer la posición del puntero (por ejemplo, tras salir el cursor
de la ventana), lo trata como un clic en (0,0), que es zona de redimensionar. La
ventana empieza a redimensionarse siguiendo al cursor real. Es raro en uso
normal, pero desconcertante cuando ocurre, y la corrección es de una línea.

## Comparación con lo que ya había

La revisión de la mañana (paneo por código y capturas propias) acertó en la
mecánica de listas, menús, alternancia, pestañas y hover. Las dos evaluaciones
la **confirman punto por punto**:

| Revisión de la mañana | QA | Usuaria |
| --- | --- | --- |
| L1–L5 rueda, ventana, 9 filas, sin barra, sin hover | QA-03, QA-41 | US-04 |
| L6–L8 menús no anclados, planos, atajos pegados | QA-08, QA-16 | — |
| L10 / D3 tooltip encima del menú | QA-09 | US-20 |
| L11 sin PgUp, PgDn, Inicio y Fin | QA-03 | — |
| L12 el índice reabre arriba | — | US-04 |
| T1–T5 alternancia de botones | QA-02 | — |
| P1–P3 pestañas | QA-19, QA-35 | — |
| H1–H3 hover y pulsación, Copiar | QA-20, QA-21 | — |
| D1–D2 avisos tapados y de más | QA-15 | — |
| D4 «Comparar» apretado | QA-34 | — |
| D5 ícono `>` | QA-22 | — |
| D7 chincheta activa | QA-21 | US-14 |

**Lo que la revisión de la mañana no vio** y aportan las evaluaciones:

1. **Funcionalidad rota:** buscadores sin texto (S1), contador sin
   actualizar, documento nuevo que dice «guardado» y contadores mal
   redactados.
2. **Integridad del documento:** las inserciones rompen la estructura
   (negrita dentro de la lista, tabla pegada al texto, símbolo fuera de la
   tabla), y salir de una lista no deja línea en blanco.
3. **Lenguaje:** jerga técnica e inglés, mezcla de voseo y tuteo, y
   nombres distintos para la misma cosa.
4. **Descubribilidad:** tema, PDF y recuperación solo están en la paleta;
   abrir una carpeta no muestra los archivos; íconos ambiguos (H de
   resaltar, N de código, carpeta y lupa repetidas).
5. **Orientación:** el primer arranque no guía; no hay barra de desplazamiento
   en el documento; el índice flotante marca la sección equivocada; la columna
   de lectura queda pegada a la izquierda al maximizar.
6. **Lectura y render:** la etiqueta del recuadro «Nota» queda pisada, dos
   bloques de código se ven fusionados, la jerarquía H2/H3 es débil y los
   grises del tema de día tienen poco contraste.
7. **Flujos:**
   - cerrar una pestaña deshace la división;
   - un enlace abre la nota en Editar;
   - el panel nuevo hereda Comparar;
   - el diálogo de cierre de la app es distinto del de la pestaña;
   - la recuperación prometida no se encuentra.

Conclusión: el plan anterior era correcto pero **insuficiente**. Cubría
unos 13 de los 44 hallazgos consolidados. El plan definitivo lo conserva como
núcleo (fases 2 a 5) y agrega correcciones funcionales, lenguaje,
descubribilidad, lectura y flujos.

## Hallazgos consolidados

Origen: **Q** = informe de QA, **U** = informe de usuaria, **R** = revisión de
la mañana, **E** = hallado al preparar la evaluación. Severidad final: S1
bloquea, S2 mayor, S3 menor, S4 cosmético.

### A. Funcionamiento

| ID | Hallazgo | Origen | Sev. |
| --- | --- | --- | --- |
| A1 | No se puede escribir en Buscar en documento ni Buscar en carpeta | Q-01, U-01 | S1 |
| A2 | Palabras y caracteres no se actualizan en Editar | Q-07 | S2 |
| A3 | Un documento nuevo figura como «guardado» | Q-28 | S3 |
| A4 | Clic sin puntero conocido redimensiona la ventana | E-01 | S3 |
| A5 | Contadores: «1 resultados», «1–7 de 7» con 8 filas, «5 notas» frente a «1–6 de 6» | Q-14, Q-29, U-21 | S4 |

### B. Integridad del documento al editar

| ID | Hallazgo | Origen | Sev. |
| --- | --- | --- | --- |
| B1 | Enter dos veces sale de la lista sin línea en blanco; lo siguiente queda dentro del último ítem | U-02, Q-23 | S2 |
| B2 | Las inserciones de bloque (tabla, código, cita, regla) no se aíslan en su propia línea | Q-23, U-13 | S2 |
| B3 | Resaltar o negrita sin selección insertan en medio de la palabra | U-17 | S3 |
| B4 | Un símbolo insertado al final de una fila rompe la tabla y desaparece en Leer | U-03 | S3 |

### C. Listas, menús y paneles

| ID | Hallazgo | Origen | Sev. |
| --- | --- | --- | --- |
| C1 | La rueda mueve el documento, no la lista; no hay PgUp, PgDn, Inicio ni Fin | R-L1/L11, Q-03, U-04 | S2 |
| C2 | 9 filas fijas, sin barra, salto por páginas; los emojis quedan escondidos | R-L2–L4, Q-41, U-03 | S2 |
| C3 | Un botón no cierra su panel ni se ve activo; cambiar de menú requiere dos clics | R-T1–T5, Q-02 | S2 |
| C4 | Menús no anclados al botón, sin sombra y con atajos pegados al nombre | R-L6–L8, Q-08, Q-16 | S3 |
| C5 | El selector de símbolos no filtra escribiendo; los rótulos están desalineados | U-03, Q-38 | S3 |
| C6 | El índice reabre arriba y tiene sangría débil; el flotante desborda y marca la sección equivocada | R-L12, U-04, Q-12, Q-13, Q-39 | S3 |
| C7 | Tooltips persistentes, detrás de paneles o encima de menús | R-L10, Q-09, U-20 | S3 |
| C8 | Alternancias de la paleta sin estado (movimiento reducido, recuperación) | Q-27 | S3 |
| C9 | El campo de búsqueda no parece un campo, dice «Sin resultados» antes de escribir y tapa texto | Q-30, Q-31 | S3 |
| C10 | El menú contextual desborda abajo y le faltan Copiar, Cortar, Deshacer y acciones de tabla, tarea y enlace | Q-32 | S3 |

### D. Respuesta visual (hover, pulsación, estados)

| ID | Hallazgo | Origen | Sev. |
| --- | --- | --- | --- |
| D1 | Sin respuesta al pulsar; hover ausente en Copiar, pestañas inactivas, cabeceras y filas | R-H1–H3, Q-19, Q-20 | S3 |
| D2 | La chincheta activa no se distingue del hover y no tiene tooltip | R-D7, Q-21, U-14 | S3 |
| D3 | El ícono de maximizar no pasa a «restaurar» | Q-33 | S4 |
| D4 | No hay barra de desplazamiento ni indicador de posición en el documento | Q-24, U-04 | S3 |

### E. Pestañas y paneles divididos

| ID | Hallazgo | Origen | Sev. |
| --- | --- | --- | --- |
| E1 | La pestaña activa casi no se distingue; las inactivas no tienen hover, ✕ ni nombre completo | R-P1–P3, Q-19 | S3 |
| E2 | La pestaña fijada apenas se ve y no se ancla al inicio | Q-35 | S4 |
| E3 | Abrir un archivo deja la pestaña vacía «sin título» | U-19 | S4 |
| E4 | Cerrar una pestaña deshace la división | U-12 | S3 |
| E5 | El panel nuevo hereda Comparar; hay dos selectores de modo con estilos distintos | U-15, Q-34 | S3 |

### F. Lenguaje y avisos

| ID | Hallazgo | Origen | Sev. |
| --- | --- | --- | --- |
| F1 | Jerga y anglicismos: atómica, Workspace, presupuesto de memoria, bóveda, callout, Backlinks, TSV, H1, «herramienta portable», «la fuente permanece intacta» | Q-14, U-09 | S3 |
| F2 | Mezcla de voseo y tuteo; un mismo concepto con 4 nombres (carpeta de trabajo, espacio, bóveda, workspace) | Q-14 | S3 |
| F3 | Avisos redundantes («Copiado» más un aviso; instrucciones al abrir cada panel) que tapan texto | R-D1/D2, Q-15 | S3 |
| F4 | El diálogo promete «restaurar después», pero no hay dónde; «Abrir recuperación» es opaco | U-11, Q-26 | S3 |
| F5 | El diálogo de cierre de la app difiere del de la pestaña: no lista documentos ni ofrece «Guardar todo» | Q-25 | S3 |

### G. Descubribilidad y orientación

| ID | Hallazgo | Origen | Sev. |
| --- | --- | --- | --- |
| G1 | Tema y PDF solo están en la paleta; la tecla T sola cambia el tema | Q-36, U-08 | S3 |
| G2 | Abrir una carpeta no muestra los archivos; el árbol no se actualiza al guardar y no marca la nota abierta | U-06, U-07, Q-29 | S2 |
| G3 | Íconos ambiguos o repetidos (H de resaltar, N de código, carpeta y lupa ×2, `>`) y chevrons pegados al rótulo | R-D5, Q-22 | S3 |
| G4 | El primer arranque no orienta: documento vacío en Editar y un cartel técnico de recuperación | Q-40, U-10 | S3 |
| G5 | Enlaces: sin señal al abrir uno web; un roto se ve como uno válido; `[[nota]]` sin carpeta no lo explica bien; abre en Editar | Q-04/05/06, U-16, U-22 | S2 |

### H. Lectura y render

| ID | Hallazgo | Origen | Sev. |
| --- | --- | --- | --- |
| H1 | La etiqueta «Nota» del recuadro queda pisada por el filete, también en el PDF | Q-10 | S3 |
| H2 | Dos bloques de código seguidos se ven fusionados; no muestran el lenguaje | Q-11, U-23 | S3 |
| H3 | Jerarquía H2/H3 débil | Q-39 | S4 |
| H4 | Al maximizar, la columna de lectura queda a la izquierda con un gran vacío | Q-33, U (T1) | S4 |
| H5 | Tema de día: grises secundarios e íconos entre 3,9:1 y 4,7:1 | Q-18 | S3 |
| H6 | Texto del PDF posiblemente desordenado (a verificar) | Q-37 | S3 |

### Decisiones del propietario

Tres hallazgos chocan con decisiones de producto vigentes. Se proponen
caminos, pero deciden el propietario:

1. **Edición visual (U-05).** Lucía esperaba un editor tipo Word y encontró
   Markdown crudo. Es el mayor motivo del SUS bajo después de la búsqueda.
   - Opción a, recomendada: mantener Markdown con más ayudas.
     - Botón «Título» visible.
     - Inserciones con contexto (fase 4).
     - Acciones de tabla en el menú contextual.
     - Primer arranque que muestre Comparar como forma de aprender.
   - Opción b: un modo de edición visual. Es un proyecto aparte, de alcance
     grande.
2. **Enlaces externos en azul (Q-17).** La QA lo marca como un segundo
   acento, porque la hoja de requerimientos de esta evaluación decía «un único
   acento verde». Sin embargo, el azul es una decisión de seguridad registrada
   en [`design.md`](design.md#enlaces-y-estados-de-seguridad): diferencia lo
   que sale de la app. Recomendación: **mantener el azul** y agregar un ícono
   ↗.
3. **La tecla T cambia el tema en Leer (Q-36).** Recomendación: conservarla y
   agregar un botón sol/luna visible, con un aviso «Tema de día · T para
   volver».

## Plan definitivo de cambios

El desglose ejecutable (tareas, funciones, pruebas y estado) vive en
[`plan-ejecucion-2026-11-14.md`](plan-ejecucion-2026-11-14.md).

El plan ordena primero lo que **rompe tareas** (A1, B1, B2, C1–C3, G2, G5),
después la **base común** que reutilizan las fases visuales, y al final el
pulido. Cada fase termina con:

- commits pequeños en `main`;
- `cargo fmt`, `cargo clippy --all-targets -- -D warnings`, `cargo test` y
  `scripts/check-docs.ps1`;
- capturas en vivo con la herramienta de evaluación.

No se agregan dependencias.

### Fase 1 · Correcciones funcionales (A1–A5)

- Buscar en documento y en carpeta aceptan `KeyEvent::text` igual que la
  paleta, sin Ctrl ni Alt. Hay una prueba que alimenta texto por ambos caminos.
- Clic sin posición de puntero: no redimensiona ni actúa (E-01).
- El contador de palabras y caracteres se recalcula desde la fuente al editar,
  con un retraso de unos 250 ms.
- Estado «nuevo · sin guardar» para documentos que nunca se guardaron.
- Concordancia y conteos correctos en pies de lista, paleta y árbol.

### Fase 2 · Base: objetivos interactivos y animación

Igual que la fase 1 de la revisión de la mañana:

- `UiTarget` y una única función `ui_target_at` para dibujo, clic y cursor;
- estado de hover y de pulsación con transiciones que respetan Reducir
  movimiento;
- pruebas de geometría a 640 px, 1100 px y maximizada.

Se suman a `UiTarget` las tooltips: una única capa superior y un único
tooltip vivo, que se oculta al pulsar, al abrir un menú y al usar la rueda.
Esto resuelve C7.

### Fase 3 · Listas, menús y alternancia (C1–C9)

Une las fases 2, 3 y 4 de la revisión de la mañana, porque comparten
componente.

- **Botones como interruptores.**
  - Un clic abre y el mismo clic cierra.
  - Cambiar de menú se hace en un solo clic.
  - El estado activo es visible en todos los botones de panel y menú.
- **`ListView` común** con scroll propio:
  - la rueda actúa bajo el puntero;
  - PgUp, PgDn, Inicio y Fin;
  - alto según el espacio disponible, con tope;
  - barra fina;
  - hover de fila y selección redondeada.
- **Índice:**
  - abre en la sección visible;
  - sangría de 14 px por nivel;
  - el flotante calcula la sección activa con el título más cercano al tercio
    superior y fuerza la última al llegar al final;
  - trunca con «…».
- **Menús:**
  - anclados al botón, con volteo en los bordes;
  - `draw_floating_surface`;
  - filas con ícono, nombre y atajo alineado a la derecha;
  - grupos en la paleta;
  - caja de búsqueda con borde y cursor;
  - estado vacío «Escribí para buscar…».
- **Símbolos y emojis:**
  - se filtran escribiendo;
  - emojis primero, en una sección propia;
  - columna de símbolo de ancho fijo.
- **Alternancias** de la paleta con estado («Movimiento reducido: activado
  ✓»).
- **Buscar en documento** como barra anclada arriba a la derecha, sin tapar
  el texto.
- **Menú contextual:**
  - se voltea si no entra;
  - Cortar, Copiar, Pegar y Deshacer en Editar;
  - en una tarea: marcar o desmarcar;
  - en un enlace: abrir y copiar dirección;
  - en una tabla: agregar fila o columna.

### Fase 4 · Integridad al editar (B1–B4)

- Salir de una lista con Enter en un ítem vacío deja una línea en blanco de
  separación.
- Tabla, código, cita, regla y recuadro se insertan en una línea propia, con
  una línea en blanco antes y después cuando hace falta.
- Formato sin selección: si el cursor está dentro de una palabra, la aplica a
  esa palabra en lugar de partirla.
- Insertar un símbolo dentro de una tabla lo coloca en la celda, nunca después
  del último `|`.
- Pruebas puras en `editor.rs` para cada caso, con la fuente intacta fuera del
  rango tocado.

### Fase 5 · Pestañas y paneles (E1–E5)

- Verde propio para pestañas (`tab`/`tab_idle`, ya medidos en la revisión de
  la mañana).
- Pestañas inactivas con hover, ✕ al pasar el puntero y tooltip con el nombre
  completo.
- La pestaña fijada se ancla a la izquierda, se compacta, lleva un ícono de
  chincheta y no muestra ✕.
- Al abrir un archivo se reemplaza la pestaña «sin título» si está vacía y sin
  cambios.
- Cerrar una pestaña que ocupa un panel cierra solo ese panel; la división se
  conserva con las demás.
- Un panel nuevo abre en Leer.
- Un único estilo de selector de modo: el de la barra actúa sobre el panel
  enfocado. Las cabeceras de panel usan el mismo componente, más chico.

### Fase 6 · Enlaces, carpeta y recuperación (G2, G5, F4, F5)

- **Enlaces:**
  - El web muestra el aviso «Abriendo en el navegador: dominio» y lleva el
    ícono ↗.
  - El roto se ve con subrayado punteado; al pasar el puntero dice «Esta nota
    no existe» y al hacer clic ofrece «Crear nota».
  - `[[nota]]` sin carpeta ofrece «Usar la carpeta de este archivo».
  - Una nota abierta desde un enlace abre en Leer.
- **Carpeta:**
  - Abrir la carpeta muestra el árbol de inmediato.
  - El árbol se actualiza al guardar y resalta la nota abierta.
  - Las carpetas usan chevron.
  - Las rutas se muestran siempre con «›».
  - Las notas que enlazan aquí muestran la línea de la mención.
- **Recuperación:**
  - «Recuperaciones» es una lista con documento de origen, fecha, vista
    previa, Restaurar y Descartar.
  - El diálogo de cerrar sin guardar nombra dónde restaurar.
- **Diálogo de cierre de la app:** mismo patrón que el de la pestaña, con la
  lista de documentos y «Guardar todo», «Cerrar sin guardar» y «Cancelar».

### Fase 7 · Lenguaje, avisos y primer arranque (F1–F3, G1, G3, G4)

- **Glosario único**, que se incorpora a [`glossary.md`](glossary.md):

  | Antes | Después |
  | --- | --- |
  | Workspace / bóveda / espacio | Carpeta de apuntes |
  | Backlinks | Notas que enlazan aquí |
  | callout | Recuadro |
  | enlace de bóveda | Enlace a otra nota |
  | guardado de forma atómica | Guardado |
  | TSV | Copiar para hoja de cálculo |
  | encabezado H1 | Título |
  | encabezado H2–H3 | Subtítulo |

  Todo en voseo. Se revisan todos los rótulos, avisos y tooltips con una prueba
  que prohíbe los términos retirados en los textos de la interfaz.
- **Avisos.**
  - Se eliminan los que repiten lo que muestra el control.
  - Las pistas de teclado van al pie del panel.
  - Los avisos se ubican en el área del documento, por encima de paneles, sin
    tapar la línea en edición.
- **Primer arranque.**
  - Bienvenida con tres acciones grandes: Abrir carpeta de apuntes, Abrir
    archivo y Nuevo apunte.
  - El aviso de recuperación pasa a una línea amable con «Más detalles»,
    sin bloquear.
- **Barra superior.**
  - Botón sol/luna.
  - «Exportar a PDF» en el menú del botón Guardar (Guardar · Guardar como ·
    Exportar a PDF).
- **Íconos.**
  - Marcador para resaltar y `</>` para código en línea.
  - Carpeta abierta y carpeta con lupa.
  - Triángulo para plegar.
  - Botón «Título» visible en Editar.
  - Chevrons a 4 px del rótulo.

### Fase 8 · Hover, pulsación y lectura (D1–D4, H1–H6)

- **Hover y pulsación:**
  - hover con fundido en todo lo interactivo y destello de 160 ms al pulsar;
  - Copiar con ícono y ✓ animado, sin aviso adicional;
  - chincheta rellena en acento cuando está activa, con tooltip «Fijada · clic
    para soltar»;
  - ícono de restaurar al maximizar;
  - barra fina de desplazamiento del documento, visible al desplazar o al pasar
    el puntero.
- **Lectura:**
  - etiqueta del recuadro dentro del bloque, con ícono;
  - margen entre bloques de código contiguos y etiqueta de lenguaje junto a
    Copiar;
  - escala H2/H3 más marcada;
  - columna de lectura centrada en ventanas anchas.
- **Tema de día:** `dim` e íconos oscurecidos hasta ≥ 4,5:1 con margen,
  validados por la prueba de contraste de `theme.rs`.
- **PDF:** verificar la capa de texto con un lector real. Si hay desorden o
  duplicación, emitir una sola capa en orden de lectura, sin duplicar enlaces
  ni celdas.

### Fase 9 · Re-evaluación

- Repetir **las mismas dos evaluaciones** (misma hoja, mismas 16 tareas, perfil
  vacío, misma herramienta) con el build nuevo.
- Criterios de salida:

  | Indicador | Objetivo |
  | --- | --- |
  | Hallazgos S1 y S2 abiertos | 0 |
  | SUS | ≥ 68 |
  | Tareas sin dificultad | ≥ 80 % |
  | Heurísticas de Nielsen | ≥ 3,5 de promedio, ninguna < 3 |
  | Requerimientos «No cumple» | 0 |

- Comprobación manual del propietario, con la lista de QA actualizada en
  [`manual-qa-v2.md`](manual-qa-v2.md).
- Actualizar [`design.md`](design.md) (roles `tab`/`tab_idle`, estados, regla
  de interruptores, glosario visual) y [`features.md`](features.md).

### Orden y dependencias

```text
Fase 1 ─┐
        ├─ Fase 2 ── Fase 3 ── Fase 5 ── Fase 8 ─┐
Fase 4 ─┘            Fase 6 ── Fase 7 ───────────┴─ Fase 9
```

Las fases 1 y 4 son independientes y van primero, porque cierran los defectos
que impiden tareas. La fase 3 depende de la 2. Las fases 6 y 7 pueden avanzar
en paralelo con la 5. Después de este plan siguen el instalador de Windows, el
paquete de Linux y el Pomodoro.

## Estado

| Fase | Estado |
| --- | --- |
| Evaluación y plan | Hecho (este documento) |
| 1 · Correcciones funcionales | Pendiente |
| 2 · Base de objetivos y animación | Pendiente |
| 3 · Listas, menús y alternancia | Pendiente |
| 4 · Integridad al editar | Pendiente |
| 5 · Pestañas y paneles | Pendiente |
| 6 · Enlaces, carpeta y recuperación | Pendiente |
| 7 · Lenguaje, avisos y primer arranque | Pendiente |
| 8 · Hover, pulsación y lectura | Pendiente |
| 9 · Re-evaluación | Pendiente |
