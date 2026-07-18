# ROADMAP

Estado de avance de `rust-operating-systems`, repositorio del camino troncal de
Jeresoft Academy para sistemas operativos en Rust.

No hay fechas límite: este es un proyecto de legado (RFC-0001 §1). Este archivo
orienta el avance, pero no convierte el curso en una carrera por terminar.

## Estado Actual

El repositorio ya tiene la fundación del curso y los siete primeros capítulos
desarrollados: procesos e hilos, mutex, semáforos, memoria, scheduling, paging
y memoria virtual. La siguiente línea natural es continuar con señales sin
perder la anatomía completa de RFC-0001 §14.

El checklist detallado vive en
[`docs/superpowers/plans/2026-07-16-rust-operating-systems-course.md`](docs/superpowers/plans/2026-07-16-rust-operating-systems-course.md).

## Capítulos Planeados

| # | Capítulo | Estado |
|---|----------|--------|
| 01 | Procesos e hilos | benchmarked |
| 02 | Mutex | benchmarked |
| 03 | Semáforos | benchmarked |
| 04 | Memoria | benchmarked |
| 05 | Scheduling | benchmarked |
| 06 | Paging | benchmarked |
| 07 | Memoria virtual | benchmarked |
| 08 | Señales | planned |
| 09 | IPC | planned |
| 10 | Filesystem | planned |

## Alineación RFC-0001

- Este repositorio sigue la plantilla de repositorio de RFC-0001 §15.
- Cada capítulo debe cumplir la anatomía de RFC-0001 §14.
- Cada ejercicio debe seguir los niveles de RFC-0001 §17.
- El uso de IA se rige por RFC-0001 §20: la IA acelera, el criterio humano
  decide.

## Fuera De Alcance Por Ahora

- Construir un kernel de producción.
- Usar `unsafe` sin justificación escrita y revisión humana explícita.
- Agregar dependencias externas para esconder mecanismos que el curso debe
  explicar.
- Reexplicar deadlocks como capítulo canónico: ese canon vive en
  `rust-concurrency`, aunque este curso puede citarlos cuando hable de mutex.
- Reexplicar redes desde cero: ese canon vive en `rust-networking`.
