# Rust Operating Systems

Repositorio del camino troncal de Jeresoft Academy para estudiar sistemas
operativos en Rust. Pertenece al Semestre 2 del plan de estudios junto con
`rust-networking` (RFC-0001 §10).

El objetivo no es fingir que vamos a escribir Linux en un fin de semana. El
objetivo es crear un recurso educativo completo: cada mecanismo debe explicar
por qué existe, qué problema resuelve, qué invariantes sostiene, qué límites
tiene, cómo falla, cómo se modela, cómo se prueba y cómo se mide.

## Qué Contiene

- Capítulos en Markdown compatibles con mdBook.
- Modelos Rust idiomáticos, un mecanismo por módulo.
- Ejemplos progresivos: básico, intermedio, avanzado y caso real.
- Tests unitarios, tests de integración y doctests.
- Benchmarks que confrontan el análisis teórico con mediciones.
- Diagramas Mermaid y recursos visuales.
- Ejercicios graduados con soluciones para niveles 1 a 3.

## Lugar En El Camino

Este curso vive en el Semestre 2. Recibe ideas de estructuras de datos,
algoritmos, Rust básico y redes; alimenta concurrencia, internals de bases de
datos, sistemas distribuidos, low-level programming, performance y seguridad.

Sistemas operativos es canónico aquí para mecanismos como procesos, hilos,
memoria, scheduling, paging, memoria virtual, señales, IPC y filesystem. Otros
cursos pueden reutilizar esos conceptos, pero no deben reexplicarlos desde cero.

## Capítulos

| # | Capítulo | Módulo | Estado |
|---|----------|--------|--------|
| 01 | Procesos e hilos | `src/processes.rs` | benchmarked |
| 02 | Mutex | `src/mutex.rs` | benchmarked |
| 03 | Semáforos | `src/semaphores.rs` | benchmarked |
| 04 | Memoria | `src/memory.rs` | benchmarked |
| 05 | Scheduling | `src/scheduling.rs` | benchmarked |
| 06 | Paging | `src/paging.rs` | benchmarked |
| 07 | Memoria virtual | `src/virtual_memory.rs` | benchmarked |
| 08 | Señales | `src/signals.rs` | planned |
| 09 | IPC | `src/ipc.rs` | planned |
| 10 | Filesystem | `src/filesystem.rs` | planned |

Estados posibles: `planned`, `draft`, `implemented`, `tested`,
`benchmarked`, `reviewed`, `published`.

## Estructura Esperada

```text
AGENTS.md
ROADMAP.md
LICENSE.md
LICENSE-MIT
LICENSE-APACHE
LICENSE-CC-BY-SA-4.0.md
docs/
  SUMMARY.md
src/
  lib.rs
examples/
  soluciones/
tests/
benches/
diagrams/
assets/
```

## Cómo Usarlo

Ejecutar tests:

```bash
cargo test
```

Formatear:

```bash
cargo fmt
```

Verificación completa:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
cargo test --doc
```

Ejecutar benchmarks:

```bash
cargo bench
```

## Gobernanza

- `AGENTS.md` es la guía de arranque para humanos e IA en este repositorio.
- `ROADMAP.md` registra el avance del curso sin convertirlo en una fecha límite.
- `docs/superpowers/plans/2026-07-16-rust-operating-systems-course.md`
  contiene el checklist de implementación inicial.
- `LICENSE.md` resume la doble licencia: código bajo `MIT OR Apache-2.0`;
  contenido educativo bajo `CC BY-SA 4.0`.

## Filosofía

Este repositorio debe poder leerse como un libro de ingeniería. La claridad
gana sobre el ingenio, la calidad gana sobre la velocidad, y ningún capítulo se
considera publicable hasta cumplir la anatomía completa de RFC-0001 §14.
