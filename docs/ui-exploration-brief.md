# Brief de exploración visual

Este brief permite explorar alternativas de interfaz antes de cambiar código.
No reemplaza [`design.md`](design.md): una maqueta bonita no autoriza cambios de
seguridad, accesibilidad, persistencia o arquitectura.

## Uso

1. Adjuntar una captura reciente de Visor MD junto con el prompt.
2. Pedir tres variantes que cambien jerarquía y distribución, no solo colores.
3. Elegir primero estructura y densidad; iconos y detalles vienen después.
4. Traer la propuesta elegida. Se compara contra `design.md` y los flujos reales
   antes de implementar nada.

## Prompt base para Google Stitch

```text
Design a high-fidelity 1440×900 native desktop UI for “Visor MD v2”: a fast,
offline, security-first Markdown reader and editor for students, people working
with AI-generated documents, and Obsidian vault users. It is not a browser,
IDE, dashboard, chat app, or clone of Obsidian.

The visual identity is “Paper + Ink”: editorial, calm, capable, trustworthy and
slightly organic. The document is always the visual priority. Use an editorial
serif for document text, a precise contemporary sans-serif for UI, and a
monospace face only for code.

Nearly monochrome color system. Green appears only as a thin accent for active
state, active tab, internal link, shortcut or alert rule. Dark: window #0C0F0D,
document #121513, raised #1A1F1C, floating #232A26, accent #5FD08A. Light:
paper #F7F5EF, dark ink, restrained neutrals, accent #2E9E5B. External web
links are conventional blue and underlined.

Use flat document surfaces. Only menus, dialogs and popovers have a soft subtle
shadow. Avoid outlines around every element. Rounded corners are selective:
about 11px for floating surfaces and modest radius for the Read/Edit/Compare
segmented control. Do not turn every button into a pill.

Required structure:
- compact title/chrome with document name and window controls;
- primary actions: New, Open, Save, Search, Workspace, More;
- a prominent elegant segmented toggle: Read / Edit / Compare;
- a contextual second row: reading tools in Read mode, Markdown writing tools
  in Edit mode, and workspace tools clearly grouped rather than mixed;
- a centered reading column around 62–72 characters with generous typography;
- bottom tab strip plus discreet status;
- foldable panels for outline, vault, search and backlinks, only on demand;
- a four-document horizontal/vertical split with calm dividers and a clear
  focused pane;
- one menu, one contextual menu and one unsaved-changes dialog using the same
  elevated visual language.

Suggest restrained 120–240ms motion for hover, tabs, mode toggle and pane
splits, plus an explicit reduced-motion state. Never animate document text,
cursor or typing.

Avoid browser chrome, web cards, permanent dashboard sidebars, neon green,
thick borders, dense IDE toolbars, terminal aesthetics, generic AI branding,
gradients, excessive rounded pills, remote content or embedded AI. Return a
desktop UI concept, not code.
```

## Variantes útiles

### A. Lectura editorial

```text
Use the base prompt. Make reading exceptionally quiet. The document is dominant
and the Read/Edit/Compare control is the clearest persistent interaction.
Explore a small left rail only if it improves wayfinding without becoming a
permanent sidebar. Give Workspace its own recognizable entry point.
```

### B. Escritura guiada

```text
Use the base prompt. Focus on Edit mode. Design a complete, progressive
Markdown writing kit: formatting, headings, lists, links, code, tables,
callouts, tasks, study notes and clean-copy actions. Everyday tools are visible;
the long tail belongs in More or a command palette. Show a selection-aware
context menu. It must feel capable but not like an IDE.
```

### C. Varias notas a la vez

```text
Use the base prompt. Focus on four different Markdown files in horizontal and
vertical splits in one window. Make pane focus, tab ownership, close-pane versus
close-document, and adding another file obvious. Keep reading hierarchy strong;
do not turn it into a dashboard. Include a compact workspace panel and outline.
```

## Preguntas para comparar propuestas

- ¿Se entiende dónde se lee, dónde se edita y cómo se cambia entre ambos?
- ¿El documento sigue siendo la zona más clara e importante?
- ¿Workspace ayuda sin invadir a quien solo quiere abrir un `.md`?
- ¿Las acciones frecuentes se encuentran sin llenar la barra?
- ¿La app parece nativa, cuidada y confiable, no web, terminal ni IDE?
- ¿Menús, paneles y diálogos comparten elevación, radio y sombra?
- ¿La composición conserva claridad en una ventana estrecha y con cuatro hojas?

## Límite de adopción

Antes de adoptar una propuesta, separar cambios visuales de cambios que afectan
flujos, accesibilidad, rendimiento o propiedad de documentos. La maqueta no
autoriza dependencias, persistencia, red ni ventanas nuevas. La opción elegida
se traduce después a cambios pequeños y pruebas.
