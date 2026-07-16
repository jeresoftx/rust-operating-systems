# Memoria

> **Curso:** rust-operating-systems · **Capítulo:** 04 ·
> **Prerrequisitos:** procesos, hilos, mutex y semáforos
> **Código:** [`src/memory.rs`](../src/memory.rs) · **Video:** pendiente
> **Lección en el sitio:** pendiente

## Introducción

La memoria es el lugar donde un sistema operativo convierte direcciones,
tamaños y permisos en una promesa concreta: cada proceso cree tener espacio
para trabajar, aunque la máquina física sea finita.

Este capítulo empieza con un modelo pequeño: una región continua, asignaciones
first-fit, liberación de bloques y fragmentación externa. Todavía no modela
páginas, tablas de páginas ni memoria virtual. Antes de ocultar la complejidad
detrás de abstracciones más grandes, necesitamos ver el suelo.

## Motivación

Cuando un programa pide memoria, no recibe "un poco de RAM" en abstracto.
Recibe una zona válida, con inicio, tamaño y reglas. Si el sistema entrega
zonas duplicadas, olvida liberar espacio o no puede reutilizar huecos, el resto
de la arquitectura se vuelve frágil.

La idea central es:

```text
Administrar memoria es conservar invariantes sobre rangos.
```

## Teoría

### Historia

Los primeros sistemas cargaban programas en regiones relativamente simples. Al
crecer la multiprogramación, la administración de memoria necesitó separar
procesos, reubicar bloques, compactar huecos y, más adelante, virtualizar
direcciones. Este capítulo se queda en el primer nivel: regiones continuas y
asignación explícita.

### Fundamentos

El modelo del crate usa:

- `Address`: dirección educativa;
- `Bytes`: tamaño en bytes;
- `MemoryRegion`: rango continuo `[inicio, fin)`;
- `Allocation`: bloque entregado;
- `AllocatorModel`: asignador first-fit;
- `MemoryError`: errores de rango, capacidad y liberación.

### Direcciones

Una dirección por sí sola no basta. El sistema siempre necesita saber si una
dirección pertenece a una región válida. Por eso `MemoryRegion` expone
`contains` y calcula `end_exclusive`. La notación de fin exclusivo evita
confusiones: una región que empieza en `1000` y mide `256` bytes contiene hasta
`1255`; `1256` ya está fuera.

### Regiones

Una región válida tiene tamaño positivo y no puede desbordar el espacio de
direcciones. Si `inicio + tamaño` excede `u64::MAX`, el modelo rechaza la
región con `AddressOverflow`.

### Asignación first-fit

First-fit recorre los huecos libres y usa el primero que alcance. Es una
política fácil de explicar y probar: no busca el hueco perfecto, busca el
primer hueco suficiente. Si sobra espacio, el hueco se divide.

### Liberación y coalescing

Liberar un bloque lo devuelve a la lista de huecos. Después, el asignador ordena
los huecos y fusiona regiones contiguas. Esa fusión evita que la memoria libre
se quede artificialmente partida cuando dos bloques vecinos ya están libres.

### Fragmentación interna y externa

La fragmentación interna aparece cuando el sistema entrega más espacio del que
el solicitante puede usar, por ejemplo por alineación o tamaños mínimos de
bloque. Este modelo no agrega padding, así que no la mide todavía.

La fragmentación externa aparece cuando sí hay memoria libre total, pero está
partida en huecos pequeños. El modelo la estima como:

```text
memoria libre total - hueco libre más grande
```

Esa métrica no lo dice todo, pero ayuda a ver cuándo la forma de los huecos se
vuelve tan importante como la cantidad de bytes libres.

### Casos de uso

Este modelo prepara el terreno para:

- arenas de memoria;
- allocators de kernel;
- regiones por proceso;
- heap educativo;
- compactación;
- paging;
- memoria virtual.

### Ventajas y limitaciones

Ventajas:

