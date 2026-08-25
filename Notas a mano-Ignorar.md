# Diseño:
Color: Opción B, "Papel + tinta"

1. Estilo de ventana: "sin borde"
2. sistema de iconos: "A - Suave"
3.  Tipografía: "Contraste editorial". A menos que por alguna razón cargar tres familias sea pesado para el programa, pero si la diferencia es infima, elijo "neutro suizo" como opción B.
4. Animación. Entiendo que no me estás dando opciones acá. Pero me gustan esas animaciones. Si más adelante al tener todo armado alguna no me gusta, lo veremos.
5. Profundidad: Plano. Y para lertas, pop-ups y cosas que literalmente estén por encima, "Elevación", pero con una sombra ligeramente más suave.

## Arquitectura:


## Visión
Además de lo que ya mencionas, me gustaría que tuviera una gran utilidad en el uso diario, por ejemplo para un estudiante. Y opciones como subrayar parecen muy interesantes, pero no sé cómo agregarlo sin dañar el formato de .md y como lo interpreta otro visor, como el de obsidian, que podría usar ambos dependiendo del contexto. Pensemos en ideas que le den esa forma al programa además de lo que ya menciona el doc.
Además de ligero, me gusta la portabilidad, por más que no sea literal un solo archivo, poder mantenerlo con opción de no instalar me gusta. Y también poder hacerlo en VM descartables de linux; así que quiero que no solo corra en windows 10/11, si no también en las versiones más estandares de linux, sin rompernos mucho la cabeza tampoco.

---
## Producto
Ok. Siguelo trabajando y actualizando.

## arquitectura
Ok. dejo que con el criterio ya hablado decidas lo mejor para nuestro objetivo.
sigue manteniendolo.Dale ua revisión profunda de seguridad, quiero que sea robusto desde los cimientos. Investiga vulnerabilidades de arquitecturas y tecnologías similares, y además de entenderlas, logra que nuestro programa no sea vulnerable a ellas y toma buenas deciciones. 

---
## Calculos.

Ok, te dejo el criterio a ti.
Intentemos que el objetivo sea <7mb. Idealmente <6. Pero el límite real será 9.44 mb. Trabaja como si el límite sea 7, pero si hay razones importantes para superarlo,se puede superar.
Además del peso, el arranque optimizado también es imuy importante. Que sea lo más rápido posible manteniendo los otros estandares.

---
## Auditoría y modelo de amenaza
Okk, siguelo trabajando y actualizando. Mantenme al tanto si surgen nuevos riesgos. 

---
## Conectividad
Ok. Dale un poco más de info e investiga mejor de las mejores prácticas.
Además de obsidian, que se pueda trabajar con otros segundos cerebros de forma mfácil, si se puede.
Me gustaría, de forma segura, darle una puerta de acceso a ia local de alguna forma. Pero no es muy importante, se puede agregar para planes futuros si agrega una capa de complejidad y vectores de ataque innecesarios. 

---
## Inferencia: IA local para estudio
Ok, me parece bien. Lo que se pueda agregar en la primera versión que hagamos, genial, si algo se pone complicado, lo dejamos para después.

---
## Brainstorm: 

superar a los readers actuales, apoyado en apps de estudio
No sé exactamente a qué te refieres con "el grafo como visualización opcional", con "Estructura de outliner" tampoco, no usé Logseq.
### Sobre ThisIs-Developer/Markdown-Viewer:
tiene muchas opciones y supongo que eso lo hace potente, pero a la vez se ve muy sobrecargado. copiemos las cosas más importantes y demosle una mejor identidad y ui/ux.
Revisa su barra de herramientas para el formato. 

Yo en una revisada rápida veo una pestaña deplegable de headers en vez de tener "H1 H2 H3" PERMANENTES, ES UNA SOLAPA EN LA QUE PUEDES ELEGIR DE H1 A H6.
tAMBIÉN VEO QUE TIENE LOS BOTONES CON SIMBOLOS EN VEZ DEL NOMBRE, Y SI TEPOSAS ENCIMA, SÍ DICE SU FUNCIÓN. Un menú de "Símbolos y entidades HTML" Tomalos. Un menú de emojis, si no suma mucho al peso, tómalo. 
Revisa el resto.  
### Ideas de estudio, ordenadas por relación valor/costo

