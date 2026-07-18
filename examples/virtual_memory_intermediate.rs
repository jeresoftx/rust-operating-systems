use rust_operating_systems::paging::{AccessType, PagePermissions, PageSize};
use rust_operating_systems::virtual_memory::{
    AddressSpace, AddressSpaceId, Mapping, PhysicalAddress, VirtualAddress,
};

fn main() {
    let mut frontend = AddressSpace::new(AddressSpaceId::new(1), PageSize::new(4_096).unwrap());
    let mut worker = AddressSpace::new(AddressSpaceId::new(2), PageSize::new(4_096).unwrap());

    frontend
        .map(Mapping::new(
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

    let frontend_physical = frontend
        .translate(VirtualAddress::new(0x4010), AccessType::Read)
        .unwrap();
    let worker_physical = worker
        .translate(VirtualAddress::new(0x4010), AccessType::Read)
        .unwrap();

    println!("frontend: 0x{:x}", frontend_physical.value());
    println!("worker: 0x{:x}", worker_physical.value());
}
