# Memoria Virtual

> **Curso:** rust-operating-systems · **Capítulo:** 07 ·
> **Prerrequisitos:** memoria, paging y scheduling
> **Código:** [`src/virtual_memory.rs`](../src/virtual_memory.rs) · **Video:** pendiente
> **Lección en el sitio:** pendiente

## Introducción

La memoria virtual permite que cada proceso vea su propio espacio de
direcciones. Dos procesos pueden usar la misma dirección virtual y, aun así,
apuntar a frames físicos distintos. Esa separación es una de las bases del
aislamiento moderno.

Este capítulo modela espacios de direcciones, mapeos virtual-físicos, permisos,
direcciones no mapeadas y un copy-on-write educativo. No reemplaza al capítulo
de paging; lo usa como vocabulario para hablar de procesos y protección.

## Motivación

Sin memoria virtual, cada programa tendría que conocer demasiado sobre la
memoria física disponible. Además, aislar procesos sería mucho más difícil:
una dirección equivocada podría tocar datos de otro proceso.

La idea central es:

```text
La misma dirección virtual solo tiene sentido dentro de un espacio de direcciones.
```

## Teoría

### Historia

La memoria virtual permitió combinar aislamiento, carga bajo demanda,
protección, swapping y abstracciones como fork. Su valor no está solo en
"tener más memoria", sino en separar la vista del proceso de la memoria física
real.

### Fundamentos

El modelo del crate usa:

- `VirtualAddress`: dirección vista por un proceso;
- `PhysicalAddress`: dirección física educativa;
- `AddressSpaceId`: identidad de un espacio de direcciones;
- `AddressSpace`: tabla de mapeos de un proceso conceptual;
- `Mapping`: relación entre página virtual y página física;
- `VirtualMemoryError`: errores de alineación, aislamiento, permisos y COW.

### Aislamiento

Una dirección virtual como `0x4000` no significa lo mismo en todos los procesos.
El espacio de direcciones decide su traducción. En este modelo, dos
`AddressSpace` distintos pueden mapear `0x4000` a direcciones físicas
diferentes sin conflicto.

### Traducción

La traducción conserva el offset dentro de la página:

```text
física = inicio_físico_del_mapeo + offset_virtual
```

El mapeo debe estar alineado al tamaño de página. Eso evita que una página
virtual empiece a mitad de una página física y simplifica las invariantes que
vienen de paging.

### Permisos

Los permisos siguen siendo relevantes en memoria virtual. Un mapeo de solo
lectura puede traducirse para lectura, pero no para escritura. La protección no
es un detalle cosmético: es parte del contrato de aislamiento.

### Page tables y TLB conceptual

En sistemas reales, la traducción se apoya en page tables y se acelera con TLB
para no consultar estructuras grandes en cada acceso. Este modelo no implementa
TLB; solo documenta su lugar conceptual: cachear traducciones recientes.

### Copy-on-write

Copy-on-write permite que un padre y un hijo compartan páginas después de un
fork conceptual. Mientras ambos leen, comparten el frame. Si alguno escribe, el
sistema debe crear una copia privada. Este modelo reporta el fault de
copy-on-write y mantiene un contador de referencias educativo.

### Casos de uso

Memoria virtual aparece en:

- aislamiento por proceso;
- fork;
- copy-on-write;
- carga bajo demanda;
- protección de regiones;
- mmap;
- sandboxes;
- ejecución de procesos con la misma vista virtual.

### Ventajas y limitaciones

Ventajas:

- Separa direcciones virtuales de direcciones físicas.
- Permite aislar procesos con la misma dirección virtual.
- Hace explícitos permisos y direcciones no mapeadas.
- Introduce copy-on-write sin `unsafe`.

Limitaciones:

- No implementa page tables jerárquicas.
- No resuelve faults de copy-on-write creando frames reales.
- No modela TLB, swapping ni mmap real.
- El contador de referencias es educativo, no un contador compartido de kernel.

## Diagramas

El diagrama principal vive en
[`diagrams/07-memoria-virtual.mmd`](../diagrams/07-memoria-virtual.mmd). Muestra
dos espacios de direcciones, traducción aislada, permisos y fork conceptual con
copy-on-write.

## Análisis de complejidad

| Operación | Mejor caso | Caso promedio | Peor caso | Espacio |
|-----------|------------|---------------|-----------|---------|
| `AddressSpace::new` | O(1) | O(1) | O(1) | O(1) |
| `map` | O(log m) | O(log m) | O(log m) | O(m) |
| `translate` | O(log m) | O(log m) | O(log m) | O(1) |
| `fork_copy_on_write` | O(m log m) | O(m log m) | O(m log m) | O(m) |
| `reference_count` | O(log m) | O(log m) | O(log m) | O(1) |

`m` es la cantidad de mapeos del espacio de direcciones.

## Visualización interactiva (opcional)

No aplica todavía. Una visualización futura podría mostrar dos procesos con la
misma dirección virtual apuntando a frames distintos y un fork que comparte
páginas hasta que aparece una escritura.

## Implementación

La implementación vive en [`src/virtual_memory.rs`](../src/virtual_memory.rs).
`AddressSpace` guarda mapeos alineados en un `BTreeMap`. `translate` calcula el
offset, busca el mapeo del espacio actual, revisa permisos y devuelve una
`PhysicalAddress`.

El modelo declara estas invariantes:

- los mapeos deben estar alineados al tamaño de página;
- no puede haber dos mapeos para la misma página virtual en un espacio;
- una dirección no mapeada se rechaza con `UnmappedAddress`;
- una escritura sobre copy-on-write produce `CopyOnWriteFault`;
- cada espacio traduce de forma aislada.

## Pruebas

Las pruebas cubren:

- traducción virtual-física;
- aislamiento entre dos espacios de direcciones;
- rechazo de direcciones no mapeadas;
- fork conceptual con copy-on-write y contador de referencias.

## Benchmarks

El benchmark manual vive en
[`benches/virtual_memory_bench.rs`](../benches/virtual_memory_bench.rs). Mide
traducción virtual-física, fallos por dirección no mapeada y fork
copy-on-write.

## Ejercicios

### Ejercicio 1: Traducción `[Nivel 1]`

Mapea `0x2000` virtual a `0x9000` físico con páginas de `4096` bytes. Traduce
`0x2123`.

**Entrada/Salida esperada:** la dirección física debe ser `0x9123`.

### Ejercicio 2: Aislamiento `[Nivel 2]`

Crea dos espacios de direcciones que usen la misma dirección virtual, pero
frames físicos distintos.

**Entrada/Salida esperada:** cada espacio debe traducir a su propio frame.

### Ejercicio 3: Copy-on-write `[Nivel 3]`

Crea un espacio padre, haz fork copy-on-write y observa el contador de
referencias.

**Entrada/Salida esperada:** padre e hijo deben reportar dos referencias, y una
escritura en el hijo debe devolver `CopyOnWriteFault`.

### Ejercicio 4: Fork conceptual `[Nivel 4]`

Diseña qué tendría que pasar para resolver un `CopyOnWriteFault`: reservar un
frame nuevo, copiar contenido, ajustar permisos y disminuir referencias.
Justifica qué errores podrían ocurrir.

## Soluciones

Las soluciones ejecutables de niveles 1 a 3 viven en:

- [`examples/soluciones/virtual_address_translation.rs`](../examples/soluciones/virtual_address_translation.rs)
- [`examples/soluciones/address_space_isolation.rs`](../examples/soluciones/address_space_isolation.rs)
- [`examples/soluciones/copy_on_write_model.rs`](../examples/soluciones/copy_on_write_model.rs)