- Hace visibles los rangos y sus invariantes.
- Permite probar asignación, liberación y fragmentación sin `unsafe`.
- Explica por qué liberar no basta: también hay que fusionar huecos.

Limitaciones:

- No modela alineación, permisos ni protección.
- No representa páginas físicas o virtuales.
- No compacta memoria ni mueve asignaciones vivas.
- No es un allocator real para producción.

## Diagramas

El diagrama principal vive en
[`diagrams/04-memoria.mmd`](../diagrams/04-memoria.mmd). Muestra una región,
asignaciones, huecos libres, liberación, coalescing y fragmentación externa.

## Análisis de complejidad

| Operación | Mejor caso | Caso promedio | Peor caso | Espacio |
|-----------|------------|---------------|-----------|---------|
| `MemoryRegion::new` | O(1) | O(1) | O(1) | O(1) |
| `allocate` | O(1) | O(h) | O(h) | O(a) |
| `free` | O(h log h) | O(h log h) | O(h log h) | O(h) |
| `free_bytes` | O(h) | O(h) | O(h) | O(1) |
| `largest_free_block` | O(h) | O(h) | O(h) | O(1) |

`h` es la cantidad de huecos libres y `a` la cantidad de asignaciones vivas.

## Visualización interactiva (opcional)

No aplica todavía. Una visualización futura podría mostrar una barra de memoria
que cambie con asignaciones, liberaciones, coalescing y fragmentación.

## Implementación

La implementación vive en [`src/memory.rs`](../src/memory.rs). El asignador
guarda una lista de huecos libres y una lista de asignaciones vivas. `allocate`
usa first-fit; `free` valida que el bloque exista, lo devuelve a la lista libre
y fusiona huecos contiguos.

El código evita `unsafe` deliberadamente. La intención no es administrar RAM
real, sino enseñar las reglas que cualquier administrador real debe respetar.

## Pruebas

Las pruebas cubren:

- creación de regiones y cálculo de fin exclusivo;
- rechazo de regiones de tamaño cero;
- asignación first-fit;
- liberación y reutilización de espacio;
- coalescing de huecos contiguos;
- medición de fragmentación externa;
- rechazo de asignaciones que no caben.

## Benchmarks

El benchmark manual vive en
[`benches/memory_bench.rs`](../benches/memory_bench.rs). Mide asignación simple,
liberación con coalescing y consulta de fragmentación.

## Ejercicios

### Ejercicio 1: Región `[Nivel 1]`

Crea una región que empiece en `4096` y mida `1024` bytes. Verifica su inicio,
tamaño y fin exclusivo.

**Entrada/Salida esperada:** el fin exclusivo debe ser `5120`.

### Ejercicio 2: Asignación `[Nivel 2]`

Crea un asignador first-fit sobre una región de `1024` bytes y reserva bloques
de `128` y `256` bytes.

**Entrada/Salida esperada:** el segundo bloque debe iniciar después del primero.

### Ejercicio 3: Fragmentación `[Nivel 3]`

Llena una región con varios bloques, libera dos bloques no contiguos y calcula
la fragmentación externa.

**Entrada/Salida esperada:** la memoria libre total debe ser mayor que el hueco
libre más grande.

### Ejercicio 4: Arena de solicitudes `[Nivel 4]`

Diseña una arena educativa para solicitudes HTTP. Decide qué hacerías cuando no
hay un hueco suficientemente grande: rechazar la solicitud, compactar, esperar o
crear otra región. Justifica el tradeoff operativo.

## Soluciones

Las soluciones ejecutables de niveles 1 a 3 viven en:

- [`examples/soluciones/memory_region.rs`](../examples/soluciones/memory_region.rs)
- [`examples/soluciones/memory_allocation.rs`](../examples/soluciones/memory_allocation.rs)
- [`examples/soluciones/memory_fragmentation.rs`](../examples/soluciones/memory_fragmentation.rs)
