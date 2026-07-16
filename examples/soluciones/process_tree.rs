use rust_operating_systems::processes::{Process, ProcessId, ProcessTable};

fn main() {
    let mut table = ProcessTable::new();
    table
        .insert(Process::new(ProcessId::new(1), "init"))
        .unwrap();
    table
        .insert(Process::new(ProcessId::new(2), "shell"))
        .unwrap();

    assert_eq!(table.get(ProcessId::new(1)).unwrap().name(), "init");
    assert_eq!(table.get(ProcessId::new(2)).unwrap().name(), "shell");
}
