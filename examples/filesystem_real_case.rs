use rust_operating_systems::filesystem::{FileSystemModel, FileType, Permissions};

fn main() {
    let mut fs = FileSystemModel::new();
    let dirs = ["/src", "/tests", "/benches", "/docs"];

    for dir in dirs {
        fs.create_directory(dir, Permissions::new(true, true, true))
            .unwrap();
    }

    fs.create_file("/src/lib.rs", Permissions::new(true, true, false))
        .unwrap();
    fs.create_file(
        "/tests/filesystem_test.rs",
        Permissions::new(true, true, false),
    )
    .unwrap();
    fs.create_file(
        "/docs/10-filesystem.md",
        Permissions::new(true, true, false),
    )
    .unwrap();

    let doc = fs.resolve_path("/docs/10-filesystem.md").unwrap();
    let inode = fs.inode(doc).unwrap();

    println!("workspace educativo resuelve inode {}", inode.id().value());
    assert_eq!(inode.file_type(), FileType::File);
}
