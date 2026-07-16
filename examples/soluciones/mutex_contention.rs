use rust_operating_systems::mutex::{MutexError, MutexId, MutexModel};
use rust_operating_systems::processes::ThreadId;

fn main() {
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
}
