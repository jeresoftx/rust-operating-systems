use rust_operating_systems::filesystem::{FileSystemModel, Permissions};

fn main() {
    let mut fs = FileSystemModel::new();

    fs.create_directory("/src", Permissions::new(true, true, true))
        .unwrap();
    let lib = fs
        .create_file("/src/lib.rs", Permissions::new(true, true, false))
        .unwrap();

    let resolved = fs.resolve_path("/src/lib.rs").unwrap();

    println!("/src/lib.rs -> inode {}", resolved.value());
    assert_eq!(resolved, lib);
}
