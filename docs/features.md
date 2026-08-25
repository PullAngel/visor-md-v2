# Catálogo de funciones — Visor MD v2

Lista completa de funciones candidatas, ordenadas de más a menos relevantes.
Cada bloque es una casilla para marcar. Las funciones con carga técnica traen
**a favor / en contra** para poder decidir sin volver a razonarlo desde cero.

Nada de esto está construido: es el mapa para decidir qué entra en cada fase.

**Leyenda de esfuerzo**  🟢 bajo · 🟡 medio · 🔴 alto · ⬛ muy alto
**Leyenda de peso**  ✳️ cabe holgado en 7 MB · ⚠️ presiona el presupuesto · 🧩 componente aparte

---

## 1 · Núcleo — sin esto no hay producto

- [ ] **Render de CommonMark + GFM completo** 🔴 ✳️
  Tablas, listas anidadas, tareas, notas al pie, tachado, autolinks, citas.
  *A favor:* es la razón de existir de la app; sin cobertura completa un
  documento real se ve roto.
  *En contra:* es el trabajo más grande después del layout de texto. Un parser
  hecho a medias produce fallos silenciosos y difíciles de encontrar.

- [ ] **Resaltado de sintaxis en bloques de código** 🟡 ⚠️
  *A favor:* la mitad de los `.md` técnicos son código; sin color se leen mal.
  *En contra:* `syntect` con sus ~200 gramáticas pesa demasiado. La salida es
  embeber 15-20 lenguajes comunes y dejar el resto sin colorear. Decidido en
  `budget.md`.

- [ ] **Botón de copiar en cada bloque de código** 🟢 ✳️
  *A favor:* la función más usada de la v1 en la práctica.
  *En contra:* ninguno. Debe copiar exactamente lo visible, nunca texto oculto.

- [ ] **Se fija como programa predeterminado de Windows para `.md`** 🟡 ✳️
  *A favor:* es el gesto central — doble clic y se abre.
  *En contra:* Windows no deja que una app se fije sola; el usuario da el paso
  final. Hay que explicarlo bien o parece que no funcionó.

- [ ] **Modo lectura y modo edición, con vista dividida** 🟡 ✳️

- [ ] **Pestañas y ventanas al estilo navegador** 🔴 ✳️
  *A favor:* abrir ocho archivos y que aterricen en una ventana es lo que
  separa un visor de un juguete.
  *En contra:* arrastrar pestañas entre ventanas fue lo más difícil de la v1 y
  en nativo hay que rehacerlo entero.

- [ ] **Distribución portable sin instalador** 🟢 ✳️

- [ ] **Temas día y noche** 🟢 ✳️

---

## 2 · Seguridad — la tesis del proyecto

- [ ] **Sin motor de scripts: nada del documento se ejecuta** 🟡 ✳️
  *A favor:* elimina la clase de ataque entera por construcción, en vez de
  contenerla. Es la ventaja estructural del enfoque nativo.
  *En contra:* pierde HTML arbitrario. Se sostiene una allowlist de nodos
  dibujables; lo desconocido se muestra inerte o se descarta.

- [ ] **Abrir un documento no genera ninguna petición de red** 🟢 ✳️
  *A favor:* una imagen remota rastrea aunque no ejecute nada. En nativo es
  casi gratis de garantizar: la capa de archivos es la única que podría salir
  a la red, y no lo hace.

- [ ] **Contención de rutas canonizadas** 🟡 ✳️
  *A favor:* impide que un documento lea archivos ajenos. Se hereda de la v1
  (`safe_media_path`), ya probado.
  *En contra:* hay que replicar exactamente el rechazo de rutas UNC y flujos
  alternativos de NTFS, o se reabre la fuga de credenciales que la v1 corrigió.

- [ ] **Un único punto de sanitización, sin forma de apagarlo** 🟡 ✳️
  *A favor:* patrón validado por ByteMD; el contraejemplo es EasyMDE, con saneo
  opcional apagado por defecto.
  *En contra:* ninguno real. Es disciplina de arquitectura, no coste.

- [ ] **Límites contra bombas de recursos** 🟢 ✳️
  Tope de nodos, de profundidad de anidamiento, de diagramas.
  *A favor:* un `.md` con anidamiento patológico no debe colgar la app.
  *En contra:* topes mal calibrados rompen documentos legítimos. Hay que
  medirlos contra el corpus real, no elegirlos a ojo.

- [ ] **Suite de seguridad con corpus de ataque** 🟡 ✳️
  *A favor:* la de la v1 encontró tres fugas reales en su primera corrida. Es
  lo que convierte "creemos que es seguro" en evidencia.

