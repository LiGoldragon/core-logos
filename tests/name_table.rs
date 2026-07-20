//! NameTable composition between schema and Logos slices.

mod support;

use core_logos::LogosNameBoundary;
use core_schema::FixtureFamily;
use name_table::{Identifier, IdentifierNamespace, Name, NameTable};

#[test]
fn composing_a_schema_slice_keeps_its_identifier_variant_and_allocates_only_logos_rows() {
    let family = FixtureFamily::build();
    let schema = family.universe().names();
    let schema_len = schema.len();
    assert!(
        schema_len > 0,
        "the core-schema fixture populates its own slice"
    );

    let mut logos = NameTable::new(IdentifierNamespace::Logos)
        .compose(schema)
        .expect("a Logos table can borrow the Schema slice");

    for index in 0..schema_len {
        let identifier = Identifier::Schema(index as u16);
        assert_eq!(
            logos.resolve(identifier).unwrap(),
            schema.resolve(identifier).unwrap()
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
    let boundary = LogosNameBoundary::from_schema(family.universe().names())
        .expect("compose schema and fixed Logos-standard slices");
    let mut logos = boundary.into_names();

    let item = support::commit_sequence(&mut logos);

    item.content_identity()
        .expect("content identity over a composed NameTable");
    assert_eq!(
        logos
            .resolve(item.name().expect("a newtype has a declared name"))
            .unwrap()
            .as_str(),
        "CommitSequence"
    );
}
