use rust_operating_systems::scheduling::{Priority, Scheduler, SchedulingPolicy, Task, TaskId};

fn main() {
    let mut scheduler = Scheduler::new(SchedulingPolicy::Priority, 1).unwrap();
    scheduler
        .add_task(Task::new(TaskId::new(1), "alta", Priority::new(10), 10).unwrap())
        .unwrap();
    scheduler
        .add_task(Task::new(TaskId::new(2), "baja", Priority::new(1), 10).unwrap())
        .unwrap();

    for _ in 0..3 {
        scheduler.dispatch_next().unwrap();
        scheduler.tick().unwrap();
    }

    assert_eq!(scheduler.waited_ticks(TaskId::new(2)).unwrap(), 3);
    assert_eq!(scheduler.starving_tasks(3), vec![TaskId::new(2)]);

    println!("la tarea 2 muestra inanición potencial");
}
