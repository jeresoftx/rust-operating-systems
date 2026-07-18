use rust_operating_systems::filesystem::{
    FileSystemError, FileSystemModel, FileType, InodeId, Permissions,
};

#[test]
fn filesystem_starts_with_root_directory_and_creates_file() {
    let mut fs = FileSystemModel::new();

    let file = fs
        .create_file("/README.md", Permissions::new(true, true, false))
        .unwrap();

    assert_eq!(fs.root_id(), InodeId::new(1));
    assert_eq!(
        fs.inode(fs.root_id()).unwrap().file_type(),
        FileType::Directory
    );
    assert_eq!(fs.inode(file).unwrap().file_type(), FileType::File);
    assert_eq!(fs.resolve_path("/README.md").unwrap(), file);
}

#[test]
fn filesystem_resolves_simple_absolute_paths() {
    let mut fs = FileSystemModel::new();

    let src = fs
        .create_directory("/src", Permissions::new(true, true, true))
        .unwrap();
    let lib = fs
        .create_file("/src/lib.rs", Permissions::new(true, true, false))
        .unwrap();

    assert_eq!(fs.resolve_path("/").unwrap(), fs.root_id());
    assert_eq!(fs.resolve_path("/src").unwrap(), src);
    assert_eq!(fs.resolve_path("/src/lib.rs").unwrap(), lib);
}

#[test]
fn permissions_expose_read_write_and_execute_bits() {
    let readonly = Permissions::new(true, false, false);
    let executable = Permissions::new(true, false, true);

    assert!(readonly.can_read());
    assert!(!readonly.can_write());
    assert!(!readonly.can_execute());
    assert!(executable.can_execute());
}

#[test]
fn filesystem_rejects_invalid_names_and_directory_cycles() {
    let mut fs = FileSystemModel::new();
    let src = fs
        .create_directory("/src", Permissions::new(true, true, true))
        .unwrap();

    assert_eq!(
        fs.create_file("/src/../secreto", Permissions::new(true, false, false)),
        Err(FileSystemError::InvalidName("..".to_string()))
    );
    assert_eq!(
        fs.link_directory("/src", "self", src),
        Err(FileSystemError::DirectoryCycle {
            parent: src,
            target: src,
        })
    );
}
