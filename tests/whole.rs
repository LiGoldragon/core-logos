use capsule_content_identity::{IdentityHasher, PortableArchive};
use core_logos::{
    WholeLogos, WholeLogosContentIdentity, WholeLogosItem, WholeLogosNewtype, WholeLogosVisibility,
};
use encoded_name_table::LocalEncodedId;
use signal_sema_translator::{VocabularyEncodedId, VocabularyRoot};

// Version 1 fixes the first positional WholeLogos archive and its pure-content
// hash. An intentional archive change replaces this under a new versioned name.
const WHOLE_LOGOS_ARCHIVE_V1_IDENTITY: [u8; 32] = [
    0x24, 0x00, 0xe6, 0x0a, 0x77, 0xcf, 0x44, 0x20, 0x36, 0xe7, 0x88, 0xcd, 0xd4, 0xd9, 0x1a, 0xaf,
    0x7b, 0x2f, 0x44, 0x64, 0x63, 0xec, 0xce, 0xd6, 0xa8, 0xf5, 0x01, 0xc2, 0xab, 0x23, 0x8d, 0x29,
];

fn encoded_id(root: VocabularyRoot, chain: &[u16]) -> VocabularyEncodedId {
    VocabularyEncodedId::new(
        root,
        chain.iter().copied().map(LocalEncodedId::new).collect(),
    )
    .expect("non-empty fixture encoded ID")
}

fn newtype(name: &[u16], wrapped: &[u16]) -> WholeLogosItem {
    WholeLogosItem::Newtype(WholeLogosNewtype::new(
        WholeLogosVisibility::Public,
        encoded_id(VocabularyRoot::Universal, name),
        WholeLogosVisibility::Private,
        encoded_id(VocabularyRoot::Universal, wrapped),
    ))
}

fn hash(whole: &WholeLogos) -> [u8; 32] {
    *whole
        .content_identity()
        .expect("whole content identity")
        .content_addressed_hash()
        .bytes()
}

#[test]
fn ordered_typed_items_and_complete_encoded_id_chains_survive_archive() {
    let original = WholeLogos::new(vec![newtype(&[17, 23, 41], &[3, 5])]);
    let bytes = original.to_archive_bytes().expect("archive whole Logos");
    let restored = WholeLogos::from_archive_bytes(&bytes).expect("restore whole Logos");

    assert_eq!(restored, original);
    let WholeLogosItem::Newtype(newtype) = &restored.items()[0];
    assert_eq!(
        newtype
            .name()
            .chain()
            .iter()
            .map(|local| local.value())
            .collect::<Vec<_>>(),
        vec![17, 23, 41],
    );
    assert_eq!(
        newtype
            .wrapped()
            .chain()
            .iter()
            .map(|local| local.value())
            .collect::<Vec<_>>(),
        vec![3, 5],
    );
    assert_eq!(newtype.name().root_variant(), &VocabularyRoot::Universal);
    assert_eq!(newtype.wrapped().root_variant(), &VocabularyRoot::Universal);
}

#[test]
fn item_order_is_part_of_whole_content_identity() {
    let first = newtype(&[1, 1], &[7]);
    let second = newtype(&[1, 2], &[7]);

    let in_source_order = WholeLogos::new(vec![first.clone(), second.clone()]);
    let reversed = WholeLogos::new(vec![second, first]);

    assert_ne!(hash(&in_source_order), hash(&reversed));
}

#[test]
fn a_behavior_affecting_item_mutation_moves_whole_content_identity() {
    let integer = WholeLogos::new(vec![newtype(&[1, 1], &[3])]);
    let boolean = WholeLogos::new(vec![newtype(&[1, 1], &[4])]);

    assert_ne!(hash(&integer), hash(&boolean));
}

#[test]
fn the_whole_logos_variant_is_outside_the_pure_content_hash() {
    let whole = WholeLogos::new(vec![newtype(&[1, 9], &[3])]);
    let canonical_bytes = whole.to_archive_bytes().expect("canonical whole content");
    let mut oracle = IdentityHasher::unprimed();
    oracle.update_length_prefixed(&canonical_bytes);
    let expected = oracle.finalize_bytes();

    let identity = whole.content_identity().expect("whole content identity");
    assert!(matches!(identity, WholeLogosContentIdentity::WholeLogos(_)));
    assert_eq!(identity.content_addressed_hash().bytes(), &expected);
    assert_eq!(expected, WHOLE_LOGOS_ARCHIVE_V1_IDENTITY);
}

#[test]
fn malformed_archives_are_refused_before_a_carrier_is_returned() {
    let whole = WholeLogos::new(vec![newtype(&[1, 9], &[3])]);
    let bytes = whole.to_archive_bytes().expect("archive whole content");

    assert!(WholeLogos::from_archive_bytes(b"not a whole Logos archive").is_err());
    assert!(WholeLogos::from_archive_bytes(&bytes[..bytes.len() - 1]).is_err());
}

#[test]
fn portable_archive_and_inherent_archive_surfaces_are_identical() {
    let whole = WholeLogos::new(vec![newtype(&[2, 6, 18], &[3])]);
    let direct = whole.to_archive_bytes().expect("inherent archive");
    let shared =
        <WholeLogos as PortableArchive>::to_archive_bytes(&whole).expect("shared portable archive");

    assert_eq!(direct, shared.as_ref());
}
