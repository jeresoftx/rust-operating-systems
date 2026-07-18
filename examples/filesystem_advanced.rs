use rust_operating_systems::filesystem::{FileSystemError, FileSystemModel, Permissions};

fn main() {
    let mut fs = FileSystemModel::new();
    let src = fs
        .create_directory("/src", Permissions::new(true, true, true))
        .unwrap();

    let invalid = fs.create_file("/src/../secreto", Permissions::new(true, false, false));
    let cycle = fs.link_directory("/src", "self", src);

    assert_eq!(invalid, Err(FileSystemError::InvalidName("..".to_string())));
    assert_eq!(
        cycle,
        Err(FileSystemError::DirectoryCycle {
            parent: src,
            target: src,
        })
    );

    println!("nombres inválidos y ciclos rechazados");
}
