use capsule_content_identity::{IdentityHasher, PortableArchive};
use core_logos::{
    WholeLogos, WholeLogosContentIdentity, WholeLogosEnumeration, WholeLogosItem,
    WholeLogosNewtype, WholeLogosTable, WholeLogosTupleFields, WholeLogosTypeApplication,
    WholeLogosTypeAttributes, WholeLogosTypeReference, WholeLogosVariant, WholeLogosVariantPayload,
    WholeLogosVisibility,
};
use encoded_name_table::LocalEncodedId;
use signal_sema_translator::{VocabularyEncodedId, VocabularyRoot};

// Version 5 adds Sema table declarations and stored-value attributes. An
// intentional archive change replaces this under a new versioned name.
const WHOLE_LOGOS_ARCHIVE_V5_IDENTITY: [u8; 32] = [
    0xc6, 0xe1, 0xd3, 0x03, 0x44, 0x5e, 0x3f, 0x31, 0x40, 0x23, 0xfa, 0x2b, 0xf7, 0xa0, 0x12, 0x4d,
    0x92, 0xa4, 0x99, 0x6c, 0x45, 0xd4, 0x49, 0xd0, 0xe9, 0x98, 0xe9, 0x78, 0x4f, 0xf6, 0xe5, 0x89,
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
        WholeLogosTypeReference::Identity(encoded_id(VocabularyRoot::Universal, wrapped)),
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
fn enum_and_application_shapes_retain_every_complete_chain_through_archive() {
    let vector = encoded_id(VocabularyRoot::Rust, &[4]);
    let integer = encoded_id(VocabularyRoot::Rust, &[3]);
    let application = WholeLogosTypeReference::Application(WholeLogosTypeApplication::new(
        vector.clone(),
        WholeLogosTypeReference::Identity(integer.clone()),
    ));
    let original = WholeLogos::new(vec![
        WholeLogosItem::Newtype(WholeLogosNewtype::new(
            WholeLogosVisibility::Public,
            encoded_id(VocabularyRoot::Universal, &[8, 5]),
            WholeLogosVisibility::Private,
            application.clone(),
        )),
        WholeLogosItem::Enumeration(WholeLogosEnumeration::new(
            WholeLogosVisibility::Public,
            encoded_id(VocabularyRoot::Universal, &[8, 6]),
            vec![
                WholeLogosVariant::new(
                    encoded_id(VocabularyRoot::Universal, &[8, 6, 1]),
                    WholeLogosVariantPayload::Unit,
                ),
                WholeLogosVariant::new(
                    encoded_id(VocabularyRoot::Universal, &[8, 6, 2]),
                    WholeLogosVariantPayload::Tuple(
                        WholeLogosTupleFields::new(vec![application]).expect("single-field tuple"),
                    ),
                ),
            ],
        )),
    ]);

    let archive = original
        .to_archive_bytes()
        .expect("archive broadened carrier");
    assert_eq!(
        WholeLogos::from_archive_bytes(&archive).expect("restore broadened carrier"),
        original
    );
}

#[test]
fn rust_vocabulary_references_survive_archive_while_declarations_stay_universal() {
    let rust_integer = encoded_id(VocabularyRoot::Rust, &[3]);
    let original = WholeLogos::new(vec![WholeLogosItem::Newtype(WholeLogosNewtype::new(
        WholeLogosVisibility::Public,
        encoded_id(VocabularyRoot::Universal, &[8, 5]),
        WholeLogosVisibility::Private,
        WholeLogosTypeReference::Identity(rust_integer.clone()),
    ))]);
    let archive = original.to_archive_bytes().expect("archive Rust reference");
    assert_eq!(
        WholeLogos::from_archive_bytes(&archive).expect("restore Rust reference"),
        original
    );

    let invalid_declaration =
        WholeLogos::new(vec![WholeLogosItem::Newtype(WholeLogosNewtype::new(
            WholeLogosVisibility::Public,
            encoded_id(VocabularyRoot::Rust, &[8, 5]),
            WholeLogosVisibility::Private,
            WholeLogosTypeReference::Identity(rust_integer),
        ))]);
    let invalid_archive = invalid_declaration
        .to_archive_bytes()
        .expect("archive invalid declaration fixture");
    assert!(matches!(
        WholeLogos::from_archive_bytes(&invalid_archive),
        Err(core_logos::WholeLogosArchiveError::NonUniversalEncodedId {
            position: core_logos::WholeLogosEncodedIdPosition::ItemName,
            root: VocabularyRoot::Rust,
            ..
        })
    ));
}

#[test]
fn ordered_typed_items_and_complete_encoded_id_chains_survive_archive() {
    let original = WholeLogos::new(vec![newtype(&[17, 23, 41], &[3, 5])]);
    let bytes = original.to_archive_bytes().expect("archive whole Logos");
    let restored = WholeLogos::from_archive_bytes(&bytes).expect("restore whole Logos");

    assert_eq!(restored, original);
    let WholeLogosItem::Newtype(newtype) = &restored.items()[0] else {
        panic!("newtype fixture")
    };
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
        match newtype.wrapped() {
            WholeLogosTypeReference::Identity(identity) => identity,
            WholeLogosTypeReference::Application(_) => panic!("identity reference"),
        }
        .chain()
        .iter()
        .map(|local| local.value())
        .collect::<Vec<_>>(),
        vec![3, 5],
    );
    assert_eq!(newtype.name().root_variant(), &VocabularyRoot::Universal);
    assert_eq!(
        match newtype.wrapped() {
            WholeLogosTypeReference::Identity(identity) => identity.root_variant(),
            WholeLogosTypeReference::Application(_) => panic!("identity reference"),
        },
        &VocabularyRoot::Universal
    );
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
    assert_eq!(expected, WHOLE_LOGOS_ARCHIVE_V5_IDENTITY);
}

