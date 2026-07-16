use rust_operating_systems::processes::ThreadId;
use rust_operating_systems::semaphores::{Semaphore, SemaphoreError, SemaphoreId, Waiter};

fn main() {
    let mut semaphore = Semaphore::new(SemaphoreId::new(2), 2).unwrap();
    semaphore.acquire(ThreadId::new(1)).unwrap();
    semaphore.acquire(ThreadId::new(2)).unwrap();

    assert_eq!(
        semaphore.acquire(ThreadId::new(3)).unwrap_err(),
        SemaphoreError::WouldBlock(Waiter::new(ThreadId::new(3)))
    );
}
