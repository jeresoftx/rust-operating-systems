use rust_operating_systems::scheduling::{Priority, Scheduler, SchedulingPolicy, Task, TaskId};

fn main() {
    let mut scheduler = Scheduler::new(SchedulingPolicy::RoundRobin, 2).unwrap();
    scheduler
        .add_task(Task::new(TaskId::new(1), "indexar", Priority::new(1), 5).unwrap())
        .unwrap();
    scheduler
        .add_task(Task::new(TaskId::new(2), "enviar correo", Priority::new(1), 5).unwrap())
        .unwrap();

    let selected = scheduler.dispatch_next().unwrap();

    println!("tarea en ejecución: {}", selected.value());
    println!("tareas listas: {:?}", scheduler.ready_task_ids());
}
