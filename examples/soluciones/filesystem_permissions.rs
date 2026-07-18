use rust_operating_systems::filesystem::Permissions;

fn main() {
    let readonly = Permissions::new(true, false, false);
    let executable = Permissions::new(true, false, true);

    assert!(readonly.can_read());
    assert!(!readonly.can_write());
    assert!(!readonly.can_execute());
    assert!(executable.can_execute());

    println!("permisos resueltos");
}
