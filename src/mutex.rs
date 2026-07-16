//! Mutex.
//!
//! Objetivo de aprendizaje: entender exclusión mutua, propietario, contención,
//! liberación y poisoning educativo.

use crate::processes::ThreadId;

/// Identificador educativo de mutex.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MutexId(u32);

impl MutexId {
    /// Crea un identificador de mutex.
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    /// Devuelve el valor numérico.
    pub fn value(self) -> u32 {
        self.0
    }
}

/// Estado educativo de un mutex.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MutexState {
    Free,
    Locked { owner: ThreadId },
    Poisoned,
}

/// Guard educativo que representa propiedad temporal de un mutex.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LockGuardModel {
    mutex_id: MutexId,
    owner: ThreadId,
}

impl LockGuardModel {
    /// Crea un guard educativo.
    pub fn new(mutex_id: MutexId, owner: ThreadId) -> Self {
        Self { mutex_id, owner }
    }

    /// Mutex protegido por el guard.
    pub fn mutex_id(&self) -> MutexId {
        self.mutex_id
    }

    /// Hilo dueño del guard.
    pub fn owner(&self) -> ThreadId {
        self.owner
    }

    /// Libera el mutex si el guard corresponde al dueño actual.
    pub fn unlock(self, mutex: &mut MutexModel) -> Result<(), MutexError> {
        if mutex.id() != self.mutex_id {
            return Err(MutexError::WrongMutex {
                expected: self.mutex_id,
                actual: mutex.id(),
            });
        }

        mutex.unlock(self.owner)
    }
}

/// Modelo educativo de mutex.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MutexModel {
    id: MutexId,
    state: MutexState,
}

impl MutexModel {
    /// Crea un mutex libre.
    pub fn new(id: MutexId) -> Self {
        Self {
            id,
            state: MutexState::Free,
        }
    }

    /// Identificador del mutex.
    pub fn id(&self) -> MutexId {
        self.id
    }

    /// Estado actual.
    pub fn state(&self) -> MutexState {
        self.state
    }

    /// Propietario actual, si está bloqueado.
    pub fn owner(&self) -> Option<ThreadId> {
        match self.state {
            MutexState::Locked { owner } => Some(owner),
            MutexState::Free | MutexState::Poisoned => None,
        }
    }

    /// Adquiere el mutex si está libre.
    pub fn lock(&mut self, contender: ThreadId) -> Result<LockGuardModel, MutexError> {
        match self.state {
            MutexState::Free => {
                self.state = MutexState::Locked { owner: contender };
                Ok(LockGuardModel::new(self.id, contender))
            }
            MutexState::Locked { owner } => Err(MutexError::AlreadyLocked {
                mutex: self.id,
                owner,
                contender,
            }),
            MutexState::Poisoned => Err(MutexError::Poisoned(self.id)),
        }
    }

    /// Libera el mutex si el hilo es el propietario.
    pub fn unlock(&mut self, attempted: ThreadId) -> Result<(), MutexError> {
        match self.state {
            MutexState::Locked { owner } if owner == attempted => {
                self.state = MutexState::Free;
                Ok(())
            }
            MutexState::Locked { owner } => Err(MutexError::NotOwner {
                mutex: self.id,
                owner,
                attempted,
            }),
            MutexState::Free => Err(MutexError::NotLocked(self.id)),
            MutexState::Poisoned => Err(MutexError::Poisoned(self.id)),
        }
    }

    /// Marca el mutex como poisoned si el hilo era propietario.
    pub fn poison(&mut self, owner: ThreadId) -> Result<(), MutexError> {
        match self.state {
            MutexState::Locked { owner: current } if current == owner => {
                self.state = MutexState::Poisoned;
                Ok(())
            }
            MutexState::Locked { owner: current } => Err(MutexError::NotOwner {
                mutex: self.id,
                owner: current,
                attempted: owner,
            }),
            MutexState::Free => Err(MutexError::NotLocked(self.id)),
            MutexState::Poisoned => Err(MutexError::Poisoned(self.id)),
        }
    }

    /// Recupera manualmente un mutex poisoned.
    pub fn recover(&mut self) -> Result<(), MutexError> {
        match self.state {
            MutexState::Poisoned => {
                self.state = MutexState::Free;
                Ok(())
            }
            MutexState::Free | MutexState::Locked { .. } => Err(MutexError::NotPoisoned(self.id)),
        }
    }
}

/// Error educativo de mutex.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MutexError {
    AlreadyLocked {
        mutex: MutexId,
        owner: ThreadId,
        contender: ThreadId,
    },
    NotOwner {
        mutex: MutexId,
        owner: ThreadId,
        attempted: ThreadId,
    },
    NotLocked(MutexId),
    Poisoned(MutexId),
    NotPoisoned(MutexId),
    WrongMutex {
        expected: MutexId,
        actual: MutexId,
    },
}
