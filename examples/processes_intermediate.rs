use rust_operating_systems::processes::{Process, ProcessId, ThreadState};

fn main() {
    let mut process = Process::new(ProcessId::new(2), "worker-pool");
    let worker = process.spawn_thread().unwrap();

    process
        .transition_thread(worker, ThreadState::Running)
        .unwrap();

    println!("hilos registrados: {}", process.threads().len());
    println!(
        "worker {:?}: {:?}",
        worker,
        process.thread(worker).unwrap().state()
    );
}
