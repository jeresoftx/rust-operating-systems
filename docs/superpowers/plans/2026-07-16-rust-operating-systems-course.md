# Rust Operating Systems Course Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** construir el curso `rust-operating-systems` como libro de ingeniería y crate Rust educativo para los mecanismos canónicos de sistemas operativos definidos en RFC-0001 §10.

**Architecture:** el repositorio se organiza como un crate Rust con un módulo por mecanismo, un capítulo Markdown por tema, pruebas de integración por módulo, ejemplos progresivos, soluciones, diagramas Mermaid y benchmarks manuales. Cada capítulo debe explicar invariantes, límites, modos de falla y tradeoffs sin prometer un kernel de producción.

**Tech Stack:** Rust 2021, biblioteca estándar, Markdown compatible con mdBook, Mermaid, pruebas con `cargo test`, lint con Clippy y benchmarks manuales con `std::time::Instant`.

---

## Contexto Obligatorio

- Este repo pertenece a Jeresoft Academy y se rige por RFC-0001.
- Respetar `AGENTS.md`, `README.md`, `ROADMAP.md` y el manual fundacional del repo padre.
- Este curso es el canon de mecanismos de sistemas operativos: procesos, hilos, sincronización básica, memoria, scheduling, paging, memoria virtual, señales, IPC y filesystem.
- No convertir esto en un repo genérico ni en un kernel de juguete sin explicación. Debe leerse como libro de ingeniería y como crate Rust educativo.
- Mantener español es-MX con buena ortografía en documentación y mensajes visibles.
- No usar `unsafe` salvo autorización humana explícita y justificación documentada.
- No agregar dependencias externas no triviales sin aprobación humana.

## Verificación Estándar

Antes de cada commit importante, cuando aplique:

- [ ] Ejecutar `cargo fmt --check`.
- [ ] Ejecutar `cargo clippy --all-targets --all-features -- -D warnings`.
- [ ] Ejecutar `cargo test --all-targets`.
- [ ] Ejecutar `cargo test --doc`.
- [ ] Ejecutar `git diff --check`.
- [ ] Revisar ortografía básica de español con búsquedas dirigidas para acentos, `ñ`, signos y términos frecuentes.
- [ ] Hacer commit pequeño con conventional commits.
- [ ] Empujar a `origin/main` después de checkpoints verdes.

## Estados Del Curso

- `planned`: capítulo definido, sin implementación.
- `draft`: capítulo o modelo iniciado, incompleto.
- `implemented`: API educativa implementada.
- `tested`: API con pruebas completas.
- `benchmarked`: capítulo con docs, ejemplos, soluciones, diagrama y benchmark.
- `reviewed`: capítulo revisado por criterio humano.
- `published`: capítulo listo para sitio o distribución pública.

---

## Task 0: Fundación Del Repositorio

**Files:**
- Done: `README.md`
- Done: `ROADMAP.md`
- Done: `AGENTS.md`
- Done: `LICENSE.md`
- Done: `LICENSE-MIT`
- Done: `LICENSE-APACHE`
- Done: `LICENSE-CC-BY-SA-4.0.md`
- Done: `Cargo.toml`
- Done: `src/lib.rs`
- Done: `docs/SUMMARY.md`

- [x] Crear repositorio remoto `jeresoftx/rust-operating-systems`.
- [x] Configurar About de GitHub en español.
- [x] Configurar topics: `jeresoft-academy`, `rust`, `operating-systems`, `systems-programming`, `computer-science`, `education`, `es-mx`.
- [x] Clonar dentro de `repos/rust-operating-systems`.
- [x] Crear crate Rust educativo mínimo.
- [x] Copiar licencias de código y contenido educativo.
- [x] Crear README con lugar en el camino y capítulos.
- [x] Crear ROADMAP inicial.
- [x] Crear AGENTS específico del repo.
- [x] Ejecutar `cargo fmt --check`.
- [x] Ejecutar `cargo clippy --all-targets --all-features -- -D warnings`.
- [x] Ejecutar `cargo test --all-targets`.
- [x] Ejecutar `cargo test --doc`.
- [x] Hacer commit: `chore: scaffold course repository`.

