use rust_operating_systems::memory::{Address, Bytes, MemoryRegion};

fn main() {
    let region = MemoryRegion::new(Address::new(4_096), Bytes::new(1_024)).unwrap();

    assert_eq!(region.start(), Address::new(4_096));
    assert_eq!(region.size(), Bytes::new(1_024));
    assert_eq!(region.end_exclusive(), Address::new(5_120));

    println!(
        "región válida: [{}..{})",
        region.start().value(),
        region.end_exclusive().value()
    );
}
