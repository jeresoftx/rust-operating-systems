use rust_operating_systems::processes::{Process, ProcessId};

fn main() {
    let mut process = Process::new(ProcessId::new(2), "workers");
    process.spawn_thread().unwrap();
    process.spawn_thread().unwrap();

    assert_eq!(process.threads().len(), 3);
}
