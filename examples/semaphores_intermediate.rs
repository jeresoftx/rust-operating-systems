use rust_operating_systems::processes::ThreadId;
use rust_operating_systems::semaphores::{Semaphore, SemaphoreId};

fn main() {
    let mut semaphore = Semaphore::new(SemaphoreId::new(2), 2).unwrap();
    semaphore.acquire(ThreadId::new(1)).unwrap();
    semaphore.acquire(ThreadId::new(2)).unwrap();

    let waiting = semaphore.acquire(ThreadId::new(3)).unwrap_err();

    println!("sin capacidad inmediata: {waiting:?}");
    println!("esperando: {}", semaphore.waiters().len());
}
