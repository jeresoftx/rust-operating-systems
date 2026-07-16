use std::hint::black_box;
use std::time::Instant;

use rust_operating_systems::scheduling::{Priority, Scheduler, SchedulingPolicy, Task, TaskId};

fn tarea(id: u32, priority: u8, required_ticks: u32) -> Task {
    Task::new(
        TaskId::new(id),
        format!("tarea-{id}"),
        Priority::new(priority),
        required_ticks,
    )
    .unwrap()
}

fn seleccionar_round_robin(iteraciones: usize) {
    for _ in 0..iteraciones {
        let mut scheduler = Scheduler::new(SchedulingPolicy::RoundRobin, 4).unwrap();
        scheduler.add_task(tarea(1, 1, 10)).unwrap();
        scheduler.add_task(tarea(2, 1, 10)).unwrap();
        scheduler.add_task(tarea(3, 1, 10)).unwrap();
        black_box(scheduler.dispatch_next().unwrap());
    }
}

fn seleccionar_prioridad(iteraciones: usize) {
    for _ in 0..iteraciones {
        let mut scheduler = Scheduler::new(SchedulingPolicy::Priority, 4).unwrap();
        scheduler.add_task(tarea(1, 1, 10)).unwrap();
        scheduler.add_task(tarea(2, 10, 10)).unwrap();
        scheduler.add_task(tarea(3, 5, 10)).unwrap();
        black_box(scheduler.dispatch_next().unwrap());
    }
}

fn tick_con_quantum(iteraciones: usize) {
    for _ in 0..iteraciones {
        let mut scheduler = Scheduler::new(SchedulingPolicy::RoundRobin, 2).unwrap();
        scheduler.add_task(tarea(1, 1, 10)).unwrap();
        scheduler.add_task(tarea(2, 1, 10)).unwrap();
        scheduler.dispatch_next().unwrap();
        black_box(scheduler.tick().unwrap());
        black_box(scheduler.tick().unwrap());
    }
}

fn main() {
    let iteraciones = 50_000usize;

    let start = Instant::now();
    seleccionar_round_robin(iteraciones);
    let round_robin_elapsed = start.elapsed();

    let start = Instant::now();
    seleccionar_prioridad(iteraciones);
    let prioridad_elapsed = start.elapsed();

    let start = Instant::now();
    tick_con_quantum(iteraciones);
    let quantum_elapsed = start.elapsed();

    println!("benchmark de scheduling (manual, std::time::Instant)");
    println!("iteraciones: {iteraciones}");
    println!("selección round-robin: {round_robin_elapsed:?}");
    println!("selección por prioridad: {prioridad_elapsed:?}");
    println!("tick con quantum: {quantum_elapsed:?}");
}