---

## Task 1: Procesos E Hilos

**Files:**
- Create: `docs/01-procesos-e-hilos.md`
- Create: `src/processes.rs`
- Modify: `src/lib.rs`
- Create: `tests/processes_test.rs`
- Create: `benches/processes_bench.rs`
- Create: `diagrams/01-procesos-e-hilos.mmd`
- Create: `examples/processes_basic.rs`
- Create: `examples/processes_intermediate.rs`
- Create: `examples/processes_advanced.rs`
- Create: `examples/processes_real_case.rs`
- Create: `examples/soluciones/process_lifecycle.rs`
- Create: `examples/soluciones/thread_spawn_join.rs`
- Create: `examples/soluciones/process_tree.rs`
- Modify: `Cargo.toml`
- Modify: `README.md`
- Modify: `ROADMAP.md`

- [x] Diseñar API mínima: `ProcessId`, `ThreadId`, `ProcessState`, `ThreadState`, `Process`, `Thread`, `ProcessTable`, `ProcessError`.
- [x] Escribir test rojo para crear un proceso con PID, nombre y estado inicial `Ready`.
- [x] Implementar `ProcessId`, `ProcessState` y `Process`.
- [x] Escribir test rojo para transición `Ready -> Running -> Blocked -> Ready -> Terminated`.
- [x] Implementar transición de estados con errores para transiciones inválidas.
- [x] Escribir test rojo para crear hilos dentro de un proceso.
- [x] Implementar `Thread`, `ThreadId` y relación proceso-hilos.
- [x] Escribir test rojo para tabla de procesos con búsqueda por PID y rechazo de duplicados.
- [x] Implementar `ProcessTable`.
- [x] Documentar diferencia entre proceso e hilo, espacio de direcciones, estado, contexto y ciclo de vida.
- [x] Crear diagrama Mermaid de ciclo de vida y tabla de procesos.
- [x] Crear ejemplos progresivos y caso real de servidor con hilos de trabajo.
- [x] Crear ejercicios y soluciones.
- [x] Crear benchmark de búsqueda en tabla de procesos y transición de estados.
- [x] Actualizar README y ROADMAP a `benchmarked`.
- [x] Verificar y hacer commit: `feat: add processes chapter`.

---

## Task 2: Mutex

**Files:**
- Create: `docs/02-mutex.md`
- Create: `src/mutex.rs`
- Modify: `src/lib.rs`
- Create: `tests/mutex_test.rs`
- Create: `benches/mutex_bench.rs`
- Create: `diagrams/02-mutex.mmd`
- Create: `examples/mutex_basic.rs`
- Create: `examples/mutex_intermediate.rs`
- Create: `examples/mutex_advanced.rs`
- Create: `examples/mutex_real_case.rs`
- Create: `examples/soluciones/mutex_lock_unlock.rs`
- Create: `examples/soluciones/mutex_contention.rs`
- Create: `examples/soluciones/mutex_poisoning_model.rs`
- Modify: `Cargo.toml`
- Modify: `README.md`
- Modify: `ROADMAP.md`

- [x] Diseñar API mínima: `MutexId`, `MutexState`, `MutexModel`, `LockGuardModel`, `MutexError`.
- [x] Escribir test rojo para adquirir un mutex libre.
- [x] Implementar adquisición y liberación explícita.
- [x] Escribir test rojo para rechazar doble adquisición por otro hilo.
- [x] Implementar contención educativa con propietario actual.
- [x] Escribir test rojo para liberar desde un hilo que no es dueño.
- [x] Implementar error `NotOwner`.
- [x] Escribir test rojo para modelo de poisoning educativo tras falla del propietario.
- [x] Implementar estado `Poisoned` sin depender de `std::sync::Mutex` como mecanismo principal.
- [x] Documentar exclusión mutua, región crítica, propietario, contención y poisoning.
- [x] Citar que deadlocks son canónicos en `rust-concurrency`; aquí se explican solo como riesgo operativo.
- [x] Crear diagrama Mermaid de bloqueo, adquisición y liberación.
- [x] Crear ejemplos progresivos y caso real de contador compartido.
- [x] Crear ejercicios y soluciones.
- [x] Crear benchmark de adquisición/liberación del modelo.
- [x] Actualizar README y ROADMAP a `benchmarked`.
- [x] Verificar y hacer commit: `feat: add mutex chapter`.

