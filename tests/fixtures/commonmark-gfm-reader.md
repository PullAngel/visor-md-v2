# Lector CommonMark y GFM

Subtítulo de referencia
-----------------------

Párrafo con **negrita**, _cursiva_, `código`, ~~tachado~~, \*asteriscos\*
y &amp; entidades. Una línea suave
continúa como espacio, pero esta termina con salto forzado.\
La siguiente empieza aparte.

Visita <https://example.com/ruta> o escribe contacto@example.com.

Una referencia a pie[^seguridad] permanece como texto nativo.

[^seguridad]: La definición no crea HTML ni navegación de navegador.

> Cita de primer nivel.
>
> > Cita anidada con **énfasis**.

3. Lista ordenada que parte de tres
4. Segundo elemento

- Viñeta exterior
  - Viñeta interior
- [ ] Tarea pendiente
- [x] Tarea realizada

---

```rust
fn main() {
    println!("hola");
}
```

| Campo | Valor |
| :--- | ---: |
| texto | 42 |

![Diagrama remoto](https://example.com/diagram.png "No se carga todavía")

Atajo <kbd>Ctrl</kbd> + <kbd>S</kbd>, <mark>concepto clave</mark>,
H<sub>2</sub>O y x<sup>2</sup>.

<script src="https://invalido.example/x.js">no se ejecuta</script>

## Casos trazables a CommonMark 0.31.2

Los fragmentos de esta sección se seleccionaron de los ejemplos 16 y 20 de
la especificación. Son entradas, no una copia del renderer HTML oficial: Visor
MD verifica el modelo nativo que realmente dibuja.

salto forzado\
otra línea

<https://example.com?find=\*>

## Unicode y fallback

Español: información y ciberseguridad. العربية: مرحبا بالعالم. हिन्दी: नमस्ते दुनिया.
日本語: 読書とメモ. 한국어: 읽기와 메모. Emoji: 🔒 📚 ✅.
