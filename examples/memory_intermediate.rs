use rust_operating_systems::memory::{Address, AllocatorModel, Bytes, MemoryRegion};

fn main() {
    let region = MemoryRegion::new(Address::new(10_000), Bytes::new(1_000)).unwrap();
    let mut allocator = AllocatorModel::new(region);

    let first = allocator.allocate(Bytes::new(200)).unwrap();
    let second = allocator.allocate(Bytes::new(300)).unwrap();
    let third = allocator.allocate(Bytes::new(100)).unwrap();

    allocator.free(second).unwrap();
    allocator.free(first).unwrap();

    let reused = allocator.allocate(Bytes::new(450)).unwrap();

    println!("tercer bloque sigue en: {}", third.start().value());
    println!("bloque reutilizado inicia en: {}", reused.start().value());
    println!("bytes asignados: {}", allocator.allocated_bytes().value());
}
