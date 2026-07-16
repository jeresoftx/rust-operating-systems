use rust_operating_systems::processes::{
    Process, ProcessError, ProcessId, ProcessState, ProcessTable, ThreadId, ThreadState,
};

#[test]
fn process_starts_ready_with_pid_name_and_main_thread() {
    let process = Process::new(ProcessId::new(42), "academy-web");

    assert_eq!(process.id(), ProcessId::new(42));
    assert_eq!(process.name(), "academy-web");
    assert_eq!(process.state(), ProcessState::Ready);
    assert_eq!(process.threads().len(), 1);
    assert_eq!(process.threads()[0].id(), ThreadId::new(1));
    assert_eq!(process.threads()[0].state(), ThreadState::Ready);
}

#[test]
fn process_lifecycle_allows_expected_transitions() {
    let mut process = Process::new(ProcessId::new(7), "worker");

    process.transition_to(ProcessState::Running).unwrap();
    process.transition_to(ProcessState::Blocked).unwrap();
    process.transition_to(ProcessState::Ready).unwrap();
    process.transition_to(ProcessState::Terminated).unwrap();

    assert_eq!(process.state(), ProcessState::Terminated);
}

#[test]
fn process_lifecycle_rejects_invalid_transitions() {
    let mut process = Process::new(ProcessId::new(9), "job");

    let error = process.transition_to(ProcessState::Blocked).unwrap_err();

    assert_eq!(
        error,
        ProcessError::InvalidTransition {
            from: ProcessState::Ready,
            to: ProcessState::Blocked,
        }
    );
}

#[test]
fn process_can_spawn_threads_with_independent_state() {
    let mut process = Process::new(ProcessId::new(11), "api");

    let first = process.spawn_thread().unwrap();
    let second = process.spawn_thread().unwrap();
    process
        .transition_thread(first, ThreadState::Running)
        .expect("transición de hilo válida");

    assert_eq!(first, ThreadId::new(2));
    assert_eq!(second, ThreadId::new(3));
    assert_eq!(process.thread(first).unwrap().state(), ThreadState::Running);
    assert_eq!(process.thread(second).unwrap().state(), ThreadState::Ready);
}

#[test]
fn terminated_process_cannot_spawn_threads() {
    let mut process = Process::new(ProcessId::new(13), "finished");
    process.transition_to(ProcessState::Running).unwrap();
    process.transition_to(ProcessState::Terminated).unwrap();

    let error = process.spawn_thread().unwrap_err();

    assert_eq!(error, ProcessError::ProcessTerminated(ProcessId::new(13)));
}

#[test]
fn process_table_finds_processes_and_rejects_duplicates() {
    let process = Process::new(ProcessId::new(21), "scheduler");
    let duplicate = Process::new(ProcessId::new(21), "duplicate");
    let mut table = ProcessTable::new();

    table.insert(process).unwrap();
    let error = table.insert(duplicate).unwrap_err();

    assert_eq!(table.get(ProcessId::new(21)).unwrap().name(), "scheduler");
    assert_eq!(table.len(), 1);
    assert_eq!(error, ProcessError::DuplicateProcess(ProcessId::new(21)));
}
