use rust_operating_systems::ipc::{IpcError, Message, MessageQueue, Pipe, ProcessEndpoint};
use rust_operating_systems::processes::ProcessId;

#[test]
fn message_queue_sends_and_receives_between_two_processes() {
    let sender = ProcessEndpoint::new(ProcessId::new(1));
    let receiver = ProcessEndpoint::new(ProcessId::new(2));
    let mut queue = MessageQueue::new(4).unwrap();

    queue
        .send(sender, receiver, Message::text("estado listo"))
        .unwrap();

    let envelope = queue.receive(receiver).unwrap().unwrap();
    assert_eq!(envelope.sender(), sender);
    assert_eq!(envelope.receiver(), receiver);
    assert_eq!(envelope.message().as_text(), Some("estado listo"));
    assert!(queue.is_empty());
}

#[test]
fn pipe_reads_messages_in_fifo_order() {
    let writer = ProcessEndpoint::new(ProcessId::new(10));
    let reader = ProcessEndpoint::new(ProcessId::new(11));
    let mut pipe = Pipe::new(2).unwrap();

    pipe.write(writer, Message::text("primero")).unwrap();
    pipe.write(writer, Message::text("segundo")).unwrap();

    assert_eq!(
        pipe.read(reader).unwrap().unwrap().as_text(),
        Some("primero")
    );
    assert_eq!(
        pipe.read(reader).unwrap().unwrap().as_text(),
        Some("segundo")
    );
}

#[test]
fn limited_capacity_reports_backpressure_when_full() {
    let sender = ProcessEndpoint::new(ProcessId::new(21));
    let receiver = ProcessEndpoint::new(ProcessId::new(22));
    let mut queue = MessageQueue::new(1).unwrap();

    queue.send(sender, receiver, Message::text("uno")).unwrap();
    let result = queue.send(sender, receiver, Message::text("dos"));

    assert_eq!(
        result,
        Err(IpcError::Backpressure {
            capacity: 1,
            pending: 1,
        })
    );
}

#[test]
fn closed_channel_rejects_new_messages_and_pending_reads() {
    let sender = ProcessEndpoint::new(ProcessId::new(31));
    let receiver = ProcessEndpoint::new(ProcessId::new(32));
    let mut pipe = Pipe::new(2).unwrap();

    pipe.close();

    assert_eq!(
        pipe.write(sender, Message::text("apagado")),
        Err(IpcError::ClosedChannel)
    );
    assert_eq!(pipe.read(receiver), Err(IpcError::ClosedChannel));
}
