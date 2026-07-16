//! Paging.
//!
//! Objetivo de aprendizaje: entender páginas, frames, offset, permisos,
//! page faults y reemplazo FIFO.

use std::collections::{BTreeMap, VecDeque};

/// Número de página virtual.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PageNumber(u64);

impl PageNumber {
    /// Crea un número de página.
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    /// Devuelve el valor numérico.
    pub fn value(self) -> u64 {
        self.0
    }
}

/// Número de frame físico.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FrameNumber(u64);

impl FrameNumber {
    /// Crea un número de frame.
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    /// Devuelve el valor numérico.
    pub fn value(self) -> u64 {
        self.0
    }
}

/// Tamaño de página en bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PageSize(u64);

impl PageSize {
    /// Crea un tamaño de página positivo.
    pub fn new(value: u64) -> Result<Self, PagingError> {
        if value == 0 {
            return Err(PagingError::ZeroPageSize);
        }

        Ok(Self(value))
    }

    /// Devuelve el tamaño en bytes.
    pub fn value(self) -> u64 {
        self.0
    }
}

/// Tipo de acceso solicitado durante la traducción.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessType {
    Read,
    Write,
}

/// Permisos educativos de una entrada de tabla.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PagePermissions {
    readable: bool,
    writable: bool,
}

impl PagePermissions {
    /// Permisos de solo lectura.
    pub fn read_only() -> Self {
        Self {
            readable: true,
            writable: false,
        }
    }

    /// Permisos de lectura y escritura.
    pub fn read_write() -> Self {
        Self {
            readable: true,
            writable: true,
        }
    }

    /// Indica si se permite lectura.
    pub fn readable(&self) -> bool {
        self.readable
    }

    /// Indica si se permite escritura.
    pub fn writable(&self) -> bool {
        self.writable
    }

    fn allows(&self, access: AccessType) -> bool {
        match access {
            AccessType::Read => self.readable,
            AccessType::Write => self.writable,
        }
    }
}

/// Entrada de tabla de páginas.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageTableEntry {
    frame: FrameNumber,
    permissions: PagePermissions,
    present: bool,
}

impl PageTableEntry {
    /// Crea una entrada presente.
    pub fn new(frame: FrameNumber, permissions: PagePermissions) -> Self {
        Self {
            frame,
            permissions,
            present: true,
        }
    }

    /// Frame físico al que apunta.
    pub fn frame(&self) -> FrameNumber {
        self.frame
    }

    /// Permisos de la entrada.
    pub fn permissions(&self) -> PagePermissions {
        self.permissions
    }

    /// Indica si la entrada está presente.
    pub fn present(&self) -> bool {
        self.present
    }
}

/// Motivo educativo de un page fault.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FaultReason {
    NotMapped,
    NotPresent,
    ProtectionViolation(AccessType),
}

/// Page fault con página y causa.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageFault {
    page: PageNumber,
    reason: FaultReason,
}

impl PageFault {
    /// Crea un page fault.
    pub fn new(page: PageNumber, reason: FaultReason) -> Self {
        Self { page, reason }
    }

    /// Página que falló.
    pub fn page(&self) -> PageNumber {
        self.page
    }

    /// Motivo del fallo.
    pub fn reason(&self) -> FaultReason {
        self.reason
    }
}

/// Tabla de páginas educativa con reemplazo FIFO opcional.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageTable {
    page_size: PageSize,
    capacity: Option<usize>,
    entries: BTreeMap<PageNumber, PageTableEntry>,
    fifo_order: VecDeque<PageNumber>,
}

impl PageTable {
    /// Crea una tabla sin límite de entradas.
    pub fn new(page_size: PageSize) -> Self {
        Self {
            page_size,
            capacity: None,
            entries: BTreeMap::new(),
            fifo_order: VecDeque::new(),
        }
    }

    /// Crea una tabla con capacidad máxima y reemplazo FIFO.
    pub fn with_capacity(page_size: PageSize, capacity: usize) -> Result<Self, PagingError> {
        if capacity == 0 {
            return Err(PagingError::ZeroCapacity);
        }

        Ok(Self {
            page_size,
            capacity: Some(capacity),
            entries: BTreeMap::new(),
            fifo_order: VecDeque::new(),
        })
    }

    /// Tamaño de página.
    pub fn page_size(&self) -> PageSize {
        self.page_size
    }

    /// Cantidad de entradas presentes.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Indica si la tabla está vacía.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Mapea una página a un frame. Devuelve la página expulsada, si existe.
    pub fn map_page(
        &mut self,
        page: PageNumber,
        entry: PageTableEntry,
    ) -> Result<Option<PageNumber>, PagingError> {
        let mut evicted = None;

        if !self.entries.contains_key(&page) {
            if let Some(capacity) = self.capacity {
                if self.entries.len() == capacity {
                    let oldest = self.fifo_order.pop_front().expect("orden FIFO consistente");
                    self.entries.remove(&oldest);
                    evicted = Some(oldest);
                }
            }

            self.fifo_order.push_back(page);
        }

        self.entries.insert(page, entry);
        Ok(evicted)
    }

    /// Traduce una página a frame después de validar presencia y permisos.
    pub fn translate_page(
        &self,
        page: PageNumber,
        access: AccessType,
    ) -> Result<FrameNumber, PagingError> {
        let entry = self
            .entries
            .get(&page)
            .ok_or_else(|| PagingError::PageFault(PageFault::new(page, FaultReason::NotMapped)))?;

        if !entry.present() {
            return Err(PagingError::PageFault(PageFault::new(
                page,
                FaultReason::NotPresent,
            )));
        }

        if !entry.permissions().allows(access) {
            return Err(PagingError::PageFault(PageFault::new(
                page,
                FaultReason::ProtectionViolation(access),
            )));
        }

        Ok(entry.frame())
    }

    /// Traduce dirección virtual a dirección física.
    pub fn translate_address(
        &self,
        virtual_address: u64,
        access: AccessType,
    ) -> Result<u64, PagingError> {
        let page = PageNumber::new(virtual_address / self.page_size.value());
        let offset = virtual_address % self.page_size.value();
        let frame = self.translate_page(page, access)?;

        frame
            .value()
            .checked_mul(self.page_size.value())
            .and_then(|base| base.checked_add(offset))
            .ok_or(PagingError::AddressOverflow)
    }
}

/// Error educativo de paging.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PagingError {
    ZeroPageSize,
    ZeroCapacity,
    AddressOverflow,
    PageFault(PageFault),
}
