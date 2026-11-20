# Matriz de pruebas

Estado de esta matriz: recuperación activa. Los nombres de tests indicados viven
en `src/main.rs` hasta que el proyecto se divida en módulos.

Estados permitidos:

- `Pendiente`: todavía no existe evidencia suficiente.
- `Parcial`: existe una prueba, pero no cubre toda la propiedad.
- `Verificado`: evidencia automatizada o manual reproducible.
- `Bloqueado`: depende de una capacidad aún no implementada.

## Núcleo Markdown

| Propiedad | Nivel | Windows | Linux | Evidencia actual |
| --- | --- | --- | --- | --- |
| CommonMark aplicable | Corpus | Parcial | Pendiente | Fixture versionada más `casos_commonmark_gfm_anunciados_llegan_a_layout` cubren parser, rangos y layout de encabezados, inline, saltos, escapes, enlaces, citas, listas, tareas, código, tablas, HTML permitido e inerte; incluye ejemplos 16 y 20 de CommonMark 0.31.2; falta suite oficial compatible |
| Formato inline anidado | Integración | Verificado | Pendiente | `el_enfasis_anidado_se_acumula` y tests de rangos |
| Listas y task lists | Integración y visual | Parcial | Pendiente | Parser, layout, píxeles, geometría de clic ampliada y reemplazo exclusivo de `[ ]`/`[x]` verificados; falta evidencia estética y manual |
| Allowlist HTML y HTML no permitido | Seguridad e integración | Verificado | Pendiente | `br`, `kbd`, `mark`, `sub` y `sup` sin atributos llegan a layout; HTML hostil, atributos y cierres defectuosos quedan visibles e inertes |
| Profundidad limitada | Adversarial | Verificado | Pendiente | 5.000 citas, listas e inline anidado |
| Línea extensa | Adversarial | Verificado | Pendiente | Más de 16 KiB degrada a tramos UTF-8 inertes y reconstruibles |
| Barrido adversarial reproducible | Property y adversarial | Verificado | Pendiente | 128 combinaciones deterministas sin `panic`; no sustituye fuzzing formal |
| Fallback a fuente segura | Modelo y end to end | Parcial | Pendiente | Fuente completa y título de aviso verificados; falta QA visual |
| Unicode y fallback | Corpus, layout y manual | Parcial | Pendiente | Corpus árabe, devanagari, japonés, coreano y emoji llega a layout; fuentes embebidas siguen siendo latinas y falta QA visual |

## Archivos y edición

| Propiedad | Nivel | Windows | Linux | Evidencia actual |
| --- | --- | --- | --- | --- |
| Apertura explícita local | Integración | Parcial | Pendiente | Un mismo handle limita 16 MiB y exige UTF-8; falta QA visual y política configurable |
| Parser fuera de UI | Integración y rendimiento | Parcial | Pendiente | Hilo de trabajo entrega un único evento al hilo de ventana; falta benchmark y cancelación |
| Apertura manual UNC | Integración | Pendiente | No aplica | Política definida |
| Traversal y escape bloqueados | Seguridad | Verificado | Pendiente | Enlaces, workspace y PNG pasan por VFS; la ruta base y el destino canonicalizado deben quedar dentro de la raíz |
| Symlinks y junctions | Seguridad | Verificado | Pendiente | La canonicalización posterior a unir descubre escapes existentes |
| Guardado atómico | Integración | Verificado | Pendiente | Temporal en el mismo filesystem, sincronización, reemplazo y conflicto externo tienen regresiones |
| Rangos de fuente preservados | Integración | Parcial | Pendiente | Bloques, tramos y destinos verificados; el editor source-first no usa el rango parcial que Comrak informa para la sintaxis de enlaces |
| Parches de fuente y undo/redo | Unitario y end to end | Parcial | Pendiente | `editor::EditHistory` preserva UTF-8, revisiones, undo/redo y presupuesto de 4 MiB; una regresión conecta apertura, Rope, parche, undo/redo, guardado atómico y reapertura con BOM, CRLF/LF/mixtos, Unicode y sintaxis inerte. Faltan interacción, IME, corpus mayor y QA multiplataforma |
| Edición de fuente inicial | Integración y manual | Parcial | Pendiente | F2 alterna fuente/lectura; IME, Backspace, Delete, Ctrl+A, Ctrl+Z/Ctrl+Y, clic, arrastre, Inicio/Fin y flechas comparten fuente. La vista se prepara fuera de UI y un fallo conserva fuente. Faltan prueba end-to-end y guardado |
| Selector de modo | Unidad, integración y manual | Parcial | Pendiente | Leer, Editar y Comparar son destinos directos; no deben activar arrastre de ventana ni desplazar el foco de acciones con F6. Falta QA visual de contraste y descubribilidad |
| Sintaxis desconocida preservada | End to end y property | Parcial | Pendiente | HTML inerte y documento defectuoso histórico; round-trip determinista de archivo, editor y guardado ya cubierto; falta property testing ampliado |
| Cambios externos detectados | Unitario e integración | Parcial | Pendiente | La apertura captura tamaño y fecha de modificación; una prueba detecta cambio de tamaño. Falta comparación antes de guardar, identidad de handle y diálogo de conflicto |
| EOL, BOM y UTF-8 | Corpus | Parcial | Pendiente | Apertura limitada exige UTF-8, separa BOM UTF-8 sin volverlo contenido, conserva CRLF/LF/mixto y reconstruye bytes sin edición en pruebas; `Fin` no deja el cursor entre CR y LF; faltan parches, guardado y política de otros encodings |