---

## Task 3: Semáforos

**Files:**
- Create: `docs/03-semaforos.md`
- Create: `src/semaphores.rs`
- Modify: `src/lib.rs`
- Create: `tests/semaphores_test.rs`
- Create: `benches/semaphores_bench.rs`
- Create: `diagrams/03-semaforos.mmd`
- Create: `examples/semaphores_basic.rs`
- Create: `examples/semaphores_intermediate.rs`
- Create: `examples/semaphores_advanced.rs`
- Create: `examples/semaphores_real_case.rs`
- Create: `examples/soluciones/semaphore_acquire_release.rs`
- Create: `examples/soluciones/semaphore_capacity.rs`
- Create: `examples/soluciones/semaphore_wait_queue.rs`
- Modify: `Cargo.toml`
- Modify: `README.md`
- Modify: `ROADMAP.md`

- [x] Diseñar API mínima: `SemaphoreId`, `Semaphore`, `Permit`, `Waiter`, `SemaphoreError`.
- [x] Escribir test rojo para adquirir un permiso cuando hay capacidad.
- [x] Implementar contador de permisos.
- [x] Escribir test rojo para agotar capacidad y poner solicitantes en espera.
- [x] Implementar cola educativa de espera.
- [x] Escribir test rojo para liberar un permiso y despertar al siguiente solicitante.
- [x] Implementar liberación determinista.
- [x] Escribir test rojo para rechazar liberaciones que excedan capacidad.
- [x] Implementar error `CapacityExceeded`.
- [x] Documentar semáforo contador, capacidad, cola de espera, backpressure y diferencias contra mutex.
- [x] Crear diagrama Mermaid de permisos y cola.
- [x] Crear ejemplos progresivos y caso real de pool de conexiones.
- [x] Crear ejercicios y soluciones.
- [x] Crear benchmark de adquisición/liberación con capacidad limitada.
- [x] Actualizar README y ROADMAP a `benchmarked`.
- [x] Verificar y hacer commit: `feat: add semaphores chapter`.

---

## Task 4: Memoria

**Files:**
- Create: `docs/04-memoria.md`
- Create: `src/memory.rs`
- Modify: `src/lib.rs`
- Create: `tests/memory_test.rs`
- Create: `benches/memory_bench.rs`
- Create: `diagrams/04-memoria.mmd`
- Create: `examples/memory_basic.rs`
- Create: `examples/memory_intermediate.rs`
- Create: `examples/memory_advanced.rs`
- Create: `examples/memory_real_case.rs`
- Create: `examples/soluciones/memory_region.rs`
- Create: `examples/soluciones/memory_allocation.rs`
- Create: `examples/soluciones/memory_fragmentation.rs`
- Modify: `Cargo.toml`
- Modify: `README.md`
- Modify: `ROADMAP.md`

- [x] Diseñar API mínima: `Address`, `Bytes`, `MemoryRegion`, `Allocation`, `AllocatorModel`, `MemoryError`.
- [x] Escribir test rojo para crear una región de memoria con inicio y tamaño.
- [x] Implementar `MemoryRegion` con validación de rango.
- [x] Escribir test rojo para reservar un bloque dentro de una región.
- [x] Implementar `AllocatorModel` first-fit educativo.
- [x] Escribir test rojo para liberar un bloque y reutilizar espacio.
- [x] Implementar liberación y coalescing simple de huecos contiguos.
- [x] Escribir test rojo para detectar fragmentación externa.
- [x] Implementar métrica educativa de fragmentación.
- [x] Documentar direcciones, regiones, asignación, liberación, fragmentación interna y externa.
- [x] Crear diagrama Mermaid de región, bloques y huecos.
- [x] Crear ejemplos progresivos y caso real de arena para solicitudes.
- [x] Crear ejercicios y soluciones.
- [x] Crear benchmark de asignación/liberación first-fit.
- [x] Actualizar README y ROADMAP a `benchmarked`.
- [x] Verificar y hacer commit: `feat: add memory chapter`.

