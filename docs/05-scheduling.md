# Scheduling

> **Curso:** rust-operating-systems · **Capítulo:** 05 ·
> **Prerrequisitos:** procesos, hilos, sincronización y memoria
> **Código:** [`src/scheduling.rs`](../src/scheduling.rs) · **Video:** pendiente
> **Lección en el sitio:** pendiente

## Introducción

Scheduling es el mecanismo que decide qué tarea recibe CPU cuando varias están
listas para ejecutar. Esa decisión parece pequeña, pero afecta latencia,
throughput, fairness y la sensación completa de respuesta de un sistema.

Este capítulo modela una cola de listos, una tarea en ejecución, un quantum y
dos políticas educativas: round-robin y prioridad. No intenta simular un kernel
real; busca que la decisión de "quién corre ahora" deje de parecer magia.

## Motivación

Un sistema con una sola tarea no necesita scheduler. En cuanto hay varias tareas
listas, el sistema debe elegir. Si siempre elige la primera, puede castigar a
otras. Si siempre elige la más importante, puede provocar inanición. Si rota
demasiado rápido, desperdicia trabajo en cambios de contexto. Si rota demasiado
lento, aumenta la latencia.

La idea central es:

```text
Un scheduler convierte recursos finitos de CPU en una política explícita.
```

## Teoría

### Historia

Los sistemas batch podían ejecutar trabajos largos con poca interacción humana.
Los sistemas interactivos hicieron más importante la latencia percibida. Los
sistemas modernos combinan prioridades, clases de tarea, afinidad, fairness,
tiempos de espera y mediciones de carga. Este capítulo se queda en el primer
nivel: una cola de tareas listas y una política visible.

### Fundamentos

El modelo del crate usa:

- `TaskId`: identidad de tarea;
- `Task`: nombre, prioridad, ticks requeridos, ticks ejecutados y espera;
- `Priority`: prioridad educativa, donde un valor mayor gana;
- `Scheduler`: cola de listos, tarea corriendo y tareas terminadas;
- `SchedulingPolicy`: round-robin o prioridad;
- `SchedulingError`: errores de quantum, cola vacía y tareas inválidas.

### Cola de listos

La cola de listos contiene tareas que pueden ejecutar, pero todavía no tienen
CPU. Una tarea bloqueada por I/O no debería estar aquí; una tarea terminada
tampoco. Este modelo se concentra en el caso "lista para correr".

### Round-robin

Round-robin toma la primera tarea lista, la ejecuta por un quantum y, si no
termina, la manda al final de la cola. Su virtud principal es la justicia
básica: todas las tareas listas avanzan con una cadencia predecible.

### Prioridad

Priority scheduling selecciona la tarea con mayor prioridad. Esto modela
servicios donde no todo trabajo vale lo mismo. Su riesgo es la inanición: una
tarea de baja prioridad puede esperar demasiado si siempre aparecen tareas más
importantes.

### Quantum

El quantum limita cuánto puede correr una tarea antes de ceder CPU. Un quantum
muy corto puede elevar el costo de cambio de contexto. Un quantum muy largo
puede aumentar latencia para las demás tareas.

### Fairness

Fairness no significa que todas las tareas reciban exactamente lo mismo. Significa
que el sistema puede explicar y defender su política de reparto. Round-robin
favorece fairness simple; priority scheduling favorece importancia explícita.

### Throughput y latencia

Throughput pregunta cuántos trabajos se completan por unidad de tiempo. Latencia
pregunta cuánto espera una tarea antes de avanzar. Una política puede mejorar
una métrica y dañar otra. Por eso el scheduler siempre es una decisión de
tradeoff, no una receta universal.

### Inanición

La inanición ocurre cuando una tarea técnicamente puede ejecutar, pero la
política la posterga una y otra vez. El modelo expone `waited_ticks` y
`starving_tasks` para hacer visible ese riesgo.

### Casos de uso

Scheduling aparece en:

- kernels;
- runtimes asincrónicos;
- colas de trabajos;
- workers de fondo;
- motores de videojuegos;
- sistemas batch;
- servicios con prioridades operativas.

