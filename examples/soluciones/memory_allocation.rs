use rust_operating_systems::memory::{Address, AllocatorModel, Bytes, MemoryRegion};

fn main() {
    let region = MemoryRegion::new(Address::new(8_000), Bytes::new(1_024)).unwrap();
    let mut allocator = AllocatorModel::new(region);

    let first = allocator.allocate(Bytes::new(128)).unwrap();
    let second = allocator.allocate(Bytes::new(256)).unwrap();

    assert_eq!(first.start(), Address::new(8_000));
    assert_eq!(second.start(), Address::new(8_128));
    assert_eq!(allocator.allocated_bytes(), Bytes::new(384));

    println!("segunda asignación en: {}", second.start().value());
}
