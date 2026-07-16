use rust_operating_systems::paging::{
    AccessType, FaultReason, PageFault, PageNumber, PageSize, PageTable, PagingError,
};

fn main() {
    let table = PageTable::new(PageSize::new(4_096).unwrap());

    let error = table
        .translate_page(PageNumber::new(9), AccessType::Read)
        .unwrap_err();

    assert_eq!(
        error,
        PagingError::PageFault(PageFault::new(PageNumber::new(9), FaultReason::NotMapped,))
    );

    println!("page fault detectado para página no mapeada");
}
