//! The golden pair as CoreLogos values, with NameTable rows shown.

mod support;

use core_logos::{CoreItem, TypeReference, Visibility};
use name_table::NameTable;

/// Print the interned NameTable rows — the continuous identifier space backing the
/// stringless Core value.
fn show_rows(names: &NameTable) {
    for index in 0..names.len() {
        let identifier = name_table::Identifier::new(index as u32);
        let name = names.resolve(identifier).expect("known identifier");
        println!("  [{index}] {}", name.as_str());
    }
}

#[test]
fn the_commit_sequence_fixture_is_a_public_newtype_with_the_full_preamble() {
    let mut names = NameTable::new();
    let item = support::commit_sequence(&mut names);

    let CoreItem::Newtype(newtype) = &item else {
        panic!("CommitSequence is a newtype");
    };
    assert_eq!(newtype.visibility, Visibility::Public);
    assert_eq!(newtype.attributes.len(), 3, "the three-attribute preamble");
    assert_eq!(
        names.resolve(newtype.name).unwrap().as_str(),
        "CommitSequence",
    );

    let TypeReference::Path(wrapped) = &newtype.wrapped else {
        panic!("wraps a bare path");
    };
    assert_eq!(
        names.resolve(wrapped.segments[0]).unwrap().as_str(),
        "Integer"
    );

    println!("CommitSequence NameTable rows:");
    show_rows(&names);
}

#[test]
fn the_database_marker_fixture_carries_visibility_as_data_at_field_level() {
    let mut names = NameTable::new();
    let item = support::database_marker(&mut names);

    let CoreItem::Struct(structure) = &item else {
        panic!("DatabaseMarker is a struct");
    };
    assert_eq!(structure.visibility, Visibility::Public);
    assert_eq!(structure.fields.len(), 3);

    assert_eq!(structure.fields[0].visibility, Visibility::Public);
    assert_eq!(
        names.resolve(structure.fields[0].name).unwrap().as_str(),
        "commit_sequence",
    );
    assert_eq!(structure.fields[1].visibility, Visibility::Public);
    assert_eq!(
        names.resolve(structure.fields[1].name).unwrap().as_str(),
        "state_digest",
    );
    // Visibility is data at the field level: the private field stores `Private`.
    assert_eq!(structure.fields[2].visibility, Visibility::Private);
    assert_eq!(
        names.resolve(structure.fields[2].name).unwrap().as_str(),
        "secret_digest",
    );

    println!("DatabaseMarker NameTable rows:");
    show_rows(&names);
}
