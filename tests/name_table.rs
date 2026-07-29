//! NameTable composition between the Ethos and Logos namespaces.

mod support;

use name_table::{Identifier, IdentifierNamespace, Name, NameTable};

fn ethos_names() -> NameTable {
    let mut names = NameTable::new(IdentifierNamespace::Schema);
    for spelling in ["CommitSequence", "Integer", "StateDigest"] {
        names
            .intern(Name::new(spelling))
            .expect("fixture names fit the Ethos slice");
    }
    names
}

#[test]
fn composing_an_ethos_slice_preserves_identifiers_and_allocates_logos_rows() {
    let ethos = ethos_names();
    let ethos_len = ethos.len();
    assert!(
        ethos_len > 0,
        "the core-ethos fixture populates its upstream compatibility namespace"
    );

    let mut logos = NameTable::new(IdentifierNamespace::Logos)
        .compose(&ethos)
        .expect("a Logos table can borrow the Schema slice");

    for index in 0..ethos_len {
        let identifier = Identifier::Schema(index as u16);
        assert_eq!(
            logos
                .resolve(identifier)
                .expect("schema identifier resolves"),
            ethos
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
    let ethos = ethos_names();
    let mut logos = NameTable::new(IdentifierNamespace::Logos)
        .compose(&ethos)
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
