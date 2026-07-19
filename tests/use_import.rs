//! The use-import item kind archives and content-addresses like every other Core
//! item: a portable-archive round-trip is identity, it declares no name but carries
//! its attributes, and a structural edit to its import group moves the identity.

mod support;

use content_identity::PortableArchive;
use core_logos::{Attribute, ConfigurationPredicate, EncodedItem, Use, Visibility};
use name_table::NameTable;

/// Build the golden NOTA import as a stringless Core value:
///
/// ```ignore
/// #[rustfmt::skip]
/// #[cfg(feature = "nota-text")]
/// pub use nota::{NotaDecodeError, NotaEncode, NotaSource};
/// ```
fn nota_import(names: &mut NameTable) -> EncodedItem {
    EncodedItem::Use(Use {
        visibility: Visibility::Public,
        attributes: vec![
            Attribute::ToolPath(support::path(names, &["rustfmt", "skip"])),
            Attribute::Cfg(ConfigurationPredicate::Feature(support::identifier(
                names,
                "nota-text",
            ))),
        ],
        base: support::path(names, &["nota"]),
        group: vec![
            support::identifier(names, "NotaDecodeError"),
            support::identifier(names, "NotaEncode"),
            support::identifier(names, "NotaSource"),
        ],
    })
}

#[test]
fn a_use_import_round_trips_through_portable_archive() {
    let mut names = NameTable::new();
    let item = nota_import(&mut names);

    let bytes = item.to_archive_bytes().expect("serialize");
    let restored = EncodedItem::from_archive_bytes(&bytes).expect("deserialize");

    assert_eq!(item, restored);
}

#[test]
fn a_use_import_declares_no_name_but_carries_its_attributes() {
    let mut names = NameTable::new();
    let item = nota_import(&mut names);

    assert_eq!(item.name(), None, "a use import declares no name");
    assert_eq!(
        item.attributes().len(),
        2,
        "the rustfmt::skip and cfg preamble",
    );
}

#[test]
fn editing_the_import_group_moves_the_use_identity() {
    let mut names = NameTable::new();
    let item = nota_import(&mut names);
    let before = item.content_identity().expect("hash");

    // Drop the last imported name — a structural edit to the group.
    let EncodedItem::Use(mut use_import) = item else {
        panic!("use import");
    };
    use_import.group.pop();
    let after = EncodedItem::Use(use_import)
        .content_identity()
        .expect("hash");

    assert_ne!(
        before.bytes(),
        after.bytes(),
        "an import-group edit moves the content identity",
    );
}

#[test]
fn stamping_visibility_lands_on_a_use_import() {
    let mut names = NameTable::new();
    let item = nota_import(&mut names).with_visibility(Visibility::Private);
    let EncodedItem::Use(use_import) = item else {
        panic!("use import");
    };
    assert_eq!(use_import.visibility, Visibility::Private);
}