## Red y recursos

| Propiedad | Nivel | Windows | Linux | Evidencia actual |
| --- | --- | --- | --- | --- |
| Apertura normal sin sockets | Seguridad de runtime | Pendiente | Pendiente | Sin monitor automatizado |
| Imagen remota bloqueada | Integración | Verificado | Pendiente | El destino permanece visible, pero no llega a red ni al decodificador |
| Consentimiento remoto delimitado | End to end | Bloqueado | Bloqueado | Componente pendiente |
| Imagen local contenida y limitada | Seguridad | Verificado | Pendiente | Confirmación por imagen, VFS desde la nota, firma PNG, 8 MiB, 8192 por lado y 16 millones de píxeles; falta QA visual |
| Hipervínculo revela destino real | UX y phishing | Parcial | Pendiente | El hover muestra el destino declarado sin abrirlo; falta QA visual y política de clic |

## Rendering y UX

| Propiedad | Nivel | Windows | Linux | Evidencia actual |
| --- | --- | --- | --- | --- |
| Tema claro y oscuro | Unitario y visual | Parcial | Pendiente | Paletas interpolan todos sus roles y la transición termina en 200 ms sin bucle permanente; la preferencia de reducir movimiento la vuelve instantánea y falta QA visual |
| Reducir movimiento | Unitario y configuración | Parcial | Pendiente | La preferencia local versionada migra desde formatos anteriores y evita la transición de tema; falta QA visual y extensión al resto de animaciones |
| Resize sin layout corrupto | Integración | Parcial | Pendiente | Prototipo |
| Chrome y controles de ventana | Unitario y manual | Parcial | Pendiente | Geometría de controles, drag y resize cubierta; falta QA manual de Windows sin borde y fallback nativo |
| Pestañas, fijado y cierre protegido | Unitario e integración | Parcial | Pendiente | Cambio, envoltura, identidad separada y protección de pestaña fijada probados; falta QA con mouse y documentos reales |
| Árbol de paneles de documentos | Unitario y adversarial | Parcial | Pendiente | Referencias sin duplicar buffers, selector de destino, colapso al cerrar, sucesor visible, geometría horizontal/vertical, tamaños transitorios, fracciones inválidas, scroll local y raster aislado probados; falta QA visual, foco de teclado, accesibilidad y ventanas separadas. |
| DPI y zoom | Unitario, manual y visual | Parcial | Pendiente | El layout escala cuerpo, márgenes, sangrías y marcadores a partir de `Window::scale_factor`; `la_escala_dpi_aumenta_la_tipografia_sin_cambiar_el_ancho_logico` verifica la propiedad geométrica. Falta QA en monitores con distintas escalas y el futuro zoom explícito. |
| Selección con mouse | Integración y visual | Parcial | Pendiente | Hit testing y geometría de Parley comparten el layout; autoscroll y copia unitaria probados, falta QA manual |
| Copia al portapapeles | Integración y seguridad | Parcial | Pendiente | En lectura, `Ctrl+C` y `Ctrl+Shift+C` distinguen vista y fuente; en edición la selección sale directamente del buffer Unicode, no de un render atrasado. Falta QA con otras aplicaciones y plataformas |
| Cursor de texto sobre contenido | Manual | Parcial | Pendiente | Cambia con el mismo hit testing de selección; requiere QA visual |
| Pérdida de foco durante selección | Manual | Parcial | Pendiente | Cancela arrastre y modificadores, conserva selección; requiere QA manual |
| Selección con teclado | Accesibilidad | Parcial | Pendiente | Flechas, Ctrl+flechas por palabra, Inicio/Fin, Ctrl+Inicio/Fin, Shift, Ctrl+A y Escape funcionan; falta QA de foco y plataforma |
| Menú contextual | End to end | Parcial | Pendiente | Menú propio contextualiza copiar, pegar, cortar, formato, tablas y estudio; falta QA manual de cierre, foco y estados deshabilitados |
| IME | Manual | Parcial | Pendiente | Los commits IME llegan al buffer Unicode; falta QA multilingüe por plataforma |
| Lector de pantalla | Accesibilidad | Bloqueado | Bloqueado | Estrategia pendiente |
| Reduce motion | Manual y unitario | Parcial | Pendiente | Preferencia local versionada elimina la transición de tema; falta aplicarla a futuras transiciones de paneles y pestañas |