---

## Task 5: Scheduling

**Files:**
- Create: `docs/05-scheduling.md`
- Create: `src/scheduling.rs`
- Modify: `src/lib.rs`
- Create: `tests/scheduling_test.rs`
- Create: `benches/scheduling_bench.rs`
- Create: `diagrams/05-scheduling.mmd`
- Create: `examples/scheduling_basic.rs`
- Create: `examples/scheduling_intermediate.rs`
- Create: `examples/scheduling_advanced.rs`
- Create: `examples/scheduling_real_case.rs`
- Create: `examples/soluciones/scheduler_round_robin.rs`
- Create: `examples/soluciones/scheduler_priority.rs`
- Create: `examples/soluciones/scheduler_starvation.rs`
- Modify: `Cargo.toml`
- Modify: `README.md`
- Modify: `ROADMAP.md`

- [ ] Diseñar API mínima: `TaskId`, `Task`, `Priority`, `Scheduler`, `SchedulingPolicy`, `SchedulingError`.
- [ ] Escribir test rojo para seleccionar la siguiente tarea en round-robin.
- [ ] Implementar política round-robin.
- [ ] Escribir test rojo para prioridad mayor cuando la política sea priority scheduling.
- [ ] Implementar política por prioridad.
- [ ] Escribir test rojo para quantum agotado y reencolado.
- [ ] Implementar `tick` educativo.
- [ ] Escribir test rojo para detectar inanición potencial.
- [ ] Implementar métrica de espera por tarea.
- [ ] Documentar scheduler, cola de listos, quantum, prioridad, fairness, throughput y latencia.
- [ ] Crear diagrama Mermaid de cola de listos y cambio de contexto.
- [ ] Crear ejemplos progresivos y caso real de jobs de fondo.
- [ ] Crear ejercicios y soluciones.
- [ ] Crear benchmark de selección de siguiente tarea.
- [ ] Actualizar README y ROADMAP a `benchmarked`.
- [ ] Verificar y hacer commit: `feat: add scheduling chapter`.

---

## Task 6: Paging

**Files:**
- Create: `docs/06-paging.md`
- Create: `src/paging.rs`
- Modify: `src/lib.rs`
- Create: `tests/paging_test.rs`
- Create: `benches/paging_bench.rs`
- Create: `diagrams/06-paging.mmd`
- Create: `examples/paging_basic.rs`
- Create: `examples/paging_intermediate.rs`
- Create: `examples/paging_advanced.rs`
- Create: `examples/paging_real_case.rs`
- Create: `examples/soluciones/page_translation.rs`
- Create: `examples/soluciones/page_fault.rs`
- Create: `examples/soluciones/page_replacement.rs`
- Modify: `Cargo.toml`
- Modify: `README.md`
- Modify: `ROADMAP.md`

- [ ] Diseñar API mínima: `PageNumber`, `FrameNumber`, `PageSize`, `PageTableEntry`, `PageTable`, `PageFault`, `PagingError`.
- [ ] Escribir test rojo para traducir página a frame.
- [ ] Implementar tabla de páginas básica.
- [ ] Escribir test rojo para page fault cuando la página no está presente.
- [ ] Implementar error `PageFault`.
- [ ] Escribir test rojo para permisos de lectura/escritura.
- [ ] Implementar flags educativos de permisos.
- [ ] Escribir test rojo para reemplazo FIFO de página.
- [ ] Implementar política FIFO mínima.
- [ ] Documentar páginas, frames, offset, page table, page fault y reemplazo.
- [ ] Crear diagrama Mermaid de traducción página-frame-offset.
- [ ] Crear ejemplos progresivos y caso real de working set pequeño.
- [ ] Crear ejercicios y soluciones.
- [ ] Crear benchmark de traducción de direcciones.
- [ ] Actualizar README y ROADMAP a `benchmarked`.
- [ ] Verificar y hacer commit: `feat: add paging chapter`.

