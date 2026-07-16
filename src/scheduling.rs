//! Scheduling.
//!
//! Objetivo de aprendizaje: entender cola de listos, política de selección,
//! quantum, reencolado y espera acumulada.

use std::collections::VecDeque;

/// Identificador educativo de tarea.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TaskId(u32);

impl TaskId {
    /// Crea un identificador de tarea.
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    /// Devuelve el valor numérico.
    pub fn value(self) -> u32 {
        self.0
    }
}

/// Prioridad educativa. Un valor mayor significa mayor prioridad.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Priority(u8);

impl Priority {
    /// Crea una prioridad.
    pub fn new(value: u8) -> Self {
        Self(value)
    }

    /// Devuelve el valor numérico.
    pub fn value(self) -> u8 {
        self.0
    }
}

/// Tarea lista para ser planificada.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    id: TaskId,
    name: String,
    priority: Priority,
    required_ticks: u32,
    executed_ticks: u32,
    waited_ticks: u32,
}

impl Task {
    /// Crea una tarea con trabajo pendiente.
    pub fn new(
        id: TaskId,
        name: impl Into<String>,
        priority: Priority,
        required_ticks: u32,
    ) -> Result<Self, SchedulingError> {
        if required_ticks == 0 {
            return Err(SchedulingError::ZeroRequiredTicks);
        }

        Ok(Self {
            id,
            name: name.into(),
            priority,
            required_ticks,
            executed_ticks: 0,
            waited_ticks: 0,
        })
    }

    /// Identificador de la tarea.
    pub fn id(&self) -> TaskId {
        self.id
    }

    /// Nombre legible.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Prioridad de la tarea.
    pub fn priority(&self) -> Priority {
        self.priority
    }

    /// Ticks requeridos para terminar.
    pub fn required_ticks(&self) -> u32 {
        self.required_ticks
    }

    /// Ticks ya ejecutados.
    pub fn executed_ticks(&self) -> u32 {
        self.executed_ticks
    }

    /// Ticks acumulados esperando en la cola de listos.
    pub fn waited_ticks(&self) -> u32 {
        self.waited_ticks
    }

    fn is_complete(&self) -> bool {
        self.executed_ticks >= self.required_ticks
    }
}

/// Política educativa de scheduling.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchedulingPolicy {
    RoundRobin,
    Priority,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RunningTask {
    task: Task,
    quantum_used: u32,
}

/// Resultado de avanzar un tick de CPU.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TickOutcome {
    Running(TaskId),
    QuantumExpired(TaskId),
    Completed(TaskId),
}

/// Scheduler educativo con una cola de listos y una tarea en ejecución.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scheduler {
    policy: SchedulingPolicy,
    quantum: u32,
    ready: VecDeque<Task>,
    running: Option<RunningTask>,
    finished: Vec<Task>,
}

impl Scheduler {
    /// Crea un scheduler con quantum positivo.
    pub fn new(policy: SchedulingPolicy, quantum: u32) -> Result<Self, SchedulingError> {
        if quantum == 0 {
            return Err(SchedulingError::ZeroQuantum);
        }

        Ok(Self {
            policy,
            quantum,
            ready: VecDeque::new(),
            running: None,
            finished: Vec::new(),
        })
    }

    /// Agrega una tarea lista.
    pub fn add_task(&mut self, task: Task) -> Result<(), SchedulingError> {
        if self.contains_task(task.id()) {
            return Err(SchedulingError::DuplicateTask(task.id()));
        }

        self.ready.push_back(task);
        Ok(())
    }

    /// Selecciona la siguiente tarea y la mueve a ejecución.
    pub fn dispatch_next(&mut self) -> Result<TaskId, SchedulingError> {
        if let Some(running) = &self.running {
            return Err(SchedulingError::TaskAlreadyRunning(running.task.id()));
        }

        let task = match self.policy {
            SchedulingPolicy::RoundRobin => self.ready.pop_front(),
            SchedulingPolicy::Priority => self.pop_highest_priority(),
        }
        .ok_or(SchedulingError::EmptyReadyQueue)?;

        let id = task.id();
        self.running = Some(RunningTask {
            task,
            quantum_used: 0,
        });
        Ok(id)
    }

    /// Avanza un tick educativo de CPU.
    pub fn tick(&mut self) -> Result<TickOutcome, SchedulingError> {
        let Some(mut running) = self.running.take() else {
            return Err(SchedulingError::NoRunningTask);
        };

        running.task.executed_ticks += 1;
        running.quantum_used += 1;
        for task in &mut self.ready {
            task.waited_ticks += 1;
        }

        let id = running.task.id();
        if running.task.is_complete() {
            self.finished.push(running.task);
            Ok(TickOutcome::Completed(id))
        } else if running.quantum_used >= self.quantum {
            self.ready.push_back(running.task);
            Ok(TickOutcome::QuantumExpired(id))
        } else {
            self.running = Some(running);
            Ok(TickOutcome::Running(id))
        }
    }

    /// Tarea ejecutándose actualmente.
    pub fn running_task(&self) -> Option<TaskId> {
        self.running.as_ref().map(|running| running.task.id())
    }

    /// Identificadores en la cola de listos, en orden actual.
    pub fn ready_task_ids(&self) -> Vec<TaskId> {
        self.ready.iter().map(Task::id).collect()
    }

    /// Ticks de espera acumulados por una tarea conocida.
    pub fn waited_ticks(&self, id: TaskId) -> Result<u32, SchedulingError> {
        self.find_task(id)
            .map(Task::waited_ticks)
            .ok_or(SchedulingError::UnknownTask(id))
    }

    /// Tareas listas con espera mayor o igual al umbral.
    pub fn starving_tasks(&self, threshold: u32) -> Vec<TaskId> {
        self.ready
            .iter()
            .filter(|task| task.waited_ticks() >= threshold)
            .map(Task::id)
            .collect()
    }

    fn contains_task(&self, id: TaskId) -> bool {
        self.find_task(id).is_some()
    }

    fn find_task(&self, id: TaskId) -> Option<&Task> {
        self.ready
            .iter()
            .chain(self.finished.iter())
            .chain(self.running.iter().map(|running| &running.task))
            .find(|task| task.id() == id)
    }

    fn pop_highest_priority(&mut self) -> Option<Task> {
        let mut best_index = 0;
        for index in 1..self.ready.len() {
            if self.ready[index].priority() > self.ready[best_index].priority() {
                best_index = index;
            }
        }

        self.ready.remove(best_index)
    }
}

/// Error educativo de scheduling.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchedulingError {
    ZeroQuantum,
    ZeroRequiredTicks,
    EmptyReadyQueue,
    NoRunningTask,
    DuplicateTask(TaskId),
    UnknownTask(TaskId),
    TaskAlreadyRunning(TaskId),
}
