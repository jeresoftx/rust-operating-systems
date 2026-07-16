use std::hint::black_box;
use std::time::Instant;

use rust_operating_systems::paging::{
    AccessType, FrameNumber, PageNumber, PagePermissions, PageSize, PageTable, PageTableEntry,
};

fn tabla_base() -> PageTable {
    let mut table = PageTable::new(PageSize::new(4_096).unwrap());
    for page in 0..128u64 {
        table
            .map_page(
                PageNumber::new(page),
                PageTableEntry::new(
                    FrameNumber::new(page + 1_000),
                    PagePermissions::read_write(),
                ),
            )
            .unwrap();
    }
    table
}

fn traducir_direcciones(iteraciones: usize) {
    let table = tabla_base();
    for i in 0..iteraciones {
        let virtual_address = ((i % 128) as u64 * 4_096) + 64;
        black_box(
            table
                .translate_address(virtual_address, AccessType::Read)
                .unwrap(),
        );
    }
}

fn fallos_no_mapeados(iteraciones: usize) {
    let table = tabla_base();
    for i in 0..iteraciones {
        let page = PageNumber::new(10_000 + i as u64);
        black_box(table.translate_page(page, AccessType::Read).unwrap_err());
    }
}

fn reemplazo_fifo(iteraciones: usize) {
    for i in 0..iteraciones {
        let mut table = PageTable::with_capacity(PageSize::new(4_096).unwrap(), 2).unwrap();
        table
            .map_page(
                PageNumber::new(i as u64),
                PageTableEntry::new(FrameNumber::new(1), PagePermissions::read_write()),
            )
            .unwrap();
        table
            .map_page(
                PageNumber::new(i as u64 + 1),
                PageTableEntry::new(FrameNumber::new(2), PagePermissions::read_write()),
            )
            .unwrap();
        black_box(
            table
                .map_page(
                    PageNumber::new(i as u64 + 2),
                    PageTableEntry::new(FrameNumber::new(3), PagePermissions::read_write()),
                )
                .unwrap(),
        );
    }
}

fn main() {
    let iteraciones = 50_000usize;

    let start = Instant::now();
    traducir_direcciones(iteraciones);
    let traduccion_elapsed = start.elapsed();

    let start = Instant::now();
    fallos_no_mapeados(iteraciones);
    let fault_elapsed = start.elapsed();

    let start = Instant::now();
    reemplazo_fifo(iteraciones);
    let fifo_elapsed = start.elapsed();

    println!("benchmark de paging (manual, std::time::Instant)");
    println!("iteraciones: {iteraciones}");
    println!("traducción de direcciones: {traduccion_elapsed:?}");
    println!("page faults no mapeados: {fault_elapsed:?}");
    println!("reemplazo FIFO: {fifo_elapsed:?}");
}
