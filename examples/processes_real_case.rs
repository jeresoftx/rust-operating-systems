use rust_operating_systems::processes::{
    Process, ProcessId, ProcessState, ProcessTable, ThreadState,
};

fn main() {
    let mut server = Process::new(ProcessId::new(100), "servidor de clases");
    let http_worker = server.spawn_thread().unwrap();
    let jobs_worker = server.spawn_thread().unwrap();

    server.transition_to(ProcessState::Running).unwrap();
    server
        .transition_thread(http_worker, ThreadState::Running)
        .unwrap();
    server
        .transition_thread(jobs_worker, ThreadState::Blocked)
        .unwrap();

    let mut table = ProcessTable::new();
    table.insert(server).unwrap();

    let server = table.get(ProcessId::new(100)).unwrap();
    println!("{} tiene {} hilos", server.name(), server.threads().len());
}
