use rust_operating_systems::filesystem::{FileSystemModel, FileType, Permissions};

fn main() {
    let mut fs = FileSystemModel::new();
    let readme = fs
        .create_file("/README.md", Permissions::new(true, true, false))
        .unwrap();

    assert_eq!(fs.resolve_path("/README.md").unwrap(), readme);
    assert_eq!(fs.inode(readme).unwrap().file_type(), FileType::File);

    println!("inodo resuelto");
}
