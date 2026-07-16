use rust_operating_systems::mutex::{MutexError, MutexId, MutexModel};
use rust_operating_systems::processes::ThreadId;

fn main() {
    let mut mutex = MutexModel::new(MutexId::new(3));
    let owner = ThreadId::new(3);

    mutex.lock(owner).unwrap();
    mutex.poison(owner).unwrap();
    assert_eq!(
        mutex.lock(owner).unwrap_err(),
        MutexError::Poisoned(MutexId::new(3))
    );

    mutex.recover().unwrap();
    mutex.lock(owner).unwrap();
}
