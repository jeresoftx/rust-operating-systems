use rust_operating_systems::memory::{Address, AllocatorModel, Bytes, MemoryRegion};

fn main() {
    let region = MemoryRegion::new(Address::new(4_096), Bytes::new(1_024)).unwrap();
    let mut allocator = AllocatorModel::new(region);

    let allocation = allocator.allocate(Bytes::new(128)).unwrap();

    println!("inicio de región: {}", allocator.region().start().value());
    println!("asignación: {} bytes", allocation.size().value());
    println!("bytes libres: {}", allocator.free_bytes().value());
}
