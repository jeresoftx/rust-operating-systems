use rust_operating_systems::scheduling::{Priority, Scheduler, SchedulingPolicy, Task, TaskId};

fn main() {
    let mut jobs = Scheduler::new(SchedulingPolicy::Priority, 1).unwrap();
    jobs.add_task(Task::new(TaskId::new(1), "generar video", Priority::new(4), 8).unwrap())
        .unwrap();
    jobs.add_task(Task::new(TaskId::new(2), "responder checkout", Priority::new(10), 3).unwrap())
        .unwrap();
    jobs.add_task(Task::new(TaskId::new(3), "limpiar caché", Priority::new(1), 5).unwrap())
        .unwrap();

    for _ in 0..4 {
        let selected = jobs.dispatch_next().unwrap();
        let outcome = jobs.tick().unwrap();
        println!(
            "job seleccionado: {}, resultado: {:?}",
            selected.value(),
            outcome
        );
    }

    println!("jobs con espera elevada: {:?}", jobs.starving_tasks(3));
}
