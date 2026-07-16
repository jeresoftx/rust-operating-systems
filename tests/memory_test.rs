use rust_operating_systems::memory::{
    Address, Allocation, AllocatorModel, Bytes, MemoryError, MemoryRegion,
};

#[test]
fn memory_region_records_start_size_and_end() {
    let region = MemoryRegion::new(Address::new(1_000), Bytes::new(256)).unwrap();

    assert_eq!(region.start(), Address::new(1_000));
    assert_eq!(region.size(), Bytes::new(256));
    assert_eq!(region.end_exclusive(), Address::new(1_256));
    assert!(region.contains(Address::new(1_128)));
    assert!(!region.contains(Address::new(1_256)));
}

#[test]
fn zero_sized_region_is_rejected() {
    let error = MemoryRegion::new(Address::new(0), Bytes::new(0)).unwrap_err();

    assert_eq!(error, MemoryError::ZeroSizedRegion);
}

#[test]
fn first_fit_allocates_inside_region() {
    let region = MemoryRegion::new(Address::new(4_096), Bytes::new(1_024)).unwrap();
    let mut allocator = AllocatorModel::new(region);

    let allocation = allocator.allocate(Bytes::new(128)).unwrap();

    assert_eq!(
        allocation,
        Allocation::new(Address::new(4_096), Bytes::new(128))
    );
    assert_eq!(allocator.free_bytes(), Bytes::new(896));
    assert_eq!(allocator.allocated_bytes(), Bytes::new(128));
}

#[test]
fn allocator_reuses_freed_space_with_coalescing() {
    let region = MemoryRegion::new(Address::new(10_000), Bytes::new(1_000)).unwrap();
    let mut allocator = AllocatorModel::new(region);
    let first = allocator.allocate(Bytes::new(200)).unwrap();
    let second = allocator.allocate(Bytes::new(300)).unwrap();
    let third = allocator.allocate(Bytes::new(100)).unwrap();

    allocator.free(second).unwrap();
    allocator.free(first).unwrap();
    let reused = allocator.allocate(Bytes::new(450)).unwrap();

    assert_eq!(reused.start(), Address::new(10_000));
    assert_eq!(reused.size(), Bytes::new(450));
    assert_eq!(allocator.free(third), Ok(()));
}

#[test]
fn allocator_reports_external_fragmentation() {
    let region = MemoryRegion::new(Address::new(20_000), Bytes::new(1_000)).unwrap();
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
}

#[test]
fn allocation_larger_than_any_free_block_is_rejected() {
    let region = MemoryRegion::new(Address::new(30_000), Bytes::new(256)).unwrap();
    let mut allocator = AllocatorModel::new(region);

    let error = allocator.allocate(Bytes::new(512)).unwrap_err();

    assert_eq!(
        error,
        MemoryError::OutOfMemory {
            requested: Bytes::new(512),
            available: Bytes::new(256),
        }
    );
}
