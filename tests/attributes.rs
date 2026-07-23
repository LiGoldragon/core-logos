//! Attribute-order preservation: the preamble is an ordered vector whose order
//! round-trips through the archive and is load-bearing for content identity.

mod support;

use content_identity::PortableArchive;
use core_logos::{Attribute, EncodedItem};
use name_table::NameTable;

#[test]
fn the_preamble_is_three_ordered_entries() {
    let mut names = NameTable::new(name_table::IdentifierNamespace::Logos);
    let item = support::commit_sequence(&mut names);
    let attributes = item.attributes();

    assert_eq!(attributes.len(), 3);
    assert!(matches!(attributes[0], Attribute::ToolPath(_)));
    assert!(matches!(attributes[1], Attribute::Configuration(_)));
    assert!(matches!(attributes[2], Attribute::Derive(_)));
}

#[test]
fn attribute_order_round_trips_through_portable_archive() {
    let mut names = NameTable::new(name_table::IdentifierNamespace::Logos);
    let item = support::commit_sequence(&mut names);
    let original = item.attributes().to_vec();

    let bytes = item.to_archive_bytes().expect("serialize");
    let restored = EncodedItem::from_archive_bytes(&bytes).expect("deserialize");

    assert_eq!(restored.attributes(), original.as_slice());
}

#[test]
fn reordering_attributes_moves_content_identity() {
    let mut names = NameTable::new(name_table::IdentifierNamespace::Logos);
    let item = support::commit_sequence(&mut names);
    let before = item.content_identity().expect("hash");

    let EncodedItem::Newtype(mut newtype) = item.clone() else {
        panic!("newtype");
    };
    newtype.attributes.reverse();
    let after = EncodedItem::Newtype(newtype)
        .content_identity()
        .expect("hash");

    assert_ne!(
        before.bytes(),
        after.bytes(),
        "attribute order is part of the encoded value",
    );
}
