//! Memoria virtual.
//!
//! Objetivo de aprendizaje: entender espacios de direcciones, traducción
//! virtual-física, aislamiento, permisos y copy-on-write conceptual.

use std::collections::BTreeMap;

use crate::paging::{AccessType, PagePermissions, PageSize};

/// Dirección virtual educativa.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VirtualAddress(u64);

impl VirtualAddress {
    /// Crea una dirección virtual.
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    /// Devuelve el valor numérico.
    pub fn value(self) -> u64 {
        self.0
    }
}

/// Dirección física educativa.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysicalAddress(u64);

impl PhysicalAddress {
    /// Crea una dirección física.
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    /// Devuelve el valor numérico.
    pub fn value(self) -> u64 {
        self.0
    }
}

/// Identificador de espacio de direcciones.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AddressSpaceId(u32);

impl AddressSpaceId {
    /// Crea un identificador de espacio.
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    /// Devuelve el valor numérico.
    pub fn value(self) -> u32 {
        self.0
    }
}

/// Mapeo de una página virtual hacia una página física.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mapping {
    virtual_start: VirtualAddress,
    physical_start: PhysicalAddress,
    permissions: PagePermissions,
    copy_on_write: bool,
    reference_count: u32,
}

impl Mapping {
    /// Crea un mapeo privado.
    pub fn new(
        virtual_start: VirtualAddress,
        physical_start: PhysicalAddress,
        permissions: PagePermissions,
    ) -> Self {
        Self {
            virtual_start,
            physical_start,
            permissions,
            copy_on_write: false,
            reference_count: 1,
        }
    }

    /// Dirección virtual inicial del mapeo.
    pub fn virtual_start(&self) -> VirtualAddress {
        self.virtual_start
    }

    /// Dirección física inicial del mapeo.
    pub fn physical_start(&self) -> PhysicalAddress {
        self.physical_start
    }

    /// Permisos del mapeo.
    pub fn permissions(&self) -> PagePermissions {
        self.permissions
    }

    /// Indica si el mapeo es copy-on-write.
    pub fn copy_on_write(&self) -> bool {
        self.copy_on_write
    }

    /// Referencias educativas al frame compartido.
    pub fn reference_count(&self) -> u32 {
        self.reference_count
    }

    fn shared_copy_on_write(&self) -> Self {
        let mut mapping = self.clone();
        mapping.copy_on_write = true;
        mapping.reference_count += 1;
        mapping
    }
}

/// Espacio de direcciones educativo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddressSpace {
    id: AddressSpaceId,
    page_size: PageSize,
    mappings: BTreeMap<u64, Mapping>,
}

impl AddressSpace {
    /// Crea un espacio de direcciones vacío.
    pub fn new(id: AddressSpaceId, page_size: PageSize) -> Self {
        Self {
            id,
            page_size,
            mappings: BTreeMap::new(),
        }
    }

    /// Identificador del espacio.
    pub fn id(&self) -> AddressSpaceId {
        self.id
    }

    /// Tamaño de página.
    pub fn page_size(&self) -> PageSize {
        self.page_size
    }

    /// Cantidad de mapeos.
    pub fn len(&self) -> usize {
        self.mappings.len()
    }

    /// Indica si no hay mapeos.
    pub fn is_empty(&self) -> bool {
        self.mappings.is_empty()
    }

    /// Agrega un mapeo alineado al tamaño de página.
    pub fn map(&mut self, mapping: Mapping) -> Result<(), VirtualMemoryError> {
        self.ensure_aligned(mapping.virtual_start(), mapping.physical_start())?;
        let page = self.page_for(mapping.virtual_start());
        if self.mappings.contains_key(&page) {
            return Err(VirtualMemoryError::DuplicateMapping {
                space: self.id,
                address: mapping.virtual_start(),
            });
        }

        self.mappings.insert(page, mapping);
        Ok(())
    }

