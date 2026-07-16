use rust_operating_systems::paging::{
    AccessType, FaultReason, FrameNumber, PageNumber, PagePermissions, PageSize, PageTable,
    PageTableEntry, PagingError,
};

fn main() {
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
        PagingError::PageFault(rust_operating_systems::paging::PageFault::new(
            PageNumber::new(1),
            FaultReason::ProtectionViolation(AccessType::Write),
        ))
    );

    println!("escritura rechazada por permisos");
}
