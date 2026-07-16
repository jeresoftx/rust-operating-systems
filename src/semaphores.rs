//! Semáforos.
//!
//! Objetivo de aprendizaje: entender permisos, capacidad, espera, liberación
//! determinista y backpressure.

use std::collections::VecDeque;

use crate::processes::ThreadId;

/// Identificador educativo de semáforo.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SemaphoreId(u32);

impl SemaphoreId {
    /// Crea un identificador de semáforo.
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    /// Devuelve el valor numérico.
    pub fn value(self) -> u32 {
        self.0
    }
}

/// Permiso otorgado por un semáforo.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Permit {
    semaphore_id: SemaphoreId,
    owner: ThreadId,
}

impl Permit {
    /// Crea un permiso educativo.
    pub fn new(semaphore_id: SemaphoreId, owner: ThreadId) -> Self {
        Self {
            semaphore_id,
            owner,
        }
    }

    /// Semáforo que emitió el permiso.
    pub fn semaphore_id(self) -> SemaphoreId {
        self.semaphore_id
    }

    /// Hilo dueño del permiso.
    pub fn owner(self) -> ThreadId {
        self.owner
    }
}

/// Solicitante esperando permiso.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Waiter {
    thread_id: ThreadId,
}

impl Waiter {
    /// Crea un solicitante.
    pub fn new(thread_id: ThreadId) -> Self {
        Self { thread_id }
    }

    /// Hilo solicitante.
    pub fn thread_id(self) -> ThreadId {
        self.thread_id
    }
}

/// Semáforo contador educativo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Semaphore {
    id: SemaphoreId,
    capacity: usize,
    in_use: usize,
    waiters: VecDeque<Waiter>,
}

impl Semaphore {
    /// Crea un semáforo con capacidad positiva.
    pub fn new(id: SemaphoreId, capacity: usize) -> Result<Self, SemaphoreError> {
        if capacity == 0 {
            return Err(SemaphoreError::ZeroCapacity(id));
        }

        Ok(Self {
            id,
            capacity,
            in_use: 0,
            waiters: VecDeque::new(),
        })
    }

    /// Identificador del semáforo.
    pub fn id(&self) -> SemaphoreId {
        self.id
    }

    /// Capacidad máxima de permisos.
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Permisos actualmente ocupados.
    pub fn in_use(&self) -> usize {
        self.in_use
    }

    /// Permisos disponibles.
    pub fn available_permits(&self) -> usize {
        self.capacity - self.in_use
    }

    /// Cola de espera en orden FIFO.
    pub fn waiters(&self) -> &[Waiter] {
        self.waiters.as_slices().0
    }

    /// Intenta adquirir un permiso.
    pub fn acquire(&mut self, owner: ThreadId) -> Result<Permit, SemaphoreError> {
        if self.in_use < self.capacity {
            self.in_use += 1;
            Ok(Permit::new(self.id, owner))
        } else {
            let waiter = Waiter::new(owner);
            self.waiters.push_back(waiter);
            Err(SemaphoreError::WouldBlock(waiter))
        }
    }

    /// Libera un permiso y despierta al siguiente solicitante si existe.
    pub fn release(&mut self, permit: Permit) -> Result<Option<Permit>, SemaphoreError> {
        if permit.semaphore_id() != self.id {
            return Err(SemaphoreError::WrongSemaphore {
                expected: self.id,
                actual: permit.semaphore_id(),
            });
        }

        if self.in_use == 0 {
            return Err(SemaphoreError::CapacityExceeded {
                semaphore: self.id,
                capacity: self.capacity,
            });
        }

        if let Some(waiter) = self.waiters.pop_front() {
            Ok(Some(Permit::new(self.id, waiter.thread_id())))
        } else {
            self.in_use -= 1;
            Ok(None)
        }
    }
}

/// Error educativo de semáforo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SemaphoreError {
    ZeroCapacity(SemaphoreId),
    WouldBlock(Waiter),
    CapacityExceeded {
        semaphore: SemaphoreId,
        capacity: usize,
    },
    WrongSemaphore {
        expected: SemaphoreId,
        actual: SemaphoreId,
    },
}