## Rendimiento

| Propiedad | Nivel | Windows | Linux | Evidencia actual |
| --- | --- | --- | --- | --- |
| Ventana visible | Benchmark | Verificado | Pendiente | Mediana 89 ms y P95 600 ms en diez ejecuciones |
| Primer contenido normal | Benchmark | Verificado | Pendiente | Mediana 102,5 ms y P95 612 ms; muestras versionadas |
| Documento de 5 MB | Benchmark | Verificado en Sprint 0 | Pendiente | `budget.md` |
| Scroll proporcional a visible | Unitario y benchmark | Parcial | Pendiente | Rango visible por búsqueda binaria; 4,4 ms medidos |
| Memoria estable | Benchmark | Parcial | Pendiente | Medición inicial |
| Binario menor de 8 MB | Release | Verificado | Pendiente | 3.515.392 bytes, 3,353 MiB, release del 9 de septiembre de 2026 |

## Cadena de suministro

| Propiedad | Nivel | Estado | Evidencia actual |
| --- | --- | --- | --- |
| Advisories conocidos | Audit | Verificado el 2026-08-26 | Cero vulnerabilidades, un crate no mantenido |
| Licencias compatibles | Legal | Parcial | SBOM completo; falta revisión de compatibilidad y notices |
| SBOM reproducible | Release | Parcial | Generador CycloneDX desde metadata bloqueada; falta validador independiente |
| Dependencias C y `unsafe` conocidas | Audit | Parcial | Diferencias Windows y Linux |
| Fuentes reproducibles | Supply chain | Verificado | Script reproduce cuatro hashes y licencia versionada |

## Regla de actualización

Una fila pasa a `Verificado` solo si enlaza a un test, comando, artifact o
registro reproducible. Una afirmación verbal o un binario antiguo no alcanza.

## Recuperación y ventanas (noviembre de 2026)

