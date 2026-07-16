use std::hint::black_box;
use std::time::Instant;

use rust_operating_systems::mutex::{MutexId, MutexModel};
use rust_operating_systems::processes::ThreadId;

fn adquirir_y_liberar(iteraciones: usize) {
    let owner = ThreadId::new(1);

    for i in 0..iteraciones {
        let mut mutex = MutexModel::new(MutexId::new(i as u32));
        mutex.lock(owner).unwrap();
        mutex.unlock(owner).unwrap();
        black_box(mutex.state());
    }
}

fn medir_contencion(iteraciones: usize) {
    let owner = ThreadId::new(1);
    let contender = ThreadId::new(2);

    for i in 0..iteraciones {
        let mut mutex = MutexModel::new(MutexId::new(i as u32));
        mutex.lock(owner).unwrap();
        let contested = mutex.lock(contender).is_err();
        black_box(contested);
    }
}

fn poison_y_recuperar(iteraciones: usize) {
    let owner = ThreadId::new(1);

    for i in 0..iteraciones {
        let mut mutex = MutexModel::new(MutexId::new(i as u32));
        mutex.lock(owner).unwrap();
        mutex.poison(owner).unwrap();
        mutex.recover().unwrap();
        black_box(mutex.state());
    }
}

fn main() {
    let iteraciones = 50_000usize;

    let start = Instant::now();
    adquirir_y_liberar(iteraciones);
    let lock_elapsed = start.elapsed();

    let start = Instant::now();
    medir_contencion(iteraciones);
    let contencion_elapsed = start.elapsed();

    let start = Instant::now();
    poison_y_recuperar(iteraciones);
    let poison_elapsed = start.elapsed();

    println!("benchmark de mutex (manual, std::time::Instant)");
    println!("iteraciones: {iteraciones}");
    println!("adquirir y liberar: {lock_elapsed:?}");
    println!("contención educativa: {contencion_elapsed:?}");
    println!("poison y recuperación: {poison_elapsed:?}");
}
