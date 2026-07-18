use rust_operating_systems::processes::ProcessId;
use rust_operating_systems::signals::{
    DeliveryOutcome, Signal, SignalAction, SignalMask, SignalNumber, SignalQueue,
};

fn main() {
    let mut queue = SignalQueue::new();
    let mask = SignalMask::new();
    let process = ProcessId::new(3);

    queue.set_action(
        SignalNumber::new(15),
        SignalAction::Handle("shutdown-handler".to_string()),
    );
    queue
        .enqueue(Signal::new(SignalNumber::new(15), process))
        .unwrap();

    let delivery = queue.dispatch_next(process, &mask).unwrap().unwrap();

    assert_eq!(
        delivery.outcome(),
        DeliveryOutcome::Handled {
            signal: SignalNumber::new(15),
            handler: "shutdown-handler".to_string(),
        }
    );

    println!("handler ejecutaría el apagado ordenado");
}