| Comportamiento | Pruebas |
| --- | --- |
| Formato `.vmdr`, suma de verificación y copias dañadas | `recovery::formato::pruebas` |
| Sesiones, cerrojo, adopción sin carreras, barrido y cuota | `recovery::almacen` y `recovery::tests` |
| Gancho de pánico: vuelca lo publicado y lo retirado no se escribe | `el_gancho_de_panico_vuelca_lo_publicado_y_lo_retirado_no_se_escribe` |
| Traspaso de una pestaña: formato, nombre válido, daño | `un_traspaso_se_escribe_se_lee_con_su_suma_y_se_descarta`, `un_nombre_de_traspaso_que_no_es_hexadecimal_nunca_llega_a_una_ruta` |
| Registro de ventanas, latido caduco y buzón | `ventanas::pruebas` |
| Abrir desde fuera en la ventana abierta | `un_archivo_enviado_por_otra_ventana_se_abre_aqui_y_el_latido_se_renueva` |
| Mover pestañas (guardadas y sin guardar) entre ventanas | `mover_una_pestana_guardada_a_otra_ventana_la_envia_y_la_cierra_aqui`, `una_pestana_sin_guardar_viaja_a_otra_ventana_y_el_origen_la_cierra_al_confirmar`, `si_la_pestana_cambia_mientras_se_mueve_el_origen_no_la_cierra` |
| Soltar una pestaña fuera de la ventana | `soltar_una_pestana_fuera_de_la_ventana_la_manda_a_la_ventana_de_debajo` |
| Restaurar ligado al original o aparte | `restaurar_con_el_original_sin_cambios_lo_liga_a_su_archivo`, `restaurar_con_el_original_cambiado_o_ausente_abre_la_copia_aparte` |
| Asociación de archivos (clave de prueba) | `asociacion::pruebas` |

## Anotaciones de estudio (noviembre de 2026)

| Comportamiento | Pruebas |
| --- | --- |
| Formato: ida y vuelta, orden, escapes, límites, contenido desconocido | `anotaciones::pruebas` (`ida_y_vuelta_conserva_bytes`, `escapa_los_separadores_de_linea_de_unicode`, `ninguna_nota_puede_cerrar_el_comentario`, `limites_vuelven_inerte_el_bloque`…) |
| Anclaje por cita y contexto, huérfanas | `camino_rapido_ancla_en_su_posicion`, `cita_repetida_desempata_por_contexto`, `texto_borrado_deja_huerfana_sin_eliminarla`, `tope_de_apariciones_se_respeta` |
| Separar cuerpo y bloque, byte a byte | `cuerpo_y_bloque_se_separan_al_abrir`, `agregar_y_quitar_deja_el_archivo_identico`, `el_bloque_de_anotaciones_viaja_aparte_y_se_guarda_con_los_mismos_bytes` |
| Leer no lo dibuja; Editar lo contrae | `leer_no_dibuja_el_bloque_ni_lo_suma_al_alto`, `editar_muestra_el_bloque_contraido_en_una_linea`, `abrir_el_bloque_lo_muestra_de_solo_lectura` |
| Recuperación con anotaciones | `recuperacion_restaura_cuerpo_y_anotaciones` |
| Paleta de estudio | `paleta_de_estudio_conserva_contraste_en_ambos_temas`, `colores_de_estudio_se_distinguen_entre_si_y_del_resaltado` |
| Subrayar, color, leyenda, ocultar | `subrayado_cruza_bloques_y_se_dibuja_por_tramos`, `subrayar_usa_el_ultimo_color_elegido`, `cambiar_el_color_no_mueve_el_ancla`, `subrayar_marca_el_documento_sin_guardar`, `ocultar_mis_anotaciones_muestra_el_original` |
| Marcas de libro e índice | `marca_suma_un_punto_al_titulo_de_su_seccion`, `muchas_marcas_no_cambian_la_fila_del_indice`, `mas_de_tres_colores_muestran_mas_n`, `la_ayuda_lista_las_marcas_de_la_seccion` |
| Editar, huérfanas, recalculo al guardar | `editar_antes_del_subrayado_lo_desplaza`, `borrar_el_texto_anclado_lo_deja_huerfano`, `deshacer_el_borrado_recupera_el_subrayado` |
| PDF, quitar todas, copia sin anotaciones | `pdf_no_incluye_el_bloque_en_la_capa_de_texto`, `pdf_sin_anotaciones_si_se_desmarca_la_casilla`, `quitar_todas_se_puede_deshacer`, `copia_sin_anotaciones_es_el_cuerpo` |