---

## Task 7: Memoria Virtual

**Files:**
- Create: `docs/07-memoria-virtual.md`
- Create: `src/virtual_memory.rs`
- Modify: `src/lib.rs`
- Create: `tests/virtual_memory_test.rs`
- Create: `benches/virtual_memory_bench.rs`
- Create: `diagrams/07-memoria-virtual.mmd`
- Create: `examples/virtual_memory_basic.rs`
- Create: `examples/virtual_memory_intermediate.rs`
- Create: `examples/virtual_memory_advanced.rs`
- Create: `examples/virtual_memory_real_case.rs`
- Create: `examples/soluciones/virtual_address_translation.rs`
- Create: `examples/soluciones/address_space_isolation.rs`
- Create: `examples/soluciones/copy_on_write_model.rs`
- Modify: `Cargo.toml`
- Modify: `README.md`
- Modify: `ROADMAP.md`

- [ ] Diseñar API mínima: `VirtualAddress`, `PhysicalAddress`, `AddressSpaceId`, `AddressSpace`, `Mapping`, `VirtualMemoryError`.
- [ ] Escribir test rojo para traducir dirección virtual a física.
- [ ] Implementar mapeo educativo virtual-físico.
- [ ] Escribir test rojo para aislamiento entre dos espacios de direcciones.
- [ ] Implementar `AddressSpace`.
- [ ] Escribir test rojo para protección de página no mapeada.
- [ ] Implementar error `UnmappedAddress`.
- [ ] Escribir test rojo para modelo simple de copy-on-write.
- [ ] Implementar contador de referencias educativo para páginas compartidas.
- [ ] Documentar aislamiento, traducción, permisos, page tables, TLB conceptual y copy-on-write.
- [ ] Crear diagrama Mermaid de espacio virtual por proceso.
- [ ] Crear ejemplos progresivos y caso real de fork conceptual.
- [ ] Crear ejercicios y soluciones.
- [ ] Crear benchmark de traducción virtual-física.
- [ ] Actualizar README y ROADMAP a `benchmarked`.
- [ ] Verificar y hacer commit: `feat: add virtual memory chapter`.

---

## Task 8: Señales

**Files:**
- Create: `docs/08-senales.md`
- Create: `src/signals.rs`
- Modify: `src/lib.rs`
- Create: `tests/signals_test.rs`
- Create: `benches/signals_bench.rs`
- Create: `diagrams/08-senales.mmd`
- Create: `examples/signals_basic.rs`
- Create: `examples/signals_intermediate.rs`
- Create: `examples/signals_advanced.rs`
- Create: `examples/signals_real_case.rs`
- Create: `examples/soluciones/signal_delivery.rs`
- Create: `examples/soluciones/signal_mask.rs`
- Create: `examples/soluciones/signal_handler.rs`
- Modify: `Cargo.toml`
- Modify: `README.md`
- Modify: `ROADMAP.md`

- [ ] Diseñar API mínima: `Signal`, `SignalNumber`, `SignalAction`, `SignalMask`, `SignalQueue`, `SignalError`.
- [ ] Escribir test rojo para encolar una señal dirigida a un proceso.
- [ ] Implementar `SignalQueue`.
- [ ] Escribir test rojo para bloquear una señal con máscara.
- [ ] Implementar `SignalMask`.
- [ ] Escribir test rojo para entregar señales no bloqueadas en orden determinista.
- [ ] Implementar despacho educativo.
- [ ] Escribir test rojo para acción por defecto, ignorar y manejar.
- [ ] Implementar `SignalAction`.
- [ ] Documentar señales, entrega asíncrona, máscaras, handlers, acciones por defecto y riesgos.
- [ ] Crear diagrama Mermaid de envío, máscara y entrega.
- [ ] Crear ejemplos progresivos y caso real de apagado ordenado.
- [ ] Crear ejercicios y soluciones.
- [ ] Crear benchmark de despacho de señales.
- [ ] Actualizar README y ROADMAP a `benchmarked`.
- [ ] Verificar y hacer commit: `feat: add signals chapter`.

