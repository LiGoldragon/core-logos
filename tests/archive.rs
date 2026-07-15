//! Portable-archive round-trips for the golden pair.

mod support;

use content_identity::PortableArchive;
use core_logos::CoreItem;
use name_table::NameTable;

#[test]
fn the_commit_sequence_newtype_round_trips_through_portable_archive() {
    let mut names = NameTable::new();
    let item = support::commit_sequence(&mut names);

    let bytes = item.to_archive_bytes().expect("serialize");
    let restored = CoreItem::from_archive_bytes(&bytes).expect("deserialize");

    assert_eq!(item, restored);
}

#[test]
fn the_database_marker_struct_round_trips_through_portable_archive() {
    let mut names = NameTable::new();
    let item = support::database_marker(&mut names);

    let bytes = item.to_archive_bytes().expect("serialize");
    let restored = CoreItem::from_archive_bytes(&bytes).expect("deserialize");

    assert_eq!(item, restored);
}

#[test]
fn a_round_tripped_item_keeps_its_content_identity() {
    let mut names = NameTable::new();
    let item = support::database_marker(&mut names);
    let identity = item.content_identity().expect("hash");

    let bytes = item.to_archive_bytes().expect("serialize");
    let restored = CoreItem::from_archive_bytes(&bytes).expect("deserialize");

    assert_eq!(
        identity.bytes(),
        restored.content_identity().unwrap().bytes()
    );
}
