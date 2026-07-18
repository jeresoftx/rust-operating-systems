use rust_operating_systems::ipc::{Message, MessageQueue, ProcessEndpoint};
use rust_operating_systems::processes::ProcessId;

fn main() {
    let supervisor = ProcessEndpoint::new(ProcessId::new(1));
    let worker = ProcessEndpoint::new(ProcessId::new(2));
    let mut queue = MessageQueue::new(4).unwrap();

    queue
        .send(worker, supervisor, Message::text("worker listo"))
        .unwrap();

    let report = queue.receive(supervisor).unwrap().unwrap();

    println!(
        "proceso {} recibió: {}",
        report.receiver().process().value(),
        report.message().as_text().unwrap()
    );
}
