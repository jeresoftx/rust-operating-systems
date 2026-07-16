use rust_operating_systems::processes::ThreadId;
use rust_operating_systems::semaphores::{Semaphore, SemaphoreId};

fn main() {
    let mut pool = Semaphore::new(SemaphoreId::new(100), 2).unwrap();
    let api_a = pool.acquire(ThreadId::new(1)).unwrap();
    let api_b = pool.acquire(ThreadId::new(2)).unwrap();

    pool.acquire(ThreadId::new(3)).unwrap_err();
    pool.release(api_a).unwrap();

    println!("conexiones ocupadas: {}", pool.in_use());
    println!("solicitantes esperando: {}", pool.waiters().len());

    pool.release(api_b).unwrap();
}
