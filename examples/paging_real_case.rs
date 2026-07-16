use rust_operating_systems::paging::{
    AccessType, FrameNumber, PageNumber, PagePermissions, PageSize, PageTable, PageTableEntry,
};

fn main() {
    let mut working_set = PageTable::with_capacity(PageSize::new(4_096).unwrap(), 2).unwrap();
    let pages = [
        (PageNumber::new(10), FrameNumber::new(100)),
        (PageNumber::new(11), FrameNumber::new(101)),
        (PageNumber::new(12), FrameNumber::new(102)),
    ];

    for (page, frame) in pages {
        let evicted = working_set
            .map_page(
                page,
                PageTableEntry::new(frame, PagePermissions::read_write()),
            )
            .unwrap();
        println!("map {page:?} -> {frame:?}, expulsada: {evicted:?}");
    }

    let current = working_set
        .translate_page(PageNumber::new(12), AccessType::Read)
        .unwrap();
    println!("página activa en frame: {}", current.value());
}
