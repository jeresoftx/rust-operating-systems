use rust_operating_systems::memory::{Address, AllocatorModel, Bytes, MemoryRegion};

fn main() {
    let region = MemoryRegion::new(Address::new(12_000), Bytes::new(1_000)).unwrap();
    let mut allocator = AllocatorModel::new(region);

    let a = allocator.allocate(Bytes::new(200)).unwrap();
    let _b = allocator.allocate(Bytes::new(200)).unwrap();
    let c = allocator.allocate(Bytes::new(200)).unwrap();
    let _d = allocator.allocate(Bytes::new(400)).unwrap();

    allocator.free(a).unwrap();
    allocator.free(c).unwrap();

    assert_eq!(allocator.free_bytes(), Bytes::new(400));
    assert_eq!(allocator.largest_free_block(), Bytes::new(200));
    assert_eq!(allocator.external_fragmentation_bytes(), Bytes::new(200));

    println!(
        "fragmentación externa: {} bytes",
        allocator.external_fragmentation_bytes().value()
    );
}
