use rust_operating_systems::scheduling::{Priority, Scheduler, SchedulingPolicy, Task, TaskId};

fn main() {
    let mut scheduler = Scheduler::new(SchedulingPolicy::Priority, 2).unwrap();
    scheduler
        .add_task(Task::new(TaskId::new(1), "baja", Priority::new(1), 5).unwrap())
        .unwrap();
    scheduler
        .add_task(Task::new(TaskId::new(2), "alta", Priority::new(10), 5).unwrap())
        .unwrap();
    scheduler
        .add_task(Task::new(TaskId::new(3), "media", Priority::new(5), 5).unwrap())
        .unwrap();

    assert_eq!(scheduler.dispatch_next().unwrap(), TaskId::new(2));

    println!("priority scheduling seleccionó la tarea 2");
}
