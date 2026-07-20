//! The fixed Logos vocabulary, composed NameTable slices, and eager boundary names.

use core_logos::{
    ARCHIVE, INTEGER, LogosNameBoundary, NameDerivation, STRING, VECTOR, standard_name_table,
};
use name_table::{Identifier, IdentifierNamespace, Name, NameTable};

#[test]
fn generated_standard_constants_resolve_through_their_own_immutable_slice() {
    let standards = standard_name_table();

    assert_eq!(INTEGER, Identifier::LogosStandard(0));
    assert_eq!(standards.namespace(), IdentifierNamespace::LogosStandard);
    assert_eq!(standards.resolve(INTEGER).unwrap().as_str(), "Integer");
    assert_eq!(standards.resolve(STRING).unwrap().as_str(), "String");
    assert_eq!(standards.resolve(VECTOR).unwrap().as_str(), "Vec");
    assert_eq!(standards.resolve(ARCHIVE).unwrap().as_str(), "Archive");
    assert_eq!(standards.lookup(&Name::new("Integer")), Some(INTEGER));
}

#[test]
fn boundary_borrows_schema_and_standard_slices_and_eagerly_allocates_only_logos_names() {
    let mut schema = NameTable::new(IdentifierNamespace::Schema);
    let commit_sequence = schema.intern(Name::new("CommitSequence"));
    assert_eq!(commit_sequence, Identifier::Schema(0));

    let mut boundary = LogosNameBoundary::from_schema(&schema).unwrap();
    assert_eq!(
        boundary.names().resolve(commit_sequence).unwrap().as_str(),
        "CommitSequence"
    );
    assert_eq!(
        boundary.names().resolve(INTEGER).unwrap().as_str(),
        "Integer"
    );
    assert_eq!(
        boundary.names().len(),
        0,
        "borrowed slices do not become Logos-owned rows"
    );

    let derived = boundary
        .derive_name(commit_sequence, NameDerivation::FieldName)
        .unwrap();
    assert_eq!(derived, Identifier::Logos(0));
    assert_eq!(
        boundary.names().resolve(derived).unwrap().as_str(),
        "commit_sequence"
    );
    assert_eq!(boundary.names().len(), 1);

    let screaming = boundary
        .derive_name(commit_sequence, NameDerivation::ScreamingSnakeCase)
        .unwrap();
    assert_eq!(screaming, Identifier::Logos(1));
    assert_eq!(
        boundary.names().resolve(screaming).unwrap().as_str(),
        "COMMIT_SEQUENCE"
    );
}
