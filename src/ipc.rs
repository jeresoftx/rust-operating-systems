//! IPC.
//!
//! Objetivo de aprendizaje: entender endpoints, mensajes, colas, pipes,
//! backpressure y cierre de canales.

use std::collections::VecDeque;

use crate::processes::ProcessId;

/// Endpoint educativo asociado a un proceso.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProcessEndpoint {
    process: ProcessId,
}

impl ProcessEndpoint {
    /// Crea un endpoint para un proceso.
    pub fn new(process: ProcessId) -> Self {
        Self { process }
    }

    /// Proceso dueño del endpoint.
    pub fn process(&self) -> ProcessId {
        self.process
    }
}

/// Mensaje transportado por un canal de IPC.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    bytes: Vec<u8>,
}

impl Message {
    /// Crea un mensaje desde bytes.
    pub fn new(bytes: impl Into<Vec<u8>>) -> Self {
        Self {
            bytes: bytes.into(),
        }
    }

    /// Crea un mensaje de texto UTF-8.
    pub fn text(value: impl Into<String>) -> Self {
        Self::new(value.into().into_bytes())
    }

    /// Bytes del mensaje.
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Texto del mensaje cuando los bytes son UTF-8 válido.
    pub fn as_text(&self) -> Option<&str> {
        std::str::from_utf8(&self.bytes).ok()
    }

    /// Tamaño del mensaje en bytes.
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Indica si el mensaje no tiene contenido.
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }
}

/// Mensaje con emisor y receptor explícitos.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Envelope {
    sender: ProcessEndpoint,
    receiver: ProcessEndpoint,
    message: Message,
}

impl Envelope {
    fn new(sender: ProcessEndpoint, receiver: ProcessEndpoint, message: Message) -> Self {
        Self {
            sender,
            receiver,
            message,
        }
    }

    /// Endpoint emisor.
    pub fn sender(&self) -> ProcessEndpoint {
        self.sender
    }

    /// Endpoint receptor.
    pub fn receiver(&self) -> ProcessEndpoint {
        self.receiver
    }

    /// Mensaje transportado.
    pub fn message(&self) -> &Message {
        &self.message
    }
}

/// Cola educativa de mensajes con capacidad finita.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageQueue {
    capacity: usize,
    pending: VecDeque<Envelope>,
    closed: bool,
}

impl MessageQueue {
    /// Crea una cola con capacidad finita.
    pub fn new(capacity: usize) -> Result<Self, IpcError> {
        if capacity == 0 {
            return Err(IpcError::ZeroCapacity);
        }

        Ok(Self {
            capacity,
            pending: VecDeque::new(),
            closed: false,
        })
    }

    /// Encola un mensaje dirigido a otro endpoint.
    pub fn send(
        &mut self,
        sender: ProcessEndpoint,
        receiver: ProcessEndpoint,
        message: Message,
    ) -> Result<(), IpcError> {
        self.ensure_open()?;
        self.ensure_capacity()?;
        self.pending
            .push_back(Envelope::new(sender, receiver, message));
        Ok(())
    }

    /// Recibe el siguiente mensaje dirigido a un endpoint.
    pub fn receive(&mut self, receiver: ProcessEndpoint) -> Result<Option<Envelope>, IpcError> {
        self.ensure_open()?;

        let Some(index) = self
            .pending
            .iter()
            .position(|envelope| envelope.receiver == receiver)
        else {
            return Ok(None);
        };

        Ok(self.pending.remove(index))
    }

    /// Cierra la cola para nuevos envíos y recepciones.
    pub fn close(&mut self) {
        self.closed = true;
    }

    /// Capacidad total de la cola.
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Mensajes pendientes.
    pub fn len(&self) -> usize {
        self.pending.len()
    }

    /// Indica si no hay mensajes pendientes.
    pub fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }

    fn ensure_open(&self) -> Result<(), IpcError> {
        if self.closed {
            Err(IpcError::ClosedChannel)
        } else {
            Ok(())
        }
    }

    fn ensure_capacity(&self) -> Result<(), IpcError> {
        if self.pending.len() >= self.capacity {
            Err(IpcError::Backpressure {
                capacity: self.capacity,
                pending: self.pending.len(),
            })
        } else {
            Ok(())
        }
    }
}

/// Pipe educativo unidireccional con orden FIFO.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pipe {
    capacity: usize,
    pending: VecDeque<Message>,
    closed: bool,
}

impl Pipe {
    /// Crea un pipe con capacidad finita.
    pub fn new(capacity: usize) -> Result<Self, IpcError> {
        if capacity == 0 {
            return Err(IpcError::ZeroCapacity);
        }

        Ok(Self {
            capacity,
            pending: VecDeque::new(),
            closed: false,
        })
    }

    /// Escribe un mensaje al final del pipe.
    pub fn write(&mut self, _writer: ProcessEndpoint, message: Message) -> Result<(), IpcError> {
        self.ensure_open()?;
        self.ensure_capacity()?;
        self.pending.push_back(message);
        Ok(())
    }

    /// Lee el siguiente mensaje del pipe en orden FIFO.
    pub fn read(&mut self, _reader: ProcessEndpoint) -> Result<Option<Message>, IpcError> {
        self.ensure_open()?;
        Ok(self.pending.pop_front())
    }

    /// Cierra el pipe.
    pub fn close(&mut self) {
        self.closed = true;
    }

    /// Capacidad total del pipe.
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Mensajes pendientes.
    pub fn len(&self) -> usize {
        self.pending.len()
    }

    /// Indica si no hay mensajes pendientes.
    pub fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }

    fn ensure_open(&self) -> Result<(), IpcError> {
        if self.closed {
            Err(IpcError::ClosedChannel)
        } else {
            Ok(())
        }
    }

    fn ensure_capacity(&self) -> Result<(), IpcError> {
        if self.pending.len() >= self.capacity {
            Err(IpcError::Backpressure {
                capacity: self.capacity,
                pending: self.pending.len(),
            })
        } else {
            Ok(())
        }
    }
}

/// Error educativo de IPC.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IpcError {
    ZeroCapacity,
    Backpressure { capacity: usize, pending: usize },
    ClosedChannel,
}
