use rust_operating_systems::scheduling::{
    Priority, Scheduler, SchedulingPolicy, Task, TaskId, TickOutcome,
};

fn task(id: u32, priority: u8, required_ticks: u32) -> Task {
    Task::new(
        TaskId::new(id),
        format!("tarea-{id}"),
        Priority::new(priority),
        required_ticks,
    )
    .unwrap()
}

#[test]
fn round_robin_selects_next_ready_task() {
    let mut scheduler = Scheduler::new(SchedulingPolicy::RoundRobin, 2).unwrap();
    scheduler.add_task(task(1, 1, 5)).unwrap();
    scheduler.add_task(task(2, 1, 5)).unwrap();
    scheduler.add_task(task(3, 1, 5)).unwrap();

    let selected = scheduler.dispatch_next().unwrap();

    assert_eq!(selected, TaskId::new(1));
    assert_eq!(scheduler.running_task(), Some(TaskId::new(1)));
    assert_eq!(
        scheduler.ready_task_ids(),
        vec![TaskId::new(2), TaskId::new(3)]
    );
}

#[test]
fn priority_policy_selects_highest_priority_task() {
    let mut scheduler = Scheduler::new(SchedulingPolicy::Priority, 2).unwrap();
    scheduler.add_task(task(1, 1, 5)).unwrap();
    scheduler.add_task(task(2, 10, 5)).unwrap();
    scheduler.add_task(task(3, 5, 5)).unwrap();

    let selected = scheduler.dispatch_next().unwrap();

    assert_eq!(selected, TaskId::new(2));
    assert_eq!(
        scheduler.ready_task_ids(),
        vec![TaskId::new(1), TaskId::new(3)]
    );
}

#[test]
fn tick_requeues_running_task_when_quantum_expires() {
    let mut scheduler = Scheduler::new(SchedulingPolicy::RoundRobin, 2).unwrap();
    scheduler.add_task(task(1, 1, 5)).unwrap();
    scheduler.add_task(task(2, 1, 5)).unwrap();
    scheduler.dispatch_next().unwrap();

    assert_eq!(
        scheduler.tick().unwrap(),
        TickOutcome::Running(TaskId::new(1))
    );
    assert_eq!(
        scheduler.tick().unwrap(),
        TickOutcome::QuantumExpired(TaskId::new(1))
    );
    assert_eq!(scheduler.running_task(), None);
    assert_eq!(
        scheduler.ready_task_ids(),
        vec![TaskId::new(2), TaskId::new(1)]
    );
}

#[test]
fn scheduler_reports_tasks_waiting_beyond_threshold() {
    let mut scheduler = Scheduler::new(SchedulingPolicy::Priority, 1).unwrap();
    scheduler.add_task(task(1, 10, 10)).unwrap();
    scheduler.add_task(task(2, 1, 10)).unwrap();

    for _ in 0..3 {
        scheduler.dispatch_next().unwrap();
        scheduler.tick().unwrap();
    }

    assert_eq!(scheduler.waited_ticks(TaskId::new(2)).unwrap(), 3);
    assert_eq!(scheduler.starving_tasks(3), vec![TaskId::new(2)]);
}
