use rust_operating_systems::paging::{AccessType, PagePermissions, PageSize};
use rust_operating_systems::virtual_memory::{
    AddressSpace, AddressSpaceId, Mapping, PhysicalAddress, VirtualAddress,
};

fn main() {
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

    println!("dirección física: 0x{:x}", physical.value());
}