- [ ] **Fuzzing del parser** 🟢 ✳️
  *A favor:* barato en Rust con `cargo-fuzz`, y encuentra los casos que nadie
  imagina. La v1 no podía hacerlo con la misma facilidad sobre su pipeline JS.

- [ ] **Bitácora de confianza auditable** 🟡 ✳️
  Registro visible de qué permiso se amplió, cuándo y por qué, con opción de
  revocar.
  *A favor:* ningún competidor lo tiene. Convierte una decisión hoy invisible
  en algo revisable.
  *En contra:* superficie de interfaz nueva que hay que diseñar bien o se
  vuelve ruido que nadie mira.

---

## 3 · Conexión con segundos cerebros

- [ ] **Wikilinks `[[nota]]` de Obsidian** 🟡 ✳️
  *A favor:* **es el mayor diferenciador barato del proyecto.** Ninguno de los
  diez competidores estudiados los entiende, y es solo resolución de nombres
  contra una carpeta. Abre la puerta al público entero de Obsidian.
  *En contra:* hay que indexar la bóveda; en una de miles de notas el índice
  tiene que ser incremental o el arranque se muere.

- [ ] **Enlaces rotos marcados visualmente** 🟢 ✳️
  *A favor:* en Obsidian un enlace roto es trabajo pendiente, no un error.
  Mostrarlo distinto es información útil.

- [ ] **Backlinks: qué notas enlazan a esta** 🟡 ✳️
  *A favor:* la mitad del valor del grafo de Obsidian con una fracción del
  trabajo — es invertir el índice de wikilinks que ya existe.

- [ ] **Callouts de Obsidian (`> [!info]`)** 🟢 ✳️
  *A favor:* casi idénticos a las alertas de GitHub que la v1 ya renderiza.
  Reutiliza el mismo mecanismo.

- [ ] **Repo de GitHub clonado: enlaces relativos y raíz del repo** 🟡 ✳️
  *A favor:* un `[guía](../docs/guia.md)` navega correcto, como en github.com.
  *En contra:* detectar la raíz (`.git/`) y resolver rutas absolutas del repo
  tiene casos límite con submódulos y worktrees.

- [ ] **README automático al abrir una carpeta** 🟢 ✳️

- [ ] **Embeds `![[nota]]`** 🔴 ✳️
  *A favor:* completa el modelo mental de Obsidian.
  *En contra:* transcluir contenido abre preguntas de ciclos (A embebe B que
  embebe A) y de profundidad. Empezar como enlace destacado.

- [ ] **Abrir desde una URL de GitHub** 🟡 ⚠️
  *A favor:* comodidad real.
  *En contra:* **introduce red y descarga automática de contenido no
  confiable**, justo lo que la política evita. Solo como opt-in explícito, con
  el mismo tratamiento hostil que todo lo demás. Descartado por ahora.

---

## 4 · Workspace — lo que separa un visor de una herramienta

- [ ] **Abrir una carpeta como espacio de trabajo** 🟡 ✳️
  *A favor:* es la diferencia entre "abre un archivo" y "es donde trabajo".

- [ ] **Barra lateral con árbol de archivos** 🟡 ✳️

- [ ] **Recientes y favoritos persistentes** 🟢 ✳️
  *A favor:* la v1 tiene recientes pero pierde el resto al cerrar.

- [ ] **Búsqueda en toda la carpeta** 🔴 ✳️
  *A favor:* casi obligatoria en un workspace real.
  *En contra:* buscar en miles de archivos exige un índice o aceptar que la
  primera búsqueda sea lenta. Un índice mal invalidado devuelve resultados
  fantasma, que es peor que no tener búsqueda.

- [ ] **Sesión restaurada al reabrir** 🟢 ✳️

- [ ] **Índice lateral desde los encabezados** 🟢 ✳️

- [ ] **Búsqueda y reemplazo en el documento** 🟡 ✳️

---

## 5 · Estudio y notas — el terreno diferencial

- [ ] **Modo foco / estudio** 🟢 ✳️
  Oculta todo el chrome, deja la nota sola, con temporizador opcional.
  *A favor:* barato y muy pedido por estudiantes. Alto valor por poco trabajo.

- [ ] **Repaso espaciado desde el propio documento** 🔴 ✳️
  Marcar pares pregunta/respuesta con una sintaxis mínima y repasarlos sin
  salir del lector.
  *A favor:* **es el hueco real del mercado** — entre Anki (tarjetas sin
  contexto) y Obsidian (contexto, pero el repaso es plugin de terceros). Nadie
  liviano lo ocupa.
  *En contra:* exige llevar estado de repaso por ítem, fuera del `.md` para no
  ensuciarlo. Ese archivo paralelo hay que sincronizarlo si la nota cambia.

