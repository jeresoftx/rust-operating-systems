use std::hint::black_box;
use std::time::Instant;

use rust_operating_systems::paging::{AccessType, PagePermissions, PageSize};
use rust_operating_systems::virtual_memory::{
    AddressSpace, AddressSpaceId, Mapping, PhysicalAddress, VirtualAddress,
};

fn espacio_base() -> AddressSpace {
    let mut space = AddressSpace::new(AddressSpaceId::new(1), PageSize::new(4_096).unwrap());
    for page in 0..128u64 {
        space
            .map(Mapping::new(
                VirtualAddress::new(page * 4_096),
                PhysicalAddress::new((page + 1_000) * 4_096),
                PagePermissions::read_write(),
            ))
            .unwrap();
    }
    space
}

fn traducir_virtual_fisica(iteraciones: usize) {
    let space = espacio_base();
    for i in 0..iteraciones {
        let address = VirtualAddress::new(((i % 128) as u64 * 4_096) + 32);
        black_box(space.translate(address, AccessType::Read).unwrap());
    }
}

fn fallos_no_mapeados(iteraciones: usize) {
    let space = espacio_base();
    for i in 0..iteraciones {
        let address = VirtualAddress::new(1_000_000 + i as u64);
        black_box(space.translate(address, AccessType::Read).unwrap_err());
    }
}

fn fork_copy_on_write(iteraciones: usize) {
    for i in 0..iteraciones {
        let mut parent = AddressSpace::new(
            AddressSpaceId::new(i as u32 + 1),
            PageSize::new(4_096).unwrap(),
        );
        parent
            .map(Mapping::new(
                VirtualAddress::new(0x1000),
                PhysicalAddress::new(0x8000),
                PagePermissions::read_write(),
            ))
            .unwrap();
        black_box(
            parent
                .fork_copy_on_write(AddressSpaceId::new(i as u32 + 10_000))
                .unwrap(),
        );
    }
}

fn main() {
    let iteraciones = 50_000usize;

    let start = Instant::now();
    traducir_virtual_fisica(iteraciones);
    let traduccion_elapsed = start.elapsed();

    let start = Instant::now();
    fallos_no_mapeados(iteraciones);
    let fault_elapsed = start.elapsed();

    let start = Instant::now();
    fork_copy_on_write(iteraciones);
    let cow_elapsed = start.elapsed();

    println!("benchmark de memoria virtual (manual, std::time::Instant)");
    println!("iteraciones: {iteraciones}");
    println!("traducción virtual-física: {traduccion_elapsed:?}");
    println!("fallos no mapeados: {fault_elapsed:?}");
    println!("fork copy-on-write: {cow_elapsed:?}");
}
