use filler::generator::{Generator, UuidGen};

#[test]
fn uuids_use_v4() {
    let u_gen = UuidGen::new();
    let id = u_gen.generate();
    let version = id.get_version_num();
    assert_eq!(version, 4);
}
