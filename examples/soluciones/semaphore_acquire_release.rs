use rust_operating_systems::processes::ThreadId;
use rust_operating_systems::semaphores::{Semaphore, SemaphoreId};

fn main() {
    let mut semaphore = Semaphore::new(SemaphoreId::new(1), 1).unwrap();
    let permit = semaphore.acquire(ThreadId::new(1)).unwrap();
    semaphore.release(permit).unwrap();

    assert_eq!(semaphore.available_permits(), 1);
}
