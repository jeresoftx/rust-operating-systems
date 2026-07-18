use rust_operating_systems::ipc::{Message, MessageQueue, ProcessEndpoint};
use rust_operating_systems::processes::ProcessId;

fn main() {
    let sender = ProcessEndpoint::new(ProcessId::new(7));
    let receiver = ProcessEndpoint::new(ProcessId::new(8));
    let mut queue = MessageQueue::new(4).unwrap();

    queue
        .send(sender, receiver, Message::text("hola desde otro proceso"))
        .unwrap();

    let envelope = queue.receive(receiver).unwrap().unwrap();

    assert_eq!(envelope.sender(), sender);
    assert_eq!(envelope.receiver(), receiver);
    assert_eq!(
        envelope.message().as_text(),
        Some("hola desde otro proceso")
    );

    println!("cola de mensajes resuelta");
}
