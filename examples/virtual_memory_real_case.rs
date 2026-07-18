use rust_operating_systems::paging::{AccessType, PagePermissions, PageSize};
use rust_operating_systems::virtual_memory::{
    AddressSpace, AddressSpaceId, Mapping, PhysicalAddress, VirtualAddress,
};

fn main() {
    let mut shell = AddressSpace::new(AddressSpaceId::new(10), PageSize::new(4_096).unwrap());
    shell
        .map(Mapping::new(
            VirtualAddress::new(0x1000),
            PhysicalAddress::new(0x20_000),
            PagePermissions::read_write(),
        ))
        .unwrap();
    shell
        .map(Mapping::new(
            VirtualAddress::new(0x2000),
            PhysicalAddress::new(0x21_000),
            PagePermissions::read_write(),
        ))
        .unwrap();

    let child = shell.fork_copy_on_write(AddressSpaceId::new(11)).unwrap();
    let read = child
        .translate(VirtualAddress::new(0x2018), AccessType::Read)
        .unwrap();

    println!("fork conceptual lee 0x{:x}", read.value());
    println!(
        "referencias en página compartida: {}",
        child.reference_count(VirtualAddress::new(0x2000)).unwrap()
    );
}
