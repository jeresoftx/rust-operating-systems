use rust_operating_systems::scheduling::{Priority, Scheduler, SchedulingPolicy, Task, TaskId};

fn main() {
    let mut scheduler = Scheduler::new(SchedulingPolicy::RoundRobin, 2).unwrap();
    scheduler
        .add_task(Task::new(TaskId::new(1), "a", Priority::new(1), 5).unwrap())
        .unwrap();
    scheduler
        .add_task(Task::new(TaskId::new(2), "b", Priority::new(1), 5).unwrap())
        .unwrap();
    scheduler
        .add_task(Task::new(TaskId::new(3), "c", Priority::new(1), 5).unwrap())
        .unwrap();

    assert_eq!(scheduler.dispatch_next().unwrap(), TaskId::new(1));
    assert_eq!(
        scheduler.ready_task_ids(),
        vec![TaskId::new(2), TaskId::new(3)]
    );

    println!("round-robin seleccionó la tarea 1");
}
