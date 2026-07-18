use std::hint::black_box;
use std::time::Instant;

use rust_operating_systems::processes::ProcessId;
use rust_operating_systems::signals::{Signal, SignalMask, SignalNumber, SignalQueue};

fn encolar_senales(iteraciones: usize) {
    let mut queue = SignalQueue::new();
    let process = ProcessId::new(1);
    for i in 0..iteraciones {
        let number = SignalNumber::new((i % 31 + 1) as u8);
        queue.enqueue(Signal::new(number, process)).unwrap();
    }
    black_box(queue.len());
}

fn despachar_no_bloqueadas(iteraciones: usize) {
    let mut queue = SignalQueue::new();
    let mask = SignalMask::new();
    let process = ProcessId::new(2);
    for i in 0..iteraciones {
        queue
            .enqueue(Signal::new(SignalNumber::new((i % 31 + 1) as u8), process))
            .unwrap();
    }
    for _ in 0..iteraciones {
        black_box(queue.dispatch_next(process, &mask).unwrap());
    }
}

fn despachar_con_bloqueadas(iteraciones: usize) {
    let mut queue = SignalQueue::new();
    let mut mask = SignalMask::new();
    let process = ProcessId::new(3);
    mask.block(SignalNumber::new(2));

    for i in 0..iteraciones {
        let number = if i % 2 == 0 {
            SignalNumber::new(2)
        } else {
            SignalNumber::new(15)
        };
        queue.enqueue(Signal::new(number, process)).unwrap();
    }

    for _ in 0..(iteraciones / 2) {
        black_box(queue.dispatch_next(process, &mask).unwrap());
    }
}

fn main() {
    let iteraciones = 50_000usize;

    let start = Instant::now();
    encolar_senales(iteraciones);
    let enqueue_elapsed = start.elapsed();

    let start = Instant::now();
    despachar_no_bloqueadas(iteraciones);
    let dispatch_elapsed = start.elapsed();

    let start = Instant::now();
    despachar_con_bloqueadas(iteraciones);
    let masked_elapsed = start.elapsed();

    println!("benchmark de señales (manual, std::time::Instant)");
    println!("iteraciones: {iteraciones}");
    println!("encolar señales: {enqueue_elapsed:?}");
    println!("despachar no bloqueadas: {dispatch_elapsed:?}");
    println!("despachar con bloqueadas: {masked_elapsed:?}");
}
