use rust_operating_systems::paging::{
    AccessType, FrameNumber, PageNumber, PagePermissions, PageSize, PageTable, PageTableEntry,
};

fn main() {
    let mut table = PageTable::new(PageSize::new(4_096).unwrap());
    table
        .map_page(
            PageNumber::new(2),
            PageTableEntry::new(FrameNumber::new(7), PagePermissions::read_write()),
        )
        .unwrap();

    let physical = table
        .translate_address(8_192 + 123, AccessType::Read)
        .unwrap();

    assert_eq!(physical, 28_672 + 123);
    println!("traducción correcta: {physical}");
}
