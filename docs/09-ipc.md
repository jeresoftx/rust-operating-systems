# IPC

> **Curso:** rust-operating-systems · **Capítulo:** 09 ·
> **Prerrequisitos:** procesos, memoria, scheduling y señales
> **Código:** [`src/ipc.rs`](../src/ipc.rs) · **Video:** pendiente
> **Lección en el sitio:** pendiente

## Introducción

IPC significa comunicación entre procesos. Un proceso tiene su propio espacio
de ejecución y, en sistemas con aislamiento, no debería escribir directamente
en la memoria privada de otro proceso. Aun así, los programas reales necesitan
coordinarse: enviar trabajo, devolver resultados, reportar errores, solicitar
apagado o transferir datos.

Este capítulo modela dos mecanismos educativos: colas de mensajes y pipes. La
meta no es copiar toda la complejidad de POSIX, sino hacer visibles los
contratos centrales: quién envía, quién recibe, en qué orden viajan los
mensajes, qué ocurre cuando el canal se llena y qué significa cerrar un canal.

## Motivación

Los procesos aislados son más seguros, pero ese aislamiento crea un problema:
si cada proceso vive en su propio mundo, ¿cómo se coordinan sin romper las
fronteras que los protegen?

La idea central es:

```text
IPC convierte cooperación entre procesos en un protocolo explícito.
```

Sin un canal claro, los procesos terminan compartiendo estado de forma
accidental, esperando activamente o mezclando responsabilidades. Con IPC, el
sistema puede validar límites, aplicar backpressure y registrar errores.

## Teoría

### Historia

Los sistemas Unix popularizaron mecanismos como pipes, señales, sockets Unix,
colas de mensajes y memoria compartida. Cada mecanismo resuelve un tipo de
coordinación distinto: algunos transmiten bytes en flujo, otros mensajes
discretos, otros comparten páginas completas de memoria.

En sistemas modernos, IPC también aparece dentro de navegadores, bases de
datos, runtimes, sistemas móviles, microservicios locales y supervisores de
procesos. El mismo vocabulario prepara el camino hacia `rust-distributed-systems`,
donde procesos separados pueden vivir en nodos diferentes.

### Fundamentos

El modelo del crate usa:

- `ProcessEndpoint`: extremo de comunicación asociado a un `ProcessId`;
- `Message`: bytes transportados por el canal;
- `Envelope`: mensaje con emisor y receptor explícitos;
- `MessageQueue`: cola finita de mensajes direccionados;
- `Pipe`: canal unidireccional con orden FIFO;
- `IpcError`: errores de capacidad, backpressure y canal cerrado.

### Pipes

Un pipe representa un flujo unidireccional. En este modelo se escribe un
`Message` al final de la cola y se lee desde el frente. La invariante principal
es FIFO: lo primero que entra debe ser lo primero que sale.

Los pipes son útiles cuando importa el orden y el receptor consume una
secuencia. No son ideales cuando se necesita seleccionar mensajes por destino,
prioridad o tipo; para eso conviene una cola de mensajes más explícita.

### Colas de mensajes

Una cola de mensajes conserva mensajes discretos. Cada mensaje se envía dentro
de un `Envelope` que registra emisor, receptor y contenido. En el modelo,
`receive(endpoint)` entrega el siguiente mensaje pendiente para ese endpoint.

Esto permite enseñar una diferencia importante: un pipe se parece a un flujo;
una cola de mensajes se parece a un buzón con destinatarios.

### Memoria compartida conceptual

La memoria compartida es otra forma de IPC: dos procesos acuerdan ver una misma
región de memoria. Es rápida porque evita copiar datos, pero exige
sincronización cuidadosa. Este capítulo la deja como concepto porque su modelo
completo cruza con memoria virtual, permisos, mutex, semáforos y cachés.

La regla educativa es: memoria compartida mejora rendimiento, pero aumenta la
cantidad de invariantes que deben sostenerse al mismo tiempo.

### Backpressure

Los canales reales tienen capacidad finita. Cuando una cola se llena, el
sistema puede bloquear al emisor, rechazar el envío, soltar mensajes o aplicar
prioridades. Este modelo usa rechazo educativo con `IpcError::Backpressure`
para que el límite sea visible en pruebas y ejemplos.

Backpressure no es un detalle operativo menor; es la diferencia entre un sistema
que falla de forma controlada y uno que consume memoria hasta caer.

### Cierre de canales

Cerrar un canal cambia el contrato. Un emisor ya no puede escribir y un receptor
ya no debe asumir que vendrán más mensajes. El modelo usa
`IpcError::ClosedChannel` para hacer explícito ese estado.

### Riesgos

IPC introduce riesgos propios:

- deadlocks cuando procesos esperan respuestas circulares;
- mensajes perdidos si se cierran canales sin protocolo;
- crecimiento de memoria si no existe capacidad finita;
- acoplamiento fuerte si los mensajes no tienen contrato estable;
- errores de orden cuando se mezclan flujo y mensajes discretos;
- confusión entre señalización, datos y control.

