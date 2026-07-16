use rust_operating_systems::processes::ThreadId;
use rust_operating_systems::semaphores::{Permit, Semaphore, SemaphoreError, SemaphoreId, Waiter};

#[test]
fn semaphore_grants_permit_when_capacity_is_available() {
    let mut semaphore = Semaphore::new(SemaphoreId::new(1), 2).unwrap();
    let owner = ThreadId::new(1);

    let permit = semaphore.acquire(owner).unwrap();

    assert_eq!(permit, Permit::new(SemaphoreId::new(1), owner));
    assert_eq!(semaphore.available_permits(), 1);
    assert_eq!(semaphore.in_use(), 1);
}

#[test]
fn semaphore_places_waiter_when_capacity_is_exhausted() {
    let mut semaphore = Semaphore::new(SemaphoreId::new(2), 1).unwrap();
    let owner = ThreadId::new(10);
    let waiter = ThreadId::new(11);

    semaphore.acquire(owner).unwrap();
    let result = semaphore.acquire(waiter).unwrap_err();

    assert_eq!(result, SemaphoreError::WouldBlock(Waiter::new(waiter)));
    assert_eq!(semaphore.waiters(), &[Waiter::new(waiter)]);
    assert_eq!(semaphore.available_permits(), 0);
}

#[test]
fn release_wakes_next_waiter_deterministically() {
    let mut semaphore = Semaphore::new(SemaphoreId::new(3), 1).unwrap();
    let owner = ThreadId::new(20);
    let first_waiter = ThreadId::new(21);
    let second_waiter = ThreadId::new(22);

    let permit = semaphore.acquire(owner).unwrap();
    semaphore.acquire(first_waiter).unwrap_err();
    semaphore.acquire(second_waiter).unwrap_err();

    let awakened = semaphore.release(permit).unwrap();

    assert_eq!(
        awakened,
        Some(Permit::new(SemaphoreId::new(3), first_waiter))
    );
    assert_eq!(semaphore.waiters(), &[Waiter::new(second_waiter)]);
    assert_eq!(semaphore.available_permits(), 0);
    assert_eq!(semaphore.in_use(), 1);
}

#[test]
fn release_without_matching_permit_is_rejected() {
    let mut semaphore = Semaphore::new(SemaphoreId::new(4), 1).unwrap();
    let foreign = Permit::new(SemaphoreId::new(99), ThreadId::new(30));

    let error = semaphore.release(foreign).unwrap_err();

    assert_eq!(
        error,
        SemaphoreError::WrongSemaphore {
            expected: SemaphoreId::new(4),
            actual: SemaphoreId::new(99),
        }
    );
}

#[test]
fn release_that_exceeds_capacity_is_rejected() {
    let mut semaphore = Semaphore::new(SemaphoreId::new(5), 1).unwrap();
    let owner = ThreadId::new(40);
    let permit = Permit::new(SemaphoreId::new(5), owner);

    let error = semaphore.release(permit).unwrap_err();

    assert_eq!(
        error,
        SemaphoreError::CapacityExceeded {
            semaphore: SemaphoreId::new(5),
            capacity: 1,
        }
    );
}

#[test]
fn zero_capacity_semaphore_is_invalid() {
    let error = Semaphore::new(SemaphoreId::new(6), 0).unwrap_err();

    assert_eq!(error, SemaphoreError::ZeroCapacity(SemaphoreId::new(6)));
}
