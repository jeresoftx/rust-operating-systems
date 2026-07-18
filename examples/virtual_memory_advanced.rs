use rust_operating_systems::paging::{AccessType, PagePermissions, PageSize};
use rust_operating_systems::virtual_memory::{
    AddressSpace, AddressSpaceId, Mapping, PhysicalAddress, VirtualAddress, VirtualMemoryError,
};

fn main() {
    let mut parent = AddressSpace::new(AddressSpaceId::new(1), PageSize::new(4_096).unwrap());
    parent
        .map(Mapping::new(
            VirtualAddress::new(0x1000),
            PhysicalAddress::new(0x8000),
            PagePermissions::read_write(),
        ))
        .unwrap();

    let child = parent.fork_copy_on_write(AddressSpaceId::new(2)).unwrap();
    let error = child
        .translate(VirtualAddress::new(0x1000), AccessType::Write)
        .unwrap_err();

    assert!(matches!(
        error,
        VirtualMemoryError::CopyOnWriteFault {
            reference_count: 2,
            ..
        }
    ));

    println!(
        "referencias compartidas: {}",
        child.reference_count(VirtualAddress::new(0x1000)).unwrap()
    );
}