### Ventajas y limitaciones

Ventajas:

- Hace visible la cola de listos.
- Permite comparar round-robin contra prioridad.
- Muestra quantum e inanición sin hilos reales.

Limitaciones:

- No modela interrupciones reales ni timers de hardware.
- No mide costo real de cambio de contexto.
- No maneja afinidad de CPU ni múltiples núcleos.
- No implementa aging automático.

## Diagramas

El diagrama principal vive en
[`diagrams/05-scheduling.mmd`](../diagrams/05-scheduling.mmd). Muestra cola de
listos, dispatch, tick, expiración de quantum, terminación y medición de espera.

## Análisis de complejidad

| Operación | Mejor caso | Caso promedio | Peor caso | Espacio |
|-----------|------------|---------------|-----------|---------|
| `Scheduler::new` | O(1) | O(1) | O(1) | O(1) |
| `add_task` | O(t) | O(t) | O(t) | O(t) |
| `dispatch_next` round-robin | O(1) | O(1) | O(1) | O(1) |
| `dispatch_next` prioridad | O(t) | O(t) | O(t) | O(1) |
| `tick` | O(t) | O(t) | O(t) | O(1) |
| `starving_tasks` | O(t) | O(t) | O(t) | O(s) |

`t` es la cantidad de tareas conocidas o listas, y `s` la cantidad de tareas que
superan el umbral de espera.

## Visualización interactiva (opcional)

No aplica todavía. Una visualización futura podría mostrar la cola de listos,
la tarea corriendo, el quantum restante y el crecimiento de espera por tarea.

## Implementación

La implementación vive en [`src/scheduling.rs`](../src/scheduling.rs). Usa
`VecDeque` para representar la cola de listos. `dispatch_next` selecciona según
la política. `tick` avanza un paso de CPU, incrementa la espera de las tareas
listas, completa tareas o reencola la tarea cuando se agota el quantum.

El modelo no usa hilos reales. Eso permite enfocarse en invariantes:

- solo una tarea corre a la vez;
- el quantum debe ser positivo;
- una tarea necesita al menos un tick de trabajo;
- no se aceptan tareas duplicadas;
- las tareas listas acumulan espera.

## Pruebas

Las pruebas cubren:

- selección round-robin;
- selección por prioridad;
- reencolado cuando expira el quantum;
- acumulación de espera;
- detección de inanición potencial.

## Benchmarks

El benchmark manual vive en
[`benches/scheduling_bench.rs`](../benches/scheduling_bench.rs). Mide selección
round-robin, selección por prioridad y tick con expiración de quantum.

## Ejercicios

### Ejercicio 1: Round-robin `[Nivel 1]`

Crea un scheduler round-robin con tres tareas y selecciona la primera.

**Entrada/Salida esperada:** la tarea `1` debe quedar en ejecución y la cola de
listos debe conservar `2, 3`.

### Ejercicio 2: Prioridad `[Nivel 2]`

Crea tres tareas con prioridades distintas y usa política por prioridad.

**Entrada/Salida esperada:** debe seleccionarse la tarea con prioridad más alta.

### Ejercicio 3: Inanición `[Nivel 3]`

Ejecuta varias rondas con una tarea de alta prioridad y otra de baja prioridad.

**Entrada/Salida esperada:** la tarea de baja prioridad debe acumular espera y
aparecer en `starving_tasks`.

### Ejercicio 4: Jobs de fondo `[Nivel 4]`

Diseña una política para jobs de fondo de una plataforma educativa. Decide cómo
balancear importaciones grandes, generación de videos, envíos de correo y
respuestas interactivas del sitio. Justifica prioridad, quantum, aging y
latencia aceptable.

## Soluciones

Las soluciones ejecutables de niveles 1 a 3 viven en:

- [`examples/soluciones/scheduler_round_robin.rs`](../examples/soluciones/scheduler_round_robin.rs)
- [`examples/soluciones/scheduler_priority.rs`](../examples/soluciones/scheduler_priority.rs)
- [`examples/soluciones/scheduler_starvation.rs`](../examples/soluciones/scheduler_starvation.rs)
