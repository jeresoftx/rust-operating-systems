//! Procesos e hilos.
//!
//! Objetivo de aprendizaje: entender identidad, estado, ciclo de vida,
//! relación proceso-hilos y tabla de procesos.

use std::collections::BTreeMap;

/// Identificador educativo de proceso.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProcessId(u32);

impl ProcessId {
    /// Crea un identificador de proceso.
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    /// Devuelve el valor numérico.
    pub fn value(self) -> u32 {
        self.0
    }
}

/// Identificador educativo de hilo.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ThreadId(u32);

impl ThreadId {
    /// Crea un identificador de hilo.
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    /// Devuelve el valor numérico.
    pub fn value(self) -> u32 {
        self.0
    }
}

/// Estado educativo de proceso.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    Ready,
    Running,
    Blocked,
    Terminated,
}

impl ProcessState {
    fn can_transition_to(self, next: Self) -> bool {
        matches!(
            (self, next),
            (Self::Ready, Self::Running)
                | (Self::Ready, Self::Terminated)
                | (Self::Running, Self::Blocked)
                | (Self::Running, Self::Terminated)
                | (Self::Blocked, Self::Ready)
        )
    }
}

/// Estado educativo de hilo.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreadState {
    Ready,
    Running,
    Blocked,
    Terminated,
}

impl ThreadState {
    fn can_transition_to(self, next: Self) -> bool {
        matches!(
            (self, next),
            (Self::Ready, Self::Running)
                | (Self::Running, Self::Blocked)
                | (Self::Running, Self::Terminated)
                | (Self::Blocked, Self::Ready)
        )
    }
}

/// Hilo educativo dentro de un proceso.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Thread {
    id: ThreadId,
    state: ThreadState,
}

impl Thread {
    /// Crea un hilo listo.
    pub fn new(id: ThreadId) -> Self {
        Self {
            id,
            state: ThreadState::Ready,
        }
    }

    /// Identificador del hilo.
    pub fn id(&self) -> ThreadId {
        self.id
    }

    /// Estado actual del hilo.
    pub fn state(&self) -> ThreadState {
        self.state
    }

    /// Cambia el estado del hilo si la transición es válida.
    pub fn transition_to(&mut self, next: ThreadState) -> Result<(), ProcessError> {
        if self.state.can_transition_to(next) {
            self.state = next;
            Ok(())
        } else {
            Err(ProcessError::InvalidThreadTransition {
                thread: self.id,
                from: self.state,
                to: next,
            })
        }
    }
}

/// Proceso educativo con un conjunto pequeño de hilos.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Process {
    id: ProcessId,
    name: String,
    state: ProcessState,
    threads: Vec<Thread>,
    next_thread_id: u32,
}

impl Process {
    /// Crea un proceso listo con un hilo principal.
    pub fn new(id: ProcessId, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            state: ProcessState::Ready,
            threads: vec![Thread::new(ThreadId::new(1))],
            next_thread_id: 2,
        }
    }

    /// Identificador del proceso.
    pub fn id(&self) -> ProcessId {
        self.id
    }

    /// Nombre legible del proceso.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Estado actual del proceso.
    pub fn state(&self) -> ProcessState {
        self.state
    }

    /// Hilos del proceso.
    pub fn threads(&self) -> &[Thread] {
        &self.threads
    }

    /// Cambia el estado del proceso si la transición es válida.
    pub fn transition_to(&mut self, next: ProcessState) -> Result<(), ProcessError> {
        if self.state.can_transition_to(next) {
            self.state = next;
            Ok(())
        } else {
            Err(ProcessError::InvalidTransition {
                from: self.state,
                to: next,
            })
        }
    }

    /// Crea un hilo nuevo dentro del proceso.
    pub fn spawn_thread(&mut self) -> Result<ThreadId, ProcessError> {
        if self.state == ProcessState::Terminated {
            return Err(ProcessError::ProcessTerminated(self.id));
        }

        let id = ThreadId::new(self.next_thread_id);
        self.next_thread_id += 1;
        self.threads.push(Thread::new(id));
        Ok(id)
    }

    /// Busca un hilo por identificador.
    pub fn thread(&self, id: ThreadId) -> Result<&Thread, ProcessError> {
        self.threads
            .iter()
            .find(|thread| thread.id() == id)
            .ok_or(ProcessError::UnknownThread(id))
    }

    /// Cambia el estado de un hilo.
    pub fn transition_thread(
        &mut self,
        id: ThreadId,
        next: ThreadState,
    ) -> Result<(), ProcessError> {
        let thread = self
            .threads
            .iter_mut()
            .find(|thread| thread.id() == id)
            .ok_or(ProcessError::UnknownThread(id))?;

        thread.transition_to(next)
    }
}

/// Tabla educativa de procesos.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessTable {
    processes: BTreeMap<ProcessId, Process>,
}

impl ProcessTable {
    /// Crea una tabla vacía.
    pub fn new() -> Self {
        Self {
            processes: BTreeMap::new(),
        }
    }

    /// Inserta un proceso y rechaza PID duplicado.
    pub fn insert(&mut self, process: Process) -> Result<(), ProcessError> {
        if self.processes.contains_key(&process.id()) {
            return Err(ProcessError::DuplicateProcess(process.id()));
        }

        self.processes.insert(process.id(), process);
        Ok(())
    }

    /// Busca un proceso por PID.
    pub fn get(&self, id: ProcessId) -> Result<&Process, ProcessError> {
        self.processes
            .get(&id)
            .ok_or(ProcessError::UnknownProcess(id))
    }

    /// Cantidad de procesos registrados.
    pub fn len(&self) -> usize {
        self.processes.len()
    }

    /// Indica si la tabla está vacía.
    pub fn is_empty(&self) -> bool {
        self.processes.is_empty()
    }
}

impl Default for ProcessTable {
    fn default() -> Self {
        Self::new()
    }
}

/// Error educativo de procesos e hilos.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcessError {
    InvalidTransition {
        from: ProcessState,
        to: ProcessState,
    },
    InvalidThreadTransition {
        thread: ThreadId,
        from: ThreadState,
        to: ThreadState,
    },
    ProcessTerminated(ProcessId),
    DuplicateProcess(ProcessId),
    UnknownProcess(ProcessId),
    UnknownThread(ThreadId),
}
