//! Content-hash discipline: a rename is hash-stable, a structural edit moves it.

mod support;

use core_logos::{CoreItem, Newtype, TypeReference, Visibility};
use name_table::{Identifier, Name, NameTable};

/// Rebuild a table identical to `original` except that `target` resolves to
/// `replacement` — a rename, expressed as a fresh append-only table so every other
/// identifier keeps its index.
fn rename(original: &NameTable, target: Identifier, replacement: &str) -> NameTable {
    let mut renamed = NameTable::new();
    for index in 0..original.len() {
        let identifier = Identifier::new(index as u32);
        let name = if identifier == target {
            Name::new(replacement)
        } else {
            original
                .resolve(identifier)
                .expect("known identifier")
                .clone()
        };
        renamed.intern(name);
    }
    renamed
}

#[test]
fn a_rename_leaves_core_identity_unchanged() {
    let mut names = NameTable::new();
    let item = support::commit_sequence(&mut names);
    let before = item.content_identity().expect("hash before rename");

    let name = item.name().expect("a newtype has a declared name");
    let renamed = rename(&names, name, "Commitment");

    // The projected name genuinely moved between the two tables.
    assert_eq!(names.resolve(name).unwrap().as_str(), "CommitSequence");
    assert_eq!(renamed.resolve(name).unwrap().as_str(), "Commitment");

    // The Core value is untouched (it carries the identifier, never the string), so
    // its content identity does not move — the NameTable is excluded from the hash.
    let after = item.content_identity().expect("hash after rename");
    assert_eq!(before.bytes(), after.bytes());
}

#[test]
fn a_structural_edit_moves_core_identity() {
    let mut names = NameTable::new();
    let integer_wrapped = support::commit_sequence(&mut names);
    let before = integer_wrapped.content_identity().expect("hash");

    // Same name and preamble, a different wrapped type — a structural edit.
    let edited = CoreItem::Newtype(Newtype {
        visibility: Visibility::Public,
        attributes: support::golden_preamble(&mut names),
        name: integer_wrapped
            .name()
            .expect("a newtype has a declared name"),
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
fn a_visibility_edit_moves_core_identity() {
    let mut names = NameTable::new();
    let public = support::commit_sequence(&mut names);
    let before = public.content_identity().expect("hash");

    // Visibility is stored data — changing it changes the Core value's identity.
    let CoreItem::Newtype(mut newtype) = public.clone() else {
        panic!("newtype");
    };
    newtype.visibility = Visibility::Crate;
    let after = CoreItem::Newtype(newtype).content_identity().expect("hash");

    assert_ne!(before.bytes(), after.bytes());
}
