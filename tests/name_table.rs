//! NameTable composition between the schema and Logos namespaces.

mod support;

use core_schema::FixtureFamily;
use name_table::{Identifier, IdentifierNamespace, Name, NameTable};

#[test]
fn composing_a_schema_slice_preserves_identifiers_and_allocates_logos_rows() {
    let family = FixtureFamily::build();
    let schema = family.universe().names();
    let schema_len = schema.len();
    assert!(
        schema_len > 0,
        "the core-schema fixture populates its own namespace"
    );

    let mut logos = NameTable::new(IdentifierNamespace::Logos)
        .compose(schema)
        .expect("a Logos table can borrow the Schema slice");

    for index in 0..schema_len {
        let identifier = Identifier::Schema(index as u16);
        assert_eq!(
            logos
                .resolve(identifier)
                .expect("schema identifier resolves"),
            schema
                .resolve(identifier)
                .expect("schema identifier resolves")
        );
    }

    let fresh = logos
        .intern(Name::new("LogosOnlyMarker"))
        .expect("allocate Logos-only marker");
    assert_eq!(fresh, Identifier::Logos(0));
    assert_eq!(
        logos.len(),
        1,
        "borrowed Schema rows are not copied into Logos"
    );
}

#[test]
fn a_logos_item_built_over_composed_slices_is_content_addressable() {
    let family = FixtureFamily::build();
    let mut logos = NameTable::new(IdentifierNamespace::Logos)
        .compose(family.universe().names())
        .expect("compose the Schema slice");

    let item = support::commit_sequence(&mut logos);

    item.content_identity()
        .expect("content identity over composed namespace slices");
    assert_eq!(
        logos
            .resolve(item.name().expect("a newtype has a declared name"))
            .expect("Logos identifier resolves")
            .as_str(),
        "CommitSequence"
    );
}
