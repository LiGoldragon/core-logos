//! Content-hash discipline: a rename is hash-stable, a structural edit moves it.

mod support;

use core_logos::{EncodedItem, Newtype, TypeReference, Visibility};
use name_table::{Identifier, Name, NameTable};

/// Rebuild a table identical to `original` except that `target` resolves to
/// `replacement` — a rename, expressed as a fresh Logos slice so every other
/// identifier keeps its local allocation.
fn rename(original: &NameTable, target: Identifier, replacement: &str) -> NameTable {
    let mut renamed = NameTable::new(name_table::IdentifierNamespace::Logos);
    for index in 0..original.len() {
        let identifier = Identifier::Logos(index as u16);
        let name = if identifier == target {
            Name::new(replacement)
        } else {
            original
                .resolve(identifier)
                .expect("known identifier")
                .clone()
        };
        renamed.intern(name).expect("rebuild Logos fixture");
    }
    renamed
}

#[test]
fn a_rename_leaves_encoded_identity_unchanged() {
    let mut names = NameTable::new(name_table::IdentifierNamespace::Logos);
    let item = support::commit_sequence(&mut names);
    let before = item.content_identity().expect("hash before rename");

    let name = item.name().expect("a newtype has a declared name");
    let renamed = rename(&names, name, "Commitment");

    // The projected name genuinely moved between the two tables.
    assert_eq!(names.resolve(name).unwrap().as_str(), "CommitSequence");
    assert_eq!(renamed.resolve(name).unwrap().as_str(), "Commitment");

    // The encoded form value is untouched (it carries the identifier, never the string), so
    // its content identity does not move — the NameTable is excluded from the hash.
    let after = item.content_identity().expect("hash after rename");
    assert_eq!(before.bytes(), after.bytes());
}

#[test]
fn a_structural_edit_moves_encoded_identity() {
    let mut names = NameTable::new(name_table::IdentifierNamespace::Logos);
    let integer_wrapped = support::commit_sequence(&mut names);
    let before = integer_wrapped.content_identity().expect("hash");

    // Same name and preamble, a different wrapped type — a structural edit.
    let edited = EncodedItem::Newtype(Newtype {
        visibility: Visibility::Public,
        attributes: support::golden_preamble(&mut names),
        name: integer_wrapped
            .name()
            .expect("a newtype has a declared name"),
        wrapped_visibility: Visibility::Private,
        wrapped: TypeReference::Path(support::path(&mut names, &["Boolean"])),
    });
    let after = edited.content_identity().expect("hash");

    assert_ne!(
        before.bytes(),
        after.bytes(),
        "the hash is structure-sensitive",
    );
}

#[test]
fn a_visibility_edit_moves_encoded_identity() {
    let mut names = NameTable::new(name_table::IdentifierNamespace::Logos);
    let public = support::commit_sequence(&mut names);
    let before = public.content_identity().expect("hash");

    // Visibility is stored data — changing it changes the encoded form value's identity.
    let EncodedItem::Newtype(mut newtype) = public.clone() else {
        panic!("newtype");
    };
    newtype.visibility = Visibility::Crate;
    let after = EncodedItem::Newtype(newtype)
        .content_identity()
        .expect("hash");

    assert_ne!(before.bytes(), after.bytes());
}

#[test]
fn a_tuple_field_visibility_edit_moves_encoded_identity() {
    let mut names = NameTable::new(name_table::IdentifierNamespace::Logos);
    let private_field = support::commit_sequence(&mut names);
    let before = private_field.content_identity().expect("hash");

    // The tuple field's own visibility is stored data exactly as the item's is:
    // promoting `CommitSequence(Integer)` to `CommitSequence(pub Integer)` is a
    // structural edit, so the encoded form value's identity moves.
    let EncodedItem::Newtype(mut newtype) = private_field.clone() else {
        panic!("newtype");
    };
    newtype.wrapped_visibility = Visibility::Public;
    let after = EncodedItem::Newtype(newtype)
        .content_identity()
        .expect("hash");

    assert_ne!(before.bytes(), after.bytes());
}
