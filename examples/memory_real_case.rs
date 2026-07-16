use rust_operating_systems::memory::{Address, AllocatorModel, Bytes, MemoryRegion};

fn main() {
    let arena = MemoryRegion::new(Address::new(1_000_000), Bytes::new(16_384)).unwrap();
    let mut requests = AllocatorModel::new(arena);

    let headers = requests.allocate(Bytes::new(2_048)).unwrap();
    let body = requests.allocate(Bytes::new(8_192)).unwrap();
    let scratch = requests.allocate(Bytes::new(1_024)).unwrap();

    requests.free(headers).unwrap();
    let retry_buffer = requests.allocate(Bytes::new(1_536)).unwrap();

    println!("cuerpo en: {}", body.start().value());
    println!("scratch en: {}", scratch.start().value());
    println!("buffer de reintento en: {}", retry_buffer.start().value());
    println!("bytes libres: {}", requests.free_bytes().value());
}
