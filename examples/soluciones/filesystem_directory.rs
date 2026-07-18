use rust_operating_systems::filesystem::{FileSystemModel, FileType, Permissions};

fn main() {
    let mut fs = FileSystemModel::new();
    let src = fs
        .create_directory("/src", Permissions::new(true, true, true))
        .unwrap();
    let lib = fs
        .create_file("/src/lib.rs", Permissions::new(true, true, false))
        .unwrap();

    assert_eq!(fs.resolve_path("/src").unwrap(), src);
    assert_eq!(fs.resolve_path("/src/lib.rs").unwrap(), lib);
    assert_eq!(fs.inode(src).unwrap().file_type(), FileType::Directory);
    assert_eq!(fs.inode(lib).unwrap().file_type(), FileType::File);

    println!("directorio resuelto");
}
