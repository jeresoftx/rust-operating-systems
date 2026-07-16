# Mutex

> **Curso:** rust-operating-systems · **Capítulo:** 02 ·
> **Prerrequisitos:** procesos, hilos y estado de ejecución
> **Código:** [`src/mutex.rs`](../src/mutex.rs) · **Video:** pendiente
> **Lección en el sitio:** pendiente

## Introducción

Un mutex protege una región crítica para que solo un hilo la use a la vez. Es
una de las ideas más pequeñas y más peligrosas de sistemas operativos: parece
un candado simple, pero detrás hay propiedad, espera, errores de liberación,
fallos del dueño y riesgo de deadlock.

Este capítulo no usa `std::sync::Mutex` como mecanismo principal. Construye un
modelo educativo para mirar el contrato: quién adquiere, quién posee, quién
puede liberar, qué ocurre si otro hilo compite y cómo se representa un mutex
poisoned.

## Motivación

Dos hilos pueden tocar el mismo recurso: un contador, una cola, un archivo, una
estructura de memoria o un índice. Si ambos modifican el recurso sin orden, el
resultado puede depender del interleaving exacto de instrucciones.

La idea central es:

```text
Un mutex convierte acceso compartido en una región crítica con propietario
explícito.
```

## Teoría

### Historia

La exclusión mutua aparece cada vez que varios flujos de ejecución comparten
estado. Los sistemas operativos, runtimes y bibliotecas de concurrencia la
ofrecen de muchas formas, pero el contrato base se mantiene: adquirir, entrar,
salir y liberar.

### Fundamentos

El modelo del crate usa:

- `MutexId`: identidad del mutex;
- `MutexState`: `Free`, `Locked { owner }` y `Poisoned`;
- `MutexModel`: estado y operaciones explícitas;
- `LockGuardModel`: representación educativa de propiedad temporal;
- `MutexError`: contención, dueño incorrecto, mutex no bloqueado y poisoning.

### Región crítica

Una región crítica es el tramo de código que no debe ejecutarse al mismo tiempo
por varios hilos sobre el mismo recurso. El mutex no protege por magia: protege
solo si todo el código que toca el recurso respeta el mismo contrato.

### Propietario

Cuando un hilo adquiere un mutex, se vuelve propietario. En este modelo, solo
el propietario puede liberar. Si otro hilo intenta liberar, el modelo responde
con `NotOwner` y conserva el estado bloqueado.

### Contención

La contención ocurre cuando un hilo intenta adquirir un mutex que ya tiene
propietario. En un sistema real, el hilo podría dormir, girar, entrar a una cola
o delegar al scheduler. Aquí devolvemos `AlreadyLocked` con dueño y contendiente
para que el caso sea observable y verificable.

### Poisoning

Poisoning significa que el dueño falló mientras tenía el mutex. El estado
protegido podría estar inconsistente. Rust usa poisoning en `std::sync::Mutex`
como señal de cuidado. Este modelo lo representa con `MutexState::Poisoned` y
una recuperación explícita con `recover`.

### Deadlocks

Los deadlocks son un riesgo central de mutex, pero el canon de condiciones de
Coffman, detección y prevención vive en `rust-concurrency`. Aquí solo se
nombran como riesgo operativo para no duplicar contenido desde cero.

### Casos de uso

Mutex aparece en:

- contadores compartidos;
- caches internas;
- colas de trabajo;
- estructuras protegidas dentro de runtimes;
- coordinación simple entre hilos;
- secciones críticas de bajo nivel.

### Ventajas y limitaciones

Ventajas:

- Contrato pequeño y claro.
- Hace visible la propiedad de una región crítica.
- Permite proteger estructuras mutables compartidas.

Limitaciones:

- Puede introducir contención.
- Puede esconder cuellos de botella.
- Puede producir deadlocks si se combina mal con otros locks.
- Este modelo no bloquea hilos reales ni implementa fairness.

## Diagramas

El diagrama principal vive en [`diagrams/02-mutex.mmd`](../diagrams/02-mutex.mmd).
Muestra estado libre, adquisición, contención, liberación y poisoning.

## Análisis de complejidad

| Operación | Mejor caso | Caso promedio | Peor caso | Espacio |
|-----------|------------|---------------|-----------|---------|
| `MutexModel::new` | O(1) | O(1) | O(1) | O(1) |
| `lock` | O(1) | O(1) | O(1) | O(1) |
| `unlock` | O(1) | O(1) | O(1) | O(1) |
| `poison` | O(1) | O(1) | O(1) | O(1) |
| `recover` | O(1) | O(1) | O(1) | O(1) |

El modelo mantiene una sola palabra de estado conceptual: libre, bloqueado o
poisoned.

## Visualización interactiva (opcional)

No aplica todavía. Una visualización futura podría mostrar dos hilos compitiendo
por el mismo mutex y resaltar el propietario actual.

## Implementación

La implementación vive en [`src/mutex.rs`](../src/mutex.rs). No depende de
`std::sync::Mutex`; usa un enum propio para que las transiciones sean visibles.

`lock` cambia `Free` a `Locked`. `unlock` solo funciona si lo llama el dueño.
`poison` representa una falla del dueño durante la región crítica. `recover`
limpia el estado poisoned de forma explícita.

## Pruebas

Las pruebas cubren:

- adquisición de mutex libre;
- rechazo de segundo dueño;
- liberación por propietario;
- rechazo de liberación por no propietario;
- poisoning y recuperación;
- liberación mediante `LockGuardModel`.

## Benchmarks

El benchmark manual vive en
[`benches/mutex_bench.rs`](../benches/mutex_bench.rs). Mide adquisición y
liberación, contención educativa y poisoning con recuperación.

## Ejercicios

### Ejercicio 1: Lock y unlock `[Nivel 1]`

Adquiere un mutex libre y libéralo con el mismo hilo.

**Entrada/Salida esperada:** el estado final debe ser `Free`.

### Ejercicio 2: Contención `[Nivel 2]`

Intenta adquirir un mutex ya bloqueado por otro hilo.

**Entrada/Salida esperada:** debe devolver `AlreadyLocked` con dueño y
contendiente.

### Ejercicio 3: Poisoning `[Nivel 3]`

Marca un mutex como poisoned, rechaza una adquisición y después recupéralo.

**Entrada/Salida esperada:** tras `recover`, el mutex debe poder adquirirse de
nuevo.

### Ejercicio 4: Riesgo de deadlock `[Nivel 4]`

Describe un caso con dos mutex y dos hilos que pueda terminar en deadlock.
Explica por qué este capítulo solo nombra el riesgo y por qué el tratamiento
canónico vive en `rust-concurrency`.

## Soluciones

Las soluciones ejecutables de niveles 1 a 3 viven en:

- [`examples/soluciones/mutex_lock_unlock.rs`](../examples/soluciones/mutex_lock_unlock.rs)
- [`examples/soluciones/mutex_contention.rs`](../examples/soluciones/mutex_contention.rs)
- [`examples/soluciones/mutex_poisoning_model.rs`](../examples/soluciones/mutex_poisoning_model.rs)
