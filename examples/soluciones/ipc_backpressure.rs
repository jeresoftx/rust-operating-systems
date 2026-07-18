use rust_operating_systems::ipc::{IpcError, Message, MessageQueue, ProcessEndpoint};
use rust_operating_systems::processes::ProcessId;

fn main() {
    let sender = ProcessEndpoint::new(ProcessId::new(3));
    let receiver = ProcessEndpoint::new(ProcessId::new(4));
    let mut queue = MessageQueue::new(1).unwrap();

    queue.send(sender, receiver, Message::text("uno")).unwrap();

    assert_eq!(
        queue.send(sender, receiver, Message::text("dos")),
        Err(IpcError::Backpressure {
            capacity: 1,
            pending: 1,
        })
    );

    println!("backpressure resuelto");
}
