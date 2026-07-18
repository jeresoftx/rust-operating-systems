use rust_operating_systems::paging::{AccessType, PagePermissions, PageSize};
use rust_operating_systems::virtual_memory::{
    AddressSpace, AddressSpaceId, Mapping, PhysicalAddress, VirtualAddress, VirtualMemoryError,
};

#[test]
fn address_space_translates_virtual_address_to_physical_address() {
    let mut space = AddressSpace::new(AddressSpaceId::new(1), PageSize::new(4_096).unwrap());
    space
        .map(Mapping::new(
            VirtualAddress::new(0x2000),
            PhysicalAddress::new(0x9000),
            PagePermissions::read_write(),
        ))
        .unwrap();

    let physical = space
        .translate(VirtualAddress::new(0x2123), AccessType::Read)
        .unwrap();

    assert_eq!(physical, PhysicalAddress::new(0x9123));
}

#[test]
fn equal_virtual_addresses_are_isolated_between_address_spaces() {
    let mut web = AddressSpace::new(AddressSpaceId::new(1), PageSize::new(4_096).unwrap());
    let mut worker = AddressSpace::new(AddressSpaceId::new(2), PageSize::new(4_096).unwrap());
    web.map(Mapping::new(
        VirtualAddress::new(0x4000),
        PhysicalAddress::new(0xA000),
        PagePermissions::read_write(),
    ))
    .unwrap();
    worker
        .map(Mapping::new(
            VirtualAddress::new(0x4000),
            PhysicalAddress::new(0xB000),
            PagePermissions::read_write(),
        ))
        .unwrap();

    assert_eq!(
        web.translate(VirtualAddress::new(0x4010), AccessType::Read)
            .unwrap(),
        PhysicalAddress::new(0xA010)
    );
    assert_eq!(
        worker
            .translate(VirtualAddress::new(0x4010), AccessType::Read)
            .unwrap(),
        PhysicalAddress::new(0xB010)
    );
}

#[test]
fn unmapped_virtual_address_is_rejected() {
    let space = AddressSpace::new(AddressSpaceId::new(7), PageSize::new(4_096).unwrap());

    let error = space
        .translate(VirtualAddress::new(0xDEAD), AccessType::Read)
        .unwrap_err();

    assert_eq!(
        error,
        VirtualMemoryError::UnmappedAddress {
            space: AddressSpaceId::new(7),
            address: VirtualAddress::new(0xDEAD),
        }
    );
}

#[test]
fn fork_marks_mappings_as_copy_on_write_with_reference_count() {
    let mut parent = AddressSpace::new(AddressSpaceId::new(1), PageSize::new(4_096).unwrap());
    parent
        .map(Mapping::new(
            VirtualAddress::new(0x1000),
            PhysicalAddress::new(0x8000),
            PagePermissions::read_write(),
        ))
        .unwrap();

    let child = parent.fork_copy_on_write(AddressSpaceId::new(2)).unwrap();

    assert_eq!(
        parent.reference_count(VirtualAddress::new(0x1000)).unwrap(),
        2
    );
    assert_eq!(
        child.reference_count(VirtualAddress::new(0x1000)).unwrap(),
        2
    );
    assert_eq!(
        child
            .translate(VirtualAddress::new(0x1000), AccessType::Write)
            .unwrap_err(),
        VirtualMemoryError::CopyOnWriteFault {
            space: AddressSpaceId::new(2),
            address: VirtualAddress::new(0x1000),
            reference_count: 2,
        }
    );
}
