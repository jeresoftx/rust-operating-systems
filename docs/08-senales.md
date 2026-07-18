# Señales

> **Curso:** rust-operating-systems · **Capítulo:** 08 ·
> **Prerrequisitos:** procesos, scheduling y memoria virtual
> **Código:** [`src/signals.rs`](../src/signals.rs) · **Video:** pendiente
> **Lección en el sitio:** pendiente

## Introducción

Una señal es una notificación asíncrona dirigida a un proceso. Puede representar
una interrupción externa, una petición de terminación, una alarma o un evento
que el proceso debe atender fuera de su flujo normal.

Este capítulo modela señales pendientes, máscaras, despacho determinista y
acciones educativas: acción por defecto, ignorar y manejar con un handler
nombrado. No ejecuta handlers reales; hace visible el contrato que un sistema
operativo debe sostener.

## Motivación

Los procesos no viven aislados de su entorno. Un usuario puede pedir cancelar un
programa; el sistema puede avisar que terminó un hijo; una operación puede
agotar su tiempo. Las señales dan un canal para interrumpir o notificar sin
convertir todo en llamadas sincrónicas.

La idea central es:

```text
Una señal separa el envío del momento exacto de entrega.
```

## Teoría

### Historia

Las señales son una pieza clásica de Unix y de sistemas POSIX. Nacieron como un
mecanismo pequeño para notificar eventos a procesos, pero con el tiempo se
volvieron delicadas: pueden llegar en momentos difíciles, interactúan con
máscaras y handlers, y no todo código es seguro dentro de un handler real.

### Fundamentos

El modelo del crate usa:

- `SignalNumber`: número de señal;
- `Signal`: señal dirigida a un `ProcessId`;
- `SignalMask`: conjunto de señales bloqueadas;
- `SignalAction`: acción por defecto, ignorar o manejar;
- `SignalQueue`: cola de señales pendientes;
- `SignalError`: errores de señal inválida.

### Entrega asíncrona

Enviar una señal no implica que se ejecute de inmediato. La señal puede quedar
pendiente hasta que el proceso esté en un punto donde pueda recibirla. Esa
separación es el origen de gran parte de su poder y de su complejidad.

### Máscaras

Una máscara bloquea temporalmente ciertas señales. Bloquear no significa borrar:
la señal queda pendiente y podrá entregarse cuando deje de estar bloqueada. En
el modelo, `dispatch_next` salta señales bloqueadas y entrega la siguiente no
bloqueada para el proceso.

### Handlers

Un handler es código que el proceso configura para atender una señal. En un
sistema real, los handlers tienen restricciones serias porque pueden ejecutarse
en momentos inesperados. Este capítulo los modela como nombres, no como
funciones, para enseñar la decisión sin meter ejecución asíncrona real.

### Acciones por defecto

Si no hay acción configurada, el modelo usa `Default`. En sistemas reales, una
acción por defecto puede terminar, ignorar, detener o continuar un proceso,
según la señal. Aquí solo se registra el resultado educativo.

### Riesgos

Las señales son útiles, pero fáciles de usar mal:

- pueden llegar mientras el proceso modifica estado compartido;
- pueden quedarse pendientes por una máscara;
- pueden ocultar errores si se ignoran sin criterio;
- los handlers reales deben ser muy pequeños y cuidadosos;
- el orden de entrega debe estar documentado.

### Casos de uso

Señales aparecen en:

- apagado ordenado;
- cancelación;
- timers;
- notificación de procesos hijos;
- recarga de configuración;
- interrupciones iniciadas por el usuario;
- supervisión operativa.

### Ventajas y limitaciones

Ventajas:

- Separan envío y entrega.
- Permiten notificar a un proceso sin acoplarlo a una llamada directa.
- Hacen explícita la máscara de señales bloqueadas.
- Permiten comparar acción por defecto, ignorar y manejar.

Limitaciones:

- Este modelo no ejecuta handlers reales.
- No integra scheduling ni interrupciones de hardware.
- No modela señales con payload ni colas POSIX avanzadas.
- No decide el efecto real de cada señal sobre el estado del proceso.

## Diagramas

El diagrama principal vive en
[`diagrams/08-senales.mmd`](../diagrams/08-senales.mmd). Muestra envío,
señales pendientes, máscara, despacho y resultado según la acción configurada.

## Análisis de complejidad

| Operación | Mejor caso | Caso promedio | Peor caso | Espacio |
|-----------|------------|---------------|-----------|---------|
| `SignalQueue::new` | O(1) | O(1) | O(1) | O(1) |
| `enqueue` | O(1) | O(1) | O(1) | O(n) |
| `pending_for` | O(n) | O(n) | O(n) | O(p) |
| `dispatch_next` | O(1) | O(n) | O(n) | O(1) |
| `SignalMask::block` | O(log b) | O(log b) | O(log b) | O(b) |

`n` es la cantidad de señales pendientes, `p` las pendientes del proceso y `b`
las señales bloqueadas por la máscara.

## Visualización interactiva (opcional)

No aplica todavía. Una visualización futura podría mostrar varias señales
pendientes, alternar la máscara y observar cuál se entrega primero.

## Implementación

La implementación vive en [`src/signals.rs`](../src/signals.rs). Usa `VecDeque`
para preservar orden de llegada, `BTreeSet` para la máscara y `BTreeMap` para
acciones configuradas por número de señal.

El modelo declara estas invariantes:

- el número de señal `0` se rechaza;
- una señal siempre tiene proceso destino;
- una señal bloqueada permanece pendiente;
- el despacho conserva orden determinista entre señales entregables;
- si no hay acción configurada, se usa `SignalAction::Default`.

## Pruebas

Las pruebas cubren:

- encolado de señal dirigida a un proceso;
- bloqueo mediante máscara;
- entrega de señales no bloqueadas en orden determinista;
- acción por defecto, ignorar y manejar.

## Benchmarks

El benchmark manual vive en
[`benches/signals_bench.rs`](../benches/signals_bench.rs). Mide encolado,
despacho de señales no bloqueadas y despacho con señales bloqueadas al inicio
de la cola.

## Ejercicios

### Ejercicio 1: Envío `[Nivel 1]`

Encola una señal `15` dirigida al proceso `42`.

**Entrada/Salida esperada:** la cola debe reportar una señal pendiente para el
proceso `42`.

### Ejercicio 2: Máscara `[Nivel 2]`

Bloquea la señal `2`, encola esa señal y pide despacho.

**Entrada/Salida esperada:** no debe entregarse nada y la señal debe seguir
pendiente.

### Ejercicio 3: Handler `[Nivel 3]`

Configura un handler llamado `shutdown-handler` para la señal `15` y entrégala.

**Entrada/Salida esperada:** el resultado debe ser `Handled` con ese handler.

### Ejercicio 4: Apagado ordenado `[Nivel 4]`

Diseña cómo usarías señales para apagar un servicio: qué señales aceptarías,
cuáles bloquearías durante secciones críticas y qué trabajo permitirías dentro
del handler.

## Soluciones

Las soluciones ejecutables de niveles 1 a 3 viven en:

- [`examples/soluciones/signal_delivery.rs`](../examples/soluciones/signal_delivery.rs)
- [`examples/soluciones/signal_mask.rs`](../examples/soluciones/signal_mask.rs)
- [`examples/soluciones/signal_handler.rs`](../examples/soluciones/signal_handler.rs)
