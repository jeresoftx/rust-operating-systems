use rust_operating_systems::processes::{Process, ProcessId, ProcessState};

fn main() {
    let mut process = Process::new(ProcessId::new(1), "job");
    process.transition_to(ProcessState::Running).unwrap();

    assert_eq!(process.state(), ProcessState::Running);
}