---

## Task 9: IPC

**Files:**
- Create: `docs/09-ipc.md`
- Create: `src/ipc.rs`
- Modify: `src/lib.rs`
- Create: `tests/ipc_test.rs`
- Create: `benches/ipc_bench.rs`
- Create: `diagrams/09-ipc.mmd`
- Create: `examples/ipc_basic.rs`
- Create: `examples/ipc_intermediate.rs`
- Create: `examples/ipc_advanced.rs`
- Create: `examples/ipc_real_case.rs`
- Create: `examples/soluciones/ipc_pipe.rs`
- Create: `examples/soluciones/ipc_message_queue.rs`
- Create: `examples/soluciones/ipc_backpressure.rs`
- Modify: `Cargo.toml`
- Modify: `README.md`
- Modify: `ROADMAP.md`

- [ ] Diseñar API mínima: `ProcessEndpoint`, `Message`, `Pipe`, `MessageQueue`, `IpcError`.
- [ ] Escribir test rojo para enviar y recibir un mensaje entre dos procesos.
- [ ] Implementar `MessageQueue`.
- [ ] Escribir test rojo para pipe con orden FIFO.
- [ ] Implementar `Pipe`.
- [ ] Escribir test rojo para capacidad limitada y backpressure.
- [ ] Implementar rechazo o espera educativa cuando la cola está llena.
- [ ] Escribir test rojo para canal cerrado.
- [ ] Implementar error `ClosedChannel`.
- [ ] Documentar pipes, colas de mensajes, memoria compartida conceptual, backpressure y límites.
- [ ] Crear diagrama Mermaid de dos procesos y canal.
- [ ] Crear ejemplos progresivos y caso real de proceso supervisor.
- [ ] Crear ejercicios y soluciones.
- [ ] Crear benchmark de envío/recepción de mensajes.
- [ ] Actualizar README y ROADMAP a `benchmarked`.
- [ ] Verificar y hacer commit: `feat: add ipc chapter`.

---

## Task 10: Filesystem

**Files:**
- Create: `docs/10-filesystem.md`
- Create: `src/filesystem.rs`
- Modify: `src/lib.rs`
- Create: `tests/filesystem_test.rs`
- Create: `benches/filesystem_bench.rs`
- Create: `diagrams/10-filesystem.mmd`
- Create: `examples/filesystem_basic.rs`
- Create: `examples/filesystem_intermediate.rs`
- Create: `examples/filesystem_advanced.rs`
- Create: `examples/filesystem_real_case.rs`
- Create: `examples/soluciones/filesystem_inode.rs`
- Create: `examples/soluciones/filesystem_directory.rs`
- Create: `examples/soluciones/filesystem_permissions.rs`
- Modify: `Cargo.toml`
- Modify: `README.md`
- Modify: `ROADMAP.md`

- [ ] Diseñar API mínima: `InodeId`, `FileType`, `Permissions`, `DirectoryEntry`, `Inode`, `FileSystemModel`, `FileSystemError`.
- [ ] Escribir test rojo para crear archivo y directorio raíz.
- [ ] Implementar `Inode` y `DirectoryEntry`.
- [ ] Escribir test rojo para resolver una ruta simple.
- [ ] Implementar resolución de rutas absolutas.
- [ ] Escribir test rojo para permisos de lectura/escritura/ejecución.
- [ ] Implementar `Permissions`.
- [ ] Escribir test rojo para rechazar ciclos y nombres inválidos.
- [ ] Implementar validación de árbol educativo.
- [ ] Documentar inodos, directorios, rutas, permisos, metadata, journaling conceptual y modos de falla.
- [ ] Crear diagrama Mermaid de árbol de directorios e inodos.
- [ ] Crear ejemplos progresivos y caso real de workspace de proyecto.
- [ ] Crear ejercicios y soluciones.
- [ ] Crear benchmark de resolución de rutas.
- [ ] Actualizar README y ROADMAP a `benchmarked`.
- [ ] Verificar y hacer commit: `feat: add filesystem chapter`.

