use std::hint::black_box;
use std::time::Instant;

use rust_operating_systems::processes::{Process, ProcessId, ProcessState, ProcessTable};

fn registrar_y_buscar_procesos(iteraciones: usize) {
    let mut table = ProcessTable::new();

    for i in 0..iteraciones {
        table
            .insert(Process::new(
                ProcessId::new(i as u32),
                format!("proceso-{i}"),
            ))
            .unwrap();
    }

    for i in 0..iteraciones {
        let process = table.get(ProcessId::new(i as u32)).unwrap();
        black_box(process.name());
    }
}

fn transicionar_estados(iteraciones: usize) {
    for i in 0..iteraciones {
        let mut process = Process::new(ProcessId::new(i as u32), "worker");
        process.transition_to(ProcessState::Running).unwrap();
        process.transition_to(ProcessState::Blocked).unwrap();
        process.transition_to(ProcessState::Ready).unwrap();
        process.transition_to(ProcessState::Terminated).unwrap();
        black_box(process.state());
    }
}

fn crear_hilos(iteraciones: usize) {
    for i in 0..iteraciones {
        let mut process = Process::new(ProcessId::new(i as u32), "pool");
        for _ in 0..4 {
            process.spawn_thread().unwrap();
        }
        black_box(process.threads().len());
    }
}

fn main() {
    let iteraciones = 20_000usize;

    let start = Instant::now();
    registrar_y_buscar_procesos(iteraciones);
    let tabla_elapsed = start.elapsed();

    let start = Instant::now();
    transicionar_estados(iteraciones);
    let estados_elapsed = start.elapsed();

    let start = Instant::now();
    crear_hilos(iteraciones);
    let hilos_elapsed = start.elapsed();

    println!("benchmark de procesos (manual, std::time::Instant)");
    println!("iteraciones: {iteraciones}");
    println!("registro y búsqueda: {tabla_elapsed:?}");
    println!("transiciones de estado: {estados_elapsed:?}");
    println!("creación de hilos: {hilos_elapsed:?}");
}