- [ ] **Resaltado persistente** 🟡 ✳️
  *A favor:* subrayar al leer es el gesto básico de estudiar.
  *En contra:* ¿se guarda en el `.md` (lo ensucia, pero viaja con el archivo) o
  aparte (limpio, pero se pierde si movés la nota)? Decisión de producto real,
  no técnica. Propuesta: aparte por defecto, con opción de incrustar.

- [ ] **Exportar a PDF con márgenes para anotar a mano** 🟢 ✳️

- [ ] **Estadísticas de lectura** 🟡 ✳️
  Tiempo estimado, palabras, densidad de código.
  *En contra:* roza el "data slop" — números que se ven bien pero nadie usa.
  Solo el tiempo estimado de lectura justifica el espacio.

- [ ] **Grafo de notas** ⬛ ⚠️
  *A favor:* la visualización estrella de Obsidian.
  *En contra:* caro de dibujar bien y de que rinda con cientos de nodos. Solo
  si la pila de dibujo demuestra que aguanta. Es la función que más fácilmente
  se convierte en un adorno bonito e inútil.

---

## 6 · Contenido enriquecido

- [ ] **Alertas de GitHub (`> [!NOTE]`)** 🟢 ✳️

- [ ] **Imágenes locales, remotas bloqueadas por defecto** 🟡 ✳️

- [ ] **Fuente visible de Mermaid en bloque con estilo** 🟢 ✳️
  *A favor:* honesto y útil — el texto Mermaid es legible. Desbloquea la v2.0
  sin pagar el coste del render.

- [ ] **Render nativo de Mermaid** ⬛ 🧩
  *A favor:* es lo que Tinta demuestra posible, con 22 tipos de diagrama.
  *En contra:* **el ítem más caro de todo el proyecto**, más que el parser —
  cada tipo trae su propio algoritmo de layout. Componente opcional posterior,
  y aun así arrancando por flowchart y secuencia, que son los que se usan.

- [ ] **Render nativo de matemática (KaTeX)** 🔴 🧩
  *En contra:* igual de pesado y sin una librería madura reutilizable clara.
  Mismo trato que Mermaid.

- [ ] **Markmap: mapa mental desde los encabezados** 🟡 🧩
  *A favor:* viable 100% local y encaja con el caso de estudio.

- [ ] **Frontmatter YAML oculto como en GitHub** 🟢 ✳️

- [ ] **Tipografía y tamaño ajustables** 🟢 ✳️

---

## 7 · Edición

- [ ] **Editor de texto plano con barra de ayudas** 🟡 ✳️

- [ ] **Listas automáticas, indentado, pegar URL sobre selección** 🟢 ✳️

- [ ] **Renombrar un encabezado actualiza los enlaces que apuntan a él** 🔴 ✳️
  *A favor:* es lo que JetBrains cobra en su plugin. Convierte "editor de
  texto" en "editor de documentos".
  *En contra:* hay que reescribir otros archivos del workspace. Un fallo acá
  corrompe notas ajenas — exige deshacer transaccional y confirmación.

- [ ] **Pegar imagen del portapapeles: la guarda y arma el enlace** 🟡 ✳️
  *A favor:* el gesto más pedido al tomar notas de estudio.
  *En contra:* decidir dónde la guarda (¿`assets/`? ¿junto a la nota?) sin
  ensuciar la bóveda del usuario.

- [ ] **Autocompletado de enlaces a otras notas** 🟡 ✳️
  *A favor:* con el índice de wikilinks ya hecho, es casi gratis.

- [ ] **Guardado atómico con codificación y fin de línea preservados** 🟡 ✳️
  *A favor:* un archivo ajeno no debe cambiar de codificación por abrirlo. Ya
  resuelto en la v1; es portar, no inventar.

- [ ] **Detección de cambios externos al archivo** 🟢 ✳️

---

## 8 · Rendimiento

- [ ] **Arranque por debajo de 1 s hasta el primer pintado** 🔴 ✳️
  *A favor:* es la crítica que Tinta le hace a todo lo demás y la razón de ser
  del enfoque nativo.
  *En contra:* exige medir de verdad en la Fase 0, no asumir.

- [ ] **Binario por debajo de 7 MB** 🔴 ✳️
  *En contra:* el presupuesto estimado da entre 2,9 y 7,6 MB. **El techo se
  pasa.** Las palancas están en `budget.md`; puede exigir renunciar a algo.

- [ ] **Parseo fuera del hilo de interfaz** 🟡 ✳️
  *A favor:* un `.md` de 20 MB no debe trabar la ventana. Moji lo hace, la v1
  no.

