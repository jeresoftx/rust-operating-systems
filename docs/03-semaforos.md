# Semáforos

> **Curso:** rust-operating-systems · **Capítulo:** 03 ·
> **Prerrequisitos:** procesos, hilos y mutex
> **Código:** [`src/semaphores.rs`](../src/semaphores.rs) · **Video:** pendiente
> **Lección en el sitio:** pendiente

## Introducción

Un semáforo contador limita cuántos hilos pueden entrar a una región o usar un
recurso al mismo tiempo. A diferencia de un mutex, que normalmente permite un
solo propietario, un semáforo puede representar capacidad: tres conexiones,
ocho workers, cien slots de una cola.

Este capítulo modela permisos, capacidad, cola de espera y liberación
determinista. No bloquea hilos reales; hace visible el contrato para aprender
qué pasa cuando la capacidad se agota.

## Motivación

Muchos recursos no son estrictamente exclusivos, pero sí son finitos. Una base
de datos puede aceptar cierto número de conexiones; una API puede procesar una
cantidad limitada de trabajos simultáneos; un runtime puede restringir cuántas
tareas entran a una sección.

La idea central es:

```text
Un semáforo convierte capacidad finita en permisos explícitos.
```

## Teoría

### Historia

Los semáforos aparecen en los fundamentos clásicos de sistemas operativos como
una primitiva para sincronización y control de acceso. El nombre viene de la
idea de señales que habilitan o detienen el avance, pero en programación se
materializa como un contador con reglas.

### Fundamentos

El modelo del crate usa:

- `SemaphoreId`: identidad del semáforo;
- `Semaphore`: capacidad, permisos en uso y cola de espera;
- `Permit`: permiso otorgado a un hilo;
- `Waiter`: solicitante esperando capacidad;
- `SemaphoreError`: capacidad cero, espera, exceso y semáforo equivocado.

### Capacidad

La capacidad define cuántos permisos pueden estar en uso al mismo tiempo. Una
capacidad de cero no enseña nada útil para este modelo, así que se rechaza con
`ZeroCapacity`.

### Permisos

Adquirir un permiso aumenta los permisos en uso. Liberarlo reduce el uso si no
hay nadie esperando. Si existe cola de espera, la liberación despierta al
siguiente solicitante y transfiere el permiso de forma determinista.

### Cola de espera

Cuando la capacidad se agota, `acquire` devuelve `WouldBlock` y registra un
`Waiter`. El modelo usa FIFO para que el comportamiento sea estable y fácil de
probar.

### Backpressure

Backpressure es la señal de que el sistema no puede aceptar más trabajo al mismo
ritmo. En este modelo aparece como `WouldBlock`: el solicitante no recibe
permiso inmediato y queda registrado como espera.

### Diferencias contra mutex

Un mutex protege una región crítica de un solo dueño. Un semáforo modela
capacidad contada. Puedes construir patrones parecidos con ambos, pero el
significado no es el mismo: mutex habla de exclusión; semáforo habla de cupo.

### Casos de uso

Semáforos aparecen en:

- pools de conexiones;
- límites de concurrencia;
- colas de trabajo;
- control de acceso a recursos escasos;
- backpressure en servicios;
- throttling interno.

### Ventajas y limitaciones

Ventajas:

- Modela capacidad mejor que un mutex.
- Hace explícita la presión sobre recursos limitados.
- Permite una cola de espera simple.

Limitaciones:

- Puede ocultar problemas de diseño si se usa como parche.
- Requiere política clara para esperas y cancelaciones.
- Este modelo no duerme hilos reales ni integra scheduler.

## Diagramas

El diagrama principal vive en
[`diagrams/03-semaforos.mmd`](../diagrams/03-semaforos.mmd). Muestra capacidad,
permisos en uso, cola FIFO y transferencia en liberación.

## Análisis de complejidad

| Operación | Mejor caso | Caso promedio | Peor caso | Espacio |
|-----------|------------|---------------|-----------|---------|
| `Semaphore::new` | O(1) | O(1) | O(1) | O(1) |
| `acquire` con capacidad | O(1) | O(1) | O(1) | O(1) |
| `acquire` sin capacidad | O(1) | O(1) | O(1) | O(w) |
| `release` sin espera | O(1) | O(1) | O(1) | O(1) |
| `release` con espera | O(1) | O(1) | O(1) | O(1) |

`w` es la cantidad de solicitantes en espera.

## Visualización interactiva (opcional)

No aplica todavía. Una visualización futura podría mostrar permisos disponibles,
hilos esperando y liberaciones que despiertan al siguiente solicitante.

## Implementación

La implementación vive en [`src/semaphores.rs`](../src/semaphores.rs). Usa
`VecDeque` para la cola FIFO y tipos pequeños para identidad, permisos y
solicitantes.

`acquire` otorga permiso si hay capacidad; si no, registra un `Waiter`.
`release` valida que el permiso pertenezca al semáforo, evita liberar más allá
de la capacidad y despierta al siguiente solicitante si existe.

## Pruebas

Las pruebas cubren:

- adquisición cuando hay capacidad;
- espera cuando se agotan permisos;
- despertar determinista del siguiente solicitante;
- rechazo de permisos de otro semáforo;
- rechazo de liberación excesiva;
- rechazo de capacidad cero.

## Benchmarks

El benchmark manual vive en
[`benches/semaphores_bench.rs`](../benches/semaphores_bench.rs). Mide adquisición
y liberación, capacidad agotada y liberación con cola de espera.

## Ejercicios

### Ejercicio 1: Adquirir y liberar `[Nivel 1]`

Crea un semáforo de capacidad 1, adquiere un permiso y libéralo.

**Entrada/Salida esperada:** los permisos disponibles deben volver a 1.

### Ejercicio 2: Capacidad `[Nivel 2]`

Crea un semáforo de capacidad 2 y adquiere dos permisos.

**Entrada/Salida esperada:** el tercer solicitante debe recibir `WouldBlock`.

### Ejercicio 3: Cola de espera `[Nivel 3]`

Agota un semáforo, agrega dos solicitantes en espera y libera un permiso.

**Entrada/Salida esperada:** el primer solicitante debe recibir el permiso.

### Ejercicio 4: Mutex o semáforo `[Nivel 4]`

Decide si usarías mutex o semáforo para limitar conexiones a una API interna.
Justifica con exclusión, capacidad, backpressure y operación.

## Soluciones

Las soluciones ejecutables de niveles 1 a 3 viven en:

- [`examples/soluciones/semaphore_acquire_release.rs`](../examples/soluciones/semaphore_acquire_release.rs)
- [`examples/soluciones/semaphore_capacity.rs`](../examples/soluciones/semaphore_capacity.rs)
- [`examples/soluciones/semaphore_wait_queue.rs`](../examples/soluciones/semaphore_wait_queue.rs)
