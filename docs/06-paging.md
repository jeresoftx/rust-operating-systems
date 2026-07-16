# Paging

> **Curso:** rust-operating-systems · **Capítulo:** 06 ·
> **Prerrequisitos:** memoria y scheduling
> **Código:** [`src/paging.rs`](../src/paging.rs) · **Video:** pendiente
> **Lección en el sitio:** pendiente

## Introducción

Paging divide la memoria en bloques de tamaño fijo. Del lado virtual hablamos
de páginas; del lado físico hablamos de frames. La traducción no entrega una
región completa: toma una dirección virtual, separa página y offset, consulta
la tabla de páginas y reconstruye una dirección física con frame y offset.

Este capítulo modela una tabla de páginas pequeña, permisos de lectura y
escritura, page faults y reemplazo FIFO. Todavía no construye memoria virtual
completa; prepara el vocabulario y las invariantes que el siguiente capítulo
necesita.

## Motivación

Asignar memoria como regiones continuas funciona para explicar rangos, pero se
vuelve rígido cuando un proceso crece, se mueve o necesita aislamiento. Paging
permite que una vista virtual parezca continua aunque los frames físicos estén
dispersos.

La idea central es:

```text
Paging traduce página + offset en frame + offset.
```

## Teoría

### Historia

Los sistemas con multiprogramación necesitaban aislar procesos y utilizar mejor
la memoria física. Paging introdujo una unidad fija de traducción: la página.
Eso simplificó asignación, protección, reemplazo y, más adelante, memoria
virtual con demanda.

### Fundamentos

El modelo del crate usa:

- `PageNumber`: número de página virtual;
- `FrameNumber`: número de frame físico;
- `PageSize`: tamaño de página en bytes;
- `PageTableEntry`: frame, permisos y presencia;
- `PageTable`: tabla de páginas educativa;
- `PageFault`: página y motivo del fallo;
- `PagingError`: errores de tamaño, capacidad, desbordamiento y page fault.

### Página, frame y offset

Una dirección virtual se separa así:

```text
página = dirección_virtual / tamaño_de_página
offset = dirección_virtual % tamaño_de_página
```

Si la página está mapeada al frame correcto, la dirección física se reconstruye
así:

```text
dirección_física = frame * tamaño_de_página + offset
```

El offset no cambia durante la traducción. Esa es una de las primeras
invariantes importantes de paging.

### Tabla de páginas

La tabla de páginas registra qué página virtual apunta a qué frame físico. En un
sistema real, esa estructura tiene muchos niveles, bits de hardware, TLB y
políticas de caché. En este modelo es una tabla pequeña y explícita para
entender la idea sin ruido.

### Permisos

Una entrada puede permitir lectura y escritura, o solo lectura. Si una tarea
intenta escribir en una página de solo lectura, el modelo devuelve un page fault
por violación de protección.

### Page fault

Un page fault no siempre significa "se rompió el programa". Puede significar:

- la página no está mapeada;
- la entrada existe pero no está presente;
- el acceso viola permisos.

El sistema operativo decide si puede resolver el fallo o si debe terminar la
tarea. Este capítulo solo modela la detección.

### Reemplazo FIFO

Cuando una tabla con capacidad limitada se llena, FIFO expulsa la página más
antigua. Es una política sencilla para estudiar reemplazo. No siempre es la más
eficiente, pero hace visible el costo de un working set mayor que la capacidad.

### Casos de uso

Paging aparece en:

- aislamiento entre procesos;
- memoria virtual;
- carga bajo demanda;
- protección de páginas;
- copy-on-write;
- swapping;
- administración de working sets.

### Ventajas y limitaciones

Ventajas:

- Evita exigir regiones físicas continuas.
- Separa dirección virtual y frame físico.
- Permite permisos por página.
- Hace explícito el page fault.

Limitaciones:

- Este modelo no implementa TLB.
- No maneja niveles jerárquicos de tablas.
- No resuelve page faults, solo los reporta.
- No modela bits de hardware como dirty, accessed o executable.

## Diagramas

El diagrama principal vive en
[`diagrams/06-paging.mmd`](../diagrams/06-paging.mmd). Muestra separación de
dirección virtual, consulta de tabla, validación de permisos, reconstrucción de
dirección física y reemplazo FIFO.

## Análisis de complejidad

| Operación | Mejor caso | Caso promedio | Peor caso | Espacio |
|-----------|------------|---------------|-----------|---------|
| `PageTable::new` | O(1) | O(1) | O(1) | O(1) |
| `map_page` sin reemplazo | O(log p) | O(log p) | O(log p) | O(p) |
| `map_page` con FIFO | O(log p) | O(log p) | O(log p) | O(p) |
| `translate_page` | O(log p) | O(log p) | O(log p) | O(1) |
| `translate_address` | O(log p) | O(log p) | O(log p) | O(1) |

`p` es la cantidad de páginas mapeadas.

## Visualización interactiva (opcional)

No aplica todavía. Una visualización futura podría mostrar una dirección
virtual separándose en página y offset, la consulta en la tabla y el frame
físico resultante.

## Implementación

La implementación vive en [`src/paging.rs`](../src/paging.rs). Usa `BTreeMap`
para la tabla y `VecDeque` para recordar el orden FIFO. `translate_address`
calcula página y offset, valida permisos mediante `translate_page` y reconstruye
la dirección física con chequeo de desbordamiento.

El modelo declara estas invariantes:

- el tamaño de página debe ser positivo;
- la capacidad FIFO debe ser positiva cuando se usa;
- una página mapeada apunta a una entrada presente;
- los permisos se revisan antes de entregar el frame;
- FIFO expulsa la página más antigua cuando la capacidad se llena.

## Pruebas

Las pruebas cubren:

- traducción de dirección virtual a física;
- page fault por página no mapeada;
- rechazo de escritura sobre página de solo lectura;
- reemplazo FIFO de la página más antigua.

## Benchmarks

El benchmark manual vive en
[`benches/paging_bench.rs`](../benches/paging_bench.rs). Mide traducción de
direcciones, fallos por página no mapeada y reemplazo FIFO.

## Ejercicios

### Ejercicio 1: Traducción `[Nivel 1]`

Mapea la página `2` al frame `7` con tamaño de página de `4096` bytes. Traduce
la dirección virtual `8192 + 123`.

**Entrada/Salida esperada:** la dirección física debe ser `28672 + 123`.

### Ejercicio 2: Page fault `[Nivel 2]`

Intenta traducir una página no mapeada.

**Entrada/Salida esperada:** el resultado debe ser `PageFault` con motivo
`NotMapped`.

### Ejercicio 3: Reemplazo FIFO `[Nivel 3]`

Crea una tabla con capacidad `2`, mapea tres páginas y observa cuál se expulsa.

**Entrada/Salida esperada:** debe expulsarse la primera página mapeada.

### Ejercicio 4: Working set pequeño `[Nivel 4]`

Modela una tarea cuyo working set necesita tres páginas, pero solo tiene dos
frames disponibles. Decide si FIFO es suficiente o si conviene otra política.
Justifica con localidad, fallos repetidos y costo de reemplazo.

## Soluciones

Las soluciones ejecutables de niveles 1 a 3 viven en:

- [`examples/soluciones/page_translation.rs`](../examples/soluciones/page_translation.rs)
- [`examples/soluciones/page_fault.rs`](../examples/soluciones/page_fault.rs)
- [`examples/soluciones/page_replacement.rs`](../examples/soluciones/page_replacement.rs)
