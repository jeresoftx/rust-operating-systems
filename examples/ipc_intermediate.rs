use rust_operating_systems::ipc::{Message, Pipe, ProcessEndpoint};
use rust_operating_systems::processes::ProcessId;

fn main() {
    let shell = ProcessEndpoint::new(ProcessId::new(10));
    let command = ProcessEndpoint::new(ProcessId::new(11));
    let mut pipe = Pipe::new(3).unwrap();

    pipe.write(shell, Message::text("línea 1")).unwrap();
    pipe.write(shell, Message::text("línea 2")).unwrap();

    while let Some(message) = pipe.read(command).unwrap() {
        println!("pipe entregó: {}", message.as_text().unwrap());
    }
}
