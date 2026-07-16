use rust_operating_systems::mutex::{MutexId, MutexModel};
use rust_operating_systems::processes::ThreadId;

fn main() {
    let mut counter_guard = MutexModel::new(MutexId::new(100));
    let worker_a = ThreadId::new(1);
    let worker_b = ThreadId::new(2);
    let mut counter = 0;

    let guard = counter_guard.lock(worker_a).unwrap();
    counter += 1;
    guard.unlock(&mut counter_guard).unwrap();

    counter_guard.lock(worker_b).unwrap();
    counter += 1;

    println!("contador protegido: {counter}");
}
