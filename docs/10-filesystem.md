# Filesystem

> **Curso:** rust-operating-systems · **Capítulo:** 10 ·
> **Prerrequisitos:** memoria, procesos, permisos básicos y IPC
> **Código:** [`src/filesystem.rs`](../src/filesystem.rs) · **Video:** pendiente
> **Lección en el sitio:** pendiente

## Introducción

Un filesystem organiza datos persistentes como nombres, directorios, archivos y
metadata. Para el usuario parece una jerarquía de rutas; para el sistema
operativo es un conjunto de estructuras que deben sostener identidad,
permisos, ubicación, tamaño y consistencia ante fallas.

Este capítulo modela un árbol educativo de directorios e inodos. No escribe en
disco real y no implementa journaling real. Su objetivo es enseñar la
representación mínima: un nombre visible dentro de un directorio apunta a un
inodo, y ese inodo declara qué tipo de objeto es y qué permisos tiene.

## Motivación

Sin filesystem, cada programa tendría que inventar su propia forma de nombrar,
guardar y encontrar datos. Con filesystem, el sistema ofrece una abstracción
común:

```text
Una ruta humana se resuelve a una identidad estable del sistema.
```

Esa identidad suele ser un inodo o una estructura equivalente. Separar nombre e
identidad permite renombrar, mover, enlazar y validar sin convertir el nombre
en la única fuente de verdad.

## Teoría

### Historia

Los filesystems evolucionaron desde tablas simples de archivos hacia árboles,
permisos, journaling, snapshots, cuotas y metadatos ricos. Unix hizo famosa la
idea de tratar muchas cosas como archivos: documentos, dispositivos, pipes y
algunas interfaces del kernel.

Este capítulo conserva una versión pequeña de esa idea: un árbol con raíz,
directorios, archivos y permisos. La intención es preparar terreno para cursos
posteriores de bases de datos, sistemas distribuidos, seguridad y performance.

### Fundamentos

El modelo del crate usa:

- `InodeId`: identidad estable dentro del filesystem;
- `FileType`: archivo o directorio;
- `Permissions`: bits educativos de lectura, escritura y ejecución;
- `DirectoryEntry`: nombre dentro de un directorio e inodo destino;
- `Inode`: metadata mínima y entradas hijas si es directorio;
- `FileSystemModel`: árbol completo con raíz y asignación de inodos;
- `FileSystemError`: errores de ruta, nombre, tipo, duplicado y ciclo.

### Inodos

Un inodo representa identidad y metadata. En un sistema real puede contener
tamaño, permisos, dueño, timestamps y punteros a bloques. En este modelo guarda
tipo, permisos, tamaño educativo e hijos cuando es directorio.

La idea importante es que el nombre no es el archivo. El nombre es una entrada
de directorio que apunta a un inodo.

### Directorios

Un directorio es una tabla de nombres. Cada nombre apunta a otro inodo. Por eso
resolver `/src/lib.rs` significa caminar desde la raíz: buscar `src`, verificar
que sea directorio y luego buscar `lib.rs`.

El modelo usa un `BTreeMap` para que las entradas tengan orden determinista, lo
cual ayuda a pruebas, ejemplos y lectura pedagógica.

### Rutas

Este capítulo acepta rutas absolutas. Una ruta absoluta inicia con `/` y se
resuelve desde la raíz. El modelo rechaza nombres vacíos, `.`, `..`, nombres
con `/` y nombres con byte nulo.

Esa restricción no pretende agotar todos los casos reales; existe para mostrar
que un filesystem necesita reglas de nombres antes de aceptar cambios al árbol.

### Permisos

`Permissions` modela lectura, escritura y ejecución con tres bits booleanos. En
un sistema real habría dueños, grupos, ACLs, capacidades y políticas de montaje.
Aquí la meta es que el estudiante pueda razonar sobre los tres permisos base
sin ocultarlos detrás de constantes numéricas.

### Metadata

La metadata da contexto sobre un archivo: tipo, permisos, tamaño, dueño,
tiempos, enlaces y ubicación física. Este modelo solo incluye la metadata que
necesita el capítulo, pero nombra el concepto porque en sistemas reales los
errores de metadata pueden ser tan graves como los errores de datos.

### Journaling conceptual

Journaling es una técnica para registrar intención antes de aplicar cambios
críticos. Si el sistema se apaga a mitad de una operación, el journal ayuda a
recuperar un estado consistente.

Este crate no implementa journaling. Lo menciona para fijar una pregunta de
ingeniería: ¿qué pasa si la máquina falla después de crear el inodo, pero antes
de escribir la entrada del directorio?

### Modos de falla

Un filesystem debe decidir qué hacer cuando:

