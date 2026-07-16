use rust_operating_systems::scheduling::{
    Priority, Scheduler, SchedulingPolicy, Task, TaskId, TickOutcome,
};

fn main() {
    let mut scheduler = Scheduler::new(SchedulingPolicy::RoundRobin, 2).unwrap();
    scheduler
        .add_task(Task::new(TaskId::new(1), "worker-a", Priority::new(1), 5).unwrap())
        .unwrap();
    scheduler
        .add_task(Task::new(TaskId::new(2), "worker-b", Priority::new(1), 5).unwrap())
        .unwrap();

    scheduler.dispatch_next().unwrap();
    println!("{:?}", scheduler.tick().unwrap());
    println!("{:?}", scheduler.tick().unwrap());

    if scheduler.running_task().is_none() {
        let next = scheduler.dispatch_next().unwrap();
        assert_eq!(next, TaskId::new(2));
    }

    assert_eq!(
        scheduler.ready_task_ids(),
        vec![TaskId::new(1)],
        "la primera tarea volvió al final de la cola"
    );
    assert_eq!(
        scheduler.tick().unwrap(),
        TickOutcome::Running(TaskId::new(2))
    );
}