---

## Task 11: Integración Entre Cursos

**Files:**
- Modify: `README.md`
- Modify: `ROADMAP.md`
- Modify: `docs/01-procesos-e-hilos.md`
- Modify: `docs/02-mutex.md`
- Modify: `docs/03-semaforos.md`
- Modify: `docs/04-memoria.md`
- Modify: `docs/05-scheduling.md`
- Modify: `docs/06-paging.md`
- Modify: `docs/07-memoria-virtual.md`
- Modify: `docs/08-senales.md`
- Modify: `docs/09-ipc.md`
- Modify: `docs/10-filesystem.md`

- [ ] Citar `rust-networking` cuando se hable de procesos de servidor, sockets o servicios.
- [ ] Citar `rust-concurrency` para deadlocks, memory ordering, atomics y estructuras lock-free.
- [ ] Citar `rust-database-internals` cuando filesystem, paging o memoria alimenten storage engines.
- [ ] Citar `rust-distributed-systems` cuando IPC o procesos se proyecten a sistemas de varios nodos.
- [ ] Citar `rust-low-level` para caché, layouts de memoria y detalles de hardware cuando ese repo exista.
- [ ] Evitar reexplicar desde cero temas cuyo canon vive en otro repo.
- [ ] Actualizar ROADMAP con estado global del curso.
- [ ] Verificar y hacer commit: `docs: add cross-course references`.

---

## Task 12: Revisión Editorial Y Calidad

**Files:**
- Modify: `README.md`
- Modify: `ROADMAP.md`
- Modify: `docs/SUMMARY.md`
- Modify: `docs/*.md`
- Modify: `AGENTS.md`

- [ ] Revisar que cada capítulo tenga introducción, motivación, teoría, diagramas, análisis de complejidad, implementación, pruebas, benchmarks, ejercicios y soluciones.
- [ ] Revisar que el español sea es-MX, con acentos, `ñ`, signos y terminología consistente.
- [ ] Mantener términos técnicos aceptados cuando sean nombres de mecanismos o convenciones: `scheduling`, `paging`, `IPC`, `filesystem`, `thread`, `mutex`.
- [ ] Confirmar que README y ROADMAP no marquen capítulos como completos si están parciales.
- [ ] Ejecutar `cargo fmt --check`.
- [ ] Ejecutar `cargo clippy --all-targets --all-features -- -D warnings`.
- [ ] Ejecutar `cargo test --all-targets`.
- [ ] Ejecutar `cargo test --doc`.
- [ ] Ejecutar `cargo bench` cuando todos los benchmarks existan.
- [ ] Verificar y hacer commit: `docs: review operating systems course`.

---

## Orden Recomendado De Ejecución

1. Completar Task 1 y validar el molde de capítulo.
2. Continuar Tasks 2 y 3 para sincronización básica.
3. Completar Tasks 4, 6 y 7 como bloque de memoria.
4. Completar Task 5 como puente entre procesos y rendimiento.
5. Completar Tasks 8 y 9 como bloque de comunicación.
6. Completar Task 10 como cierre de mecanismos locales.
7. Ejecutar integración entre cursos.
8. Ejecutar revisión editorial y calidad.

## Primer Paso Natural

Comenzar con **Task 1: Procesos e hilos**. Es el punto de entrada conceptual:
define identidad, estado, transición, ejecución y relación entre proceso e hilo.
Ese vocabulario sostiene mutex, semáforos, scheduling, señales, IPC y filesystem.
