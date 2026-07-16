use rust_operating_systems::mutex::{MutexError, MutexId, MutexModel};
use rust_operating_systems::processes::ThreadId;

fn main() {
    let mut mutex = MutexModel::new(MutexId::new(2));
    let owner = ThreadId::new(10);
    let contender = ThreadId::new(11);

    mutex.lock(owner).unwrap();
    let result = mutex.lock(contender);

    match result {
        Err(MutexError::AlreadyLocked {
            owner, contender, ..
        }) => {
            println!("dueño {:?}, contendiente {:?}", owner, contender);
        }
        other => println!("resultado inesperado: {other:?}"),
    }
}
