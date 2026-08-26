# Glosario de ciberseguridad y QA

Definiciones breves vinculadas a Visor MD. Los documentos técnicos deben seguir
explicando cada concepto en contexto.

## Ciberseguridad

### Activo

Algo que vale proteger: documentos, privacidad, disponibilidad o confianza.

### Allowlist

Lista cerrada de lo permitido. Visor MD solo interpreta los nodos HTML que conoce
y trata el resto como texto.

### Attack surface

Superficie de ataque. Conjunto de entradas y componentes que un atacante puede
intentar abusar.

### Capability

Capacidad o permiso concreto, como leer archivos o usar red. Un renderer que solo
dibuja necesita menos capacidades que un navegador.

### Defensa en profundidad

Varias barreras independientes. Una imagen necesita control de ruta, bytes,
formato, dimensiones y memoria.

### Denegación de servicio

Entrada que agota CPU, memoria, pila o disco para volver inutilizable la app.

### Path traversal

Uso de rutas como `../` para escapar de una carpeta permitida.

### Phishing

Engaño que presenta un enlace o acción como algo distinto de su destino real.

### Riesgo residual

Riesgo que queda después de aplicar controles y que se acepta o sigue vigilando.

### Sandbox

Entorno con permisos limitados. Reduce el daño posible si un componente falla.

### SBOM

Software Bill of Materials. Inventario de bibliotecas y componentes incluidos en
una build.

### SSRF

Server-Side Request Forgery. En una app de escritorio aparece cuando contenido
hostil logra que un componente de red consulte routers o servicios internos.

### Supply chain

Cadena de suministro de crates, fuentes, herramientas y sistemas usados para
construir el producto.

### Threat model

Modelo que conecta activos, atacantes, entradas, amenazas, controles y riesgo
residual.

### TOCTOU

Time of check to time of use. Algo se valida y cambia antes de ser usado, por
ejemplo un symlink reemplazado entre comprobación y lectura.

### VFS

Capa única que aplica política a los accesos de archivos. No es otro disco, sino
una frontera controlada.

## QA

### Criterio de aceptación

Condición observable que debe cumplirse para considerar terminada una función.

### End to end

Prueba de un recorrido completo desde la interacción del usuario hasta el
resultado, como abrir, editar, guardar y volver a abrir.

### Fault injection

Provocar fallos controlados, como simular un error de disco durante guardado, para
comprobar recuperación.

### Fuzzing

Generación automatizada de muchas entradas extrañas para descubrir crashes,
bloqueos y estados inesperados.

### Gate

Comprobación que debe pasar antes de integrar o publicar: tests, auditoría,
benchmark o revisión manual.

### Matriz de pruebas

Tabla que relaciona requisitos y riesgos con plataformas, pruebas y evidencia.

### Property testing

Prueba muchas entradas contra una propiedad general. Ejemplo: guardar sin editar
debe preservar los bytes.

### Regresión

Comportamiento que funcionaba y deja de funcionar después de un cambio.

### Smoke test

Prueba rápida que comprueba que los recorridos básicos arrancan. No reemplaza una
suite profunda.

### Test oracle

Fuente que dice cuál es el resultado correcto, como el corpus oficial CommonMark
o un archivo esperado revisado.
