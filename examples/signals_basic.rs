use rust_operating_systems::processes::ProcessId;
use rust_operating_systems::signals::{Signal, SignalNumber, SignalQueue};

fn main() {
    let mut queue = SignalQueue::new();
    let process = ProcessId::new(42);

    queue
        .enqueue(Signal::new(SignalNumber::new(15), process))
        .unwrap();

    println!("señales pendientes: {}", queue.pending_for(process).len());
}
