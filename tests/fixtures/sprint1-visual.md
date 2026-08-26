# Control visual del Sprint 1

Texto normal con **negrita**, _cursiva_, **negrita con _cursiva anidada_**,
`código inline`, ~~tachado~~ y un [enlace visible](https://example.com).

HTML semántico seguro: <kbd>Ctrl</kbd> + <kbd>S</kbd>, <mark>idea clave</mark>,
H<sub>2</sub>O y x<sup>2</sup>. El HTML con atributos sigue siendo visible como
fuente: <mark onclick="alert(1)">no se interpreta</mark>.

## Listas y tareas

- Primer elemento con una línea suficientemente larga para comprobar que el
  ajuste queda alineado debajo del texto y no debajo de la viñeta.
- Segundo elemento
  - Elemento anidado

3. Lista ordenada desde tres
4. Siguiente elemento

- [ ] Tarea pendiente
- [x] Tarea terminada

> Una cita con acentos: información, ñandú y ciberseguridad.
>
> > Una cita anidada.

---

```rust
fn main() {
    println!("Visor MD");
}
```

| Propiedad | Estado |
| --- | --- |
| Seguridad | Activa |
| Rendimiento | Medido |

HTML permitido todavía mostrado como fuente inerte: <kbd>Ctrl</kbd>.

HTML peligroso también visible e inerte: <script src="https://example.invalid/x.js">.

Emoji y Unicode general mediante fallback: 😀 🧠 🔐 日本語 العربية.
