use rust_operating_systems::mutex::{LockGuardModel, MutexError, MutexId, MutexModel, MutexState};
use rust_operating_systems::processes::ThreadId;

#[test]
fn free_mutex_can_be_acquired_by_thread() {
    let mut mutex = MutexModel::new(MutexId::new(1));
    let owner = ThreadId::new(7);

    let guard = mutex.lock(owner).unwrap();

    assert_eq!(guard.mutex_id(), MutexId::new(1));
    assert_eq!(guard.owner(), owner);
    assert_eq!(mutex.state(), MutexState::Locked { owner });
    assert_eq!(mutex.owner(), Some(owner));
}

#[test]
fn locked_mutex_rejects_second_owner() {
    let mut mutex = MutexModel::new(MutexId::new(2));
    let owner = ThreadId::new(1);
    let contender = ThreadId::new(2);

    mutex.lock(owner).unwrap();
    let error = mutex.lock(contender).unwrap_err();

    assert_eq!(
        error,
        MutexError::AlreadyLocked {
            mutex: MutexId::new(2),
            owner,
            contender,
        }
    );
    assert_eq!(mutex.state(), MutexState::Locked { owner });
}

#[test]
fn owner_can_unlock_mutex() {
    let mut mutex = MutexModel::new(MutexId::new(3));
    let owner = ThreadId::new(5);

    mutex.lock(owner).unwrap();
    mutex.unlock(owner).unwrap();

    assert_eq!(mutex.state(), MutexState::Free);
    assert_eq!(mutex.owner(), None);
}

#[test]
fn non_owner_cannot_unlock_mutex() {
    let mut mutex = MutexModel::new(MutexId::new(4));
    let owner = ThreadId::new(8);
    let intruder = ThreadId::new(9);

    mutex.lock(owner).unwrap();
    let error = mutex.unlock(intruder).unwrap_err();

    assert_eq!(
        error,
        MutexError::NotOwner {
            mutex: MutexId::new(4),
            owner,
            attempted: intruder,
        }
    );
    assert_eq!(mutex.state(), MutexState::Locked { owner });
}

#[test]
fn poisoning_marks_mutex_until_recovered() {
    let mut mutex = MutexModel::new(MutexId::new(5));
    let owner = ThreadId::new(11);

    mutex.lock(owner).unwrap();
    mutex.poison(owner).unwrap();
    let error = mutex.lock(owner).unwrap_err();

    assert_eq!(mutex.state(), MutexState::Poisoned);
    assert_eq!(error, MutexError::Poisoned(MutexId::new(5)));

    mutex.recover().unwrap();
    mutex.lock(owner).unwrap();
    assert_eq!(mutex.state(), MutexState::Locked { owner });
}

#[test]
fn guard_can_unlock_matching_mutex() {
    let mut mutex = MutexModel::new(MutexId::new(6));
    let owner = ThreadId::new(13);
    let guard = LockGuardModel::new(MutexId::new(6), owner);

    mutex.lock(owner).unwrap();
    guard.unlock(&mut mutex).unwrap();

    assert_eq!(mutex.state(), MutexState::Free);
}
