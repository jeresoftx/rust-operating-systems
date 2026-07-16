# Procesos e hilos

> **Curso:** rust-operating-systems · **Capítulo:** 01 ·
> **Prerrequisitos:** Rust básico, ownership y nociones de ejecución
> **Código:** [`src/processes.rs`](../src/processes.rs) · **Video:** pendiente
> **Lección en el sitio:** pendiente

## Introducción

Un proceso es la unidad clásica de aislamiento y administración de recursos en
un sistema operativo. Un hilo es una línea de ejecución dentro de un proceso.
Ambos conceptos aparecen en servidores, navegadores, bases de datos, runtimes,
terminales y herramientas de línea de comandos.

Este capítulo no crea procesos reales del sistema operativo. Modela la
semántica que necesitamos entender: identidad, estado, transición, tabla de
procesos y relación entre proceso e hilos.

## Motivación

Cuando una computadora ejecuta varias cosas a la vez, el sistema necesita
contestar preguntas concretas:

- ¿Qué programa está vivo?
- ¿Cuál puede ejecutarse ahora?
- ¿Cuál está esperando entrada, disco o red?
- ¿Qué líneas de ejecución pertenecen al mismo proceso?
- ¿Cómo se encuentra un proceso por su identificador?

La idea central es:

```text
Un proceso organiza recursos; un hilo representa ejecución dentro de ese
contexto.
```

## Teoría

### Historia

Los primeros sistemas ejecutaban trabajos de forma más rígida y secuencial. A
medida que los sistemas se volvieron interactivos, multiusuario y conectados,
se hizo necesario representar programas vivos, pausados, bloqueados y
terminados. Esa representación es el lenguaje cotidiano del kernel.

### Fundamentos

El modelo del crate usa:

- `ProcessId`: identidad estable de proceso;
- `ThreadId`: identidad de hilo dentro del modelo;
- `ProcessState`: `Ready`, `Running`, `Blocked` y `Terminated`;
- `ThreadState`: estado equivalente para hilos;
- `Process`: nombre, estado e hilos;
- `Thread`: identificador y estado;
- `ProcessTable`: registro de procesos por PID;
- `ProcessError`: transiciones inválidas, duplicados y búsquedas fallidas.

### Proceso

Un proceso suele agrupar un espacio de direcciones, archivos abiertos,
credenciales, señales pendientes y metadatos de ejecución. En este capítulo
solo modelamos identidad, nombre, estado e hilos porque son suficientes para
construir el vocabulario de los capítulos siguientes.

### Hilo

Un hilo comparte el contexto del proceso, pero tiene su propia ejecución. En la
vida real tendría stack, registros, contador de programa y estado de
scheduling. En el modelo educativo tiene identidad y estado.

### Estados

El ciclo de vida mínimo es:

- `Ready`: puede ejecutarse cuando el scheduler lo elija.
- `Running`: está ejecutándose.
- `Blocked`: espera un evento externo.
- `Terminated`: ya terminó y no debe crear más trabajo.

El modelo permite transiciones acotadas para enseñar que no todo cambio de
estado tiene sentido. Por ejemplo, pasar de `Ready` a `Blocked` sin haber
ejecutado nada se rechaza.

### Tabla de procesos

La tabla de procesos permite registrar y encontrar procesos por PID. Un PID
duplicado es un error porque rompe la identidad. En un sistema real habría más
estructura, permisos, jerarquías y referencias; aquí usamos un mapa ordenado
para que la operación sea visible y determinista.

### Casos de uso

Procesos e hilos aparecen en:

- servidores con varios workers;
- navegadores con procesos aislados por pestaña o sitio;
- shells que lanzan comandos;
- bases de datos con hilos de fondo;
- runtimes que administran pools de trabajo.

### Ventajas y limitaciones

Ventajas del modelo:

- Enseña identidad y estado sin depender del sistema operativo anfitrión.
- Hace explícitas las transiciones válidas.
- Permite probar hilos y tabla de procesos de forma determinista.

Limitaciones:

- No crea procesos reales.
- No modela permisos, memoria real, file descriptors ni señales.
- No modela scheduling todavía; eso vive en el capítulo 5.
- No modela deadlocks; ese canon vive en `rust-concurrency`.

## Diagramas

El diagrama principal vive en
[`diagrams/01-procesos-e-hilos.mmd`](../diagrams/01-procesos-e-hilos.mmd).
Muestra estados, tabla de procesos e hilos dentro de un proceso.

## Análisis de complejidad

| Operación | Mejor caso | Caso promedio | Peor caso | Espacio |
|-----------|------------|---------------|-----------|---------|
| `Process::new` | O(n) | O(n) | O(n) | O(n) |
| `transition_to` | O(1) | O(1) | O(1) | O(1) |
| `spawn_thread` | O(1) amortizado | O(1) amortizado | O(t) | O(1) |
| `thread` | O(1) | O(t) | O(t) | O(1) |
| `ProcessTable::insert` | O(log p) | O(log p) | O(log p) | O(s) |
| `ProcessTable::get` | O(log p) | O(log p) | O(log p) | O(1) |

`n` es la longitud del nombre, `t` la cantidad de hilos, `p` la cantidad de
procesos y `s` el tamaño del proceso insertado.

## Visualización interactiva (opcional)

No aplica todavía. Una visualización futura podría permitir mover un proceso
entre estados y observar qué transiciones son rechazadas.

## Implementación

La implementación vive en [`src/processes.rs`](../src/processes.rs). Usa
tipos pequeños para identidad, enums para estado y `BTreeMap` para la tabla de
procesos. Las transiciones se validan antes de mutar el estado.

El proceso inicia en `Ready` con un hilo principal `ThreadId(1)`. Los hilos
adicionales se crean con IDs crecientes. Un proceso `Terminated` no puede crear
nuevos hilos.

## Pruebas

Las pruebas cubren:

- creación de proceso con PID, nombre y estado inicial;
- ciclo de vida del proceso;
- rechazo de transición inválida;
- creación de hilos con estado independiente;
- rechazo de hilos nuevos cuando el proceso terminó;
- tabla de procesos con búsqueda y rechazo de duplicados.

## Benchmarks

El benchmark manual vive en
[`benches/processes_bench.rs`](../benches/processes_bench.rs). Mide registro y
búsqueda en tabla de procesos, transición de estados y creación de hilos.

## Ejercicios

### Ejercicio 1: Ciclo de vida `[Nivel 1]`

Crea un proceso y llévalo de `Ready` a `Running`.

**Entrada/Salida esperada:** el estado final debe ser `Running`.

### Ejercicio 2: Hilos `[Nivel 2]`

Crea dos hilos adicionales en un proceso.

**Entrada/Salida esperada:** el proceso debe tener tres hilos contando el hilo
principal.

### Ejercicio 3: Tabla de procesos `[Nivel 3]`

Registra dos procesos con PIDs distintos y búscalos por identificador.

**Entrada/Salida esperada:** cada búsqueda debe devolver el nombre correcto.

### Ejercicio 4: Proceso o hilo `[Nivel 4]`

Decide si aislarías pestañas de un navegador con procesos, hilos o una mezcla.
Justifica con aislamiento, memoria, seguridad y costo de cambio de contexto.

## Soluciones

Las soluciones ejecutables de niveles 1 a 3 viven en:

- [`examples/soluciones/process_lifecycle.rs`](../examples/soluciones/process_lifecycle.rs)
- [`examples/soluciones/thread_spawn_join.rs`](../examples/soluciones/thread_spawn_join.rs)
- [`examples/soluciones/process_tree.rs`](../examples/soluciones/process_tree.rs)
