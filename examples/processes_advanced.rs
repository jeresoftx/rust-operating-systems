use rust_operating_systems::processes::{
    Process, ProcessId, ProcessState, ProcessTable, ThreadState,
};

fn main() {
    let mut api = Process::new(ProcessId::new(10), "api");
    let background = api.spawn_thread().unwrap();
    api.transition_to(ProcessState::Running).unwrap();
    api.transition_thread(background, ThreadState::Blocked)
        .unwrap();

    let mut table = ProcessTable::new();
    table.insert(api).unwrap();

    let registered = table.get(ProcessId::new(10)).unwrap();
    println!("proceso: {}", registered.name());
    println!("hilos: {}", registered.threads().len());
}
