use std::hint::black_box;
use std::time::Instant;

use rust_operating_systems::processes::ThreadId;
use rust_operating_systems::semaphores::{Semaphore, SemaphoreId};

fn adquirir_y_liberar(iteraciones: usize) {
    for i in 0..iteraciones {
        let mut semaphore = Semaphore::new(SemaphoreId::new(i as u32), 1).unwrap();
        let permit = semaphore.acquire(ThreadId::new(1)).unwrap();
        semaphore.release(permit).unwrap();
        black_box(semaphore.available_permits());
    }
}

fn agotar_capacidad(iteraciones: usize) {
    for i in 0..iteraciones {
        let mut semaphore = Semaphore::new(SemaphoreId::new(i as u32), 2).unwrap();
        semaphore.acquire(ThreadId::new(1)).unwrap();
        semaphore.acquire(ThreadId::new(2)).unwrap();
        let waiting = semaphore.acquire(ThreadId::new(3)).is_err();
        black_box(waiting);
    }
}

fn liberar_con_espera(iteraciones: usize) {
    for i in 0..iteraciones {
        let mut semaphore = Semaphore::new(SemaphoreId::new(i as u32), 1).unwrap();
        let permit = semaphore.acquire(ThreadId::new(1)).unwrap();
        semaphore.acquire(ThreadId::new(2)).unwrap_err();
        let awakened = semaphore.release(permit).unwrap();
        black_box(awakened);
    }
}

fn main() {
    let iteraciones = 50_000usize;

    let start = Instant::now();
    adquirir_y_liberar(iteraciones);
    let simple_elapsed = start.elapsed();

    let start = Instant::now();
    agotar_capacidad(iteraciones);
    let capacidad_elapsed = start.elapsed();

    let start = Instant::now();
    liberar_con_espera(iteraciones);
    let espera_elapsed = start.elapsed();

    println!("benchmark de semáforos (manual, std::time::Instant)");
    println!("iteraciones: {iteraciones}");
    println!("adquirir y liberar: {simple_elapsed:?}");
    println!("capacidad agotada: {capacidad_elapsed:?}");
    println!("liberar con espera: {espera_elapsed:?}");
}
