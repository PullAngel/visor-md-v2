# Herramientas para el rediseño y las correcciones · 14 de noviembre de 2026

Investigación de skills, plugins y herramientas que pueden mejorar la
ejecución del plan de
[`evaluacion-usabilidad-2026-11-14.md`](evaluacion-usabilidad-2026-11-14.md#plan-definitivo-de-cambios).
El objetivo no es sumarlas todas: es elegir **dos** que suban la calidad del
resultado sin agregar peso al producto.

## Criterios

Cada candidata se juzgó por cinco preguntas:

1. **Encaje con el stack.** La app es nativa: Rust, winit, tiny-skia y dibujo
   propio, sin HTML ni DOM. Una herramienta pensada para la web no ve nada de
   la app.
2. **Hallazgos que ataca.** Se miden contra las áreas A–H de la evaluación.
3. **Costo para el binario.** El presupuesto es de 7 MB. Toda dependencia nueva
   necesita un ADR y aprobación ([`AGENTS.md`](../AGENTS.md)).
4. **Repetibilidad.** ¿Sirve en cada fase o solo una vez?
5. **Riesgo.** ¿Cuánto cuesta integrarla y qué tan madura es?

## Candidatas evaluadas

| # | Herramienta | Tipo | Qué aporta | Encaje | Veredicto |
| --- | --- | --- | --- | --- | --- |
| 1 | **Design** (plugin de Anthropic): skills `ux-writing`, `design-critique`, `accessibility-review`, `user-research` | Skill de Claude | Microcopy coherente, crítica de diseño desde capturas, auditoría WCAG y síntesis de pruebas con usuarios. Funciona sin Figma. | Alto: trabaja con capturas y texto, no necesita DOM | **Elegida** |
| 2 | **Pruebas de imagen de referencia** (render tiny-skia a PNG comparado contra una referencia; [kompari](https://github.com/linebender/kompari) como visor de diferencias) | Técnica más herramienta de desarrollo | Cada estado de la interfaz (menú abierto, hover, pestañas, temas y tamaños) queda fijado como imagen. Un cambio visual involuntario rompe una prueba. | Alto: la app ya dibuja en un `Pixmap` | **Elegida** |
| 3 | [AccessKit](https://github.com/AccessKit/accesskit) con `accesskit_winit` | Biblioteca | Árbol de accesibilidad real: UI Automation en Windows y AT-SPI en Linux. Habilita lectores de pantalla y pruebas por nombre de control en vez de coordenadas. | Alto, pero es una dependencia nueva con un ADR, y su cobertura de texto enriquecido todavía es parcial | Recomendada **después** del plan |
| 4 | [kittest](https://docs.rs/crate/kittest/0.2.0) | Biblioteca de pruebas de GUI sobre AccessKit | Pruebas que buscan y pulsan controles por rol y nombre | Depende de la #3 | Junto con AccessKit |
| 5 | [Accessibility Insights for Windows](https://accessibilityinsights.io/docs/windows/overview/) | App de Microsoft | Inspección de UI Automation, FastPass y analizador de contraste con cuentagotas | Medio: hoy la app no expone UIA, así que solo sirve el contraste. Gana valor con la #3. | Uso manual puntual (contraste en la fase 8) |
| 6 | [Vale](https://vale.sh/docs/topics/vocab) | Linter de prosa | Reglas de sustitución para un glosario (Workspace → Carpeta de apuntes, etc.) | Bajo: los textos de la interfaz viven en Rust, no en documentos. Una prueba Rust que prohíba términos retirados hace lo mismo sin otra herramienta. | Descartada; se reemplaza por una prueba propia |
| 7 | [insta](https://docs.rs/insta) | Snapshots de texto | Fija en texto tablas de geometría (zonas de clic, anclajes de menú) | Medio: útil como complemento de la #2, pero pide un ADR | Opcional |
| 8 | [image_diff_review](https://lib.rs/crates/image_diff_review) | Visor de diferencias de imagen | Hace lo mismo que kompari | Igual que kompari, con menos actividad | Alternativa a kompari |
| 9 | **UI Design Review** (plugin de UXKIN) | Skill de Claude | Revisión «de diseñador sénior» y comparación con productos reales | Bajo: está orientado a web y móvil, su servidor MCP es de terceros y se superpone con la #1 | Descartada |
| 10 | **frontend-design** (plugin de Anthropic) | Skill de Claude | Diseño de interfaces web | Nulo: genera HTML/CSS/React | Descartada |
| 11 | **Axe Accessibility** (Deque) y **A11y Enforcer** | Plugins | Auditoría WCAG del DOM | Nulo: no hay DOM | Descartadas |
| 12 | Herramienta de evaluación propia ([`scripts/evaluacion/`](../scripts/evaluacion/README.md)) | Ya existe | QA y usuaria simuladas, sin tocar el mouse ni el teclado reales | Total | Se usa en las fases 1–9 y en la re-evaluación |
| 13 | `/code-review` y `/simplify` (skills incluidas en Claude Code) | Skills | Revisión de correctitud y limpieza de cada diff antes del commit | Total, sin costo | Se usan al cierre de cada fase |

## Elección

### 1. Plugin «Design» de Anthropic

**Por qué.**
- Ataca las áreas que la evaluación marcó como más flojas y que el código no
  resuelve solo:
  - lenguaje: F1–F3, que corresponden a las heurísticas H2 y H4;
  - jerarquía y consistencia visual: C4, D1, G3 y H1–H5;
  - accesibilidad: H5 y foco.
- Trabaja desde las capturas que ya genera la herramienta de evaluación.
- No toca el binario.

**Cómo se usa en el plan.**
- **Fase 7.**
  - `ux-writing` produce el glosario definitivo y reescribe todos los rótulos,
    avisos y tooltips en voseo con una voz única.
  - Esos textos se fijan con una prueba Rust que prohíbe los términos
    retirados.
- **Fases 3, 5 y 8.** Al cierre de cada una, `design-critique` revisa las
  capturas de antes y después en los dos temas y en tres tamaños. Sus
  observaciones de severidad S2 o más se corrigen antes de pasar de fase.
- **Fase 8.** `accessibility-review` audita contraste, foco y tamaño de las
  zonas de clic. El contraste se confirma con el cuentagotas de Accessibility
  Insights.
- **Fase 9.** `user-research` sintetiza la segunda ronda (QA y usuaria) y la
  compara con la primera.

**Instalación.** La hace el propietario desde el catálogo de plugins de
claude.ai. No requiere Figma; las integraciones que trae (Figma, Notion,
Slack…) son opcionales y no hacen falta.

### 2. Pruebas de imagen de referencia del dibujo

**Por qué.** El plan cambia el aspecto de casi todas las superficies. Sin una
red de seguridad visual, cada fase puede romper lo que arregló la anterior. Ya
pasó antes: el desfase de geometría entre dibujo y clic fue el origen de
varios defectos.

Además, la app dibuja todo en un `tiny_skia::Pixmap`, que puede guardarse como
PNG sin dependencias nuevas, porque tiny-skia ya codifica PNG.

**Cómo se integra.**
- **Fase 2.** Separar el dibujo de un cuadro de la ventana para poder llamarlo
  en pruebas con un `App` de prueba, un tamaño fijo y el tema elegido.
- Se crea un conjunto de unas 30 escenas:
  - lectura, edición y comparación;
  - paleta abierta;
  - menú Bloques;
  - índice desplazado;
  - pestañas con hover;
  - diálogo;
  - aviso;
  - cada una en noche y día, y a 640×480 y 1100×720.
- Las referencias PNG quedan en `tests/visual/`.
- La comparación es propia, por píxel y con una tolerancia baja para el
  antialias, y escribe la imagen de diferencia cuando falla.
- **kompari** se usa solo como herramienta de revisión externa (CLI, `cargo
  install`, fuera de `Cargo.toml`) para ver las diferencias y aprobar las
  nuevas referencias. No entra en el binario ni en las dependencias del
  proyecto.
- Cada fase actualiza las referencias que cambia a propósito, y el commit
  muestra el antes y el después.

**Riesgo.** Las fuentes del sistema pueden variar entre máquinas. Para
evitarlo, las escenas usan solo las fuentes embebidas, y la CI de Linux
compara con tolerancia o se limita a Windows si hiciera falta.

## Lo que queda para después

**AccessKit con kittest** es la mejora de fondo más valiosa:

- lectores de pantalla en Windows y Linux;
- pruebas por nombre de control, que harían innecesaria la herramienta por
  coordenadas;
- base para la versión Android.

Implica una dependencia nueva, un ADR y exponer el árbol de toda la interfaz.
Es un proyecto propio. Conviene después de la fase 2, cuando `UiTarget` ya
describa cada control interactivo; ese enum es casi el árbol que pide
AccessKit.

## Fuentes

- Plugin Design de Anthropic: [anthropics/knowledge-work-plugins/design](https://github.com/anthropics/knowledge-work-plugins/tree/main/design)
- [AccessKit](https://github.com/AccessKit/accesskit) y [accesskit_winit](https://docs.rs/crate/accesskit_winit/)
- [kittest](https://docs.rs/crate/kittest/0.2.0)
- [kompari](https://github.com/linebender/kompari) e [image_diff_review](https://lib.rs/crates/image_diff_review)
- [insta](https://docs.rs/insta)
- [Accessibility Insights for Windows](https://accessibilityinsights.io/docs/windows/overview/) y su [analizador de contraste](https://accessibilityinsights.io/docs/windows/getstarted/colorcontrast/)
- [Vale: vocabularios y sustituciones](https://vale.sh/docs/topics/vocab)