### Casos de uso

IPC aparece en:

- shells que conectan procesos con pipes;
- procesos supervisores que reciben reportes de workers;
- navegadores con procesos separados por pestaña;
- bases de datos que separan coordinador, workers y almacenamiento;
- runtimes que comunican procesos de sistema y procesos de usuario;
- servicios locales que exponen comandos mediante sockets Unix.

### Ventajas y limitaciones

Ventajas:

- Hace explícito quién envía y quién recibe.
- Permite razonar sobre orden FIFO.
- Declara capacidad finita y backpressure.
- Modela cierre de canal como error verificable.
- Sirve como puente hacia sistemas distribuidos.

Limitaciones:

- No modela scheduling de procesos bloqueados.
- No implementa permisos del kernel ni descriptores reales.
- No modela memoria compartida con páginas reales.
- No serializa estructuras complejas.
- No garantiza entrega si el protocolo de aplicación ignora errores.

## Diagramas

El diagrama principal vive en [`diagrams/09-ipc.mmd`](../diagrams/09-ipc.mmd).
Muestra dos procesos, endpoints, una cola finita, backpressure y cierre del
canal.

## Análisis de complejidad

| Operación | Mejor caso | Caso promedio | Peor caso | Espacio |
|-----------|------------|---------------|-----------|---------|
| `MessageQueue::new` | O(1) | O(1) | O(1) | O(1) |
| `MessageQueue::send` | O(1) | O(1) | O(1) | O(n) |
| `MessageQueue::receive` | O(1) | O(n) | O(n) | O(1) |
| `Pipe::write` | O(1) | O(1) | O(1) | O(n) |
| `Pipe::read` | O(1) | O(1) | O(1) | O(1) |

`n` es la cantidad de mensajes pendientes. `receive` puede ser O(n) porque
busca el siguiente mensaje dirigido al endpoint solicitado.

## Visualización interactiva (opcional)

No aplica todavía. Una visualización futura podría mostrar dos procesos,
capacidad del canal, mensajes en cola y el momento donde aparece backpressure.

## Implementación

La implementación vive en [`src/ipc.rs`](../src/ipc.rs). Usa `VecDeque` para
preservar orden de llegada y evita dependencias externas para que el mecanismo
sea visible.

El modelo declara estas invariantes:

- una cola o pipe no puede tener capacidad `0`;
- un mensaje conserva sus bytes sin interpretar obligatoriamente su formato;
- una cola de mensajes conserva emisor y receptor en un `Envelope`;
- un pipe entrega mensajes en orden FIFO;
- una cola llena rechaza el envío con `IpcError::Backpressure`;
- un canal cerrado rechaza lectura y escritura con `IpcError::ClosedChannel`.

## Pruebas

Las pruebas cubren:

- envío y recepción entre dos procesos;
- lectura FIFO en pipe;
- capacidad limitada y backpressure;
- rechazo de operaciones sobre canal cerrado.

## Benchmarks

El benchmark manual vive en [`benches/ipc_bench.rs`](../benches/ipc_bench.rs).
Mide envío y recepción en cola de mensajes, escritura y lectura en pipe, y el
costo de rechazar envíos cuando aparece backpressure.

## Ejercicios

### Ejercicio 1: Pipe FIFO `[Nivel 1]`

Crea un pipe de capacidad `2`, escribe dos mensajes y léelos.

**Entrada/Salida esperada:** los mensajes deben salir en el mismo orden en que
entraron.

### Ejercicio 2: Cola de mensajes `[Nivel 2]`

Crea dos endpoints, envía un mensaje de un proceso a otro y recíbelo desde el
endpoint correcto.

**Entrada/Salida esperada:** el receptor obtiene un `Envelope` con emisor,
receptor y mensaje esperados.

### Ejercicio 3: Backpressure `[Nivel 3]`

Crea una cola con capacidad `1`, envía un mensaje y luego intenta enviar otro
sin recibir primero.

**Entrada/Salida esperada:** el segundo envío debe regresar
`IpcError::Backpressure`.

### Ejercicio 4: Supervisor `[Nivel 4]`

Diseña un supervisor que reciba reportes de workers. Define qué mensajes son
datos, cuáles son control, qué ocurre si la cola está llena y cómo se cierra el
canal durante apagado ordenado.

## Soluciones

Las soluciones ejecutables de niveles 1 a 3 viven en:

- [`examples/soluciones/ipc_pipe.rs`](../examples/soluciones/ipc_pipe.rs)
- [`examples/soluciones/ipc_message_queue.rs`](../examples/soluciones/ipc_message_queue.rs)
- [`examples/soluciones/ipc_backpressure.rs`](../examples/soluciones/ipc_backpressure.rs)