- una ruta no existe;
- un componente intermedio no es directorio;
- un nombre es inválido;
- una entrada ya existe;
- falta un inodo que una entrada referencia;
- una operación crearía un ciclo de directorios;
- los permisos no permiten la operación solicitada.

### Casos de uso

Filesystem aparece en:

- carga de configuración;
- resolución de rutas de proyecto;
- logs y artefactos de build;
- bases de datos embebidas;
- servidores web que publican archivos estáticos;
- sandboxes que limitan acceso por directorio;
- herramientas de desarrollo que recorren árboles.

### Ventajas y limitaciones

Ventajas:

- Separa nombres visibles e identidad interna.
- Hace explícito el recorrido de rutas absolutas.
- Declara permisos como datos verificables.
- Rechaza ciclos de directorios para preservar un árbol.
- Permite medir resolución de rutas.

Limitaciones:

- No escribe bloques en disco.
- No implementa journaling real.
- No modela usuarios, grupos ni ACLs.
- No tiene enlaces duros a archivos ni enlaces simbólicos.
- No simula caché de directorios ni buffer cache.

## Diagramas

El diagrama principal vive en
[`diagrams/10-filesystem.mmd`](../diagrams/10-filesystem.mmd). Muestra la raíz,
entradas de directorio, inodos y resolución de una ruta simple.

## Análisis de complejidad

| Operación | Mejor caso | Caso promedio | Peor caso | Espacio |
|-----------|------------|---------------|-----------|---------|
| `FileSystemModel::new` | O(1) | O(1) | O(1) | O(1) |
| `create_file` | O(d log e) | O(d log e) | O(d log e) | O(n) |
| `create_directory` | O(d log e) | O(d log e) | O(d log e) | O(n) |
| `resolve_path` | O(1) | O(d log e) | O(d log e) | O(1) |
| `link_directory` | O(d) | O(d + s) | O(d + s) | O(1) |

`d` es la profundidad de la ruta, `e` es la cantidad de entradas por directorio
y `s` es el tamaño del subárbol revisado para evitar ciclos.

## Visualización interactiva (opcional)

No aplica todavía. Una visualización futura podría mostrar el árbol, seleccionar
una ruta, resaltar cada entrada visitada y enseñar dónde falla la resolución.

## Implementación

La implementación vive en [`src/filesystem.rs`](../src/filesystem.rs). Usa
`BTreeMap` para entradas e inodos porque el orden determinista simplifica el
aprendizaje y las pruebas.

El modelo declara estas invariantes:

- la raíz existe y siempre es directorio;
- los inodos tienen identidad estable;
- las entradas de directorio tienen nombres validados;
- las rutas públicas son absolutas;
- un componente intermedio debe ser directorio;
- no se aceptan ciclos de directorios;
- permisos se expresan como lectura, escritura y ejecución.

## Pruebas

Las pruebas cubren:

- creación de raíz y archivo;
- resolución de rutas absolutas simples;
- permisos de lectura, escritura y ejecución;
- rechazo de nombres inválidos y ciclos de directorio.

## Benchmarks

El benchmark manual vive en
[`benches/filesystem_bench.rs`](../benches/filesystem_bench.rs). Mide creación
de directorios, resolución de rutas y rechazo de rutas inexistentes.

## Ejercicios

### Ejercicio 1: Inodo `[Nivel 1]`

Crea un archivo `/README.md` y verifica que su inodo sea de tipo `File`.

**Entrada/Salida esperada:** resolver `/README.md` debe regresar el mismo
`InodeId` creado.

### Ejercicio 2: Directorio `[Nivel 2]`

Crea `/src` y `/src/lib.rs`. Luego resuelve ambas rutas.

**Entrada/Salida esperada:** `/src` debe ser directorio y `/src/lib.rs` debe ser
archivo.

### Ejercicio 3: Permisos `[Nivel 3]`

Construye permisos de solo lectura y permisos ejecutables.

**Entrada/Salida esperada:** `can_read`, `can_write` y `can_execute` deben
reflejar exactamente los bits configurados.

### Ejercicio 4: Workspace `[Nivel 4]`

Diseña un árbol de proyecto con `src`, `tests`, `target` y archivos de
configuración. Decide qué permisos tendría cada directorio y qué operaciones
debería rechazar el modelo.

## Soluciones

Las soluciones ejecutables de niveles 1 a 3 viven en:

- [`examples/soluciones/filesystem_inode.rs`](../examples/soluciones/filesystem_inode.rs)
- [`examples/soluciones/filesystem_directory.rs`](../examples/soluciones/filesystem_directory.rs)
- [`examples/soluciones/filesystem_permissions.rs`](../examples/soluciones/filesystem_permissions.rs)
