use rust_operating_systems::ipc::{IpcError, Message, MessageQueue, ProcessEndpoint};
use rust_operating_systems::processes::ProcessId;

fn main() {
    let supervisor = ProcessEndpoint::new(ProcessId::new(100));
    let worker_a = ProcessEndpoint::new(ProcessId::new(101));
    let worker_b = ProcessEndpoint::new(ProcessId::new(102));
    let mut queue = MessageQueue::new(2).unwrap();

    queue
        .send(worker_a, supervisor, Message::text("worker-a: listo"))
        .unwrap();
    queue
        .send(worker_b, supervisor, Message::text("worker-b: listo"))
        .unwrap();

    let overflow = queue.send(worker_a, supervisor, Message::text("worker-a: extra"));
    if let Err(IpcError::Backpressure { pending, .. }) = overflow {
        println!("supervisor aplica backpressure con {pending} reportes pendientes");
    }

    while let Some(report) = queue.receive(supervisor).unwrap() {
        println!("supervisor recibió {}", report.message().as_text().unwrap());
    }

    queue.close();
    println!("canal cerrado: {:?}", queue.receive(supervisor));
}
