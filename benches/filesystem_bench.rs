use std::hint::black_box;
use std::time::Instant;

use rust_operating_systems::filesystem::{FileSystemModel, Permissions};

fn crear_directorios(iteraciones: usize) {
    let mut fs = FileSystemModel::new();

    for i in 0..iteraciones {
        fs.create_directory(&format!("/dir-{i}"), Permissions::new(true, true, true))
            .unwrap();
    }

    black_box(fs.root_id());
}

fn resolver_rutas(iteraciones: usize) {
    let mut fs = FileSystemModel::new();

    for i in 0..iteraciones {
        fs.create_directory(&format!("/dir-{i}"), Permissions::new(true, true, true))
            .unwrap();
        fs.create_file(
            &format!("/dir-{i}/archivo.rs"),
            Permissions::new(true, true, false),
        )
        .unwrap();
    }

    for i in 0..iteraciones {
        black_box(fs.resolve_path(&format!("/dir-{i}/archivo.rs")).unwrap());
    }
}

fn rechazar_rutas_inexistentes(iteraciones: usize) {
    let fs = FileSystemModel::new();

    for i in 0..iteraciones {
        let _ = black_box(fs.resolve_path(&format!("/faltante-{i}")));
    }
}

fn main() {
    let iteraciones = 20_000usize;

    let start = Instant::now();
    crear_directorios(iteraciones);
    let create_elapsed = start.elapsed();

    let start = Instant::now();
    resolver_rutas(iteraciones);
    let resolve_elapsed = start.elapsed();

    let start = Instant::now();
    rechazar_rutas_inexistentes(iteraciones);
    let missing_elapsed = start.elapsed();

    println!("benchmark de filesystem (manual, std::time::Instant)");
    println!("iteraciones: {iteraciones}");
    println!("crear directorios: {create_elapsed:?}");
    println!("resolver rutas: {resolve_elapsed:?}");
    println!("rechazar rutas inexistentes: {missing_elapsed:?}");
}
