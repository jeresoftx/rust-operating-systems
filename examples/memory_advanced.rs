use rust_operating_systems::memory::{Address, AllocatorModel, Bytes, MemoryRegion};

fn main() {
    let region = MemoryRegion::new(Address::new(20_000), Bytes::new(1_000)).unwrap();
    let mut allocator = AllocatorModel::new(region);

    let a = allocator.allocate(Bytes::new(200)).unwrap();
    let _b = allocator.allocate(Bytes::new(200)).unwrap();
    let c = allocator.allocate(Bytes::new(200)).unwrap();
    let _d = allocator.allocate(Bytes::new(400)).unwrap();

    allocator.free(a).unwrap();
    allocator.free(c).unwrap();

    println!("bytes libres: {}", allocator.free_bytes().value());
    println!("mayor hueco: {}", allocator.largest_free_block().value());
    println!(
        "fragmentación externa: {}",
        allocator.external_fragmentation_bytes().value()
    );
}
