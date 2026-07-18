use rust_operating_systems::ipc::{Message, Pipe, ProcessEndpoint};
use rust_operating_systems::processes::ProcessId;

fn main() {
    let writer = ProcessEndpoint::new(ProcessId::new(1));
    let reader = ProcessEndpoint::new(ProcessId::new(2));
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

    println!("pipe FIFO resuelto");
}