-"Repaso desde el documento", no entiendo muy bien a qué te refieres y como se implementaía, explicamelo.  Ni sí ni no hasta no tenerlo claro.
-Modo estudio que oculta todo, no me encanta. Descatalo a medias. Pero quedemosnos con el modo pomodoro si es fácil de integrar. Esto tendría que estar para habilitar en "más opciones" y sería una interfaz muy minimalista y simple en la parte de arriba o algún lugar bien adecuado y que no moleste. al darle click la personalizas (tiempos, etc).
-Backlinks, no lo entiendo del todo, explicamelo. Pero igalmente, si es de bajo costo: aprobado.
-Grafo de notas. No sé muy bien qué es, explicamelo. Si es el modo de diagrama que conecta archivos visualmente como en obsidian no me interesa mucho para esta versión, se puede documentar y agregar en un futuro, tal vez.
-Generación de tarjetas con IA local. Explicame bien eso de tarjetas y todo, como se usaría. Por ahora, a menos que lo aclare,será para futuras versiones.
-Referencia a nivel de bloque. Lo mismo que en el item anterior. 
-Resaltado persistente. No entiendo bien como se implementa eso que dices "sin tocar el .md". Quedaría en el caché de la app? Me gusta la idea de poer resaltar desde el modo lectura, pero tiene que ser coherente el formato.
-tabla de contenido flotante. Ok. Que se pueda activvar y desactivar desde configuración avanzada. 
-Exportar una nota a PDF con estilo de estudio. Ok. El exportar a pdf es algo que me gustaría mejorar de la versión 1, donde había que pasar por "imprimir".
-Búsqueda en toda la bóveda. Ok. 
---
### Catálogo de funciones — Visor MD v2
si no hago comentario alguno, es porque está ok, confio en tu criterio:
-Render de CommonMark + GFM completo: Me parece muy importante que el render se vea bien y completo, con sus limites de seguridad obvio. Hay que dedicarle esfuerzo en que salga bien y no desestabilice el amaño ni genere fallos.
-Modo lectura y modo edición, con vista dividida : Lo menciono en otra parte, me gusta que en obsidian puedes hacer las 2 cosas, escribir sobre el archivo ya renderizado y también dividir para trabajar en los 2 modos a la vez. Me gustaría que revises la posibilidad de tener entonces Modo Lectura (con sus opciones de subrayado y todo eso), modo edición/fuente, modo vista dividida y modo edición pero renderizado automatico (dificl de hacer que funcione bien con solo vibecoding, pero si es posible, sería bueno. Quizás agregarlo a las cosas futuras con mayor urgencia).
-Pestañas y ventanas al estilo navegador: ok, habrá que trabajar en hacerlo de nuevo y que quede bien y prolijo, con sus animaciones de diseño. 
Hubo una nota arriba relacionada a esto. la duplico: "También veo que tiene la opción de "dividir a la derecha" y "dividir abajo". Me gusta esa forma de abrir pestañas, pero no me gusta que la que se abra sea la misma, un duplicado, no le encuentro uso. Me gustaría poder aplicar ese tipo de ventanas y de abrirlas tan fácil, pero que abra una nueva pestña y dentro de ella tenga la opcion de "crear un nuevo archivo", "abrir un archivo" o cerrar"
-seguridad: después de definir la arquitectura y otros sectoes, dale una buena revisada a todo esto, piensa como atacante, como defensor, y blinda esta parte, es una parte fundamental, así que o nos quedemos solo con los trabajos que hicimos en la v1 o la tranquilidad de que no es necesario sanitizar. Si algo termina en una decisión de diseño y ver si se sacrifica algo o se agrea algo que no estaba pensado, hazmelo saber para ver qué decisión tomamos. Además documenta todo en una sección a parte, para que al momento de comunicar hallazgos y el visor listo en general, nos sea más facil habalr con criterio.
-wikilinks. Ok. Cuidado con la seguridad y el rendimiento acá.
- Sesión restaurada al reabrir. Que no se autoguarden los archivos de forma predeterminada (se puede cambiar desde opciones avanzadas) pero sí que vaya creando un save termporal por si se cierra de golpe o algún fallo, se pueda retomar, pero que eso no signifique que autoguarde las modificaciones en el archivo original.
- Modo foco / estudio : solo si es muy muy fácil y ligero de implementar, bien abajo en "mas opciones"
- Repaso espaciado desde el propio documento. si lo sabes implementar sin que cree problemas de memoria/peso, ok.
-Resaltado persistente: Propuesta: "aparte por defecto, con opción de incrustar": ok.
Estadísticas de lectura: lo había comentado abajo antes de leer esto. Podrían "esconderse" en la parte inferior de la pestaña índice, así no ensucia la vista estandard.
-Fuente visible de Mermaid en bloque con estilo : la parte de mermaid me gustaría que pueda estar bien. me parece importante una buena capacidad de mostrar los diagramas. Es de las pocas cosas por las que amentaría un poquito el limite de tamaño, si no se puede resolver de otra forma.
-Render nativo de matemática (KaTeX) . en contra. quizás en un futuro se pueda agregar plugins descargables y personalizables y acá entre esto, pero por ahora no.
 Renombrar un encabezado actualiza los enlaces que apuntan a él. Revisar. si choca contra las opciones de obsidian, buscar otra manera o postergarlo.
