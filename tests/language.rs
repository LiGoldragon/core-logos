//! The four fixture roots share one Logos grammar/declaration pair.

use core_logos::{LogosLanguage, LogosLanguageTypeIds, LogosLanguageWords};
use encoded_name_table::LocalEncodedId;
use signal_sema_translator::{VocabularyEncodedId, VocabularyRoot};
use structural_codec::LanguageDeclaration;

fn encoded(chain: &[u16]) -> VocabularyEncodedId {
    VocabularyEncodedId::new(
        VocabularyRoot::Universal,
        chain.iter().copied().map(LocalEncodedId::new).collect(),
    )
    .expect("non-empty fixture identity")
}

fn types() -> LogosLanguageTypeIds {
    LogosLanguageTypeIds {
        newtype: encoded(&[1]),
        structure: encoded(&[13]),
        enumeration: encoded(&[2]),
        visibility: encoded(&[3]),
        attributes: encoded(&[4]),
        attribute: encoded(&[5]),
        path: encoded(&[6]),
        configuration_predicate: encoded(&[7]),
        derive_group: encoded(&[8]),
        generics: encoded(&[9]),
        generic_parameter: encoded(&[10]),
        type_reference: encoded(&[11]),
        field: encoded(&[14]),
        variant: encoded(&[12]),
    }
}

#[test]
fn running_example_roots_are_one_addressed_language_declaration() {
    let language = LogosLanguage::seal(
        types(),
        LogosLanguageWords {
            public: encoded(&[20]),
            private: encoded(&[21]),
        },
    )
    .expect("grammar and landing declarations agree");
    let declaration = LanguageDeclaration::new(language.grammar(), language.landing());

    let newtype = declaration
        .verify_root(language.newtype_type())
        .expect("WireNewtype landing closure");
    let enumeration = declaration
        .verify_root(language.enumeration_type())
        .expect("Enumeration landing closure");
    let attributes = declaration
        .verify_root(language.attributes_type())
        .expect("WireAttributes landing closure");
    let structure = declaration
        .verify_root(language.struct_type())
        .expect("ParticularStruct landing closure");

    assert!(newtype.addressed_types().contains(language.newtype_type()));
    assert!(structure.addressed_types().contains(language.struct_type()));
    assert!(
        enumeration
            .addressed_types()
            .contains(language.enumeration_type())
    );
    assert!(
        attributes
            .addressed_types()
            .contains(language.attributes_type())
    );
    assert!(
        [newtype, structure, enumeration, attributes]
            .iter()
            .all(|closure| closure.addressed_types().len() > 1),
        "all four roots follow the same recursively addressed source catalogs"
    );
}

#[test]
fn source_contains_no_computed_twin_type() {
    let source = include_str!("../src/language.rs");
    for forbidden in [
        "struct Authored",
        "enum Authored",
        "struct Template",
        "enum Template",
    ] {
        assert!(
            !source.contains(forbidden),
            "source grammar/declarations must not author {forbidden}"
        );
    }
}
