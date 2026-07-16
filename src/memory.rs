//! Memoria.
//!
//! Objetivo de aprendizaje: entender direcciones, regiones, asignación,
//! liberación, huecos libres y fragmentación externa.

/// Dirección educativa.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Address(u64);

impl Address {
    /// Crea una dirección.
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    /// Devuelve el valor numérico.
    pub fn value(self) -> u64 {
        self.0
    }

    fn checked_add(self, bytes: Bytes) -> Option<Self> {
        self.0.checked_add(bytes.value()).map(Self)
    }
}

/// Cantidad de bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Bytes(u64);

impl Bytes {
    /// Crea una cantidad de bytes.
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    /// Devuelve el valor numérico.
    pub fn value(self) -> u64 {
        self.0
    }
}

/// Región continua de memoria.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemoryRegion {
    start: Address,
    size: Bytes,
}

impl MemoryRegion {
    /// Crea una región con tamaño positivo.
    pub fn new(start: Address, size: Bytes) -> Result<Self, MemoryError> {
        if size.value() == 0 {
            return Err(MemoryError::ZeroSizedRegion);
        }

        if start.checked_add(size).is_none() {
            return Err(MemoryError::AddressOverflow);
        }

        Ok(Self { start, size })
    }

    /// Dirección inicial.
    pub fn start(&self) -> Address {
        self.start
    }

    /// Tamaño de la región.
    pub fn size(&self) -> Bytes {
        self.size
    }

    /// Dirección final exclusiva.
    pub fn end_exclusive(&self) -> Address {
        self.start
            .checked_add(self.size)
            .expect("validado en MemoryRegion::new")
    }

    /// Indica si la dirección cae dentro de la región.
    pub fn contains(&self, address: Address) -> bool {
        self.start <= address && address < self.end_exclusive()
    }
}

/// Bloque asignado.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Allocation {
    start: Address,
    size: Bytes,
}

impl Allocation {
    /// Crea una asignación educativa.
    pub fn new(start: Address, size: Bytes) -> Self {
        Self { start, size }
    }

    /// Dirección inicial.
    pub fn start(&self) -> Address {
        self.start
    }

    /// Tamaño asignado.
    pub fn size(&self) -> Bytes {
        self.size
    }

    fn end_exclusive(&self) -> Address {
        self.start
            .checked_add(self.size)
            .expect("asignación válida")
    }
}

/// Asignador educativo first-fit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllocatorModel {
    region: MemoryRegion,
    free: Vec<MemoryRegion>,
    allocated: Vec<Allocation>,
}

impl AllocatorModel {
    /// Crea un asignador con toda la región libre.
    pub fn new(region: MemoryRegion) -> Self {
        Self {
            region,
            free: vec![region],
            allocated: Vec::new(),
        }
    }

    /// Región administrada.
    pub fn region(&self) -> MemoryRegion {
        self.region
    }

    /// Asigna con política first-fit.
    pub fn allocate(&mut self, size: Bytes) -> Result<Allocation, MemoryError> {
        if size.value() == 0 {
            return Err(MemoryError::ZeroSizedAllocation);
        }

        let Some(index) = self.free.iter().position(|region| region.size() >= size) else {
            return Err(MemoryError::OutOfMemory {
                requested: size,
                available: self.free_bytes(),
            });
        };

        let free_region = self.free[index];
        let allocation = Allocation::new(free_region.start(), size);
        let remaining = free_region.size().value() - size.value();

        if remaining == 0 {
            self.free.remove(index);
        } else {
            self.free[index] = MemoryRegion::new(allocation.end_exclusive(), Bytes::new(remaining))
                .expect("remanente válido");
        }

        self.allocated.push(allocation);
        Ok(allocation)
    }

    /// Libera una asignación y fusiona huecos contiguos.
    pub fn free(&mut self, allocation: Allocation) -> Result<(), MemoryError> {
        let Some(index) = self
            .allocated
            .iter()
            .position(|current| *current == allocation)
        else {
            return Err(MemoryError::UnknownAllocation(allocation));
        };

        self.allocated.remove(index);
        self.free
            .push(MemoryRegion::new(allocation.start(), allocation.size()).expect("válida"));
        self.coalesce_free_regions();
        Ok(())
    }

    /// Bytes libres totales.
    pub fn free_bytes(&self) -> Bytes {
        Bytes::new(self.free.iter().map(|region| region.size().value()).sum())
    }

    /// Bytes asignados totales.
    pub fn allocated_bytes(&self) -> Bytes {
        Bytes::new(
            self.allocated
                .iter()
                .map(|allocation| allocation.size().value())
                .sum(),
        )
    }

    /// Tamaño del hueco libre más grande.
    pub fn largest_free_block(&self) -> Bytes {
        self.free
            .iter()
            .map(MemoryRegion::size)
            .max()
            .unwrap_or(Bytes::new(0))
    }

    /// Fragmentación externa: memoria libre total que no está en el mayor hueco.
    pub fn external_fragmentation_bytes(&self) -> Bytes {
        Bytes::new(self.free_bytes().value() - self.largest_free_block().value())
    }

    fn coalesce_free_regions(&mut self) {
        self.free.sort_by_key(MemoryRegion::start);
        let mut coalesced: Vec<MemoryRegion> = Vec::new();

        for region in self.free.drain(..) {
            if let Some(last) = coalesced.last_mut() {
                if last.end_exclusive() == region.start() {
                    let size = last.size().value() + region.size().value();
                    *last = MemoryRegion::new(last.start(), Bytes::new(size))
                        .expect("región fusionada válida");
                    continue;
                }
            }

            coalesced.push(region);
        }

        self.free = coalesced;
    }
}

/// Error educativo de memoria.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemoryError {
    ZeroSizedRegion,
    ZeroSizedAllocation,
    AddressOverflow,
    OutOfMemory { requested: Bytes, available: Bytes },
    UnknownAllocation(Allocation),
}