#[test]
fn tuple_variants_refuse_zero_and_multiple_fields_without_rewriting_payloads() {
    assert_eq!(
        WholeLogosTupleFields::new(Vec::new())
            .expect_err("unit payload has its own variant")
            .found(),
        0,
    );
    let error = WholeLogosTupleFields::new(vec![
        WholeLogosTypeReference::Identity(encoded_id(VocabularyRoot::Universal, &[1])),
        WholeLogosTypeReference::Identity(encoded_id(VocabularyRoot::Universal, &[2])),
    ])
    .expect_err("multi-field data requires a named struct payload");
    assert_eq!(error.found(), 2);
}

#[test]
fn type_attribute_policy_is_canonical_content() {
    let plain = newtype(&[1, 1], &[7]);
    let WholeLogosItem::Newtype(newtype) = plain.clone() else {
        panic!("newtype fixture")
    };
    let wire = WholeLogosItem::Newtype(newtype.with_attributes(WholeLogosTypeAttributes::Wire));

    assert_ne!(
        hash(&WholeLogos::new(vec![plain])),
        hash(&WholeLogos::new(vec![wire]))
    );
}

#[test]
fn sema_table_record_and_key_shape_survive_archive_and_move_schema_identity() {
    let name = encoded_id(VocabularyRoot::Universal, &[31]);
    let record = WholeLogosTypeReference::Identity(encoded_id(VocabularyRoot::Universal, &[32]));
    let domain_key =
        WholeLogosTypeReference::Identity(encoded_id(VocabularyRoot::Universal, &[33]));
    let identifier_key =
        WholeLogosTypeReference::Identity(encoded_id(VocabularyRoot::Universal, &[34]));
    let table = WholeLogosTable::new(name.clone(), record.clone(), domain_key);
    let changed = WholeLogosTable::new(name, record, identifier_key);
    let whole = WholeLogos::new(vec![WholeLogosItem::Table(table.clone())]);

    let archive = whole.to_archive_bytes().expect("archive Sema table");
    assert_eq!(
        WholeLogos::from_archive_bytes(&archive).expect("restore Sema table"),
        whole,
    );
    assert_ne!(
        table.schema_hash().expect("table schema hash"),
        changed.schema_hash().expect("changed table schema hash"),
    );
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
