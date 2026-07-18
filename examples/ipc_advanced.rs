use rust_operating_systems::ipc::{IpcError, Message, MessageQueue, ProcessEndpoint};
use rust_operating_systems::processes::ProcessId;

fn main() {
    let worker = ProcessEndpoint::new(ProcessId::new(20));
    let logger = ProcessEndpoint::new(ProcessId::new(21));
    let mut queue = MessageQueue::new(1).unwrap();

    queue
        .send(worker, logger, Message::text("evento crítico"))
        .unwrap();

    match queue.send(worker, logger, Message::text("evento duplicado")) {
        Err(IpcError::Backpressure { capacity, pending }) => {
            println!("backpressure: capacidad={capacity}, pendientes={pending}");
        }
        other => println!("resultado inesperado: {other:?}"),
    }
}
