use std::hint::black_box;
use std::time::Instant;

use rust_operating_systems::memory::{Address, AllocatorModel, Bytes, MemoryRegion};

fn asignacion_simple(iteraciones: usize) {
    for i in 0..iteraciones {
        let region =
            MemoryRegion::new(Address::new((i as u64) * 10_000), Bytes::new(1_024)).unwrap();
        let mut allocator = AllocatorModel::new(region);
        let allocation = allocator.allocate(Bytes::new(128)).unwrap();
        black_box(allocation);
        black_box(allocator.free_bytes());
    }
}

fn liberar_con_coalescing(iteraciones: usize) {
    for i in 0..iteraciones {
        let region =
            MemoryRegion::new(Address::new((i as u64) * 10_000), Bytes::new(1_000)).unwrap();
        let mut allocator = AllocatorModel::new(region);
        let first = allocator.allocate(Bytes::new(200)).unwrap();
        let second = allocator.allocate(Bytes::new(300)).unwrap();
        let _third = allocator.allocate(Bytes::new(100)).unwrap();

        allocator.free(second).unwrap();
        allocator.free(first).unwrap();
        black_box(allocator.largest_free_block());
    }
}

fn consultar_fragmentacion(iteraciones: usize) {
    for i in 0..iteraciones {
        let region =
            MemoryRegion::new(Address::new((i as u64) * 10_000), Bytes::new(1_000)).unwrap();
        let mut allocator = AllocatorModel::new(region);
        let a = allocator.allocate(Bytes::new(200)).unwrap();
        let _b = allocator.allocate(Bytes::new(200)).unwrap();
        let c = allocator.allocate(Bytes::new(200)).unwrap();
        let _d = allocator.allocate(Bytes::new(400)).unwrap();

        allocator.free(a).unwrap();
        allocator.free(c).unwrap();
        black_box(allocator.external_fragmentation_bytes());
    }
}

fn main() {
    let iteraciones = 50_000usize;

    let start = Instant::now();
    asignacion_simple(iteraciones);
    let simple_elapsed = start.elapsed();

    let start = Instant::now();
    liberar_con_coalescing(iteraciones);
    let coalescing_elapsed = start.elapsed();

    let start = Instant::now();
    consultar_fragmentacion(iteraciones);
    let fragmentacion_elapsed = start.elapsed();

    println!("benchmark de memoria (manual, std::time::Instant)");
    println!("iteraciones: {iteraciones}");
    println!("asignación simple: {simple_elapsed:?}");
    println!("liberar con coalescing: {coalescing_elapsed:?}");
    println!("consultar fragmentación: {fragmentacion_elapsed:?}");
}