- [ ] **Virtualización del renderizado en documentos gigantes** 🔴 ✳️
  *A favor:* dibujar solo lo visible es lo que permite abrir archivos enormes.
  *En contra:* rompe la búsqueda en página y el scroll proporcional si no se
  hace con cuidado.

---

## 9 · Detalles de interfaz

- [ ] **Barra de título propia con pestañas integradas** 🟡 ✳️
  *En contra:* quitar el marco nativo se lleva bordes de redimensionado, Aero
  Snap y el maximizado correcto. Cada pieza hay que reponerla a mano — la v1
  ya documentó ese camino completo.

- [ ] **Menú contextual según dónde se hace clic derecho** 🟢 ✳️

- [ ] **Conservar el punto de lectura al cambiar de modo** 🟢 ✳️

- [ ] **Pestañas sin guardar nombradas por su contenido** 🟢 ✳️

- [ ] **Pantalla completa sin bordes (F11)** 🟢 ✳️

- [ ] **Arrastrar pestañas entre ventanas** 🔴 ✳️

- [ ] **Sistema de movimiento acotado** 🟢 ✳️
  Duraciones de 120 a 240 ms, solo `transform` y `opacity`, respeto a
  `prefers-reduced-motion`. El detalle está en el lienzo de diseño.

---

## 10 · Distribución y mantenimiento

- [ ] **Firma vía Microsoft Store para evitar SmartScreen** 🟡 ✳️
  *A favor:* es como Tinta evita la advertencia sin pagar un certificado.
  *En contra:* ata una parte de la distribución a la Store y su revisión.

- [ ] **Instalación y desinstalación limpias, sin permisos de administrador** 🟡 ✳️

- [ ] **Release verificable por hash** 🟢 ✳️

- [ ] **Multiplataforma (macOS, Linux)** ⬛ ✳️
  *A favor:* Rust y la pila de dibujo son portables; el stack se elige para no
  impedirlo.
  *En contra:* triplica la superficie de prueba. Fuera de la v2.0, pero la
  puerta queda abierta — es una diferencia real con la v1 y con Tinta, ambas
  atadas a Windows.

---

## 11 · IA local — opcional y aparte

- [ ] **Hablar con un Ollama que el usuario ya tenga** 🟡 🧩
  *A favor:* **la vía más perezosa y la mejor**: cero peso agregado, cero
  modelo que mantener, y respeta el principio de que nada salga del equipo.
  *En contra:* depende de que el usuario ya lo tenga instalado.

- [ ] **Resumir una nota larga** 🟡 🧩
- [ ] **Generar tarjetas de repaso desde el contenido** 🔴 🧩
- [ ] **Responder preguntas sobre las propias notas** 🔴 🧩
- [ ] **Sugerir enlaces entre notas relacionadas** 🔴 🧩

  *Regla común, no negociable:* corre **100% local**, es **opt-in**, y es un
  **componente de descarga separada** que no cuenta contra los 7 MB. Un
  segundo cerebro que manda tus notas a un servidor ajeno para resumirlas es
  exactamente lo que este proyecto existe para no ser.

- [ ] **Embeber un runtime de inferencia propio** ⬛ 🧩
  *En contra:* cientos de MB y un modelo que mantener. Solo si hablar con
  Ollama demuestra no alcanzar.

---

## Fuera de alcance, con motivo

| Función | Por qué no |
| --- | --- |
| Colaboración en vivo / compartir por enlace | Rompe la política de red. No es el problema que la app resuelve |
| Plugins de terceros en tiempo de ejecución | Multiplica la superficie de confianza justo donde se la quiere angosta |
| Sincronización en la nube | No es un servicio; los archivos son del usuario y viven en su disco |
| Motores de diagramas remotos (PlantUML, Kroki) | Mandan la fuente del diagrama a un servicio externo. Es el defecto concreto de ThisIs-Developer que la v2 no repite |
| Ser un reemplazo de Obsidian | La v2 es la ventana rápida sobre la bóveda, no un segundo dueño de ella |

---

## Las cinco decisiones que ordenan todo lo demás

1. **¿Se llega a 7 MB?** Lo responde el prototipo de la Fase 0. Si no, hay que
   recortar alcance o mover el presupuesto — antes de construir nada grande.
2. **¿Wikilinks en la v2.0?** Es el mayor diferenciador por unidad de esfuerzo
   de toda la lista. Mi recomendación: sí, y temprano.
3. **¿Repaso espaciado, o solo lector?** Define si el producto compite con
   readers o con apps de estudio. Son dos públicos y dos alcances.
4. **¿Mermaid nativo alguna vez?** Es el ítem más caro. Mientras no se
   resuelva, la fuente visible es una respuesta honesta.
5. **¿Multiplataforma?** No cambia la v2.0, pero sí cómo se eligen las
   dependencias desde el primer día.
