use rust_operating_systems::processes::ProcessId;
use rust_operating_systems::signals::{Signal, SignalMask, SignalNumber, SignalQueue};

fn main() {
    let mut queue = SignalQueue::new();
    let mut mask = SignalMask::new();
    let process = ProcessId::new(7);

    mask.block(SignalNumber::new(2));
    queue
        .enqueue(Signal::new(SignalNumber::new(2), process))
        .unwrap();

    let delivery = queue.dispatch_next(process, &mask).unwrap();

    assert!(delivery.is_none());
    println!("la señal quedó pendiente por la máscara");
}
