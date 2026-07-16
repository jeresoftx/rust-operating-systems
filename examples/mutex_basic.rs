use rust_operating_systems::mutex::{MutexId, MutexModel};
use rust_operating_systems::processes::ThreadId;

fn main() {
    let mut mutex = MutexModel::new(MutexId::new(1));
    let owner = ThreadId::new(1);

    mutex.lock(owner).unwrap();
    mutex.unlock(owner).unwrap();

    println!("estado final: {:?}", mutex.state());
}
