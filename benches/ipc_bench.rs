use std::hint::black_box;
use std::time::Instant;

use rust_operating_systems::ipc::{Message, MessageQueue, Pipe, ProcessEndpoint};
use rust_operating_systems::processes::ProcessId;

fn enviar_y_recibir_mensajes(iteraciones: usize) {
    let sender = ProcessEndpoint::new(ProcessId::new(1));
    let receiver = ProcessEndpoint::new(ProcessId::new(2));
    let mut queue = MessageQueue::new(iteraciones).unwrap();

    for i in 0..iteraciones {
        queue
            .send(sender, receiver, Message::text(format!("mensaje-{i}")))
            .unwrap();
    }

    for _ in 0..iteraciones {
        black_box(queue.receive(receiver).unwrap());
    }
}

fn escribir_y_leer_pipe(iteraciones: usize) {
    let writer = ProcessEndpoint::new(ProcessId::new(3));
    let reader = ProcessEndpoint::new(ProcessId::new(4));
    let mut pipe = Pipe::new(iteraciones).unwrap();

    for i in 0..iteraciones {
        pipe.write(writer, Message::text(format!("línea-{i}")))
            .unwrap();
    }

    for _ in 0..iteraciones {
        black_box(pipe.read(reader).unwrap());
    }
}

fn rechazar_por_backpressure(iteraciones: usize) {
    let sender = ProcessEndpoint::new(ProcessId::new(5));
    let receiver = ProcessEndpoint::new(ProcessId::new(6));
    let mut queue = MessageQueue::new(1).unwrap();
    queue
        .send(sender, receiver, Message::text("ocupado"))
        .unwrap();

    for i in 0..iteraciones {
        let _ = black_box(queue.send(sender, receiver, Message::text(format!("rechazado-{i}"))));
    }
}

fn main() {
    let iteraciones = 50_000usize;

    let start = Instant::now();
    enviar_y_recibir_mensajes(iteraciones);
    let queue_elapsed = start.elapsed();

    let start = Instant::now();
    escribir_y_leer_pipe(iteraciones);
    let pipe_elapsed = start.elapsed();

    let start = Instant::now();
    rechazar_por_backpressure(iteraciones);
    let backpressure_elapsed = start.elapsed();

    println!("benchmark de IPC (manual, std::time::Instant)");
    println!("iteraciones: {iteraciones}");
    println!("cola enviar/recibir: {queue_elapsed:?}");
    println!("pipe escribir/leer: {pipe_elapsed:?}");
    println!("rechazo por backpressure: {backpressure_elapsed:?}");
}
