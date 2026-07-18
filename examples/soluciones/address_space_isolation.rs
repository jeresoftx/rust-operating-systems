use rust_operating_systems::paging::{AccessType, PagePermissions, PageSize};
use rust_operating_systems::virtual_memory::{
    AddressSpace, AddressSpaceId, Mapping, PhysicalAddress, VirtualAddress,
};

fn main() {
    let mut a = AddressSpace::new(AddressSpaceId::new(1), PageSize::new(4_096).unwrap());
    let mut b = AddressSpace::new(AddressSpaceId::new(2), PageSize::new(4_096).unwrap());
    a.map(Mapping::new(
        VirtualAddress::new(0x4000),
        PhysicalAddress::new(0xA000),
        PagePermissions::read_write(),
    ))
    .unwrap();
    b.map(Mapping::new(
        VirtualAddress::new(0x4000),
        PhysicalAddress::new(0xB000),
        PagePermissions::read_write(),
    ))
    .unwrap();

    assert_eq!(
        a.translate(VirtualAddress::new(0x4010), AccessType::Read)
            .unwrap(),
        PhysicalAddress::new(0xA010)
    );
    assert_eq!(
        b.translate(VirtualAddress::new(0x4010), AccessType::Read)
            .unwrap(),
        PhysicalAddress::new(0xB010)
    );

    println!("la misma dirección virtual quedó aislada por espacio");
}
