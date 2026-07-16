use rust_operating_systems::paging::{
    AccessType, FrameNumber, PageNumber, PagePermissions, PageSize, PageTable, PageTableEntry,
};

fn main() {
    let mut table = PageTable::with_capacity(PageSize::new(4_096).unwrap(), 2).unwrap();
    table
        .map_page(
            PageNumber::new(1),
            PageTableEntry::new(FrameNumber::new(10), PagePermissions::read_write()),
        )
        .unwrap();
    table
        .map_page(
            PageNumber::new(2),
            PageTableEntry::new(FrameNumber::new(20), PagePermissions::read_write()),
        )
        .unwrap();
    let evicted = table
        .map_page(
            PageNumber::new(3),
            PageTableEntry::new(FrameNumber::new(30), PagePermissions::read_write()),
        )
        .unwrap();

    assert_eq!(evicted, Some(PageNumber::new(1)));
    assert!(table
        .translate_page(PageNumber::new(1), AccessType::Read)
        .is_err());

    println!("página expulsada por FIFO: {:?}", evicted);
}
