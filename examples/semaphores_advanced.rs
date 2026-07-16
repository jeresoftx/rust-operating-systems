use rust_operating_systems::processes::ThreadId;
use rust_operating_systems::semaphores::{Permit, Semaphore, SemaphoreId};

fn main() {
    let mut semaphore = Semaphore::new(SemaphoreId::new(3), 1).unwrap();
    let permit = semaphore.acquire(ThreadId::new(1)).unwrap();
    semaphore.acquire(ThreadId::new(2)).unwrap_err();

    let awakened: Option<Permit> = semaphore.release(permit).unwrap();

    println!("permiso transferido: {awakened:?}");
    println!("en uso: {}", semaphore.in_use());
}
