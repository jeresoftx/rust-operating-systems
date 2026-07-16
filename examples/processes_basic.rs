use rust_operating_systems::processes::{Process, ProcessId, ProcessState};

fn main() {
    let mut process = Process::new(ProcessId::new(1), "academy-web");
    process.transition_to(ProcessState::Running).unwrap();

    println!("{} está en {:?}", process.name(), process.state());
}
