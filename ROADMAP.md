# ROADMAP

Estado de avance de `rust-operating-systems`, repositorio del camino troncal de
Jeresoft Academy para sistemas operativos en Rust.

No hay fechas límite: este es un proyecto de legado (RFC-0001 §1). Este archivo
orienta el avance, pero no convierte el curso en una carrera por terminar.

## Estado Actual

El repositorio acaba de iniciar su fundación: estructura base, crate Rust,
licencias, documentación de arranque y mapa de capítulos. La siguiente línea
natural es crear el checklist detallado del curso y comenzar con procesos e
hilos sin perder la anatomía completa de RFC-0001 §14.

## Capítulos Planeados

| # | Capítulo | Estado |
|---|----------|--------|
| 01 | Procesos e hilos | planned |
| 02 | Mutex | planned |
| 03 | Semáforos | planned |
| 04 | Memoria | planned |
| 05 | Scheduling | planned |
| 06 | Paging | planned |
| 07 | Memoria virtual | planned |
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
