use rust_operating_systems::processes::ProcessId;
use rust_operating_systems::signals::{
    Signal, SignalAction, SignalMask, SignalNumber, SignalQueue,
};

fn main() {
    let mut queue = SignalQueue::new();
    let mut mask = SignalMask::new();
    let service = ProcessId::new(100);

    queue.set_action(
        SignalNumber::new(15),
        SignalAction::Handle("graceful-shutdown".to_string()),
    );
    mask.block(SignalNumber::new(2));

    queue
        .enqueue(Signal::new(SignalNumber::new(2), service))
        .unwrap();
    queue
        .enqueue(Signal::new(SignalNumber::new(15), service))
        .unwrap();

    let delivery = queue.dispatch_next(service, &mask).unwrap().unwrap();

    println!("apagado ordenado: {:?}", delivery.outcome());
    println!("pendientes restantes: {}", queue.pending_for(service).len());
}
