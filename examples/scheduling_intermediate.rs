use rust_operating_systems::scheduling::{Priority, Scheduler, SchedulingPolicy, Task, TaskId};

fn main() {
    let mut scheduler = Scheduler::new(SchedulingPolicy::Priority, 2).unwrap();
    scheduler
        .add_task(Task::new(TaskId::new(1), "reporte", Priority::new(2), 5).unwrap())
        .unwrap();
    scheduler
        .add_task(Task::new(TaskId::new(2), "checkout", Priority::new(9), 5).unwrap())
        .unwrap();
    scheduler
        .add_task(Task::new(TaskId::new(3), "limpieza", Priority::new(1), 5).unwrap())
        .unwrap();

    let selected = scheduler.dispatch_next().unwrap();

    println!("prioridad seleccionada: {}", selected.value());
    println!("tareas que siguen listas: {:?}", scheduler.ready_task_ids());
}
