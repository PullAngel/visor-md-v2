# Evaluación inicial de exportación

Fecha de consulta: 9 de septiembre de 2026. Esta nota no aprueba una
dependencia ni cambia el producto: conserva evidencia para la Etapa 10 del
roadmap.

## Restricción de partida

La exportación recibe el modelo Markdown ya validado y una ruta de salida
elegida explícitamente por la persona. No reabre rutas declaradas por el
documento, no usa HTML arbitrario, no carga imágenes remotas y nunca cambia la
fuente abierta. Un fallo deja intacto el documento y no debe reemplazar una
salida existente sin una operación atómica verificable.

La fidelidad de PDF no justifica convertir Visor MD en un navegador: quedan
fuera WebView, HTML ejecutable, CSS de terceros y recursos secundarios
automáticos.

## Candidatos revisados

### PDF

- [`printpdf` 0.12](https://docs.rs/printpdf/latest/printpdf/) ofrece operaciones
  de página, texto, formas, fuentes y serialización. Es un candidato para una
  capa que traduzca exclusivamente el modelo interno a operaciones PDF.
- La misma API también expone conversión desde HTML. Esa parte queda prohibida:
  permitirla reintroduciría una segunda interpretación de contenido no fiable y
  contradice la arquitectura nativa.
- [`genpdf` 0.2](https://docs.rs/genpdf/latest/genpdf/) se descarta por ahora:
  su propia documentación informa que incorpora familias de fuentes completas,
  con un coste aproximado de 100 a 200 KiB por fuente o 500 a 1.000 KiB por
  familia. Ese comportamiento choca con el presupuesto de Visor MD.

### DOCX

- [`docx-rs` 0.4](https://github.com/bokuweb/docx-rs) ofrece escritura OOXML de
  párrafos, runs, tablas, listas, estilos, enlaces y notas. Su licencia
  declarada es MIT y su API puede escribir directamente a un `File`.
- No se aceptará por esa descripción solamente. Antes de incorporarlo hay que
  fijar la versión, revisar su lockfile transitivo, licencia de cada componente,
  advisories, funciones por defecto, soporte Windows/Linux y el delta exacto de
  tamaño release. La exportación inicial solo necesita escritura; cualquier
  capacidad de lectura o previsualización que amplíe superficie debe quedar
  deshabilitada o fuera del binario.

## Alternativas y recomendación provisional

Un único ejecutable con PDF y DOCX es cómodo, pero puede convertir una función
ocasional en coste permanente de arranque, tamaño y superficie. La arquitectura
ya permite que un exportador pesado sea un componente o binario separado.

La recomendación provisional es evaluar primero dos builds reproducibles:

1. exportador PDF aislado que recibe el modelo seguro, sin HTML ni imágenes;
2. exportador DOCX de solo escritura, también aislado.

Se compararán contra el núcleo actual. Si la variante integrada queda bajo 8 MB
y no empeora arranque ni la cadena de suministro, podrá ser más simple para la
persona usuaria. Si no, se propondrá un componente opcional distribuido junto a
la aplicación, con un protocolo local limitado y sin red.

Esta elección cambia distribución, arquitectura y dependencias. Requiere
aprobación explícita antes de modificar `Cargo.toml`.

## Gate previo a implementar

Para cada candidato aprobado:

1. fijar versión y features mínimas en una rama de experimento;
2. revisar licencia, advisories, árbol transitivo y código nativo por target;
3. construir release Windows y Linux, medir bytes, arranque y memoria;
4. exportar fixtures Unicode, tablas, listas, enlaces y contenido hostil;
5. comprobar que ninguna entrada activa HTML, abre rutas secundarias ni altera
   el Markdown;
6. documentar la decisión en un ADR y mantener los artefactos generados fuera
   de las bóvedas de prueba.

Hasta completar el gate, PDF y DOCX siguen planificados. La copia preparada
para Discord o correo ya existe como operación local explícita y no depende de
estos componentes.
