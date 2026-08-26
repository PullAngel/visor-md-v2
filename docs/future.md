# Futuro

Este documento contiene ideas válidas fuera del plan activo. No son promesas de
v2.0. Una idea entra al roadmap solo con caso de uso, coste, threat model, diseño
y criterio de salida.

## Principios

- El núcleo permanece pequeño y offline.
- Nada ejecuta contenido de documentos.
- Lo pesado se aísla o se ofrece como componente opcional.
- No se agrega una función solo para igualar a un competidor.
- Cada release debe ser un punto de parada útil.

## Candidatos

### Edición en vivo

Editar directamente sobre el documento renderizado. Requiere selección, cursor,
IME, actualizaciones incrementales y round-trip ya resueltos. Se evalúa después
del editor fuente y la vista dividida.

### Mermaid nativo

Diagramas sin JavaScript ni servicio remoto. Empezaría por familias de alto valor
como flowchart y secuencia. Debe medir tamaño, complejidad y corpus adversarial.

### Matemática

Componente opcional sin motor web. La compatibilidad y la accesibilidad importan
más que reconocer muchas fórmulas parcialmente.

### Corrector ortográfico

Diccionarios descargables por idioma para no inflar binario y memoria de quienes
no lo usan. Debe funcionar offline y no enviar texto.

### Grafo de notas

Visualización secundaria. Solo entra si usuarios reales demuestran que mejora la
navegación más que backlinks y búsqueda.

### Referencias de bloque creadas por Visor MD

Requieren identidad estable de fragmentos y una política compatible con Obsidian.
Navegar referencias existentes puede llegar antes.

### Plugins declarativos

Extensiones de tema, sintaxis o comandos descritas como datos. No se acepta
código arbitrario dentro del proceso. Cualquier sistema de plugins necesita un
threat model independiente.

### macOS

La arquitectura evita dependencias innecesarias de Windows, pero macOS no bloquea
v2.0. Se evalúa cuando Windows y Linux tengan releases mantenibles.

### Espacio cifrado

Una bóveda cifrada abre preguntas sobre índice, nombres, sidecars, backups,
recuperación y contraseña olvidada. No se diseña hasta poder ofrecer una solución
criptográfica completa y auditable.

### Actualizaciones firmadas

Opcionales, visibles y sin telemetría. Requieren firma, protección contra
rollback, canal de publicación y recuperación segura.

## Ideas descartadas actualmente

- IA propia o chatbot;
- motores remotos de diagramas;
- ejecución de HTML o JavaScript;
- colaboración online en tiempo real;
- outliner como modelo de escritura principal;
- plugins con código arbitrario;
- autoguardado silencioso como única protección.

## Riesgo de sostenibilidad

El proyecto tiene un mantenedor principal. La mejor defensa contra abandono es un
alcance disciplinado, automatización y etapas que produzcan software útil. Una
función futura no puede volver frágil lo que ya funciona.
