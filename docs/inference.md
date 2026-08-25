# Inferencia: IA local para estudio

**Estado: exploración. No es parte de la v2.0**, pero el camino barato
(hablar con un Ollama existente) es el punto 3 del futuro, no un "algún día" (ver `product.md`). Este
documento fija los principios para el día que se sume, no un compromiso de
sumarla ya.

## Qué problema resolvería

El caso "tomar notas para y durante el estudio" tiene funciones donde la IA
aporta de verdad: resumir una nota larga, generar tarjetas de repaso desde el
contenido, responder preguntas sobre lo que uno ya escribió, sugerir enlaces
entre notas relacionadas. Son exactamente las funciones por las que apps como
RemNote o los plugins de IA de Obsidian tienen demanda.

## El principio que no se negocia

Si Visor MD v2 suma IA, corre **100% local**. Ni una palabra de las notas sale
del equipo. Esto no es una preferencia: es coherencia con toda la tesis del
proyecto. Un segundo cerebro que manda tus notas —muchas veces lo más privado
que uno escribe— a un servidor ajeno para "resumirlas" es precisamente lo que
este proyecto existe para no ser. Una app que se vende como "seguro por
construcción" y después filtra el contenido a una API de terceros sería una
contradicción que la haría inservible para su público.

## Cómo encaja sin romper el presupuesto de 7 MB

La IA local **no va en el núcleo**. Un modelo pequeño de lenguaje pesa cientos
de MB o más —imposible dentro de los 7 MB—. Por eso:

- Es un **componente opcional, de descarga separada y explícita**. Quien no lo
  quiere, no lo baja, y su Visor MD v2 sigue pesando <7 MB.
- El núcleo habla con ese componente por un contrato local (un proceso aparte o
  una librería cargada bajo demanda), no lo empaqueta.
- **Alternativa más perezosa a evaluar primero:** en vez de embeber un runtime
  de inferencia, detectar si el usuario ya tiene uno corriendo localmente
  (por ejemplo Ollama, que mucha gente técnica ya usa) y hablarle a ese, por
  loopback. Cero peso agregado, cero modelo que mantener, y respeta el
  principio local. Solo si eso no alcanza se evalúa embeber un runtime propio.

## Qué se evalúa cuando llegue el momento

- Runtime de inferencia liviano en Rust (candidatos del ecosistema `candle` o
  `llama.cpp` vía binding) contra "usar el Ollama del usuario".
- Modelos pequeños suficientes para resumir y generar tarjetas (no hace falta
  un modelo gigante para eso).
- Que toda la interacción sea opt-in por documento: la IA no toca una nota
  hasta que el usuario se lo pide en esa nota.

## Qué NO haría, nunca

- Mandar notas a una API remota, ni siquiera "solo para esta función".
- Correr en segundo plano indexando todo sin permiso.
- Ser obligatoria para usar el resto de la app.

La IA es una herramienta opcional sobre un segundo cerebro privado, no la razón
de ser del producto. Si genera dudas de privacidad, no entra.
