//! Señales.
//!
//! Objetivo de aprendizaje: entender envío asíncrono, señales pendientes,
//! máscaras, acciones y entrega determinista.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use crate::processes::ProcessId;

/// Número educativo de señal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SignalNumber(u8);

impl SignalNumber {
    /// Crea un número de señal.
    pub fn new(value: u8) -> Self {
        Self(value)
    }

    /// Devuelve el valor numérico.
    pub fn value(self) -> u8 {
        self.0
    }
}

/// Señal dirigida a un proceso.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Signal {
    number: SignalNumber,
    target: ProcessId,
}

impl Signal {
    /// Crea una señal dirigida a un proceso.
    pub fn new(number: SignalNumber, target: ProcessId) -> Self {
        Self { number, target }
    }

    /// Número de señal.
    pub fn number(&self) -> SignalNumber {
        self.number
    }

    /// Proceso destino.
    pub fn target(&self) -> ProcessId {
        self.target
    }
}

/// Acción configurada para una señal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignalAction {
    Default,
    Ignore,
    Handle(String),
}

/// Resultado educativo de entregar una señal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeliveryOutcome {
    Default(SignalNumber),
    Ignored(SignalNumber),
    Handled {
        signal: SignalNumber,
        handler: String,
    },
}

/// Entrega concreta de una señal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignalDelivery {
    signal: Signal,
    outcome: DeliveryOutcome,
}

impl SignalDelivery {
    fn new(signal: Signal, action: SignalAction) -> Self {
        let outcome = match action {
            SignalAction::Default => DeliveryOutcome::Default(signal.number()),
            SignalAction::Ignore => DeliveryOutcome::Ignored(signal.number()),
            SignalAction::Handle(handler) => DeliveryOutcome::Handled {
                signal: signal.number(),
                handler,
            },
        };

        Self { signal, outcome }
    }

    /// Señal entregada.
    pub fn signal(&self) -> Signal {
        self.signal
    }

    /// Resultado de aplicar la acción configurada.
    pub fn outcome(&self) -> DeliveryOutcome {
        self.outcome.clone()
    }
}

/// Máscara de señales bloqueadas.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignalMask {
    blocked: BTreeSet<SignalNumber>,
}

impl SignalMask {
    /// Crea una máscara sin señales bloqueadas.
    pub fn new() -> Self {
        Self {
            blocked: BTreeSet::new(),
        }
    }

    /// Bloquea una señal.
    pub fn block(&mut self, number: SignalNumber) {
        self.blocked.insert(number);
    }

    /// Desbloquea una señal.
    pub fn unblock(&mut self, number: SignalNumber) {
        self.blocked.remove(&number);
    }

    /// Indica si una señal está bloqueada.
    pub fn blocks(&self, number: SignalNumber) -> bool {
        self.blocked.contains(&number)
    }
}

impl Default for SignalMask {
    fn default() -> Self {
        Self::new()
    }
}

/// Cola educativa de señales pendientes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignalQueue {
    pending: VecDeque<Signal>,
    actions: BTreeMap<SignalNumber, SignalAction>,
}

impl SignalQueue {
    /// Crea una cola vacía.
    pub fn new() -> Self {
        Self {
            pending: VecDeque::new(),
            actions: BTreeMap::new(),
        }
    }

    /// Encola una señal dirigida a un proceso.
    pub fn enqueue(&mut self, signal: Signal) -> Result<(), SignalError> {
        if signal.number().value() == 0 {
            return Err(SignalError::InvalidSignalNumber(signal.number()));
        }

        self.pending.push_back(signal);
        Ok(())
    }

    /// Configura la acción para una señal.
    pub fn set_action(&mut self, number: SignalNumber, action: SignalAction) {
        self.actions.insert(number, action);
    }

    /// Cantidad de señales pendientes.
    pub fn len(&self) -> usize {
        self.pending.len()
    }

    /// Indica si no hay señales pendientes.
    pub fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }

    /// Señales pendientes para un proceso, en orden.
    pub fn pending_for(&self, target: ProcessId) -> Vec<Signal> {
        self.pending
            .iter()
            .copied()
            .filter(|signal| signal.target() == target)
            .collect()
    }

    /// Entrega la siguiente señal no bloqueada para un proceso.
    pub fn dispatch_next(
        &mut self,
        target: ProcessId,
        mask: &SignalMask,
    ) -> Result<Option<SignalDelivery>, SignalError> {
        let Some(index) = self
            .pending
            .iter()
            .position(|signal| signal.target() == target && !mask.blocks(signal.number()))
        else {
            return Ok(None);
        };

        let signal = self
            .pending
            .remove(index)
            .expect("índice obtenido desde la cola");
        let action = self
            .actions
            .get(&signal.number())
            .cloned()
            .unwrap_or(SignalAction::Default);

        Ok(Some(SignalDelivery::new(signal, action)))
    }
}

impl Default for SignalQueue {
    fn default() -> Self {
        Self::new()
    }
}

/// Error educativo de señales.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignalError {
    InvalidSignalNumber(SignalNumber),
}
