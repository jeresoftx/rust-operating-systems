use rust_operating_systems::processes::ThreadId;
use rust_operating_systems::semaphores::{Permit, Semaphore, SemaphoreId};

fn main() {
    let mut semaphore = Semaphore::new(SemaphoreId::new(3), 1).unwrap();
    let permit = semaphore.acquire(ThreadId::new(1)).unwrap();
    semaphore.acquire(ThreadId::new(2)).unwrap_err();
    semaphore.acquire(ThreadId::new(3)).unwrap_err();

    let awakened = semaphore.release(permit).unwrap();

    assert_eq!(
        awakened,
        Some(Permit::new(SemaphoreId::new(3), ThreadId::new(2)))
    );
}
