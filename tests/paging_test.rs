use rust_operating_systems::paging::{
    AccessType, FaultReason, FrameNumber, PageFault, PageNumber, PagePermissions, PageSize,
    PageTable, PageTableEntry, PagingError,
};

#[test]
fn page_table_translates_virtual_address_to_physical_address() {
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
}

#[test]
fn missing_page_reports_page_fault() {
    let table = PageTable::new(PageSize::new(4_096).unwrap());

    let error = table
        .translate_page(PageNumber::new(9), AccessType::Read)
        .unwrap_err();

    assert_eq!(
        error,
        PagingError::PageFault(PageFault::new(PageNumber::new(9), FaultReason::NotMapped,))
    );
}

#[test]
fn write_requires_writable_permission() {
    let mut table = PageTable::new(PageSize::new(4_096).unwrap());
    table
        .map_page(
            PageNumber::new(1),
            PageTableEntry::new(FrameNumber::new(3), PagePermissions::read_only()),
        )
        .unwrap();

    let error = table
        .translate_page(PageNumber::new(1), AccessType::Write)
        .unwrap_err();

    assert_eq!(
        error,
        PagingError::PageFault(PageFault::new(
            PageNumber::new(1),
            FaultReason::ProtectionViolation(AccessType::Write),
        ))
    );
}

#[test]
fn fifo_replacement_evicts_oldest_mapping() {
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
    assert_eq!(
        table
            .translate_page(PageNumber::new(3), AccessType::Read)
            .unwrap(),
        FrameNumber::new(30)
    );
}
