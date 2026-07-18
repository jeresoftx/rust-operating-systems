use rust_operating_systems::processes::ProcessId;
use rust_operating_systems::signals::{Signal, SignalNumber, SignalQueue};

fn main() {
    let mut queue = SignalQueue::new();
    let signal = Signal::new(SignalNumber::new(15), ProcessId::new(42));

    queue.enqueue(signal).unwrap();

    assert_eq!(queue.pending_for(ProcessId::new(42)), vec![signal]);
    println!("señal encolada para el proceso 42");
}
