use rust_operating_systems::filesystem::{FileSystemModel, FileType, Permissions};

fn main() {
    let mut fs = FileSystemModel::new();

    let readme = fs
        .create_file("/README.md", Permissions::new(true, true, false))
        .unwrap();

    println!("raíz: inode {}", fs.root_id().value());
    println!("README.md: {:?}", fs.inode(readme).unwrap().file_type());
    assert_eq!(fs.inode(readme).unwrap().file_type(), FileType::File);
}