    /// Traduce una dirección virtual dentro de este espacio.
    pub fn translate(
        &self,
        virtual_address: VirtualAddress,
        access: AccessType,
    ) -> Result<PhysicalAddress, VirtualMemoryError> {
        let page = self.page_for(virtual_address);
        let offset = self.offset_for(virtual_address);
        let mapping = self
            .mappings
            .get(&page)
            .ok_or(VirtualMemoryError::UnmappedAddress {
                space: self.id,
                address: virtual_address,
            })?;

        if mapping.copy_on_write() && access == AccessType::Write {
            return Err(VirtualMemoryError::CopyOnWriteFault {
                space: self.id,
                address: mapping.virtual_start(),
                reference_count: mapping.reference_count(),
            });
        }

        if !allows(mapping.permissions(), access) {
            return Err(VirtualMemoryError::ProtectionViolation {
                space: self.id,
                address: virtual_address,
                access,
            });
        }

        mapping
            .physical_start()
            .value()
            .checked_add(offset)
            .map(PhysicalAddress::new)
            .ok_or(VirtualMemoryError::AddressOverflow)
    }

    /// Crea un hijo con mapeos compartidos copy-on-write.
    pub fn fork_copy_on_write(
        &mut self,
        child_id: AddressSpaceId,
    ) -> Result<AddressSpace, VirtualMemoryError> {
        if child_id == self.id {
            return Err(VirtualMemoryError::DuplicateAddressSpace(child_id));
        }

        let mut child = AddressSpace::new(child_id, self.page_size);
        for mapping in self.mappings.values_mut() {
            let shared = mapping.shared_copy_on_write();
            *mapping = shared.clone();
            child
                .mappings
                .insert(child.page_for(shared.virtual_start()), shared);
        }

        Ok(child)
    }

    /// Devuelve el contador de referencias educativo para la página de una dirección.
    pub fn reference_count(
        &self,
        virtual_address: VirtualAddress,
    ) -> Result<u32, VirtualMemoryError> {
        let page = self.page_for(virtual_address);
        self.mappings
            .get(&page)
            .map(Mapping::reference_count)
            .ok_or(VirtualMemoryError::UnmappedAddress {
                space: self.id,
                address: virtual_address,
            })
    }

    fn ensure_aligned(
        &self,
        virtual_start: VirtualAddress,
        physical_start: PhysicalAddress,
    ) -> Result<(), VirtualMemoryError> {
        if !virtual_start.value().is_multiple_of(self.page_size.value()) {
            return Err(VirtualMemoryError::UnalignedVirtualAddress(virtual_start));
        }

        if !physical_start
            .value()
            .is_multiple_of(self.page_size.value())
        {
            return Err(VirtualMemoryError::UnalignedPhysicalAddress(physical_start));
        }

        Ok(())
    }

    fn page_for(&self, address: VirtualAddress) -> u64 {
        address.value() / self.page_size.value()
    }

    fn offset_for(&self, address: VirtualAddress) -> u64 {
        address.value() % self.page_size.value()
    }
}

fn allows(permissions: PagePermissions, access: AccessType) -> bool {
    match access {
        AccessType::Read => permissions.readable(),
        AccessType::Write => permissions.writable(),
    }
}

/// Error educativo de memoria virtual.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VirtualMemoryError {
    UnalignedVirtualAddress(VirtualAddress),
    UnalignedPhysicalAddress(PhysicalAddress),
    DuplicateAddressSpace(AddressSpaceId),
    DuplicateMapping {
        space: AddressSpaceId,
        address: VirtualAddress,
    },
    UnmappedAddress {
        space: AddressSpaceId,
        address: VirtualAddress,
    },
    ProtectionViolation {
        space: AddressSpaceId,
        address: VirtualAddress,
        access: AccessType,
    },
    CopyOnWriteFault {
        space: AddressSpaceId,
        address: VirtualAddress,
        reference_count: u32,
    },
    AddressOverflow,
}