- Firma vía Microsoft Store para evitar SmartScreen: explicame los requisitos para aparecer en Microsoft store y otras stores gratis usadas.
- - Multiplataforma (macOS, Linux): Linux 100% ok. MacOS no es una urgencia, pero compara el esfuerzo entre no trabajar en el y despues tener que trabajar extra para hacerlo vs el esfuerzo de las pruebas que puedes hacer a la par. Si no es mucho lo que cuesta que lo vayas probando, hagamoslo. De ultima lo trabajamos para mc pero no lo publicitamos, y que una versión bien probada sea algo para después. Decidelo tú. Windows y linux sí.
- IA local — opcional y aparte: aplica lo que sea de costo bajo y medio. Lo cosotoso, doscumentalo y que quede para una versión futura. 
- Sincronización en la nube: no como un servicio, pero si se puede guardar directamente en un drive, es algo para revisar y aprobar después de pasar filtros de seguridad. 

---
## Roadmap.
ok. Confio en vos. Pero al terminar de revisar todo este doc, dale una buena actualización al roadmap. Es un archivo muy importante ya que muchas veces te iré "Ok, avanza con el roadmap" sin tener mucha seguridad de qué estás haciendo, así que organizalo bien. haz pruebas entre sprints para mantener una calidad excelente y evitar problemas en momentos más avanzados del proyecto. Si necesitas que haga pruebas manuales, las hago. 

---
## Futuro.
Ok. Actualizalo con las cosas que mencionamos. 

---

## Comentarios extras:
Probando Obsidian, veo que el formato se aplica automaticamente. Si hago un ##me lo coloca como header2 y primero veo un preview aún con los "## " y al salir de esa línea desaparecen y queda ése formato. Lo veo con una curva de aprendizaje más dura, pero con un resultado más limpio.
Y justo usando ese formato ##, veo que lo que queda entre headers así, lo puedo minimizar. Eso también me parece interesante. Lo mismo pasa con h1 y h3, Al parecer este modo lo puedo cambiar a donde se ven todos los caracteres del formato, pasandolo a "modo fuente".

También veo que tiene la opción de "dividir a la derecha" y "dividir abajo". Me gusta esa forma de abrir pestañas, pero no me gusta que la que se abra sea la misma, un duplicado, no le encuentro uso. Me gustaría poder aplicar ese tipo de ventanas y de abrirlas tan fácil, pero que abra una nueva pestña y dentro de ella tenga la opcion de "crear un nuevo archivo", "abrir un archivo" o cerrar

Al darle click derecho en la solapa de una pestaña tengo la opción de fijar. Sirve para que al abrir el visor siempre e abra ese archivo fijado también. No me encanta, pero puede ser util en algunos contextos y además nadie e obliga a usarlo. Ponlo en funciones extras que entrarían si tenemos espacio de sobra en el tamaño.

Descargué, instalé y probé Tinta. Es veloz y ligero. Pero quizás demasiado minimalista para mi gusto. Siento que Visor md es más bonito y la versión v2 será mil veces mejor. 
Tinta tiene una función, botón, al lado de minimizar/cerrar y eso, que es para fijar. Lo que hace es que mantiene la ventana en esa posición, si interactuo con el navegador o lo que sea que esté atrás, este no se superpone a la ventana de Tinta, queda tinta ahí encima. Me gusta, va de la mano con las herramientas de estudio. Planifica bien esto para evitar bugs y haz casos de prueba fuera del happy path, cuando lo implementes.

Al instalar tinta me abre una ventana que pregunta que si quiero hacer Tinta mi default viewer for Markkdown and mermaid files. Ya algo así hace Visor MD al instalarlo. 

Agregar un contador de caracteres o palabras del documento, en la parte inferior derecha de la pestaña índice, solo si es de costo bajo.

---


## Bugs nuevos encontrados en Visor-Md
1. Encontré un pequeño bug en nuestro visor md. De las herramientas de formato, al usar "numerada" no funciona como esperaría. Si lo presiono solo, no inicia una numeración como sí lo hace con los bloques, tablas etc. Pero el mayor problema es que si selecciono algo dentro de un párrafo, agarra el párrafo entero. Eso me parecería bien si no selecciono nada y estoy sobre un párrafo ya escrito. Pero si tengo algo selecciado, esperaría que se le aplique a esa parte seleccionada. Arréglalo. Y mantén que seleccione el párrafo entero si ya estoy sobre un párrafo escrito. Pruebalo antes de cambiarlo para que entiendas exactamente a lo que me refiero. Además, agrega que si no estoy sobre ningún parráfo escrito si no uno nuevo, se inice la numeración ahí. 
2. Si escribo sobre un md con ya bastante contenido, desde el modo "Vista dividida", y escribo por ejemplo en el medio (con más párrafos abajo), mientras escribo, se empieza a subir la vista, hasta que llega a un punto donde no veo lo que estoy escribiendo.

              