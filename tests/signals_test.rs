use rust_operating_systems::processes::ProcessId;
use rust_operating_systems::signals::{
    DeliveryOutcome, Signal, SignalAction, SignalMask, SignalNumber, SignalQueue,
};

#[test]
fn signal_queue_records_signal_targeted_to_process() {
    let mut queue = SignalQueue::new();
    let signal = Signal::new(SignalNumber::new(15), ProcessId::new(42));

    queue.enqueue(signal).unwrap();

    assert_eq!(queue.pending_for(ProcessId::new(42)), vec![signal]);
    assert_eq!(queue.len(), 1);
}

#[test]
fn signal_mask_blocks_matching_signal() {
    let mut queue = SignalQueue::new();
    let mut mask = SignalMask::new();
    mask.block(SignalNumber::new(2));
    queue
        .enqueue(Signal::new(SignalNumber::new(2), ProcessId::new(7)))
        .unwrap();

    let delivery = queue.dispatch_next(ProcessId::new(7), &mask).unwrap();

    assert_eq!(delivery, None);
    assert_eq!(queue.len(), 1);
}

#[test]
fn dispatch_delivers_unblocked_signals_in_deterministic_order() {
    let mut queue = SignalQueue::new();
    let mut mask = SignalMask::new();
    mask.block(SignalNumber::new(2));
    queue
        .enqueue(Signal::new(SignalNumber::new(2), ProcessId::new(1)))
        .unwrap();
    queue
        .enqueue(Signal::new(SignalNumber::new(15), ProcessId::new(1)))
        .unwrap();
    queue
        .enqueue(Signal::new(SignalNumber::new(9), ProcessId::new(1)))
        .unwrap();

    let first = queue
        .dispatch_next(ProcessId::new(1), &mask)
        .unwrap()
        .unwrap();
    let second = queue
        .dispatch_next(ProcessId::new(1), &mask)
        .unwrap()
        .unwrap();

    assert_eq!(first.signal().number(), SignalNumber::new(15));
    assert_eq!(second.signal().number(), SignalNumber::new(9));
    assert_eq!(
        queue.pending_for(ProcessId::new(1)),
        vec![Signal::new(SignalNumber::new(2), ProcessId::new(1))]
    );
}

#[test]
fn dispatch_applies_default_ignore_and_handler_actions() {
    let mut queue = SignalQueue::new();
    let mask = SignalMask::new();
    queue.set_action(SignalNumber::new(1), SignalAction::Ignore);
    queue.set_action(
        SignalNumber::new(2),
        SignalAction::Handle("shutdown-handler".to_string()),
    );
    queue
        .enqueue(Signal::new(SignalNumber::new(15), ProcessId::new(3)))
        .unwrap();
    queue
        .enqueue(Signal::new(SignalNumber::new(1), ProcessId::new(3)))
        .unwrap();
    queue
        .enqueue(Signal::new(SignalNumber::new(2), ProcessId::new(3)))
        .unwrap();

    let default = queue
        .dispatch_next(ProcessId::new(3), &mask)
        .unwrap()
        .unwrap();
    let ignored = queue
        .dispatch_next(ProcessId::new(3), &mask)
        .unwrap()
        .unwrap();
    let handled = queue
        .dispatch_next(ProcessId::new(3), &mask)
        .unwrap()
        .unwrap();

    assert_eq!(
        default.outcome(),
        DeliveryOutcome::Default(SignalNumber::new(15))
    );
    assert_eq!(
        ignored.outcome(),
        DeliveryOutcome::Ignored(SignalNumber::new(1))
    );
    assert_eq!(
        handled.outcome(),
        DeliveryOutcome::Handled {
            signal: SignalNumber::new(2),
            handler: "shutdown-handler".to_string(),
        }
    );
}
